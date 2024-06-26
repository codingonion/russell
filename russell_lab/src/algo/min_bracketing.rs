use super::{Bracket, Stats, UNINITIALIZED};
use crate::StrError;

/// Implements algorithms for bracketing a local minimum of f(x)
#[derive(Clone, Copy, Debug)]
pub struct MinBracketing {
    /// Max number of iterations
    ///
    /// ```text
    /// n_iteration_max ≥ 2
    /// ```
    ///
    /// e.g., 100
    pub n_iteration_max: usize,

    /// Initial step
    ///
    /// e.g., 1e-2
    pub initial_step: f64,

    /// Step expansion factor
    ///
    /// e.g., 2.0
    pub expansion_factor: f64,
}

impl MinBracketing {
    /// Allocates a new instance with default parameters
    pub fn new() -> Self {
        MinBracketing {
            n_iteration_max: 100,
            initial_step: 1e-2,
            expansion_factor: 2.0,
        }
    }

    /// Validates the parameters
    fn validate_params(&self) -> Result<(), StrError> {
        if self.n_iteration_max < 2 {
            return Err("n_iteration_max must be ≥ 2");
        }
        Ok(())
    }

    /// Employs a basic algorithm to try to bracket the minimum of f(x)
    ///
    /// **Note:** This function is suitable for *unimodal functions*---it may fail otherwise.
    /// The code is based on the one presented in Chapter 3 (page 36) of the Reference.
    ///
    /// Searches (iteratively) for `a`, `b` and `xo` such that:
    ///
    /// ```text
    /// f(xo) < f(a)  and  f(xo) < f(b)
    ///
    /// with a < xo < b
    /// ```
    ///
    /// Thus, `f(xo)` is the minimum of `f(x)` in the `[a, b]` interval.
    ///
    /// # Input
    ///
    /// * `x_guess` -- a starting guess
    /// * `args` -- extra arguments for the callback function
    /// * `f` -- is the callback function implementing `f(x)` as `f(x, args)`; it returns `f @ x` or it may return an error.
    ///
    /// # Output
    ///
    /// Returns `(bracket, stats)` where:
    ///
    /// * `bracket` -- holds the results
    /// * `stats` -- holds statistics about the computations
    ///
    /// # Reference
    ///
    /// * Kochenderfer MJ and Wheeler TA (2019) Algorithms for Optimization, The MIT Press, 512p
    ///
    /// # Examples
    ///
    /// ![004](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/test_function_004.svg)
    ///
    /// ```
    /// use russell_lab::*;
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // "4: f(x) = (x - 1)² + 5 sin(x)"
    ///     let f = |x: f64, _: &mut NoArgs| Ok(f64::powi(x - 1.0, 2) + 5.0 * f64::sin(x));
    ///     let args = &mut 0;
    ///
    ///     // bracketing
    ///     let bracketing = MinBracketing::new();
    ///     let (bracket, stats) = bracketing.basic(-3.0, args, f)?;
    ///     println!("\n(a, b) = ({}, {})", bracket.a, bracket.b);
    ///     println!("\n{}", stats);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The output looks like:
    ///
    /// ```text
    /// (a, b) = (-1.7200000000000002, 2.12)
    ///
    /// Number of function evaluations   = 11
    /// Number of Jacobian evaluations   = 0
    /// Number of iterations             = 9
    /// Error estimate                   = unavailable
    /// Total computation time           = 7.293µs
    /// ```
    pub fn basic<F, A>(&self, x_guess: f64, args: &mut A, mut f: F) -> Result<(Bracket, Stats), StrError>
    where
        F: FnMut(f64, &mut A) -> Result<f64, StrError>,
    {
        // validate parameters
        self.validate_params()?;

        // allocate stats struct
        let mut stats = Stats::new();

        // initialization
        let mut step = self.initial_step;
        let (mut a, mut xo) = (x_guess, x_guess + step);
        let (mut fa, mut fxo) = (f(a, args)?, f(xo, args)?);
        stats.n_function += 2;

        // swap values (make sure to go "downhill")
        if fxo > fa {
            swap(&mut a, &mut xo);
            swap(&mut fa, &mut fxo);
            step = -step;
        }

        // iterations
        let mut converged = false;
        let mut b = UNINITIALIZED;
        let mut fb = UNINITIALIZED;
        for _ in 0..self.n_iteration_max {
            stats.n_iterations += 1;
            stats.n_function += 1;
            b = xo + step;
            fb = f(b, args)?;
            if fb > fxo {
                converged = true;
                break;
            }
            a = xo;
            fa = fxo;
            xo = b;
            fxo = fb;
            step *= self.expansion_factor;
        }

        // check
        if !converged {
            return Err("try_bracket_min failed to converge");
        }

        // done
        if a > b {
            swap(&mut a, &mut b);
            swap(&mut fa, &mut fb);
        }
        stats.stop_sw_total();
        Ok((Bracket { a, b, fa, fb, xo, fxo }, stats))
    }
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
    use super::{swap, Bracket, MinBracketing};
    use crate::algo::testing::get_test_functions;
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
    fn validate_params_works() {
        let mut solver = MinBracketing::new();
        solver.n_iteration_max = 0;
        assert_eq!(solver.validate_params().err(), Some("n_iteration_max must be ≥ 2"));
    }

    #[test]
    fn basic_captures_errors_1() {
        let f = |x, _: &mut NoArgs| Ok(x * x - 1.0);
        let args = &mut 0;
        assert_eq!(f(1.0, args).unwrap(), 0.0);
        let mut solver = MinBracketing::new();
        solver.n_iteration_max = 0;
        assert_eq!(solver.basic(0.0, args, f).err(), Some("n_iteration_max must be ≥ 2"));
    }

    #[test]
    fn basic_captures_errors_2() {
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
        let solver = MinBracketing::new();
        // first function call
        assert_eq!(solver.basic(0.0, args, f).err(), Some("stop"));
        // second function call
        args.count = 0;
        args.target = 1;
        assert_eq!(solver.basic(0.0, args, f).err(), Some("stop"));
        // third function call
        args.count = 0;
        args.target = 2;
        assert_eq!(solver.basic(0.0, args, f).err(), Some("stop"));
    }

    fn check_consistency(bracket: &Bracket) {
        assert!(bracket.a < bracket.xo);
        assert!(bracket.xo < bracket.b);
        assert!(bracket.fa > bracket.fxo);
        assert!(bracket.fb > bracket.fxo);
    }

    #[test]
    fn basic_works_1() {
        let args = &mut 0;
        let solver = MinBracketing::new();
        for (i, test) in get_test_functions().iter().enumerate() {
            if test.min1.is_none() {
                continue;
            }
            println!("\n===================================================================");
            println!("\n{}", test.name);
            let x_guess = if i == 4 {
                0.15
            } else {
                if i % 2 == 0 {
                    -0.1
                } else {
                    0.1
                }
            };
            let (bracket, stats) = solver.basic(x_guess, args, test.f).unwrap();
            println!("\n{}", bracket);
            println!("\n{}", stats);
            check_consistency(&bracket);
            approx_eq((test.f)(bracket.a, args).unwrap(), bracket.fa, 1e-15);
            approx_eq((test.f)(bracket.b, args).unwrap(), bracket.fb, 1e-15);
            approx_eq((test.f)(bracket.xo, args).unwrap(), bracket.fxo, 1e-15);
        }
        println!("\n===================================================================\n");
    }

    #[test]
    fn basic_fails_on_non_converged() {
        let f = |x, _: &mut NoArgs| Ok(f64::powi(x - 1.0, 2) + 5.0 * f64::sin(x));
        let args = &mut 0;
        assert!(f(1.0, args).unwrap() > 0.0);
        let mut solver = MinBracketing::new();
        solver.n_iteration_max = 2;
        assert_eq!(
            solver.basic(0.0, args, f).err(),
            Some("try_bracket_min failed to converge")
        );
    }
}
