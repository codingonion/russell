use super::{AlgoStats, UNINITIALIZED};
use crate::StrError;

/// Holds parameters for a bracket algorithm
#[derive(Clone, Copy, Debug)]
pub struct BracketMinParams {
    /// Max number of iterations
    ///
    /// ```text
    /// n_iteration_max ≥ 2
    /// ```
    pub n_iteration_max: usize,

    /// Initial step
    ///
    /// e.g., 1e-2
    pub initial_step: f64,

    /// Step expansion factor
    ///
    /// e.g., 2.0
    pub expansion_factor: f64,

    /// Uses a nonlinear step
    pub nonlinear_step: bool,
}

impl BracketMinParams {
    /// Allocates a new instance
    pub fn new() -> Self {
        BracketMinParams {
            n_iteration_max: 20,
            initial_step: 1e-2,
            expansion_factor: 2.0,
            nonlinear_step: true,
        }
    }

    /// Validates the parameters
    pub fn validate(&self) -> Result<(), StrError> {
        if self.n_iteration_max < 2 {
            return Err("n_iteration_max must be ≥ 2");
        }
        Ok(())
    }
}

/// Holds the results of a root or minimum bracketing algorithm
///
/// The goal is to bracket a root or minimum of a function.
///
///	A root of a function is bracketed by a pair of points, `a` and `b`, such that
/// the function has opposite sign at those two points, i.e., `f(a) * f(b) < 0.0`
///
///	A minimum is bracketed by a triple of points `a`, `b`, and `c`, such that
/// `f(b) < f(a)` and `f(b) < f(c)`, with `a < b < c`.
#[derive(Clone, Copy, Debug)]
pub struct BracketMin {
    /// Holds the point `a`
    pub a: f64,

    /// Holds the point `b`
    pub b: f64,

    /// Holds the point `c`
    pub c: f64,

    /// Holds the function evaluation `f(x=a)`
    pub fa: f64,

    /// Holds the function evaluation `f(x=b)`
    pub fb: f64,

    /// Holds the function evaluation `f(x=c)`
    pub fc: f64,
}

/// Tries to bracket the minimum of f(x)
///
/// **Note:** This function is suitable for *unimodal functions*---it may fail otherwise.
/// The code is based on the one presented in Chapter 3 (page 36) of the Reference.
///
/// Searches (iteratively) for `a`, `b` and `c` such that:
///
/// ```text
/// f(b) < f(a)  and  f(b) < f(c)
///
/// with a < b < c
/// ```
///
/// I.e., `f(b)` is the minimum in the `[a, b]` interval.
///
/// # Input
///
/// `x_guess` -- a starting guess
/// `params` -- Optional parameters
///
/// # Output
///
/// Returns `(bracket, stats)` where:
///
/// * `bracket` -- the `a`, `b`, and `c` points and the functions evaluated at those points.
/// * `stats` -- some statistics about the computations
///
/// # Reference
///
/// * Kochenderfer MJ and Wheeler TA (2019) Algorithms for Optimization, The MIT Press, 512p
pub fn try_bracket_min<F, A>(
    x_guess: f64,
    params: Option<BracketMinParams>,
    args: &mut A,
    mut f: F,
) -> Result<(BracketMin, AlgoStats), StrError>
where
    F: FnMut(f64, &mut A) -> Result<f64, StrError>,
{
    // parameters
    let par = match params {
        Some(p) => p,
        None => BracketMinParams::new(),
    };
    par.validate()?;

    // allocate stats struct
    let mut stats = AlgoStats::new();

    // initialization
    let mut step = par.initial_step;
    let (mut a, mut b) = (x_guess, x_guess + step);
    let (mut fa, mut fb) = (f(a, args)?, f(b, args)?);
    stats.n_function += 2;

    // swap values (make sure to go "downhill")
    if fb > fa {
        swap(&mut a, &mut b);
        swap(&mut fa, &mut fb);
        step = -step;
    }

    // iterations
    let mut converged = false;
    let mut c = UNINITIALIZED;
    let mut fc = UNINITIALIZED;
    for k in 0..par.n_iteration_max {
        stats.n_iterations += 1;
        stats.n_function += 1;
        c = b + step;
        fc = f(c, args)?;
        if fc > fb {
            converged = true;
            break;
        }
        a = b;
        b = c;
        fa = fb;
        fb = fc;
        if par.nonlinear_step {
            step *= par.expansion_factor * f64::powf(2.0, k as f64);
        } else {
            step *= par.expansion_factor;
        }
    }

    // check
    if !converged {
        return Err("try_bracket_min failed to converge");
    }

    // done
    if a > c {
        swap(&mut a, &mut c);
        swap(&mut fa, &mut fc);
    }
    Ok((BracketMin { a, b, c, fa, fb, fc }, stats))
}

/// Swaps two numbers
#[inline]
pub(super) fn swap(a: &mut f64, b: &mut f64) {
    let a_copy = a.clone();
    *a = *b;
    *b = a_copy;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{swap, try_bracket_min, BracketMin, BracketMinParams};
    use crate::algo::testing::get_functions;
    use crate::algo::NoArgs;
    use crate::approx_eq;

    #[test]
    fn swap_works() {
        let mut a = 12.34;
        let mut b = 56.78;
        swap(&mut a, &mut b);
        assert_eq!(a, 56.78);
        assert_eq!(b, 12.34);
    }

    #[test]
    fn params_validate_captures_errors() {
        let mut params = BracketMinParams::new();
        params.n_iteration_max = 0;
        assert_eq!(params.validate().err(), Some("n_iteration_max must be ≥ 2"));
    }

    #[test]
    fn try_bracket_min_captures_errors_1() {
        let f = |x, _: &mut NoArgs| Ok(x * x - 1.0);
        let args = &mut 0;
        assert_eq!(f(1.0, args).unwrap(), 0.0);
        let mut params = BracketMinParams::new();
        params.n_iteration_max = 0;
        assert_eq!(
            try_bracket_min(0.0, Some(params), args, f).err(),
            Some("n_iteration_max must be ≥ 2")
        );
    }

    #[test]
    fn try_bracket_min_captures_errors_2() {
        struct Args {
            count: usize,
            target: usize,
        }
        let f = |x, args: &mut Args| {
            let res = if args.count == args.target {
                Err("stop")
            } else {
                Ok(x * x - 1.0)
            };
            args.count += 1;
            res
        };
        let args = &mut Args { count: 0, target: 0 };
        // first function call
        assert_eq!(try_bracket_min(0.0, None, args, f).err(), Some("stop"));
        // second function call
        args.count = 0;
        args.target = 1;
        assert_eq!(try_bracket_min(0.0, None, args, f).err(), Some("stop"));
        // third function call
        args.count = 0;
        args.target = 2;
        assert_eq!(try_bracket_min(0.0, None, args, f).err(), Some("stop"));
    }

    fn check_consistency(bracket: &BracketMin) {
        assert!(bracket.a < bracket.b);
        assert!(bracket.b < bracket.c);
        assert!(bracket.fa > bracket.fb);
        assert!(bracket.fc > bracket.fb);
    }

    #[test]
    fn try_bracket_min_works_1() {
        let args = &mut 0;
        for (i, test) in get_functions().iter().enumerate() {
            println!("\n\n========================================================================================");
            println!("\n{}", test.name);
            let x_guess = if i % 2 == 0 { -0.1 } else { 0.1 };
            let (bracket, _stats) = try_bracket_min(x_guess, None, args, test.f).unwrap();
            println!("\n{:?}", _stats);
            println!("\n{:?}", bracket);
            check_consistency(&bracket);
            approx_eq((test.f)(bracket.a, args).unwrap(), bracket.fa, 1e-15);
            approx_eq((test.f)(bracket.b, args).unwrap(), bracket.fb, 1e-15);
            approx_eq((test.f)(bracket.c, args).unwrap(), bracket.fc, 1e-15);
        }
    }

    #[test]
    fn try_bracket_min_fails_on_non_converged() {
        let f = |x, _: &mut NoArgs| Ok(f64::powi(x - 1.0, 2) + 5.0 * f64::sin(x));
        let args = &mut 0;
        assert!(f(1.0, args).unwrap() > 0.0);
        let mut params = BracketMinParams::new();
        params.n_iteration_max = 2;
        params.nonlinear_step = false;
        assert_eq!(
            try_bracket_min(0.0, Some(params), args, f).err(),
            Some("try_bracket_min failed to converge")
        );
    }
}
