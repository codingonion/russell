use crate::constants::N_EQUAL_STEPS;
use crate::StrError;
use crate::{EulerForward, ExplicitRungeKutta, Method, NumSolver, OdeParams};
use russell_lab::Vector;

/// Defines the solver for systems of ODEs
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
/// For some solution methods, the Jacobian is defined by:
///
/// ```text
/// d{f}
/// ———— = [J](x, {y})
/// d{y}
/// where [J] is the Jacobian matrix
/// ```
pub struct OdeSolver<'a> {
    /// Holds the parameters
    params: &'a OdeParams,

    /// Dimension of the ODE system
    ndim: usize,

    /// Holds a pointer to the actual ODE system solver
    actual: Box<dyn NumSolver + 'a>,

    /// Scaling vector
    ///
    /// ```text
    /// scaling[i] = abs_tol + rel_tol ⋅ |x[i]|
    /// ```
    scaling: Vector,

    /// Collects the number of steps, successful or not
    n_performed_steps: usize,

    /// Collects the number of rejected steps
    n_rejected_steps: usize,
}

impl<'a> OdeSolver<'a> {
    pub fn new<F>(params: &'a OdeParams, ndim: usize, system: F) -> Result<Self, StrError>
    where
        F: 'a + FnMut(&mut Vector, f64, &Vector) -> Result<(), StrError>,
    {
        params.validate()?;
        let actual: Box<dyn NumSolver> = if params.method == Method::Radau5 {
            panic!("TODO: Radau5");
        } else if params.method == Method::BwEuler {
            panic!("TODO: BwEuler");
        } else if params.method == Method::FwEuler {
            Box::new(EulerForward::new(ndim, system))
        } else {
            Box::new(ExplicitRungeKutta::new(params, ndim, system)?)
        };
        Ok(OdeSolver {
            params,
            ndim,
            actual,
            scaling: Vector::new(ndim),
            n_performed_steps: 0,
            n_rejected_steps: 0,
        })
    }

    /// Solves the ODE system
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
    /// For some solution methods, the Jacobian is defined by:
    ///
    /// ```text
    /// d{f}
    /// ———— = [J](x, {y})
    /// d{y}
    /// where [J] is the Jacobian matrix
    /// ```
    ///
    /// # Input
    ///
    /// * `y0` -- the initial vector of dependent variables; it will be updated to `y1`
    /// * `x0` -- the initial independent variable
    /// * `x1` -- the final independent variable
    /// * `h_equal` -- a constant stepsize for solving with equal-steps; otherwise,
    ///   if possible, variable step sizes are automatically calculated. If automatic
    ///   sub-stepping is not possible (e.g., the RK method is not embedded),
    ///   a constant (and equal) stepsize will be calculated for [N_EQUAL_STEPS] steps.
    pub fn solve<S, D>(
        &mut self,
        y0: &mut Vector,
        x0: f64,
        x1: f64,
        h_equal: Option<f64>,
        mut output_step: S,
        mut _output_dense: D,
    ) -> Result<(), StrError>
    where
        S: FnMut(usize, f64, f64, &Vector) -> Result<bool, StrError>,
        D: FnMut(&mut Vector, f64, usize, f64, f64, &Vector) -> Result<bool, StrError>,
    {
        // check data
        if y0.dim() != self.ndim {
            return Err("y0.dim() must be equal to ndim");
        }
        if x1 < x0 {
            return Err("x1 must be greater than x0");
        }

        // information
        let info = self.params.method.information();

        // initial stepsize
        let (equal_stepping, h) = match h_equal {
            Some(h_eq) => {
                if h_eq < 0.0 {
                    return Err("h_equal must be greater than zero");
                }
                let n = f64::ceil((x1 - x0) / h_eq) as usize;
                let h = (x1 - x0) / (n as f64);
                (true, h)
            }
            None => {
                if info.embedded {
                    let h = f64::min(self.params.initial_stepsize, x1 - x0);
                    (false, h)
                } else {
                    let h = (x1 - x0) / (N_EQUAL_STEPS as f64);
                    (true, h)
                }
            }
        };
        assert!(h > 0.0);

        // mutable x0 (will become x1 at the end)
        let mut x0 = x0;

        // restart variables
        self.actual.initialize();
        self.n_performed_steps = 0;
        self.n_rejected_steps = 0;

        // equal-stepping loop
        if equal_stepping {
            const IGNORED: f64 = 0.0;
            let nstep = f64::ceil((x1 - x0) / h) as usize;
            for step in 0..nstep {
                // step
                self.actual.step(x0, &y0, h)?;
                self.n_performed_steps += 1;

                // update x0
                x0 = ((step + 1) as f64) * h;

                // update y0
                self.actual.accept(y0, x0, h, IGNORED, IGNORED)?;

                // output
                let stop = (output_step)(step, h, x0, y0)?;
                if stop {
                    return Ok(());
                }
            }
            return Ok(());
        }

        // first scaling variables
        for i in 0..self.ndim {
            self.scaling[i] = self.params.abs_tol + self.params.rel_tol * f64::abs(y0[i]);
        }

        // variable steps
        Ok(())
    }
}

pub fn output_step_none(_step: usize, _h: f64, _x: f64, _y: &Vector) -> Result<bool, StrError> {
    Ok(false)
}

pub fn output_dense_none(
    _y_out: &mut Vector,
    _x_out: f64,
    _step: usize,
    _h: f64,
    _x: f64,
    _y: &Vector,
) -> Result<bool, StrError> {
    Ok(false)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{output_dense_none, OdeSolver};
    use crate::{Method, OdeParams};
    use crate::{StrError, N_EQUAL_STEPS};
    use russell_lab::{vec_approx_eq, Vector};

    #[test]
    fn solve_works_1() {
        // solving
        //
        // dy
        // —— = 1   with   y(x=0)=0    thus   y(x) = x
        // dx

        let params = OdeParams::new(Method::FwEuler, None, None);
        let system = |f: &mut Vector, _: f64, _: &Vector| -> Result<(), StrError> {
            f[0] = 1.0;
            Ok(())
        };

        // consistent initial conditions
        let y_ana = |x| x;
        let mut x0 = 0.0;
        let mut y0 = Vector::from(&[y_ana(x0)]);

        // output arrays
        let mut h_values = Vec::new();
        let mut x_values = vec![x0];
        let mut y_values = vec![y0[0]];
        let mut e_values = vec![0.0]; // global errors
        let output_step = |_, h, x, y: &Vector| {
            h_values.push(h);
            x_values.push(x);
            y_values.push(y[0]);
            e_values.push(y_ana(x) - y[0]);
            Ok(false)
        };

        // solve the ODE system
        let mut solver = OdeSolver::new(&params, 1, system).unwrap();
        let xf = 1.0;
        solver
            .solve(&mut y0, x0, xf, None, output_step, output_dense_none)
            .unwrap();

        // check
        assert_eq!(h_values.len(), N_EQUAL_STEPS);
        assert_eq!(x_values.len(), N_EQUAL_STEPS + 1);
        assert_eq!(y_values.len(), N_EQUAL_STEPS + 1);
        assert_eq!(e_values.len(), N_EQUAL_STEPS + 1);
        let h_equal_correct = (xf - x0) / (N_EQUAL_STEPS as f64);
        let h_values_correct = Vector::filled(N_EQUAL_STEPS, h_equal_correct);
        let x_values_correct = Vector::linspace(x0, xf, N_EQUAL_STEPS + 1).unwrap();
        let e_values_correct = Vector::new(N_EQUAL_STEPS + 1); // all 0.0
        vec_approx_eq(&h_values, h_values_correct.as_data(), 1e-17);
        vec_approx_eq(&x_values, x_values_correct.as_data(), 1e-17);
        vec_approx_eq(&y_values, x_values_correct.as_data(), 1e-15);
        vec_approx_eq(&e_values, e_values_correct.as_data(), 1e-15);

        // reset problem
        x0 = 0.0;
        y0[0] = y_ana(x0);
        h_values.clear();
        x_values.clear();
        y_values.clear();
        e_values.clear();
        x_values.push(x0);
        y_values.push(y0[0]);
        e_values.push(0.0);
        let output_step = |_, h, x, y: &Vector| {
            h_values.push(h);
            x_values.push(x);
            y_values.push(y[0]);
            e_values.push(y_ana(x) - y[0]);
            Ok(false)
        };

        // solve the ODE system again with prescribed h_equal
        let h_equal = Some(0.3);
        let xf = 1.2; // => will generate 4 steps
        solver
            .solve(&mut y0, x0, xf, h_equal, output_step, output_dense_none)
            .unwrap();

        // check again
        let nstep = 4;
        let h_values_correct = Vector::filled(nstep, 0.3);
        let x_values_correct = Vector::linspace(x0, xf, nstep + 1).unwrap();
        let e_values_correct = Vector::new(nstep + 1); // all 0.0
        vec_approx_eq(&h_values, h_values_correct.as_data(), 1e-17);
        vec_approx_eq(&x_values, x_values_correct.as_data(), 1e-17);
        vec_approx_eq(&y_values, x_values_correct.as_data(), 1e-15);
        vec_approx_eq(&e_values, e_values_correct.as_data(), 1e-15);
    }
}
