use super::{frexp, ldexp};

/// Evaluates the modified Bessel function I0(x) for any real x.
pub fn bessel_mod_i0(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }
    let ax = f64::abs(x);
    if ax < 15.0 {
        // rational approximation
        let y = x * x;
        return poly(&I0P, 13, y) / poly(&I0Q, 4, 225.0 - y);
    }
    // rational approximation with exp(x)/sqrt(x) factored out.
    let z = 1.0 - 15.0 / ax;
    return f64::exp(ax) * poly(&I0PP, 4, z) / (poly(&I0QQ, 5, z) * f64::sqrt(ax));
}

/// Evaluates the modified Bessel function I1(x) for any real x.
pub fn bessel_mod_i1(x: f64) -> f64 {
    let ax = f64::abs(x);
    if ax < 15.0 {
        // rational approximation
        let y = x * x;
        return x * poly(&I1P, 13, y) / poly(&I1Q, 4, 225.0 - y);
    }
    // rational approximation with exp(x)/sqrt(x) factored out.
    let z = 1.0 - 15.0 / ax;
    let ans = f64::exp(ax) * poly(&I1PP, 4, z) / (poly(&I1QQ, 5, z) * f64::sqrt(ax));
    if x > 0.0 {
        ans
    } else {
        -ans
    }
}

/// Evaluates the modified Bessel function In(x) for any real x and n ≥ 0
pub fn bessel_mod_in(n: i32, x: f64) -> f64 {
    if n == 0 {
        return bessel_mod_i0(x);
    }
    if n == 1 {
        return bessel_mod_i1(x);
    }
    if x * x <= 8.0 * f64::MIN_POSITIVE {
        return 0.0;
    }
    const ACC: f64 = 200.0; // determines accuracy
    const I_EXP: i32 = 1024 / 2; // aka numeric_limits<double>max_exponent / 2
    let tox = 2.0 / f64::abs(x);
    let mut bip = 0.0;
    let mut bi = 1.0;
    let mut j = 2 * (n + (f64::sqrt(ACC * (n as f64)) as i32));
    let mut ans: f64 = 0.0;
    while j > 0 {
        // downward recurrence
        let bim = bip + (j as f64) * tox * bi;
        bip = bi;
        bi = bim;
        let (_, k) = frexp(bi);
        if k > I_EXP {
            // re-normalize to prevent overflows
            ans = ldexp(ans, -I_EXP);
            bi = ldexp(bi, -I_EXP);
            bip = ldexp(bip, -I_EXP);
        }
        if j == n {
            ans = bip;
        }
        j -= 1;
    }
    ans *= bessel_mod_i0(x) / bi; // normalize using I0
    if x < 0.0 && (n & 1) != 0 {
        // negative and odd
        return -ans;
    }
    ans
}

/// Evaluates the modified Bessel function K0(x) for positive real x.
//   Special cases
//     K0(x=0) = +Inf
//     K0(x<0) = NaN
pub fn bessel_mod_k0(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::INFINITY;
    }
    if x <= 1.0 {
        // use two rational approximations
        let z = x * x;
        let term = poly(&K0PI, 4, z) * f64::ln(x) / poly(&K0QI, 2, 1. - z);
        return poly(&K0P, 4, z) / poly(&K0Q, 2, 1. - z) - term;
    }
    // rational approximation with exp(-x) / sqrt(x) factored out
    let z = 1.0 / x;
    f64::exp(-x) * poly(&K0PP, 7, z) / (poly(&K0QQ, 7, z) * f64::sqrt(x))
}

/// Evaluates the modified Bessel function K1(x) for positive real x.
//   Special cases
//     K0(x=0) = +Inf
//     K0(x<0) = NaN
pub fn bessel_mod_k1(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::INFINITY;
    }
    if x <= 1.0 {
        // use two rational approximations
        let z = x * x;
        let term = poly(&K1PI, 4, z) * f64::ln(x) / poly(&K1QI, 2, 1. - z);
        return x * (poly(&K1P, 4, z) / poly(&K1Q, 2, 1. - z) + term) + 1. / x;
    }
    // rational approximation with exp(-x)/sqrt(x) factored out
    let z = 1.0 / x;
    f64::exp(-x) * poly(&K1PP, 7, z) / (poly(&K1QQ, 7, z) * f64::sqrt(x))
}

/// Evaluates the modified Bessel function Kn(x) for positive x and n ≥ 0
pub fn bessel_mod_kn(n: i32, x: f64) -> f64 {
    if n == 0 {
        return bessel_mod_k0(x);
    }
    if n == 1 {
        return bessel_mod_k1(x);
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::INFINITY;
    }
    let tox = 2.0 / x;
    let mut bkm = bessel_mod_k0(x); // upward recurrence for all x...
    let mut bk = bessel_mod_k1(x);
    for j in 1..n {
        let bkp = bkm + (j as f64) * tox * bk;
        bkm = bk;
        bk = bkp;
    }
    return bk;
}

/// evaluates a polynomial for the modified Bessel functions
fn poly(cof: &[f64], n: i32, x: f64) -> f64 {
    let mut ans = cof[n as usize];
    let mut i = n - 1;
    while i >= 0 {
        ans = ans * x + cof[i as usize];
        i -= 1;
    }
    ans
}

// constants --------------------------------------------------------------------

const I0P: [f64; 14] = [
    9.999999999999997e-1,
    2.466405579426905e-1,
    1.478980363444585e-2,
    3.826993559940360e-4,
    5.395676869878828e-6,
    4.700912200921704e-8,
    2.733894920915608e-10,
    1.115830108455192e-12,
    3.301093025084127e-15,
    7.209167098020555e-18,
    1.166898488777214e-20,
    1.378948246502109e-23,
    1.124884061857506e-26,
    5.498556929587117e-30,
];

const I0Q: [f64; 5] = [
    4.463598170691436e-1,
    1.702205745042606e-3,
    2.792125684538934e-6,
    2.369902034785866e-9,
    8.965900179621208e-13,
];

const I0PP: [f64; 5] = [
    1.192273748120670e-1,
    1.947452015979746e-1,
    7.629241821600588e-2,
    8.474903580801549e-3,
    2.023821945835647e-4,
];

const I0QQ: [f64; 6] = [
    2.962898424533095e-1,
    4.866115913196384e-1,
    1.938352806477617e-1,
    2.261671093400046e-2,
    6.450448095075585e-4,
    1.529835782400450e-6,
];

const I1P: [f64; 14] = [
    5.000000000000000e-1,
    6.090824836578078e-2,
    2.407288574545340e-3,
    4.622311145544158e-5,
    5.161743818147913e-7,
    3.712362374847555e-9,
    1.833983433811517e-11,
    6.493125133990706e-14,
    1.693074927497696e-16,
    3.299609473102338e-19,
    4.813071975603122e-22,
    5.164275442089090e-25,
    3.846870021788629e-28,
    1.712948291408736e-31,
];

const I1Q: [f64; 5] = [
    4.665973211630446e-1,
    1.677754477613006e-3,
    2.583049634689725e-6,
    2.045930934253556e-9,
    7.166133240195285e-13,
];

const I1PP: [f64; 5] = [
    1.286515211317124e-1,
    1.930915272916783e-1,
    6.965689298161343e-2,
    7.345978783504595e-3,
    1.963602129240502e-4,
];

const I1QQ: [f64; 6] = [
    3.309385098860755e-1,
    4.878218424097628e-1,
    1.663088501568696e-1,
    1.473541892809522e-2,
    1.964131438571051e-4,
    -1.034524660214173e-6,
];

const K0PI: [f64; 5] = [
    1.0,
    2.346487949187396e-1,
    1.187082088663404e-2,
    2.150707366040937e-4,
    1.425433617130587e-6,
];

const K0QI: [f64; 3] = [9.847324170755358e-1, 1.518396076767770e-2, 8.362215678646257e-5];

const K0P: [f64; 5] = [
    1.159315156584126e-1,
    2.770731240515333e-1,
    2.066458134619875e-2,
    4.574734709978264e-4,
    3.454715527986737e-6,
];

const K0Q: [f64; 3] = [9.836249671709183e-1, 1.627693622304549e-2, 9.809660603621949e-5];

const K0PP: [f64; 8] = [
    1.253314137315499,
    1.475731032429900e1,
    6.123767403223466e1,
    1.121012633939949e2,
    9.285288485892228e1,
    3.198289277679660e1,
    3.595376024148513,
    6.160228690102976e-2,
];

const K0QQ: [f64; 8] = [
    1.0,
    1.189963006673403e1,
    5.027773590829784e1,
    9.496513373427093e1,
    8.318077493230258e1,
    3.181399777449301e1,
    4.443672926432041,
    1.408295601966600e-1,
];

const K1PI: [f64; 5] = [
    0.5,
    5.598072040178741e-2,
    1.818666382168295e-3,
    2.397509908859959e-5,
    1.239567816344855e-7,
];

const K1QI: [f64; 3] = [9.870202601341150e-1, 1.292092053534579e-2, 5.881933053917096e-5];

const K1P: [f64; 5] = [
    -3.079657578292062e-1,
    -8.109417631822442e-2,
    -3.477550948593604e-3,
    -5.385594871975406e-5,
    -3.110372465429008e-7,
];

const K1Q: [f64; 3] = [9.861813171751389e-1, 1.375094061153160e-2, 6.774221332947002e-5];

const K1PP: [f64; 8] = [
    1.253314137315502,
    1.457171340220454e1,
    6.063161173098803e1,
    1.147386690867892e2,
    1.040442011439181e2,
    4.356596656837691e1,
    7.265230396353690,
    3.144418558991021e-1,
];

const K1QQ: [f64; 8] = [
    1.0,
    1.125154514806458e1,
    4.427488496597630e1,
    7.616113213117645e1,
    5.863377227890893e1,
    1.850303673841586e1,
    1.857244676566022,
    2.538540887654872e-2,
];

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::bessel_mod_i0;
    use crate::approx_eq;

    #[test]
    fn bessel_mod_i0_works() {
        assert_eq!(bessel_mod_i0(0.0), 1.0);

        // Mathematica: N[BesselI[0, 1], 50]
        approx_eq(
            bessel_mod_i0(1.0),
            1.2660658777520083355982446252147175376076703113550,
            1e-15,
        );

        // Mathematica: N[BesselI[0, -29], 50]
        approx_eq(
            bessel_mod_i0(-25.0),
            5.7745606064663103157713397973069382664332453978739e9,
            1e-50,
        );
    }
}
