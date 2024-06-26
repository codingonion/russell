use super::{PI, SQRT_PI};

// This implementation is based on j0.go file from Go (1.22.1),
// which, in turn, is based on the FreeBSD code as explained below.
//
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Bessel function of the first and second kinds of order zero.
//
// The original C code and the long comment below are
// from FreeBSD's /usr/src/lib/msun/src/e_j0.c and
// came with this notice. The go code is a simplified
// version of the original C.
//
// ====================================================
// Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
//
// Developed at SunPro, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this
// software is freely granted, provided that this notice
// is preserved.
// ====================================================

// constants computed with Mathematica
pub(crate) const TWO_M27: f64 = 7.4505805969238281250000000000000000000000000000000e-9; // 2**-27 0x3e40000000000000 Mathematica: N[2^-27, 50]
pub(crate) const TWO_M13: f64 = 0.00012207031250000000000000000000000000000000000000000; // 2**-13 0x3f20000000000000 Mathematica: N[2^-13, 50]
pub(crate) const TWO_129: f64 = 6.8056473384187692692674921486353642291200000000000e38; // 2**129 0x4800000000000000 Mathematica: N[2^129, 50]

// R0/S0 on [0, 2]
const R02: f64 = 1.56249999999999947958e-02; // 0x3F8FFFFFFFFFFFFD
const R03: f64 = -1.89979294238854721751e-04; // 0xBF28E6A5B61AC6E9
const R04: f64 = 1.82954049532700665670e-06; // 0x3EBEB1D10C503919
const R05: f64 = -4.61832688532103189199e-09; // 0xBE33D5E773D63FCE
const S01: f64 = 1.56191029464890010492e-02; // 0x3F8FFCE882C8C2A4
const S02: f64 = 1.16926784663337450260e-04; // 0x3F1EA6D2DD57DBF4
const S03: f64 = 5.13546550207318111446e-07; // 0x3EA13B54CE84D5A9
const S04: f64 = 1.16614003333790000205e-09; // 0x3E1408BCF4745D8F

/// Evaluates the Bessel function J0(x) for any real x
///
/// See: <https://mathworld.wolfram.com/BesselFunctionoftheFirstKind.html>
///
/// See also: <https://en.wikipedia.org/wiki/Bessel_function>
///
/// # Special cases
///
///	* `J0(NaN)  = NaN`
///	* `J0(±Inf) = 0.0`
///	* `J0(0.0)  = 1.0`
///
/// # Examples
///
/// ![Bessel J0](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_bessel_functions_j0.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// approx_eq(math::bessel_j0(2.0), 0.22389077914123567, 1e-15);
/// ```
pub fn bessel_j0(x: f64) -> f64 {
    //
    // For tiny x, use j0(x) = 1 - x**2/4 + x**4/64 - ...
    //
    // Reduce x to |x| since j0(x)=j0(-x), and:
    //
    // for x in (0,2)
    //
    // j0(x) = 1-z/4+ z**2*R0/S0,  where z = x*x;
    //
    // (precision:  |j0-1+z/4-z**2R0/S0 |<2**-63.67 )
    //
    // for x in (2,inf)
    //
    // j0(x) = sqrt(2/(pi*x))*(p0(x)*cos(x0)-q0(x)*sin(x0))
    //
    // where x0 = x-pi/4.
    //
    // Compute sin(x0),cos(x0) as follows:
    //
    // cos(x0) = cos(x)cos(pi/4)+sin(x)sin(pi/4)
    //         = 1/sqrt(2) * (cos(x) + sin(x))
    // sin(x0) = sin(x)cos(pi/4)-cos(x)sin(pi/4)
    //         = 1/sqrt(2) * (sin(x) - cos(x))
    //
    // To avoid cancellation, use:
    //
    // sin(x) +- cos(x) = -cos(2x)/(sin(x) -+ cos(x))

    if f64::is_nan(x) {
        return f64::NAN;
    } else if f64::is_infinite(x) {
        return 0.0;
    } else if x == 0.0 {
        return 1.0;
    }

    let x = f64::abs(x);
    if x >= 2.0 {
        let (s, c) = f64::sin_cos(x);
        let mut ss = s - c;
        let mut cc = s + c;

        // make sure x+x does not overflow
        if x < f64::MAX / 2.0 {
            let z = -f64::cos(x + x);
            if s * c < 0.0 {
                cc = z / ss;
            } else {
                ss = z / cc;
            }
        }

        let z = if x > TWO_129 {
            (1.0 / SQRT_PI) * cc / f64::sqrt(x)
        } else {
            let u = pzero(x);
            let v = qzero(x);
            (1.0 / SQRT_PI) * (u * cc - v * ss) / f64::sqrt(x)
        };

        return z;
    }

    if x < TWO_M13 {
        if x < TWO_M27 {
            return 1.0;
        }
        return 1.0 - 0.25 * x * x;
    }

    let z = x * x;
    let r = z * (R02 + z * (R03 + z * (R04 + z * R05)));
    let s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * S04)));
    if x < 1.0 {
        return 1.0 + z * (-0.25 + (r / s));
    }

    let u = 0.5 * x;
    (1.0 + u) * (1.0 - u) + z * (r / s)
}

const U00: f64 = -7.38042951086872317523e-02; // 0xBFB2E4D699CBD01F
const U01: f64 = 1.76666452509181115538e-01; // 0x3FC69D019DE9E3FC
const U02: f64 = -1.38185671945596898896e-02; // 0xBF8C4CE8B16CFA97
const U03: f64 = 3.47453432093683650238e-04; // 0x3F36C54D20B29B6B
const U04: f64 = -3.81407053724364161125e-06; // 0xBECFFEA773D25CAD
const U05: f64 = 1.95590137035022920206e-08; // 0x3E5500573B4EABD4
const U06: f64 = -3.98205194132103398453e-11; // 0xBDC5E43D693FB3C8
const V01: f64 = 1.27304834834123699328e-02; // 0x3F8A127091C9C71A
const V02: f64 = 7.60068627350353253702e-05; // 0x3F13ECBBF578C6C1
const V03: f64 = 2.59150851840457805467e-07; // 0x3E91642D7FF202FD
const V04: f64 = 4.41110311332675467403e-10; // 0x3DFE50183BD6D9EF

/// Evaluates the Bessel function Y0(x) for positive real x
///
/// See: <https://mathworld.wolfram.com/BesselFunctionoftheSecondKind.html>
///
/// See also: <https://en.wikipedia.org/wiki/Bessel_function>
///
/// # Special cases
///
/// * `Y0(x < 0.0) = NaN`
/// * `Y0(NaN)     = NaN`
/// * `Y0(+Inf)    = 0.0`
/// * `Y0(0.0)     = -Inf`
///
/// # Examples
///
/// ![Bessel Y0](https://raw.githubusercontent.com/cpmech/russell/main/russell_lab/data/figures/math_bessel_functions_y0.svg)
///
/// ```
/// use russell_lab::{approx_eq, math};
///
/// approx_eq(math::bessel_y0(2.0), 0.51037567264974512, 1e-15);
/// ```
pub fn bessel_y0(x: f64) -> f64 {
    //
    // For x<2. Since
    //
    // y0(x) = 2/pi*(j0(x)*(ln(x/2)+Euler) + x**2/4 - ...)
    //
    // therefore y0(x)-2/pi*j0(x)*ln(x) is an even function.
    //
    // Use the following function to approximate y0,
    //
    // y0(x) = U(z)/V(z) + (2/pi)*(j0(x)*ln(x)), z= x**2
    //
    // where
    //
    // U(z) = u00 + u01*z + ... + u06*z**6
    // V(z) = 1  + v01*z + ... + v04*z**4
    //
    // with absolute approximation error bounded by 2**-72.
    //
    // Note: For tiny x, U/V = u0 and j0(x)~1, hence
    // y0(tiny) = u0 + (2/pi)*ln(tiny), (choose tiny<2**-27)
    //
    // For x>=2.
    //
    // y0(x) = sqrt(2/(pi*x))*(p0(x)*cos(x0)+q0(x)*sin(x0))
    //
    // where x0 = x-pi/4.
    //
    // Compute sin(x0),cos(x0) by the method mentioned in bessel_j0

    if x < 0.0 || f64::is_nan(x) {
        return f64::NAN;
    } else if f64::is_infinite(x) {
        // this handles +Inf since x < 0.0 was handled already
        return 0.0;
    } else if x == 0.0 {
        return f64::NEG_INFINITY;
    }

    if x >= 2.0 {
        let (s, c) = f64::sin_cos(x);
        let mut ss = s - c;
        let mut cc = s + c;

        // make sure x+x does not overflow
        if x < f64::MAX / 2.0 {
            let z = -f64::cos(x + x);
            if s * c < 0.0 {
                cc = z / ss;
            } else {
                ss = z / cc;
            }
        }

        let z = if x > TWO_129 {
            (1.0 / SQRT_PI) * ss / f64::sqrt(x)
        } else {
            let u = pzero(x);
            let v = qzero(x);
            (1.0 / SQRT_PI) * (u * ss + v * cc) / f64::sqrt(x)
        };

        return z;
    }

    if x <= TWO_M27 {
        return U00 + (2.0 / PI) * f64::ln(x); // |x| < ~7.4506e-9
    }

    let z = x * x;
    let u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06)))));
    let v = 1.0 + z * (V01 + z * (V02 + z * (V03 + z * V04)));
    u / v + (2.0 / PI) * bessel_j0(x) * f64::ln(x) // ~7.4506e-9 < |x| < 2.0
}

// The asymptotic expansions of pzero is
//
// 1 - 9/128 s**2 + 11025/98304 s**4 - ..., where s = 1/x.
//
// For x >= 2, We approximate pzero by
//
// pzero(x) = 1 + (R/S)
//
// where  R = pR0 + pR1*s**2 + pR2*s**4 + ... + pR5*s**10
//
// S = 1 + pS0*s**2 + ... + pS4*s**10
//
// and
//
// | pzero(x)-1-R/S | <= 2  ** ( -60.26)

// for x in [inf, 8]=1/[0,0.125]
const P0R8: [f64; 6] = [
    0.00000000000000000000e+00,  // 0x0000000000000000
    -7.03124999999900357484e-02, // 0xBFB1FFFFFFFFFD32
    -8.08167041275349795626e+00, // 0xC02029D0B44FA779
    -2.57063105679704847262e+02, // 0xC07011027B19E863
    -2.48521641009428822144e+03, // 0xC0A36A6ECD4DCAFC
    -5.25304380490729545272e+03, // 0xC0B4850B36CC643D
];
const P0S8: [f64; 5] = [
    1.16534364619668181717e+02, // 0x405D223307A96751
    3.83374475364121826715e+03, // 0x40ADF37D50596938
    4.05978572648472545552e+04, // 0x40E3D2BB6EB6B05F
    1.16752972564375915681e+05, // 0x40FC810F8F9FA9BD
    4.76277284146730962675e+04, // 0x40E741774F2C49DC
];

// for x in [8,4.5454]=1/[0.125,0.22001]
const P0R5: [f64; 6] = [
    -1.14125464691894502584e-11, // 0xBDA918B147E495CC
    -7.03124940873599280078e-02, // 0xBFB1FFFFE69AFBC6
    -4.15961064470587782438e+00, // 0xC010A370F90C6BBF
    -6.76747652265167261021e+01, // 0xC050EB2F5A7D1783
    -3.31231299649172967747e+02, // 0xC074B3B36742CC63
    -3.46433388365604912451e+02, // 0xC075A6EF28A38BD7
];
const P0S5: [f64; 5] = [
    6.07539382692300335975e+01, // 0x404E60810C98C5DE
    1.05125230595704579173e+03, // 0x40906D025C7E2864
    5.97897094333855784498e+03, // 0x40B75AF88FBE1D60
    9.62544514357774460223e+03, // 0x40C2CCB8FA76FA38
    2.40605815922939109441e+03, // 0x40A2CC1DC70BE864
];

// for x in [4.547,2.8571]=1/[0.2199,0.35001]
const P0R3: [f64; 6] = [
    -2.54704601771951915620e-09, // 0xBE25E1036FE1AA86
    -7.03119616381481654654e-02, // 0xBFB1FFF6F7C0E24B
    -2.40903221549529611423e+00, // 0xC00345B2AEA48074
    -2.19659774734883086467e+01, // 0xC035F74A4CB94E14
    -5.80791704701737572236e+01, // 0xC04D0A22420A1A45
    -3.14479470594888503854e+01, // 0xC03F72ACA892D80F
];
const P0S3: [f64; 5] = [
    3.58560338055209726349e+01, // 0x4041ED9284077DD3
    3.61513983050303863820e+02, // 0x40769839464A7C0E
    1.19360783792111533330e+03, // 0x4092A66E6D1061D6
    1.12799679856907414432e+03, // 0x40919FFCB8C39B7E
    1.73580930813335754692e+02, // 0x4065B296FC379081
];

// for x in [2.8570,2]=1/[0.3499,0.5]
const P0R2: [f64; 6] = [
    -8.87534333032526411254e-08, // 0xBE77D316E927026D
    -7.03030995483624743247e-02, // 0xBFB1FF62495E1E42
    -1.45073846780952986357e+00, // 0xBFF736398A24A843
    -7.63569613823527770791e+00, // 0xC01E8AF3EDAFA7F3
    -1.11931668860356747786e+01, // 0xC02662E6C5246303
    -3.23364579351335335033e+00, // 0xC009DE81AF8FE70F
];
const P0S2: [f64; 5] = [
    2.22202997532088808441e+01, // 0x40363865908B5959
    1.36206794218215208048e+02, // 0x4061069E0EE8878F
    2.70470278658083486789e+02, // 0x4070E78642EA079B
    1.53875394208320329881e+02, // 0x40633C033AB6FAFF
    1.46576176948256193810e+01, // 0x402D50B344391809
];

fn pzero(x: f64) -> f64 {
    let (p, q) = if x >= 8.0 {
        (&P0R8, &P0S8)
    } else if x >= 4.5454 {
        (&P0R5, &P0S5)
    } else if x >= 2.8571 {
        (&P0R3, &P0S3)
    } else if x >= 2.0 {
        (&P0R2, &P0S2)
    } else {
        panic!("INTERNAL ERROR: x must be ≥ 2.0 for pzero");
    };
    let z = 1.0 / (x * x);
    let r = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    let s = 1.0 + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * q[4]))));
    1.0 + r / s
}

// For x >= 8, the asymptotic expansions of qzero is
//
// -1/8 s + 75/1024 s**3 - ..., where s = 1/x.
//
// Approximate pzero by
//
// qzero(x) = s*(-1.25 + (R/S))
//
// where
//
// R = qR0 + qR1*s**2 + qR2*s**4 + ... + qR5*s**10
// S = 1 + qS0*s**2 + ... + qS5*s**12
//
// and
//
// | qzero(x)/s +1.25-R/S | <= 2**(-61.22)

// for x in [inf, 8]=1/[0,0.125]
const Q0R8: [f64; 6] = [
    0.00000000000000000000e+00, // 0x0000000000000000
    7.32421874999935051953e-02, // 0x3FB2BFFFFFFFFE2C
    1.17682064682252693899e+01, // 0x402789525BB334D6
    5.57673380256401856059e+02, // 0x40816D6315301825
    8.85919720756468632317e+03, // 0x40C14D993E18F46D
    3.70146267776887834771e+04, // 0x40E212D40E901566
];
const Q0S8: [f64; 6] = [
    1.63776026895689824414e+02,  // 0x406478D5365B39BC
    8.09834494656449805916e+03,  // 0x40BFA2584E6B0563
    1.42538291419120476348e+05,  // 0x4101665254D38C3F
    8.03309257119514397345e+05,  // 0x412883DA83A52B43
    8.40501579819060512818e+05,  // 0x4129A66B28DE0B3D
    -3.43899293537866615225e+05, // 0xC114FD6D2C9530C5
];

// for x in [8,4.5454]=1/[0.125,0.22001]
const Q0R5: [f64; 6] = [
    1.84085963594515531381e-11, // 0x3DB43D8F29CC8CD9
    7.32421766612684765896e-02, // 0x3FB2BFFFD172B04C
    5.83563508962056953777e+00, // 0x401757B0B9953DD3
    1.35111577286449829671e+02, // 0x4060E3920A8788E9
    1.02724376596164097464e+03, // 0x40900CF99DC8C481
    1.98997785864605384631e+03, // 0x409F17E953C6E3A6
];
const Q0S5: [f64; 6] = [
    8.27766102236537761883e+01,  // 0x4054B1B3FB5E1543
    2.07781416421392987104e+03,  // 0x40A03BA0DA21C0CE
    1.88472887785718085070e+04,  // 0x40D267D27B591E6D
    5.67511122894947329769e+04,  // 0x40EBB5E397E02372
    3.59767538425114471465e+04,  // 0x40E191181F7A54A0
    -5.35434275601944773371e+03, // 0xC0B4EA57BEDBC609
];

// for x in [4.547,2.8571]=1/[0.2199,0.35001]
const Q0R3: [f64; 6] = [
    4.37741014089738620906e-09, // 0x3E32CD036ADECB82
    7.32411180042911447163e-02, // 0x3FB2BFEE0E8D0842
    3.34423137516170720929e+00, // 0x400AC0FC61149CF5
    4.26218440745412650017e+01, // 0x40454F98962DAEDD
    1.70808091340565596283e+02, // 0x406559DBE25EFD1F
    1.66733948696651168575e+02, // 0x4064D77C81FA21E0
];
const Q0S3: [f64; 6] = [
    4.87588729724587182091e+01,  // 0x40486122BFE343A6
    7.09689221056606015736e+02,  // 0x40862D8386544EB3
    3.70414822620111362994e+03,  // 0x40ACF04BE44DFC63
    6.46042516752568917582e+03,  // 0x40B93C6CD7C76A28
    2.51633368920368957333e+03,  // 0x40A3A8AAD94FB1C0
    -1.49247451836156386662e+02, // 0xC062A7EB201CF40F
];

// for x in [2.8570,2]=1/[0.3499,0.5]
const Q0R2: [f64; 6] = [
    1.50444444886983272379e-07, // 0x3E84313B54F76BDB
    7.32234265963079278272e-02, // 0x3FB2BEC53E883E34
    1.99819174093815998816e+00, // 0x3FFFF897E727779C
    1.44956029347885735348e+01, // 0x402CFDBFAAF96FE5
    3.16662317504781540833e+01, // 0x403FAA8E29FBDC4A
    1.62527075710929267416e+01, // 0x403040B171814BB4
];
const Q0S2: [f64; 6] = [
    3.03655848355219184498e+01,  // 0x403E5D96F7C07AED
    2.69348118608049844624e+02,  // 0x4070D591E4D14B40
    8.44783757595320139444e+02,  // 0x408A664522B3BF22
    8.82935845112488550512e+02,  // 0x408B977C9C5CC214
    2.12666388511798828631e+02,  // 0x406A95530E001365
    -5.31095493882666946917e+00, // 0xC0153E6AF8B32931
];

fn qzero(x: f64) -> f64 {
    let (p, q) = if x >= 8.0 {
        (&Q0R8, &Q0S8)
    } else if x >= 4.5454 {
        (&Q0R5, &Q0S5)
    } else if x >= 2.8571 {
        (&Q0R3, &Q0S3)
    } else if x >= 2.0 {
        (&Q0R2, &Q0S2)
    } else {
        panic!("INTERNAL ERROR: x must be ≥ 2.0 for qzero");
    };
    let z = 1.0 / (x * x);
    let r = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    let s = 1.0 + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * (q[4] + z * q[5])))));
    (-0.125 + r / s) / x
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{bessel_j0, bessel_y0, pzero, qzero, TWO_129};
    use crate::{approx_eq, assert_alike};

    #[test]
    #[should_panic(expected = "INTERNAL ERROR: x must be ≥ 2.0 for pzero")]
    fn pzero_panics_on_wrong_input() {
        pzero(1.99);
    }

    #[test]
    #[should_panic(expected = "INTERNAL ERROR: x must be ≥ 2.0 for qzero")]
    fn qzero_panics_on_wrong_input() {
        qzero(1.99);
    }

    #[test]
    fn bessel_j0_handles_special_cases() {
        assert!(bessel_j0(f64::NAN).is_nan());
        assert_eq!(bessel_j0(f64::NEG_INFINITY), 0.0);
        assert_eq!(bessel_j0(f64::INFINITY), 0.0);
        assert_eq!(bessel_j0(0.0), 1.0);
    }

    #[test]
    fn bessel_y0_handles_special_cases() {
        assert!(bessel_y0(f64::NEG_INFINITY).is_nan());
        assert!(bessel_y0(-0.01).is_nan());
        assert!(bessel_y0(f64::NAN).is_nan());
        assert_eq!(bessel_y0(f64::INFINITY), 0.0);
        assert_eq!(bessel_y0(0.0), f64::NEG_INFINITY);
    }

    #[test]
    fn bessel_j0_works() {
        // Mathematica: N[BesselJ[0, -123], 100]
        assert_eq!(
            bessel_j0(-123.0),
            -0.06854552119354654773723060960911865200289142665003329651340461580967205232587549262115495507220123533
        );

        // Mathematica: N[BesselJ[0, -5], 100]
        assert_eq!(
            bessel_j0(-5.0),
            -0.1775967713143383043473970130747587110711303560085091289906582682081766005733631207589074141654715325
        );

        // Mathematica: N[BesselJ[0, -2], 100]
        assert_eq!(
            bessel_j0(-2.0),
            0.2238907791412356680518274546499486258251544822186076031283497060108539577680107050148115118534293660
        );

        // Mathematica: N[BesselJ[0, -1], 100]
        assert_eq!(
            bessel_j0(-1.0),
            0.7651976865579665514497175261026632209092742897553252418615475491192789122152724401671806000989156340
        );

        // Mathematica: N[BesselJ[0, 10^-9], 100]
        assert_eq!(
            bessel_j0(1e-9),
            0.99999999999999999975000000000000000001562499999999999999956597222222222222222900390624999999999993218
        );

        // Mathematica: N[BesselJ[0, 1], 100]
        assert_eq!(
            bessel_j0(1.0),
            0.7651976865579665514497175261026632209092742897553252418615475491192789122152724401671806000989156340
        );

        // Mathematica: N[BesselJ[0, 2], 100]
        assert_eq!(
            bessel_j0(2.0),
            0.2238907791412356680518274546499486258251544822186076031283497060108539577680107050148115118534293660
        );

        // Mathematica: N[BesselJ[0, 5], 100]
        assert_eq!(
            bessel_j0(5.0),
            -0.1775967713143383043473970130747587110711303560085091289906582682081766005733631207589074141654715325
        );

        // Mathematica: N[BesselJ[0, 123], 100]
        assert_eq!(
            bessel_j0(123.0),
            -0.06854552119354654773723060960911865200289142665003329651340461580967205232587549262115495507220123533
        );
    }

    #[test]
    fn bessel_y0_works() {
        // Mathematica: N[BesselY[0, 10^-9], 100]
        assert_eq!(
            bessel_y0(1e-9),
            -13.26664507493838655750870141542857521844493660738045065740801097731190898310202900407610027191316534
        );

        // Mathematica: N[BesselY[0, 1], 100]
        approx_eq(
            bessel_y0(1.0),
            0.08825696421567695798292676602351516282781752309067554671104384761199978932351337130107720035921993680,
            1e-16,
        );

        // Mathematica: N[BesselY[0, 2], 100]
        assert_eq!(
            bessel_y0(2.0),
            0.5103756726497451195966065927271578732681392270858461355718392719327313950418217977114130750041427489
        );

        // Mathematica: N[BesselY[0, 5], 100]
        assert_eq!(
            bessel_y0(5.0),
            -0.3085176252490337800736489842120466113863470616273440437958514158982579156428808518901793164635184073
        );

        // Mathematica: N[BesselY[0, 123], 100]
        approx_eq(
            bessel_y0(123.0),
            0.02184580674805452669889249502284870517465956644502777321367441073587120026032506563177310919183224620,
            1e-17,
        );
    }

    #[test]
    fn bessel_j0_edge_cases_work() {
        //
        // x > f64::MAX / 2.0
        //
        // println!("x = {:?}", (f64::MAX / 2.0)); // 8.988465674311579e307
        // println!("J0(x) = {:?}", bessel_j0(8.988465674311579e307));
        // Reference value from Go 1.22.1
        approx_eq(bessel_j0(f64::MAX / 2.0), 5.965640685080747e-155, 1e-310);

        //
        // x > TWO_129
        //
        // Mathematica: N[BesselJ[0, 2 2^129], 100]
        approx_eq(
            bessel_j0(2.0 * TWO_129),
            -2.444353286102078069059175500103428864399201270217340894358804227316802405824590992570065626827792071e-21,
            1e-36,
        );

        //
        // x < 1.0
        //
        // Mathematica: N[BesselJ[0, 0.5], 100]
        approx_eq(bessel_j0(0.5), 0.938469807240813, 1e-15);
    }

    #[test]
    fn bessel_y0_edge_cases_work() {
        //
        // x > f64::MAX / 2.0
        //
        // println!("x = {:?}", (f64::MAX / 2.0)); // 8.988465674311579e307
        // println!("Y0(x) = {:?}", bessel_y0(8.988465674311579e307));
        // Reference value from Go 1.22.1
        approx_eq(bessel_y0(f64::MAX / 2.0), 5.936112522662019e-155, 1e-310);
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

    const SOLUTION_J0: [f64; 10] = [
        -1.8444682230601672018219338e-01,
        2.27353668906331975435892e-01,
        9.809259936157051116270273e-01,
        -1.741170131426226587841181e-01,
        -2.1389448451144143352039069e-01,
        -2.340905848928038763337414e-01,
        -1.0029099691890912094586326e-01,
        -1.5466726714884328135358907e-01,
        3.252650187653420388714693e-01,
        -8.72218484409407250005360235e-03,
    ];

    const SOLUTION_Y0: [f64; 10] = [
        -3.053399153780788357534855e-01,
        1.7437227649515231515503649e-01,
        -8.6221781263678836910392572e-01,
        -3.100664880987498407872839e-01,
        1.422200649300982280645377e-01,
        4.000004067997901144239363e-01,
        -3.3340749753099352392332536e-01,
        4.5399790746668954555205502e-01,
        4.8290004112497761007536522e-01,
        2.7036697826604756229601611e-01,
    ];

    const SC_VALUES_J0: [f64; 4] = [f64::NEG_INFINITY, 0.0, f64::INFINITY, f64::NAN];

    const SC_SOLUTION_J0: [f64; 4] = [0.0, 1.0, 0.0, f64::NAN];

    const SC_VALUES_Y0: [f64; 5] = [f64::NEG_INFINITY, 0.0, f64::INFINITY, f64::NAN, -1.0];

    const SC_SOLUTION_Y0: [f64; 5] = [f64::NAN, f64::NEG_INFINITY, 0.0, f64::NAN, f64::NAN];

    #[test]
    fn test_bessel_j0() {
        for (i, v) in VALUES.iter().enumerate() {
            let f = bessel_j0(*v);
            approx_eq(SOLUTION_J0[i], f, 1e-16);
        }
        for (i, v) in SC_VALUES_J0.iter().enumerate() {
            let f = bessel_j0(*v);
            assert_alike(SC_SOLUTION_J0[i], f);
        }
    }

    #[test]
    fn test_bessel_y0() {
        for (i, v) in VALUES.iter().enumerate() {
            let a = f64::abs(*v);
            let f = bessel_y0(a);
            approx_eq(SOLUTION_Y0[i], f, 1e-15);
        }
        for (i, v) in SC_VALUES_Y0.iter().enumerate() {
            let f = bessel_y0(*v);
            assert_alike(SC_SOLUTION_Y0[i], f);
        }
    }
}
