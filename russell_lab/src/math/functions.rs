/// Calculates negative one raised to the power of n  
///
/// Calculates:
///
/// ```text
///           ⎧  1  if n is even
/// (-1)ⁿ = = ⎨
///           ⎩ -1  if n is odd
/// ```
///
/// {1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1}
///
/// # Examples
///
/// ```
/// use russell_lab::math;
///
/// let v: Vec<_> = (0..11).into_iter().map(|n| math::neg_one_pow_n(n)).collect();
/// assert_eq!(&v, &[1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0]);
/// ```
#[inline]
pub fn neg_one_pow_n(n: i32) -> f64 {
    // this formula works for negative integers as well, e.g.,
    // Mathematica: Simplify[(-1)^n == (-1)^-n, Assumptions -> {n \[Element] Integers}]
    //   True
    if n & 1 == 0 {
        // even
        1.0
    } else {
        // odd
        -1.0
    }
}

/// Evaluates the sign function
///
/// ```text
///           ⎧ -1   if x < 0
/// sign(x) = ⎨  0   if x = 0
///           ⎩  1   if x > 0
///
///           |x|    x
/// sign(x) = ——— = ———
///            x    |x|
///
/// sign(x) = 2 · heaviside(x) - 1
/// ```
///
/// See: <https://mathworld.wolfram.com/Sign.html>
///
/// See also: <https://en.wikipedia.org/wiki/Sign_function>
///
/// # Examples
///
/// ![sign](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_sign.svg)
///
/// ```
/// use russell_lab::math;
///
/// assert_eq!(math::sign(-0.5), -1.0);
/// assert_eq!(math::sign( 0.0),  0.0);
/// assert_eq!(math::sign( 0.5),  1.0);
/// ```
#[inline]
pub fn sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

/// Evaluates the ramp function (Macaulay brackets)
///
/// ```text
///           ⎧ 0   if x < 0
/// ramp(x) = ⎨
///           ⎩ x   otherwise
///
/// ramp(x) = max(x, 0)
///
///           x + |x|
/// ramp(x) = ———————
///              2
///
/// ramp(x) =〈x〉  (Macaulay brackets)
///
/// ramp(x) = x · heaviside(x)
/// ```
///
/// See: <https://mathworld.wolfram.com/RampFunction.html>
///
/// See also: <https://en.wikipedia.org/wiki/Ramp_function>
///
/// # Examples
///
/// ![ramp](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_ramp.svg)
///
/// ```
/// use russell_lab::math;
///
/// assert_eq!(math::ramp(-0.5), 0.0);
/// assert_eq!(math::ramp( 0.0), 0.0);
/// assert_eq!(math::ramp( 0.5), 0.5);
/// ```
#[inline]
pub fn ramp(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        x
    }
}

/// Evaluates the Heaviside step function (derivative of ramp(x))
///
/// ```text
///                ⎧ 0    if x < 0
/// heaviside(x) = ⎨ 1/2  if x = 0
///                ⎩ 1    if x > 0
///
/// heaviside(x) = ½ + ½ · sign(x)
/// ```
///
/// See: <https://mathworld.wolfram.com/HeavisideStepFunction.html>
///
/// See also: <https://en.wikipedia.org/wiki/Heaviside_step_function>
///
/// # Examples
///
/// ![heaviside](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_heaviside.svg)
///
/// ```
/// use russell_lab::math;
///
/// assert_eq!(math::heaviside(-0.5), 0.0);
/// assert_eq!(math::heaviside( 0.0), 0.5);
/// assert_eq!(math::heaviside( 0.5), 1.0);
/// ```
#[inline]
pub fn heaviside(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 0.0 {
        1.0
    } else {
        0.5
    }
}

/// Evaluates the boxcar function
///
/// ```text
///                   ⎧ 0    if x < a or  x > b
/// boxcar(x; a, b) = ⎨ 1/2  if x = a or  x = b
///                   ⎩ 1    if x > a and x < b
/// with b > a
/// ```
///
/// See <https://mathworld.wolfram.com/BoxcarFunction.html>
///
/// See also: <https://en.wikipedia.org/wiki/Boxcar_function>
///
/// # Panics
///
/// **Warning:** This function panics if the `a ≥ b`.
///
/// # Relation with the Heaviside function
///
/// Note that:
///
/// ```text
/// boxcar(x; a, b) = heaviside(x - a) - heaviside(x - b)
/// ```
///
/// Considering that `b > a`, the difference `H(x - a) - H(x - b)` can be tabulated as follows:
///
/// |             |          | `x < b`         | `x = b`               | `x > b`             |
/// |:-----------:|:--------:|:---------------:|:---------------------:|:-------------------:|
/// |             | `H(x-a)` | `H(x-b)=0 `     | `H(x-b)=1/2 `         | `H(x-b)=1 `         |
/// | **`x < a`** | `0`      | `  0 - 0 = 0`   | impossible            | impossible          |
/// | **`x = a`** | `1/2`    | `1/2 - 0 = 1/2` | `1/2 - 1/2 = 0`       | impossible          |
/// | **`x > a`** | `1`      | `  1 - 0 = 1`   | `  1 - 1/2 = 1/2`     | `1 - 1 = 0`         |
///
/// which corresponds to the definition of the `boxcar(x; a, b)` function given earlier.
///
/// # Examples
///
/// ![boxcar](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_boxcar.svg)
///
/// ```
/// use russell_lab::math;
///
/// let (a, b) = (1.0, 2.0);
/// assert_eq!(math::boxcar(      0.0, a, b), 0.0);
/// assert_eq!(math::boxcar(        a, a, b), 0.5);
/// assert_eq!(math::boxcar((a+b)/2.0, a, b), 1.0);
/// assert_eq!(math::boxcar(        b, a, b), 0.5);
/// assert_eq!(math::boxcar(      3.0, a, b), 0.0);
/// ```
#[inline]
pub fn boxcar(x: f64, a: f64, b: f64) -> f64 {
    if a >= b {
        panic!("b must be greater than a");
    }
    if x < a || x > b {
        0.0
    } else if x > a && x < b {
        1.0
    } else {
        0.5
    }
}

/// Evaluates the standard logistic (sigmoid) function
///
/// ```text
///                   1
/// logistic(x) = ———————————
///               1 + exp(-x)
/// ```
///
/// See: <https://mathworld.wolfram.com/SigmoidFunction.html>
///
/// See also: <https://en.wikipedia.org/wiki/Logistic_function>
///
/// # Examples
///
/// ![logistic](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_logistic.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// approx_eq(math::logistic(-2.0), 0.11920292202211756, 1e-15);
/// approx_eq(math::logistic( 2.0), 0.88079707797788244, 1e-15);
/// ```
#[inline]
pub fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

/// Returns the first derivative of the standard logistic function
///
/// Reference: <https://en.wikipedia.org/wiki/Logistic_function>
///
/// # Examples
///
/// ![logistic_deriv1](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_logistic_deriv1.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// let y = 0.10499358540350652;
/// approx_eq(math::logistic_deriv1(-2.0), y, 1e-15);
/// approx_eq(math::logistic_deriv1( 2.0), y, 1e-15);
/// ```
#[inline]
pub fn logistic_deriv1(x: f64) -> f64 {
    let f = logistic(x);
    f * (1.0 - f)
}

/// Evaluates the smooth ramp function
///
/// ```text
///                  ⎧ 0   if -β·x > 500
///                  │
/// smooth_ramp(x) = ⎨     log(1 + exp(-β·x))
///                  │ x + ——————————————————  otherwise
///                  ⎩            β
/// with β > 0
/// ```
///
/// This function was used in the following paper:
///
/// * Pedroso DM, Zhang Y, and Ehlers W (2017) Solution of liquid-gas-solid coupled equations
///   for porous media considering dynamics and hysteretic retention behavior,
///   [Journal of Engineering Mechanics](https://ascelibrary.org/doi/10.1061/%28ASCE%29EM.1943-7889.0001208)
///
/// # Examples
///
/// ![smooth_ramp](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_smooth_ramp.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// let beta = 10.0;
/// approx_eq(math::smooth_ramp(-2.0, beta), 0.0, 1e-9);
/// approx_eq(math::smooth_ramp( 2.0, beta), 2.0, 1e-9);
/// ```
#[inline]
pub fn smooth_ramp(x: f64, beta: f64) -> f64 {
    if -beta * x > 500.0 {
        return 0.0;
    }
    x + f64::ln(1.0 + f64::exp(-beta * x)) / beta
}

/// Returns the first derivative of smooth_ramp
///
/// See [smooth_ramp()] for further information.
///
/// # Examples
///
/// ![smooth_ramp_deriv1](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_smooth_ramp_deriv1.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// let beta = 10.0;
/// approx_eq(math::smooth_ramp_deriv1(-2.0, beta), 0.0, 1e-8);
/// approx_eq(math::smooth_ramp_deriv1( 2.0, beta), 1.0, 1e-8);
/// ```
#[inline]
pub fn smooth_ramp_deriv1(x: f64, beta: f64) -> f64 {
    if -beta * x > 500.0 {
        return 0.0;
    }
    return 1.0 / (1.0 + f64::exp(-beta * x));
}

/// Returns the second derivative of smooth_ramp
///
/// See [smooth_ramp()] for further information.
///
/// # Examples
///
/// ![smooth_ramp_deriv2](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_smooth_ramp_deriv2.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// let beta = 10.0;
/// approx_eq(math::smooth_ramp_deriv2(-2.0, beta), 0.0, 1e-7);
/// approx_eq(math::smooth_ramp_deriv2( 2.0, beta), 0.0, 1e-7);
/// ```
#[inline]
pub fn smooth_ramp_deriv2(x: f64, beta: f64) -> f64 {
    if beta * x > 500.0 {
        return 0.0;
    }
    beta * f64::exp(beta * x) / f64::powf(f64::exp(beta * x) + 1.0, 2.0)
}

/// Evaluates the superquadric function involving sin(x)
///
/// ```text
/// suq_sin(x;k) = sign(sin(x)) · |sin(x)|ᵏ
/// ```
///
/// `suq_sin(x;k)` is the `f(ω;m)` function from <https://en.wikipedia.org/wiki/Superquadrics>
///
/// # Examples
///
/// ![suq_sin](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_suq_sin.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// approx_eq(math::suq_sin(-math::PI / 2.0, 2.0), -1.0, 1e-15);
/// approx_eq(math::suq_sin(            0.0, 2.0),  0.0, 1e-15);
/// approx_eq(math::suq_sin( math::PI / 2.0, 2.0),  1.0, 1e-15);
/// ```
///
/// This function is useful to plot superquadric surfaces as shown below (see the examples directory).
///
/// ![superquadric](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_superquadric.jpg)
#[inline]
pub fn suq_sin(x: f64, k: f64) -> f64 {
    sign(f64::sin(x)) * f64::powf(f64::abs(f64::sin(x)), k)
}

/// Evaluates the superquadric function involving cos(x)
///
/// ```text
/// suq_cos(x;k) = sign(cos(x)) · |cos(x)|ᵏ
/// ```
///
/// `suq_cos(x;k)` is the `g(ω;m)` function from <https://en.wikipedia.org/wiki/Superquadrics>
///
/// # Examples
///
/// ![suq_cos](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_suq_cos.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// approx_eq(math::suq_cos(-math::PI / 2.0, 2.0), 0.0, 1e-15);
/// approx_eq(math::suq_cos(            0.0, 2.0), 1.0, 1e-15);
/// approx_eq(math::suq_cos( math::PI / 2.0, 2.0), 0.0, 1e-15);
/// ```
///
/// This function is useful to plot superquadric surfaces as shown below (see the examples directory).
///
/// ![superquadric](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_plot_functions_superquadric.jpg)
#[inline]
pub fn suq_cos(x: f64, k: f64) -> f64 {
    sign(f64::cos(x)) * f64::powf(f64::abs(f64::cos(x)), k)
}

/// Holds factorial numbers up to 22! (exact double precision)
const FACTORIAL_22: [f64; 23] = [
    1.0,                      // 0
    1.0,                      // 1
    2.0,                      // 2
    6.0,                      // 3
    24.0,                     // 4
    120.0,                    // 5
    720.0,                    // 6
    5040.0,                   // 7
    40320.0,                  // 8
    362880.0,                 // 9
    3628800.0,                // 10
    39916800.0,               // 11
    479001600.0,              // 12
    6227020800.0,             // 13
    87178291200.0,            // 14
    1307674368000.0,          // 15
    20922789888000.0,         // 16
    355687428096000.0,        // 17
    6402373705728000.0,       // 18
    121645100408832000.0,     // 19
    2432902008176640000.0,    // 20
    51090942171709440000.0,   // 21
    1124000727777607680000.0, // 22
];

/// Returns the factorial of n smaller than or equal to 22 by table lookup
///
/// See: <https://mathworld.wolfram.com/Factorial.html>
///
/// See also: <https://en.wikipedia.org/wiki/Factorial>
///
/// # Panics
///
/// Will panic if n > 22
///
/// # Reference
///
/// According to the reference, factorials up to 22! have exact double precision representations
/// (52 bits of mantissa, not counting powers of two that are absorbed into the exponent)
///
/// * Press WH, Teukolsky SA, Vetterling WT, Flannery BP (2007) Numerical Recipes: The Art of
///   Scientific Computing. Third Edition. Cambridge University Press. 1235p.
///
/// # Examples
///
/// ```
/// use russell_lab::math;
///
/// assert_eq!(math::factorial_lookup_22(0), 1.0);
/// assert_eq!(math::factorial_lookup_22(1), 1.0);
/// assert_eq!(math::factorial_lookup_22(2), 2.0);
/// assert_eq!(math::factorial_lookup_22(3), 6.0);
/// assert_eq!(math::factorial_lookup_22(22), 1_124_000_727_777_607_680_000.0);
/// ```
pub fn factorial_lookup_22(n: usize) -> f64 {
    if n > 22 {
        panic!("factorial_lookup_22 requires n ≤ 22");
    }
    FACTORIAL_22[n]
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, deriv1_approx_eq};
    use std::f64::consts::PI;

    #[test]
    fn neg_one_pow_n_works() {
        let mut n: i32 = -12;
        while n < 12 {
            assert_eq!(neg_one_pow_n(n), f64::powi(-1.0, n));
            n += 1;
        }
    }

    #[test]
    #[should_panic(expected = "b must be greater than a")]
    fn boxcar_panics_on_incorrect_input() {
        boxcar(0.0, 0.0, 0.0);
    }

    #[test]
    fn sign_ramp_heaviside_boxcar_work() {
        let xx = [-2.0, -1.6, -1.2, -0.8, -0.4, 0.0, 0.4, 0.8, 1.2, 1.6, 2.0];
        let (a, b) = (-1.2, 0.4);
        for x in xx {
            let s = sign(x);
            let r = ramp(x);
            let h = heaviside(x);
            let bxc = boxcar(x, a, b);
            if x == 0.0 {
                assert_eq!(s, 0.0);
            } else {
                assert_eq!(s, f64::abs(x) / x);
            }
            assert_eq!(s, 2.0 * h - 1.0);
            assert_eq!(r, f64::max(x, 0.0));
            assert_eq!(r, (x + f64::abs(x)) / 2.0);
            assert_eq!(r, x * h);
            assert_eq!(h, 0.5 + 0.5 * s);
            assert_eq!(bxc, heaviside(x - a) - heaviside(x - b));
        }
    }

    #[test]
    fn logistic_and_deriv_work() {
        struct Arguments {}
        let args = &mut Arguments {};
        let f = |x: f64, _: &mut Arguments| Ok(logistic(x));
        let xx = [-2.0, -1.6, -1.2, -0.8, -0.4, 0.0, 0.4, 0.8, 1.2, 1.6, 2.0];
        for x in xx {
            let l = logistic(x);
            let d = logistic_deriv1(x);
            approx_eq(l, 0.5 + 0.5 * f64::tanh(x / 2.0), 1e-14);
            deriv1_approx_eq(d, x, args, 1e-10, f);
        }
    }

    #[test]
    fn smooth_ramp_and_deriv_work() {
        assert_eq!(smooth_ramp(-1.0, 500.1), 0.0);
        assert_eq!(smooth_ramp(-1.0, 499.9), 0.0);
        assert_eq!(smooth_ramp_deriv1(-1.0, 500.1), 0.0);
        approx_eq(smooth_ramp_deriv1(-1.0, 499.99), 0.0, 1e-15);
        assert_eq!(smooth_ramp_deriv2(1.0, 500.1), 0.0);
        approx_eq(smooth_ramp_deriv2(1.0, 499.99), 0.0, 1e-15);
        let beta = 2.0;
        struct Arguments {
            beta: f64,
        }
        let args = &mut Arguments { beta };
        let f = |x: f64, args: &mut Arguments| Ok(smooth_ramp(x, args.beta));
        let g = |x: f64, args: &mut Arguments| Ok(smooth_ramp_deriv1(x, args.beta));
        let xx = [-2.0, -1.6, -1.2, -0.8, -0.4, 0.0, 0.4, 0.8, 1.2, 1.6, 2.0];
        for x in xx {
            let d = smooth_ramp_deriv1(x, beta);
            let d2 = smooth_ramp_deriv2(x, beta);
            deriv1_approx_eq(d, x, args, 1e-9, f);
            deriv1_approx_eq(d2, x, args, 1e-9, g);
        }
    }

    #[test]
    fn suq_sin_and_cos_work() {
        approx_eq(suq_sin(0.0, 1.0), 0.0, 1e-14);
        approx_eq(suq_sin(PI, 1.0), 0.0, 1e-14);
        approx_eq(suq_sin(PI / 2.0, 0.0), 1.0, 1e-14);
        approx_eq(suq_sin(PI / 2.0, 1.0), 1.0, 1e-14);
        approx_eq(suq_sin(PI / 2.0, 2.0), 1.0, 1e-14);
        approx_eq(suq_sin(PI / 4.0, 2.0), 0.5, 1e-14);
        approx_eq(suq_sin(-PI / 4.0, 2.0), -0.5, 1e-14);

        approx_eq(suq_cos(0.0, 1.0), 1.0, 1e-14);
        approx_eq(suq_cos(PI, 1.0), -1.0, 1e-14);
        approx_eq(suq_cos(PI / 2.0, 0.0), 1.0, 1e-14); // because sign(cos(pi/2))=1
        approx_eq(suq_cos(PI / 2.0, 1.0), 0.0, 1e-14);
        approx_eq(suq_cos(PI / 2.0, 2.0), 0.0, 1e-14);
        approx_eq(suq_cos(PI / 4.0, 2.0), 0.5, 1e-14);
        approx_eq(suq_cos(-PI / 4.0, 2.0), 0.5, 1e-14);
    }

    #[test]
    #[should_panic(expected = "factorial_lookup_22 requires n ≤ 22")]
    fn factorial_lookup_22_captures_error() {
        factorial_lookup_22(23);
    }

    #[test]
    fn factorial_lookup_22_works() {
        assert_eq!(factorial_lookup_22(0), 1.0);
        assert_eq!(factorial_lookup_22(1), 1.0);
        assert_eq!(factorial_lookup_22(2), 2.0);
        assert_eq!(factorial_lookup_22(3), 6.0);
        assert_eq!(factorial_lookup_22(4), 24.0);
        assert_eq!(factorial_lookup_22(10), 3628800.0,);
        assert_eq!(factorial_lookup_22(22), 1124000727_7776076800_00.0);
    }
}
