use super::LN2;

// This implementation is based on erfinv.go file from Go (1.22.1),
// which, in turn, is based on the code described below.
//
// Copyright 2017 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Inverse of the floating-point error function.
//
// This implementation is based on the rational approximation
// of percentage points of normal distribution available from
// https://www.jstor.org/stable/2347330.

// Coefficients for approximation to erf in |x| <= 0.85
const A0: f64 = 1.1975323115670912564578e0;
const A1: f64 = 4.7072688112383978012285e1;
const A2: f64 = 6.9706266534389598238465e2;
const A3: f64 = 4.8548868893843886794648e3;
const A4: f64 = 1.6235862515167575384252e4;
const A5: f64 = 2.3782041382114385731252e4;
const A6: f64 = 1.1819493347062294404278e4;
const A7: f64 = 8.8709406962545514830200e2;
const B0: f64 = 1.0000000000000000000e0;
const B1: f64 = 4.2313330701600911252e1;
const B2: f64 = 6.8718700749205790830e2;
const B3: f64 = 5.3941960214247511077e3;
const B4: f64 = 2.1213794301586595867e4;
const B5: f64 = 3.9307895800092710610e4;
const B6: f64 = 2.8729085735721942674e4;
const B7: f64 = 5.2264952788528545610e3;

// Coefficients for approximation to erf in 0.85 < |x| <= 1-2*exp(-25)
const C0: f64 = 1.42343711074968357734e0;
const C1: f64 = 4.63033784615654529590e0;
const C2: f64 = 5.76949722146069140550e0;
const C3: f64 = 3.64784832476320460504e0;
const C4: f64 = 1.27045825245236838258e0;
const C5: f64 = 2.41780725177450611770e-1;
const C6: f64 = 2.27238449892691845833e-2;
const C7: f64 = 7.74545014278341407640e-4;
const D0: f64 = 1.4142135623730950488016887e0;
const D1: f64 = 2.9036514445419946173133295e0;
const D2: f64 = 2.3707661626024532365971225e0;
const D3: f64 = 9.7547832001787427186894837e-1;
const D4: f64 = 2.0945065210512749128288442e-1;
const D5: f64 = 2.1494160384252876777097297e-2;
const D6: f64 = 7.7441459065157709165577218e-4;
const D7: f64 = 1.4859850019840355905497876e-9;

// Coefficients for approximation to erf in 1-2*exp(-25) < |x| < 1
const E0: f64 = 6.65790464350110377720e0;
const E1: f64 = 5.46378491116411436990e0;
const E2: f64 = 1.78482653991729133580e0;
const E3: f64 = 2.96560571828504891230e-1;
const E4: f64 = 2.65321895265761230930e-2;
const E5: f64 = 1.24266094738807843860e-3;
const E6: f64 = 2.71155556874348757815e-5;
const E7: f64 = 2.01033439929228813265e-7;
const F0: f64 = 1.414213562373095048801689e0;
const F1: f64 = 8.482908416595164588112026e-1;
const F2: f64 = 1.936480946950659106176712e-1;
const F3: f64 = 2.103693768272068968719679e-2;
const F4: f64 = 1.112800997078859844711555e-3;
const F5: f64 = 2.611088405080593625138020e-5;
const F6: f64 = 2.010321207683943062279931e-7;
const F7: f64 = 2.891024605872965461538222e-15;

/// Evaluates the inverse error function of x
///
/// **Note:** x must be in `[-1, +1]` with `x = ±1` being `±Inf`.
///
/// See: <https://mathworld.wolfram.com/InverseErf.html>
///
/// See also: <https://en.wikipedia.org/wiki/Error_function>
///
/// # Special cases
///
/// * `erf_inv(NaN)  = NaN`
/// * `erf_inv(-1.0) = -Inf`
/// * `erf_inv(1.0)  = +Inf`
/// * `erf_inv(x)    = NaN   if x < -1 or x > 1`
///
/// # Examples
///
/// ![erf_inv](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_erf_erfc_functions_erf_inv.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// let y = 0.4769362762044699;
/// approx_eq(math::erf_inv(-0.5), -y, 1e-15);
/// approx_eq(math::erf_inv( 0.5),  y, 1e-15);
/// ```
pub fn erf_inv(x: f64) -> f64 {
    // special cases
    if f64::is_nan(x) {
        return f64::NAN;
    } else if x == -1.0 {
        return f64::NEG_INFINITY;
    } else if x == 1.0 {
        return f64::INFINITY;
    } else if x < -1.0 || x > 1.0 {
        return f64::NAN;
    }

    let mut xx = x;
    let mut negative = false;
    if xx < 0.0 {
        xx = -x;
        negative = true;
    }

    let ans = if xx <= 0.85 {
        // |x| <= 0.85
        let r = 0.180625 - 0.25 * xx * xx;
        let z1 = ((((((A7 * r + A6) * r + A5) * r + A4) * r + A3) * r + A2) * r + A1) * r + A0;
        let z2 = ((((((B7 * r + B6) * r + B5) * r + B4) * r + B3) * r + B2) * r + B1) * r + B0;
        (xx * z1) / z2
    } else {
        let mut r = f64::sqrt(LN2 - f64::ln(1.0 - xx));
        let (z1, z2) = if r <= 5.0 {
            r -= 1.6;
            (
                ((((((C7 * r + C6) * r + C5) * r + C4) * r + C3) * r + C2) * r + C1) * r + C0,
                ((((((D7 * r + D6) * r + D5) * r + D4) * r + D3) * r + D2) * r + D1) * r + D0,
            )
        } else {
            r -= 5.0;
            (
                ((((((E7 * r + E6) * r + E5) * r + E4) * r + E3) * r + E2) * r + E1) * r + E0,
                ((((((F7 * r + F6) * r + F5) * r + F4) * r + F3) * r + F2) * r + F1) * r + F0,
            )
        };
        z1 / z2
    };

    if negative {
        return -ans;
    }
    ans
}

/// Evaluates the inverse of the complementary error function
///
/// **Note:** x must be in `[0, 2]` with `x = 0` being `Inf` and `x = 2` being `-Inf`.
///
/// See: <https://mathworld.wolfram.com/InverseErfc.html>
///
/// See also: <https://en.wikipedia.org/wiki/Error_function>
///
/// # Special cases
///
/// * `erfc_inv(0.0) = +Inf`
/// * `erfc_inv(2.0) = -Inf`
/// * `erfc_inv(x)   = NaN  if x < 0 or x > 2`
/// * `erfc_inv(NaN) = NaN`
///
/// # Examples
///
/// ![erfc_inv](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_erf_erfc_functions_erfc_inv.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// approx_eq(math::erfc_inv(0.5), 0.4769362762044699, 1e-15);
/// approx_eq(math::erfc_inv(1.0), 0.0, 1e-15);
/// ```
pub fn erfc_inv(x: f64) -> f64 {
    erf_inv(1.0 - x)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{erf_inv, erfc_inv};
    use crate::math::{erf, erfc};
    use crate::{approx_eq, assert_alike};

    #[test]
    fn erf_inv_works_1() {
        assert_eq!(erf_inv(-1.0), f64::NEG_INFINITY);
        assert_eq!(erf_inv(0.0), 0.0);
        assert_eq!(erf_inv(1.0), f64::INFINITY);

        // Mathematica
        // res = Table[{x, N[InverseErf[x], 50]}, {x, -1, 1, 0.05}]
        // Export["test.txt", res, "Table", "FieldSeparators" -> ", "]
        #[rustfmt::skip]
        let mathematica =[
            (-0.95                , 1e-15, -1.3859038243496777),
            (-0.9                 , 1e-15, -1.1630871536766745),
            (-0.85                , 1e-50, -1.0179024648320276),
            (-0.8                 , 1e-15, -0.9061938024368232),
            (-0.75                , 1e-15, -0.8134198475976185),
            (-0.7                 , 1e-50, -0.7328690779592167),
            (-0.6499999999999999  , 1e-15, -0.6608544253423858),
            (-0.6                 , 1e-50, -0.5951160814499948),
            (-0.55                , 1e-15, -0.5341590877497026),
            (-0.5                 , 1e-50, -0.4769362762044699),
            (-0.44999999999999996 , 1e-16, -0.4226802386475618),
            (-0.3999999999999999  , 1e-16, -0.3708071585935579),
            (-0.35                , 1e-15, -0.3208583217151815),
            (-0.29999999999999993 , 1e-16, -0.2724627147267543),
            (-0.25                , 1e-16, -0.2253120550121781),
            (-0.19999999999999996 , 1e-16, -0.17914345462129166),
            (-0.1499999999999999  , 1e-50, -0.13372692166481961),
            (-0.09999999999999998 , 1e-16, -0.08885599049425766),
            (-0.04999999999999993 , 1e-17, -0.044340387910005434), 
            (0.                   , 1e-50, 0.0),
            (0.050000000000000044 , 1e-50, 0.04434038791000553),
            (0.10000000000000009  , 1e-16, 0.08885599049425776),
            (0.15000000000000013  , 1e-16, 0.13372692166481984),
            (0.20000000000000018  , 1e-16, 0.17914345462129186),
            (0.25                 , 1e-16, 0.2253120550121781),
            (0.30000000000000004  , 1e-16, 0.27246271472675443),
            (0.3500000000000001   , 1e-16, 0.32085832171518164),
            (0.40000000000000013  , 1e-16, 0.370807158593558),
            (0.4500000000000002   , 1e-50, 0.4226802386475621),
            (0.5                  , 1e-50, 0.4769362762044699),
            (0.55                 , 1e-15, 0.5341590877497026),
            (0.6000000000000001   , 1e-15, 0.595116081449995),
            (0.6500000000000001   , 1e-15, 0.6608544253423861),
            (0.7000000000000002   , 1e-15, 0.7328690779592172),
            (0.75                 , 1e-15, 0.8134198475976185),
            (0.8                  , 1e-15, 0.9061938024368232),
            (0.8500000000000001   , 1e-15, 1.0179024648320278),
            (0.9000000000000001   , 1e-15, 1.1630871536766747),
            (0.9500000000000002   , 1e-15, 1.385903824349679),
        ];
        for (x, tol, reference) in mathematica {
            // println!("x = {:?}", x);
            approx_eq(erf_inv(x), reference, tol);
        }
    }

    #[test]
    fn erf_inv_works_2() {
        // Special case: should yield sqrt(LN2 - ln(1.0 - x)) == 5.0
        let x = 1.0 - 2.0 * f64::exp(-25.0);

        // Mathematica: N[1 - 2/E^25, 50]
        let xm = 0.99999999997222411227007195881067647250782628617920;
        assert_eq!(x, xm);

        // Mathematica: N[InverseErf[1 - 2/E^25], 50]
        let ym = 4.7078495219130335562325296830779236928390597996186;
        let y5 = erf_inv(x);
        approx_eq(y5, ym, 1e-7); // "only" 1e-6

        // need a tiny little more x to make sqrt(LN2 - ln(1.0 - xx)) > 5.0
        let x = x + 1e-16;
        let y5plus = erf_inv(x);
        approx_eq(y5plus, y5, 1e-6);

        // Mathematica: NumberForm[N[InverseErf[0.99999999999], 50], 50]
        approx_eq(erf_inv(0.99999999999), 4.812924058944833, 1e-15);
    }

    #[test]
    fn erfc_inv_works_1() {
        // Mathematica
        // res = Table[{x, N[InverseErfc[x], 50]}, {x, 0, 2, 0.05}]
        // Export["test.txt", res, "Table", "FieldSeparators" -> ", "]
        #[rustfmt::skip]
        let mathematica =[
            (0.05                , 1e-15, 1.3859038243496782),
            (0.1                 , 1e-15, 1.1630871536766743),
            (0.15000000000000002 , 1e-50, 1.0179024648320276),
            (0.2                 , 1e-15, 0.9061938024368236),
            (0.25                , 1e-15, 0.8134198475976185),
            (0.30000000000000004 , 1e-15, 0.732869077959217),
            (0.35000000000000003 , 1e-15, 0.6608544253423858),
            (0.4                 , 1e-15, 0.5951160814499951),
            (0.45                , 1e-50, 0.5341590877497024),
            (0.5                 , 1e-16, 0.47693627620446993),
            (0.55                , 1e-16, 0.42268023864756193),
            (0.6000000000000001  , 1e-16, 0.3708071585935579),
            (0.65                , 1e-16, 0.32085832171518147),
            (0.7000000000000001  , 1e-16, 0.2724627147267543),
            (0.75                , 1e-15, 0.22531205501217816),
            (0.8                 , 1e-16, 0.17914345462129166),
            (0.8500000000000001  , 1e-50, 0.13372692166481961),
            (0.9                 , 1e-16, 0.08885599049425766),
            (0.9500000000000001  , 1e-16, 0.044340387910005455),
            (1.                  , 1e-50, 0.),
            (1.05                , 1e-16, -0.04434038791000555),
            (1.1                 , 1e-16, -0.08885599049425776),
            (1.1500000000000001  , 1e-16, -0.1337269216648198),
            (1.2000000000000002  , 1e-16, -0.17914345462129186),
            (1.25                , 1e-15, -0.22531205501217816),
            (1.3                 , 1e-16, -0.27246271472675443),
            (1.35                , 1e-50, -0.3208583217151816),
            (1.4000000000000001  , 1e-15, -0.3708071585935582),
            (1.4500000000000002  , 1e-50, -0.4226802386475621),
            (1.5                 , 1e-16, -0.47693627620446993),
            (1.55                , 1e-50, -0.5341590877497024),
            (1.6                 , 1e-50, -0.5951160814499951),
            (1.6500000000000001  , 1e-50, -0.660854425342386),
            (1.7000000000000002  , 1e-50, -0.7328690779592172),
            (1.75                , 1e-15, -0.8134198475976185),
            (1.8                 , 1e-15, -0.9061938024368236),
            (1.85                , 1e-15, -1.017902464832028),
            (1.9000000000000001  , 1e-15, -1.1630871536766745),
            (1.9500000000000002  , 1e-15, -1.385903824349679),
        ];
        for (x, tol, reference) in mathematica {
            // println!("x = {:?}", x);
            approx_eq(erfc_inv(x), reference, tol);
        }
    }

    // The code below is based on all_test.go file from Go (1.22.1)

    const VALUES: [f64; 10] = [
        4.9790119248836735e+00,
        7.7388724745781045e+00,
        -2.7688005719200159e-01,
        -5.0106036182710749e+00,
        9.6362937071984173e+00,
        2.9263772392439646e+00,
        5.2290834314593066e+00,
        2.7279399104360102e+00,
        1.8253080916808550e+00,
        -8.6859247685756013e+00,
    ];

    const SOLUTION_ERF_INV: [f64; 10] = [
        4.746037673358033586786350696e-01,
        8.559054432692110956388764172e-01,
        -2.45427830571707336251331946e-02,
        -4.78116683518973366268905506e-01,
        1.479804430319470983648120853e+00,
        2.654485787128896161882650211e-01,
        5.027444534221520197823192493e-01,
        2.466703532707627818954585670e-01,
        1.632011465103005426240343116e-01,
        -1.06672334642196900710000389e+00,
    ];

    const SC_VALUES_ERF_INV: [f64; 6] = [1.0, -1.0, 0.0, f64::NEG_INFINITY, f64::INFINITY, f64::NAN];

    const SC_SOLUTION_ERF_INV: [f64; 6] = [f64::INFINITY, f64::NEG_INFINITY, 0.0, f64::NAN, f64::NAN, f64::NAN];

    const SC_VALUES_ERFC_INV: [f64; 6] = [0.0, 2.0, 1.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN];

    const SC_SOLUTION_ERFC_INV: [f64; 6] = [f64::INFINITY, f64::NEG_INFINITY, 0.0, f64::NAN, f64::NAN, f64::NAN];

    #[test]
    fn test_erf_inv() {
        for (i, v) in VALUES.iter().enumerate() {
            let a = *v / 10.0;
            let f = erf_inv(a);
            approx_eq(SOLUTION_ERF_INV[i], f, 1e-15);
        }
        for (i, v) in SC_VALUES_ERF_INV.iter().enumerate() {
            let f = erf_inv(*v);
            assert_alike(SC_SOLUTION_ERF_INV[i], f);
        }
        let mut x = -0.9;
        let dx = 0.3;
        while x <= 0.90 {
            let f = erf(erf_inv(x));
            approx_eq(x, f, 1e-15);
            x += dx;
        }
        let mut x = -0.9;
        while x <= 0.90 {
            let f = erf_inv(erf(x));
            approx_eq(x, f, 1e-15);
            x += dx;
        }
    }

    #[test]
    fn test_erfc_inv() {
        for (i, v) in VALUES.iter().enumerate() {
            let a = 1.0 - (*v / 10.0);
            let f = erfc_inv(a);
            approx_eq(SOLUTION_ERF_INV[i], f, 1e-15);
        }
        for (i, v) in SC_VALUES_ERFC_INV.iter().enumerate() {
            let f = erfc_inv(*v);
            assert_alike(SC_SOLUTION_ERFC_INV[i], f);
        }
        let mut x = 0.1;
        let dx = 0.3;
        while x <= 1.9 {
            let f = erfc(erfc_inv(x));
            approx_eq(x, f, 1e-15);
            x += dx;
        }
        let mut x = 0.1;
        while x <= 1.9 {
            let f = erfc_inv(erfc(x));
            approx_eq(x, f, 1e-14);
            x += dx;
        }
    }
}
