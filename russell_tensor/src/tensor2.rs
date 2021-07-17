use super::*;
use std::fmt;

/// Implements a second-order tensor, symmetric or not
pub struct Tensor2 {
    pub(super) comps_mandel: Vec<f64>, // components in Mandel basis. len = 9 or 6 (symmetric)
    pub(super) symmetric: bool,        // this is a symmetric tensor
}

impl Tensor2 {
    /// Returns a new Tensor2, symmetric or not, with 0-valued components
    pub fn new(symmetric: bool) -> Self {
        let size = if symmetric { 6 } else { 9 };
        Tensor2 {
            comps_mandel: vec![0.0; size],
            symmetric,
        }
    }

    /// Returns a new Tensor2 constructed from the "standard" components
    ///
    /// # Arguments
    ///
    /// * tt - the standard components given with respect to an orthonormal Cartesian basis
    /// * symmetric - this is a symmetric tensor
    ///
    /// # Panics
    ///
    /// This method panics symmetric=true but the components are not symmetric.
    ///
    pub fn from_tensor(tt: &[[f64; 3]; 3], symmetric: bool) -> Self {
        if symmetric {
            if tt[1][0] != tt[0][1] || tt[2][1] != tt[1][2] || tt[2][0] != tt[0][2] {
                panic!("the components of symmetric second order tensor do not pass symmetry check");
            }
        }
        let size = if symmetric { 6 } else { 9 };
        let mut tt_bar = vec![0.0; size];
        for i in 0..3 {
            let j0 = if symmetric { i } else { 0 };
            for j in j0..3 {
                let a = IJ_TO_I[i][j];
                if i == j {
                    tt_bar[a] = tt[i][j];
                }
                if i < j {
                    tt_bar[a] = (tt[i][j] + tt[j][i]) / SQRT_2;
                }
                if i > j {
                    tt_bar[a] = (tt[j][i] - tt[i][j]) / SQRT_2;
                }
            }
        }
        Tensor2 {
            comps_mandel: tt_bar,
            symmetric,
        }
    }

    /// Returns a 2D array with the standard components of this second-order tensor
    pub fn to_tensor(&self) -> Vec<Vec<f64>> {
        let mut tt = vec![vec![0.0; 3]; 3];
        if self.symmetric {
            for m in 0..6 {
                let (i, j) = I_TO_IJ[m];
                if i == j {
                    tt[i][j] = self.comps_mandel[m];
                }
                if i < j {
                    tt[i][j] = self.comps_mandel[m] / SQRT_2;
                    tt[j][i] = tt[i][j];
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    let m = IJ_TO_I[i][j];
                    let val = self.comps_mandel[m];
                    if i == j {
                        tt[i][j] = val;
                    }
                    if i < j {
                        let n = IJ_TO_I[j][i];
                        let next = self.comps_mandel[n];
                        tt[i][j] = (val + next) / SQRT_2;
                    }
                    if i > j {
                        let n = IJ_TO_I[j][i];
                        let next = self.comps_mandel[n];
                        tt[i][j] = (next - val) / SQRT_2;
                    }
                }
            }
        }
        tt
    }
}

impl fmt::Display for Tensor2 {
    /// Implements the Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t2 = self.to_tensor();
        match format_matrix(&t2, 3, 3) {
            Ok(buf) => write!(f, "{}", buf),
            Err(e) => Err(e),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use russell_chk::*;

    #[test]
    fn new_tensor2_works() {
        let t2 = Tensor2::new(false);
        let correct = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_vec_approx_eq!(t2.comps_mandel, correct, 1e-15);
    }

    #[test]
    fn new_symmetric_tensor2_works() {
        let t2 = Tensor2::new(true);
        let correct = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_vec_approx_eq!(t2.comps_mandel, correct, 1e-15);
    }

    #[test]
    fn from_tensor_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let t2 = Tensor2::from_tensor(comps_std, false);
        let correct = &[
            1.0,
            5.0,
            9.0,
            6.0 / SQRT_2,
            14.0 / SQRT_2,
            10.0 / SQRT_2,
            -2.0 / SQRT_2,
            -2.0 / SQRT_2,
            -4.0 / SQRT_2,
        ];
        assert_vec_approx_eq!(t2.comps_mandel, correct, 1e-15);
    }

    #[test]
    fn from_symmetric_tensor_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let t2 = Tensor2::from_tensor(comps_std, true);
        let correct = &[1.0, 2.0, 3.0, 4.0 * SQRT_2, 5.0 * SQRT_2, 6.0 * SQRT_2];
        assert_vec_approx_eq!(t2.comps_mandel, correct, 1e-14);
    }

    #[test]
    #[should_panic(expected = "the components of symmetric second order tensor do not pass symmetry check")]
    fn from_symmetric_tensor_panics_on_invalid_data_10() {
        let eps = 1e-15;
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0+eps, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        Tensor2::from_tensor(comps_std, true);
    }

    #[test]
    #[should_panic(expected = "the components of symmetric second order tensor do not pass symmetry check")]
    fn from_symmetric_tensor_panics_on_invalid_data_21() {
        let eps = 1e-15;
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0+eps, 5.0, 3.0],
        ];
        Tensor2::from_tensor(comps_std, true);
    }

    #[test]
    #[should_panic(expected = "the components of symmetric second order tensor do not pass symmetry check")]
    fn from_symmetric_tensor_panics_on_invalid_data_20() {
        let eps = 1e-15;
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0+eps, 3.0],
        ];
        Tensor2::from_tensor(comps_std, true);
    }

    #[test]
    fn to_tensor_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let t2 = Tensor2::from_tensor(comps_std, false);
        let res = t2.to_tensor();
        for i in 0..3 {
            for j in 0..3 {
                assert_approx_eq!(res[i][j], comps_std[i][j], 1e-14);
            }
        }
    }

    #[test]
    fn to_tensor_symmetric_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let t2 = Tensor2::from_tensor(comps_std, true);
        let res = t2.to_tensor();
        for i in 0..3 {
            for j in 0..3 {
                assert_approx_eq!(res[i][j], comps_std[i][j], 1e-14);
            }
        }
    }
}
