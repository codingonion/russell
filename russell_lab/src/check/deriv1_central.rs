use crate::StrError;

/// Initial stepsize h for deriv_central5
const STEPSIZE_CENTRAL5: f64 = 1e-3;

/// Approximates the first derivative using central differences with 5 points (with errors)
///
/// Given `f(x)`, approximate:
///
/// ```text
/// df │   
/// —— │   
/// dx │x=at_x
/// ```
///
/// # Input
///
/// * `at_x` -- location for the derivative of f(x, {arguments}) w.r.t x
/// * `f` -- function f(x, {arguments})
/// * `args` -- extra arguments for f(x, {arguments})
/// * `h` -- stepsize (1e-3 recommended)
///
/// **IMPORTANT:** The function is evaluated in [at_x-h, at_x+h].
///
/// # Output
///
/// Returns the triple (dfdx, abs_trunc_err, abs_round_err) where:
///
/// * `dfdx` -- numerical derivative of f(x) w.r.t x @ x
/// * `abs_trunc_err` -- estimated truncation error O(h²)
/// * `abs_round_err` -- rounding error due to cancellations
///
/// # Notes
///
/// * Computes the derivative using the 5-point rule (at_x-h, at_x-h/2, at_x, at_x+h/2, at_x+h)
fn deriv1_and_errors_central5<F, A>(at_x: f64, args: &mut A, h: f64, mut f: F) -> Result<(f64, f64, f64), StrError>
where
    F: FnMut(f64, &mut A) -> Result<f64, StrError>,
{
    // numerical derivative
    let fm1 = f(at_x - h, args)?;
    let fp1 = f(at_x + h, args)?;
    let fmh = f(at_x - h / 2.0, args)?;
    let fph = f(at_x + h / 2.0, args)?;
    let r3 = 0.5 * (fp1 - fm1);
    let r5 = (4.0 / 3.0) * (fph - fmh) - (1.0 / 3.0) * r3;
    let dfdx = r5 / h;

    // error estimation
    let e3 = (f64::abs(fp1) + f64::abs(fm1)) * f64::EPSILON;
    let e5 = 2.0 * (f64::abs(fph) + f64::abs(fmh)) * f64::EPSILON + e3;
    let dy = f64::max(f64::abs(r3 / h), f64::abs(r5 / h)) * (f64::abs(at_x) / h) * f64::EPSILON;
    let abs_trunc_err = f64::abs((r5 - r3) / h);
    let abs_round_err = f64::abs(e5 / h) + dy;

    // results
    Ok((dfdx, abs_trunc_err, abs_round_err))
}

/// Approximates the first derivative using central differences with 5 points
///
/// Given `f(x)`, approximate:
///
/// ```text
/// df │   
/// —— │   
/// dx │x=at_x
/// ```
///
/// # Input
///
/// * `at_x` -- location for the derivative of f(x, {arguments}) w.r.t x
/// * `f` -- function f(x, {arguments})
/// * `args` -- extra arguments for f(x, {arguments})
///
/// **IMPORTANT:** The function is evaluated around at_x (with a small tolerance).
///
/// # Output
///
/// * `dfdx` -- numerical derivative of f(x) w.r.t x @ x
///
/// # Notes
///
/// * Computes the derivative using the 5-point rule (at_x-h, at_x-h/2, at_x, at_x+h/2, at_x+h)
/// * A pre-selected stepsize is scaled based on error estimates
///
/// # Examples
///
/// ```
/// use russell_lab::{deriv1_central5, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // arguments
///     struct Args {}
///     let args = &mut Args {};
///
///     // function
///     let f = |x: f64, _: &mut Args| Ok(f64::exp(-2.0 * x));
///
///     // numerical derivative
///     let at_x = 1.0;
///     let num = deriv1_central5(at_x, args, f)?;
///
///     // check
///     let ana = -2.0 * f64::exp(-2.0 * at_x);
///     assert!(f64::abs(num - ana) < 1e-11);
///     Ok(())
/// }
/// ```
pub fn deriv1_central5<F, A>(at_x: f64, args: &mut A, mut f: F) -> Result<f64, StrError>
where
    F: FnMut(f64, &mut A) -> Result<f64, StrError>,
{
    // trial derivative
    let h = STEPSIZE_CENTRAL5;
    let (dfdx, err, rerr) = deriv1_and_errors_central5(at_x, args, h, &mut f)?;
    let err_total = err + rerr;

    // done with zero-error
    if err == 0.0 || rerr == 0.0 {
        return Ok(dfdx);
    }

    // done with very small truncation error
    if err < rerr {
        return Ok(dfdx);
    }

    // improved derivative
    let h_improv = h * f64::powf(rerr / (2.0 * err), 1.0 / 3.0);
    let (dfdx_improv, err_improv, rerr_improv) = deriv1_and_errors_central5(at_x, args, h_improv, &mut f)?;
    let err_total_improv = err_improv + rerr_improv;

    // ignore improved estimate because of larger error
    if err_total_improv > err_total {
        return Ok(dfdx);
    }

    // ignore improved estimate because of out-of-bounds value
    if f64::abs(dfdx_improv - dfdx) > 4.0 * err_total {
        return Ok(dfdx);
    }

    // return improved derivative
    Ok(dfdx_improv)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{deriv1_and_errors_central5, deriv1_central5};
    use crate::check::testing;

    #[test]
    fn deriv1_and_errors_central5_works() {
        let tests = testing::get_functions();
        println!(
            "{:>10}{:>15}{:>22}{:>11}{:>10}{:>10}",
            "function", "numerical", "analytical", "|num-ana|", "err", "rerr"
        );
        for test in &tests {
            let args = &mut 0;
            let (d, err, rerr) = deriv1_and_errors_central5(test.at_x, args, 1e-3, test.f).unwrap();
            let d_correct = (test.g)(test.at_x, args).unwrap();
            println!(
                "{:>10}{:15.9}{:22}{:11.2e}{:10.2e}{:10.2e}",
                test.name,
                d,
                d_correct,
                f64::abs(d - d_correct),
                err,
                rerr,
            );
            assert!(f64::abs(d - d_correct) < test.tol_g);
            assert!(err < test.tol_g_err);
            assert!(rerr < test.tol_g_rerr);
        }
    }

    #[test]
    fn deriv1_central5_works() {
        let tests = testing::get_functions();
        println!(
            "{:>10}{:>15}{:>22}{:>11}",
            "function", "numerical", "analytical", "|num-ana|"
        );
        // for test in &[&tests[2]] {
        for test in &tests {
            let args = &mut 0;
            let d = deriv1_central5(test.at_x, args, test.f).unwrap();
            let d_correct = (test.g)(test.at_x, args).unwrap();
            println!(
                "{:>10}{:15.9}{:22}{:11.2e}",
                test.name,
                d,
                d_correct,
                f64::abs(d - d_correct),
            );
            assert!(f64::abs(d - d_correct) < test.improv_tol_g_diff);
        }
    }
}
