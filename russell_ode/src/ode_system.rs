use crate::StrError;
use russell_lab::Vector;
use russell_sparse::{CooMatrix, Symmetry};
use std::marker::PhantomData;

/// Returns an error to indicate that the Jacobian function is not available
pub fn no_jacobian<A>(
    _jj: &mut CooMatrix,
    _x: f64,
    _y: &Vector,
    _multiplier: f64,
    _args: &mut A,
) -> Result<(), StrError> {
    Err("Jacobian function is not available")
}

/// Defines the system of ordinary differential equations (ODEs)
///
/// The system is defined by:
///
/// ```text
/// d{y}
/// ———— = {f}(x, {y})
///  dx
/// where x is a scalar and {y} and {f} are vectors
/// ```
///
/// The Jacobian is defined by:
///
/// ```text
/// ∂{f}
/// ———— = [J](x, {y})
/// ∂{y}
/// where [J] is the Jacobian matrix
/// ```
pub struct OdeSystem<'a, F, J, A>
where
    F: FnMut(&mut Vector, f64, &Vector, &mut A) -> Result<(), StrError>,
    J: FnMut(&mut CooMatrix, f64, &Vector, f64, &mut A) -> Result<(), StrError>,
{
    /// System dimension
    pub(crate) ndim: usize,

    /// ODE system function
    pub(crate) function: F,

    /// Jacobian function
    pub(crate) jacobian: J,

    /// Use numerical Jacobian
    pub(crate) jac_numerical: bool,

    /// Number of non-zeros in the Jacobian matrix
    pub(crate) jac_nnz: usize,

    /// Symmetry properties of the Jacobian matrix
    pub(crate) jac_symmetry: Option<Symmetry>,

    /// Holds the mass matrix
    pub(crate) mass_matrix: Option<&'a CooMatrix>,

    /// workspace for numerical Jacobian
    work: Vector,

    /// Handle generic argument
    phantom: PhantomData<A>,
}

impl<'a, F, J, A> OdeSystem<'a, F, J, A>
where
    F: FnMut(&mut Vector, f64, &Vector, &mut A) -> Result<(), StrError>,
    J: FnMut(&mut CooMatrix, f64, &Vector, f64, &mut A) -> Result<(), StrError>,
{
    pub fn new(
        ndim: usize,
        function: F,
        jacobian: J,
        jac_numerical: bool,
        jac_nnz: Option<usize>,
        jac_symmetry: Option<Symmetry>,
    ) -> Self
    where
        F: FnMut(&mut Vector, f64, &Vector, &mut A) -> Result<(), StrError>,
    {
        OdeSystem {
            ndim,
            function,
            jacobian,
            jac_numerical,
            jac_nnz: if let Some(n) = jac_nnz { n } else { ndim * ndim },
            jac_symmetry,
            mass_matrix: None,
            work: if jac_numerical {
                Vector::new(ndim)
            } else {
                Vector::new(0)
            },
            phantom: PhantomData,
        }
    }

    pub fn set_mass_matrix(&mut self, mass: &'a CooMatrix) {
        self.mass_matrix = Some(mass);
    }

    /// Computes the numerical Jacobian
    ///
    /// ```text
    /// ∂{f}                          Δfᵢ
    /// ———— = [J](x, {y})      Jᵢⱼ ≈ ———
    /// ∂{y}                          Δyⱼ
    /// ```
    ///
    /// N = ndim - 1
    ///
    /// ```text
    /// Δf0     Δf0     Δf0           Δf0
    /// ———     ———     ———    ···    ———
    /// Δy0     Δy1     Δy2           ΔyN
    ///
    /// Δf1     Δf1     fΔ1           Δf1
    /// ———     ———     ———    ···    ———
    /// Δy0     Δy1     Δy2           ΔyN
    ///
    ///                ···
    ///
    /// ΔfN     ∂fN     ΔfN           ΔfN
    /// ———     ———     ———    ···    ———
    /// Δy0     Δy1     Δy2           ΔyN
    ///
    /// ```
    pub(crate) fn numerical_jacobian(
        &mut self,
        jj: &mut CooMatrix,
        x: f64,
        y: &mut Vector,
        fxy: &Vector,
        multiplier: f64,
        args: &mut A,
    ) -> Result<(), StrError> {
        const THRESHOLD: f64 = 1e-5;
        jj.reset();
        for j in 0..self.ndim {
            let yj_original = y[j]; // create copy
            let delta_yj = f64::sqrt(f64::EPSILON * f64::max(THRESHOLD, f64::abs(y[j])));
            y[j] += delta_yj; // y[j] := y[j] + Δy
            (self.function)(&mut self.work, x, y, args)?; // work := f(x, y + Δy)
            for i in 0..self.ndim {
                let delta_fi = self.work[i] - fxy[i]; // compute Δf[..]
                jj.put(i, j, multiplier * delta_fi / delta_yj).unwrap(); // Δfi/Δfj
            }
            y[j] = yj_original; // restore value
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{no_jacobian, OdeSystem};
    use russell_lab::Vector;
    use russell_sparse::CooMatrix;

    #[test]
    fn ode_system_most_none_works() {
        let mut n_function_eval = 0;
        struct Args {
            more_data_goes_here: bool,
        }
        let mut args = Args {
            more_data_goes_here: false,
        };
        let mut ode = OdeSystem::new(
            2,
            |f, x, y, args: &mut Args| {
                n_function_eval += 1;
                f[0] = -x * y[1];
                f[1] = x * y[0];
                args.more_data_goes_here = true;
                Ok(())
            },
            no_jacobian,
            true,
            None,
            None,
        );
        // call system function
        let x = 0.0;
        let y = Vector::new(2);
        let mut k = Vector::new(2);
        (ode.function)(&mut k, x, &y, &mut args).unwrap();
        // call jacobian function
        let mut jj = CooMatrix::new(2, 2, 2, None, false).unwrap();
        let m = 1.0;
        assert_eq!(
            (ode.jacobian)(&mut jj, x, &y, m, &mut args),
            Err("Jacobian function is not available")
        );
        // check
        println!("n_function_eval = {}", n_function_eval);
        assert_eq!(n_function_eval, 1);
        assert_eq!(args.more_data_goes_here, true);
    }

    #[test]
    fn ode_system_some_none_works() {
        let mass = CooMatrix::new(2, 2, 2, None, false).unwrap();
        let mut n_function_eval = 0;
        let mut n_jacobian_eval = 0;
        struct Args {
            more_data_goes_here_fn: bool,
            more_data_goes_here_jj: bool,
        }
        let mut args = Args {
            more_data_goes_here_fn: false,
            more_data_goes_here_jj: false,
        };
        let mut ode = OdeSystem::new(
            2,
            |f, x, y, args: &mut Args| {
                n_function_eval += 1;
                f[0] = -x * y[1];
                f[1] = x * y[0];
                args.more_data_goes_here_fn = true;
                Ok(())
            },
            |jj, x, _y, _multiplier, args: &mut Args| {
                n_jacobian_eval += 1;
                jj.reset();
                jj.put(0, 1, -x).unwrap();
                jj.put(1, 0, x).unwrap();
                args.more_data_goes_here_jj = true;
                Ok(())
            },
            false,
            Some(2),
            None,
        );
        // ode.set_analytical_solution(Box::new(|y, x| {
        //     y[0] = f64::cos(x * x / 2.0) - 2.0 * f64::sin(x * x / 2.0);
        //     y[1] = 2.0 * f64::cos(x * x / 2.0) + f64::sin(x * x / 2.0);
        // }));
        ode.set_mass_matrix(&mass);
        // call system function
        let x = 0.0;
        let y = Vector::new(2);
        let mut k = Vector::new(2);
        (ode.function)(&mut k, x, &y, &mut args).unwrap();
        // call jacobian function
        let mut jj = CooMatrix::new(2, 2, 2, None, false).unwrap();
        let m = 1.0;
        (ode.jacobian)(&mut jj, x, &y, m, &mut args).unwrap();
        // check
        println!("n_function_eval = {}", n_function_eval);
        println!("n_jacobian_eval = {}", n_jacobian_eval);
        assert_eq!(n_function_eval, 1);
        assert_eq!(n_jacobian_eval, 1);
        assert_eq!(args.more_data_goes_here_fn, true);
        assert_eq!(args.more_data_goes_here_jj, true);
    }
}
