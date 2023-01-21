use super::{IJ_TO_M, IJ_TO_M_SYM, M_TO_IJ, SQRT_2};
use crate::{Mandel, StrError};
use russell_lab::{vec_copy, vec_update, Matrix, Vector};
use serde::{Deserialize, Serialize};

/// Implements a second-order tensor, symmetric or not
///
/// Internally, the components are converted to the Mandel basis. On the Mandel basis,
/// depending on the symmetry, we may store fewer components. Also, we may store
/// only 4 components of Symmetric 2D tensors.
///
/// **General case:**
///
/// ```text
///                       ┌                ┐
///                    00 │      T00       │ 0
///                    11 │      T11       │ 1
/// ┌             ┐    22 │      T22       │ 2
/// │ T00 T01 T02 │    01 │ (T01+T10) / √2 │ 3
/// │ T10 T11 T12 │ => 12 │ (T12+T21) / √2 │ 4
/// │ T20 T21 T22 │    02 │ (T02+T20) / √2 │ 5
/// └             ┘    10 │ (T01-T10) / √2 │ 6
///                    21 │ (T12-T21) / √2 │ 7
///                    20 │ (T02-T20) / √2 │ 8
///                       └                ┘
/// ```
///
/// **Symmetric 3D:**
///
/// ```text
///                       ┌          ┐
/// ┌             ┐    00 │   T00    │ 0
/// │ T00 T01 T02 │    11 │   T11    │ 1
/// │ T01 T11 T12 │ => 22 │   T22    │ 2
/// │ T02 T12 T22 │    01 │ T01 * √2 │ 3
/// └             ┘    12 │ T12 * √2 │ 4
///                    02 │ T02 * √2 │ 5
///                       └          ┘
/// ```
///
/// **Symmetric 2D:**
///
/// ```text
/// ┌             ┐       ┌          ┐
/// │ T00 T01     │    00 │   T00    │ 0
/// │ T01 T11     │ => 11 │   T11    │ 1
/// │         T22 │    22 │   T22    │ 2
/// └             ┘    01 │ T01 * √2 │ 3
///                       └          ┘
/// ```
///
/// # Notes
///
/// * The tensor is represented as a 9D, 6D or 4D vector and saved as `vec`
/// * You may perform operations on `vec` directly because it is isomorphic with the tensor itself
/// * For example, the norm of the tensor equals `vec.norm()`
/// * However, you must be careful when setting a single component of `vec` directly
///   because you may "break" the Mandel representation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tensor2 {
    /// Holds the components in Mandel basis as a vector.
    ///
    /// * General: `vec.dim = 9`
    /// * Symmetric in 3D: `vec.dim = 6`
    /// * Symmetric in 2D: `vec.dim = 4`
    pub vec: Vector,
}

impl Tensor2 {
    /// Creates a new (zeroed) Tensor2
    ///
    /// # Input
    ///
    /// * `case` -- the [Mandel] case
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, StrError, Tensor2};
    ///
    /// fn main() {
    ///     let a = Tensor2::new(Mandel::General);
    ///     assert_eq!(a.vec.as_data(), &[0.0,0.0,0.0,  0.0,0.0,0.0,  0.0,0.0,0.0]);
    ///
    ///     let b = Tensor2::new(Mandel::Symmetric);
    ///     assert_eq!(b.vec.as_data(), &[0.0,0.0,0.0,  0.0,0.0,0.0]);
    ///
    ///     let c = Tensor2::new(Mandel::Symmetric2D);
    ///     assert_eq!(c.vec.as_data(), &[0.0,0.0,0.0,  0.0]);
    /// }
    /// ```
    pub fn new(case: Mandel) -> Self {
        Tensor2 {
            vec: Vector::new(case.dim()),
        }
    }

    /// Creates a new Tensor2 constructed from a matrix
    ///
    /// # Input
    ///
    /// * `tt` -- the standard (not Mandel) Tij components given
    ///   with respect to an orthonormal Cartesian basis
    /// * `case` -- the [Mandel] case
    ///
    /// # Notes
    ///
    /// * In all cases, even in 2D, the input matrix must be 3×3
    /// * If symmetric, the off-diagonal components must equal the corresponding ones
    /// * If 2D, tt[1][2] and tt[0][2] must both be equal to zero
    /// * If 2D, symmetric must be true
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, StrError, Tensor2, SQRT_2};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // general
    ///     let a = Tensor2::from_matrix(
    ///         &[
    ///             [1.0, SQRT_2 * 2.0, SQRT_2 * 3.0],
    ///             [SQRT_2 * 4.0, 5.0, SQRT_2 * 6.0],
    ///             [SQRT_2 * 7.0, SQRT_2 * 8.0, 9.0],
    ///         ],
    ///         Mandel::General,
    ///     )?;
    ///     assert_eq!(
    ///         format!("{:.1}", a.vec),
    ///         "┌      ┐\n\
    ///          │  1.0 │\n\
    ///          │  5.0 │\n\
    ///          │  9.0 │\n\
    ///          │  6.0 │\n\
    ///          │ 14.0 │\n\
    ///          │ 10.0 │\n\
    ///          │ -2.0 │\n\
    ///          │ -2.0 │\n\
    ///          │ -4.0 │\n\
    ///          └      ┘"
    ///     );
    ///
    ///     // symmetric-3D
    ///     let b = Tensor2::from_matrix(
    ///         &[
    ///             [1.0, 4.0 / SQRT_2, 6.0 / SQRT_2],
    ///             [4.0 / SQRT_2, 2.0, 5.0 / SQRT_2],
    ///             [6.0 / SQRT_2, 5.0 / SQRT_2, 3.0],
    ///         ],
    ///         Mandel::Symmetric,
    ///     )?;
    ///     assert_eq!(
    ///         format!("{:.1}", b.vec),
    ///         "┌     ┐\n\
    ///          │ 1.0 │\n\
    ///          │ 2.0 │\n\
    ///          │ 3.0 │\n\
    ///          │ 4.0 │\n\
    ///          │ 5.0 │\n\
    ///          │ 6.0 │\n\
    ///          └     ┘"
    ///     );
    ///
    ///     // symmetric-2D
    ///     let c = Tensor2::from_matrix(
    ///         &[
    ///             [       1.0, 4.0/SQRT_2, 0.0],
    ///             [4.0/SQRT_2,        2.0, 0.0],
    ///             [       0.0,        0.0, 3.0],
    ///         ],
    ///         Mandel::Symmetric2D,
    ///     )?;
    ///     assert_eq!(
    ///         format!("{:.1}", c.vec),
    ///         "┌     ┐\n\
    ///          │ 1.0 │\n\
    ///          │ 2.0 │\n\
    ///          │ 3.0 │\n\
    ///          │ 4.0 │\n\
    ///          └     ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn from_matrix(tt: &[[f64; 3]; 3], case: Mandel) -> Result<Self, StrError> {
        if case.symmetric() {
            if tt[1][0] != tt[0][1] || tt[2][1] != tt[1][2] || tt[2][0] != tt[0][2] {
                return Err("symmetric Tensor2 does not pass symmetry check");
            }
        }
        let dim = case.dim();
        if case.dim() == 4 {
            if tt[1][2] != 0.0 || tt[0][2] != 0.0 {
                return Err("cannot define 2D Tensor2 due to non-zero off-diagonal values");
            }
        }
        let mut vec = Vector::new(dim);
        for m in 0..dim {
            let (i, j) = M_TO_IJ[m];
            if i == j {
                vec[m] = tt[i][j];
            }
            if i < j {
                vec[m] = (tt[i][j] + tt[j][i]) / SQRT_2;
            }
            if i > j {
                vec[m] = (tt[j][i] - tt[i][j]) / SQRT_2;
            }
        }
        Ok(Tensor2 { vec })
    }

    /// Tells whether this tensor is symmetric or not
    #[inline]
    pub fn symmetric(&self) -> bool {
        self.vec.dim() != 9
    }

    /// Tells whether this tensor is 2D or not
    #[inline]
    pub fn two_dim(&self) -> bool {
        self.vec.dim() == 4
    }

    /// Returns the (i,j) component (standard; not Mandel)
    ///
    /// # Example
    ///
    /// ```
    /// use russell_chk::approx_eq;
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [1.0,  2.0, 0.0],
    ///         [3.0, -1.0, 5.0],
    ///         [0.0,  4.0, 1.0],
    ///     ], Mandel::General)?;
    ///
    ///     approx_eq(a.get(1,2), 5.0, 1e-15);
    ///     Ok(())
    /// }
    /// ```
    pub fn get(&self, i: usize, j: usize) -> f64 {
        match self.vec.dim() {
            4 => {
                let m = IJ_TO_M_SYM[i][j];
                if m > 3 {
                    0.0
                } else if i == j {
                    self.vec[m]
                } else {
                    self.vec[m] / SQRT_2
                }
            }
            6 => {
                let m = IJ_TO_M_SYM[i][j];
                if i == j {
                    self.vec[m]
                } else {
                    self.vec[m] / SQRT_2
                }
            }
            _ => {
                let m = IJ_TO_M[i][j];
                if i == j {
                    self.vec[m]
                } else if i < j {
                    let n = IJ_TO_M[j][i];
                    (self.vec[m] + self.vec[n]) / SQRT_2
                } else {
                    let n = IJ_TO_M[j][i];
                    (self.vec[n] - self.vec[m]) / SQRT_2
                }
            }
        }
    }

    /// Returns a matrix (standard components; not Mandel) representing this tensor
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [1.0,  1.0, 0.0],
    ///         [1.0, -1.0, 0.0],
    ///         [0.0,  0.0, 1.0],
    ///     ], Mandel::Symmetric2D)?;
    ///     assert_eq!(
    ///         format!("{:.1}", a.to_matrix()),
    ///         "┌                ┐\n\
    ///          │  1.0  1.0  0.0 │\n\
    ///          │  1.0 -1.0  0.0 │\n\
    ///          │  0.0  0.0  1.0 │\n\
    ///          └                ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn to_matrix(&self) -> Matrix {
        let mut tt = Matrix::new(3, 3);
        let dim = self.vec.dim();
        if dim < 9 {
            for m in 0..dim {
                let (i, j) = M_TO_IJ[m];
                tt.set(i, j, self.get(i, j));
                if i != j {
                    tt.set(j, i, tt.get(i, j));
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    tt.set(i, j, self.get(i, j));
                }
            }
        }
        tt
    }

    /// Returns a 2x2 matrix (standard components; not Mandel) representing this tensor (2D)
    ///
    /// This function returns the third diagonal component T22 and the 2x2 matrix
    ///
    /// # Panics
    ///
    /// This function works only if the Tensor is 2D
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let tt = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 0.0],
    ///         [2.0, 3.0, 0.0],
    ///         [0.0, 0.0, 4.0],
    ///     ], Mandel::Symmetric2D)?;
    ///     let (t22, res) = tt.to_matrix_2d();
    ///     assert_eq!(t22, 4.0);
    ///     assert_eq!(
    ///         format!("{:.1}", res),
    ///         "┌         ┐\n\
    ///          │ 1.0 2.0 │\n\
    ///          │ 2.0 3.0 │\n\
    ///          └         ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn to_matrix_2d(&self) -> (f64, Matrix) {
        assert!(self.two_dim());
        let mut tt = Matrix::new(2, 2);
        tt.set(0, 0, self.get(0, 0));
        tt.set(0, 1, self.get(0, 1));
        tt.set(1, 0, self.get(1, 0));
        tt.set(1, 1, self.get(1, 1));
        (self.get(2, 2), tt)
    }

    /// Set all values to zero
    #[inline]
    pub fn clear(&mut self) {
        self.vec.fill(0.0);
    }

    /// Sets the (i,j) component of a symmetric Tensor2
    ///
    /// ```text
    /// σᵢⱼ = value
    /// ```
    ///
    /// **Note:** Only the diagonal and upper-diagonal components need to be set.
    ///
    /// # Panics
    ///
    /// The tensor must be symmetric and (i,j) must correspond to the possible
    /// combination due to the space dimension, otherwise a panic may occur.
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() {
    ///     let mut a = Tensor2::new(Mandel::Symmetric2D);
    ///     a.sym_set(0, 0, 1.0);
    ///     a.sym_set(1, 1, 2.0);
    ///     a.sym_set(2, 2, 3.0);
    ///     a.sym_set(0, 1, 4.0);
    ///     assert_eq!(
    ///         format!("{:.1}", a.to_matrix()),
    ///         "┌             ┐\n\
    ///          │ 1.0 4.0 0.0 │\n\
    ///          │ 4.0 2.0 0.0 │\n\
    ///          │ 0.0 0.0 3.0 │\n\
    ///          └             ┘"
    ///     );
    ///
    ///     let mut b = Tensor2::new(Mandel::Symmetric);
    ///     b.sym_set(0, 0, 1.0);
    ///     b.sym_set(1, 1, 2.0);
    ///     b.sym_set(2, 2, 3.0);
    ///     b.sym_set(0, 1, 4.0);
    ///     b.sym_set(1, 0, 4.0);
    ///     b.sym_set(2, 0, 5.0);
    ///     assert_eq!(
    ///         format!("{:.1}", b.to_matrix()),
    ///         "┌             ┐\n\
    ///          │ 1.0 4.0 5.0 │\n\
    ///          │ 4.0 2.0 0.0 │\n\
    ///          │ 5.0 0.0 3.0 │\n\
    ///          └             ┘"
    ///     );
    /// }
    /// ```
    pub fn sym_set(&mut self, i: usize, j: usize, value: f64) {
        let m = IJ_TO_M_SYM[i][j];
        if i == j {
            self.vec[m] = value;
        } else {
            self.vec[m] = value * SQRT_2;
        }
    }

    /// Updates the (i,j) component of a symmetric Tensor2
    ///
    /// ```text
    /// σᵢⱼ += α·value
    /// ```
    ///
    /// **Note:** Only the diagonal and upper-diagonal components **must** be set.
    ///
    /// # Panics
    ///
    /// The tensor must be symmetric and (i,j) must correspond to the possible
    /// combination due to the space dimension, otherwise a panic may occur.
    ///
    /// This function will panic also if i > j (lower-diagonal)
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let mut a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [2.0, 5.0, 6.0],
    ///         [3.0, 6.0, 9.0],
    ///     ], Mandel::Symmetric)?;
    ///
    ///     a.sym_add(0, 1, 2.0, 10.0);
    ///
    ///     assert_eq!(
    ///         format!("{:.1}", a.to_matrix()),
    ///         "┌                ┐\n\
    ///          │  1.0 22.0  3.0 │\n\
    ///          │ 22.0  5.0  6.0 │\n\
    ///          │  3.0  6.0  9.0 │\n\
    ///          └                ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn sym_add(&mut self, i: usize, j: usize, alpha: f64, value: f64) {
        assert!(self.symmetric());
        assert!(i <= j);
        let m = IJ_TO_M_SYM[i][j];
        if i == j {
            self.vec[m] += alpha * value;
        } else {
            self.vec[m] += alpha * value * SQRT_2;
        }
    }

    /// Sets this tensor equal to another one
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let mut a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [7.0, 8.0, 9.0],
    ///     ], Mandel::General)?;
    ///     let b = Tensor2::from_matrix(&[
    ///         [10.0, 20.0, 30.0],
    ///         [40.0, 50.0, 60.0],
    ///         [70.0, 80.0, 90.0],
    ///     ], Mandel::General)?;
    ///
    ///     a.set(&b);
    ///
    ///     assert_eq!(
    ///         format!("{:.1}", a.to_matrix()),
    ///         "┌                ┐\n\
    ///          │ 10.0 20.0 30.0 │\n\
    ///          │ 40.0 50.0 60.0 │\n\
    ///          │ 70.0 80.0 90.0 │\n\
    ///          └                ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn set(&mut self, other: &Tensor2) -> Result<(), StrError> {
        vec_copy(&mut self.vec, &other.vec)
    }

    /// Adds another tensor to this one
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let mut a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [7.0, 8.0, 9.0],
    ///     ], Mandel::General)?;
    ///     let b = Tensor2::from_matrix(&[
    ///         [10.0, 20.0, 30.0],
    ///         [40.0, 50.0, 60.0],
    ///         [70.0, 80.0, 90.0],
    ///     ], Mandel::General)?;
    ///
    ///     a.add(2.0, &b);
    ///
    ///     assert_eq!(
    ///         format!("{:.1}", a.to_matrix()),
    ///         "┌                   ┐\n\
    ///          │  21.0  42.0  63.0 │\n\
    ///          │  84.0 105.0 126.0 │\n\
    ///          │ 147.0 168.0 189.0 │\n\
    ///          └                   ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn add(&mut self, alpha: f64, other: &Tensor2) -> Result<(), StrError> {
        vec_update(&mut self.vec, alpha, &other.vec)
    }

    /// Calculates the determinant
    ///
    /// # Example
    ///
    /// ```
    /// use russell_chk::approx_eq;
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [7.0, 8.0, 9.0],
    ///     ], Mandel::General)?;
    ///
    ///     approx_eq(a.determinant(), 0.0, 1e-13);
    ///     Ok(())
    /// }
    /// ```
    pub fn determinant(&self) -> f64 {
        let a = &self.vec;
        match self.vec.dim() {
            4 => a[0] * a[1] * a[2] - (a[2] * a[3] * a[3]) / 2.0,
            6 => {
                a[0] * a[1] * a[2] - (a[2] * a[3] * a[3]) / 2.0 - (a[0] * a[4] * a[4]) / 2.0
                    + (a[3] * a[4] * a[5]) / SQRT_2
                    - (a[1] * a[5] * a[5]) / 2.0
            }
            _ => {
                a[0] * a[1] * a[2] - (a[2] * a[3] * a[3]) / 2.0 - (a[0] * a[4] * a[4]) / 2.0
                    + (a[3] * a[4] * a[5]) / SQRT_2
                    - (a[1] * a[5] * a[5]) / 2.0
                    + (a[2] * a[6] * a[6]) / 2.0
                    + (a[5] * a[6] * a[7]) / SQRT_2
                    + (a[0] * a[7] * a[7]) / 2.0
                    - (a[4] * a[6] * a[8]) / SQRT_2
                    - (a[3] * a[7] * a[8]) / SQRT_2
                    + (a[1] * a[8] * a[8]) / 2.0
            }
        }
    }

    /// Calculates the trace
    ///
    /// ```text
    /// tr(σ) = σ:I = Σᵢ σᵢᵢ
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use russell_chk::approx_eq;
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [7.0, 8.0, 9.0],
    ///     ], Mandel::General)?;
    ///
    ///     approx_eq(a.trace(), 15.0, 1e-15);
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn trace(&self) -> f64 {
        self.vec[0] + self.vec[1] + self.vec[2]
    }

    /// Calculates the Euclidean norm
    ///
    /// ```text
    /// norm(σ) = √(σ:σ)
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use russell_chk::approx_eq;
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [7.0, 8.0, 9.0],
    ///     ], Mandel::General)?;
    ///
    ///     approx_eq(a.norm(), f64::sqrt(285.0), 1e-13);
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn norm(&self) -> f64 {
        let mut sm = self.vec[0] * self.vec[0]
            + self.vec[1] * self.vec[1]
            + self.vec[2] * self.vec[2]
            + self.vec[3] * self.vec[3];
        let dim = self.vec.dim();
        if dim > 4 {
            sm += self.vec[4] * self.vec[4] + self.vec[5] * self.vec[5];
        }
        if dim > 6 {
            sm += self.vec[6] * self.vec[6] + self.vec[7] * self.vec[7] + self.vec[8] * self.vec[8];
        }
        f64::sqrt(sm)
    }

    /// Calculates the deviator tensor
    ///
    /// ```text
    /// dev(σ) = σ - ⅓ tr(σ) I
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use russell_chk::approx_eq;
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [1.0, 2.0, 3.0],
    ///         [4.0, 5.0, 6.0],
    ///         [7.0, 8.0, 9.0],
    ///     ], Mandel::General)?;
    ///
    ///     let mut dev = Tensor2::new(Mandel::General);
    ///     a.deviator(&mut dev).unwrap();
    ///     approx_eq(dev.trace(), 0.0, 1e-15);
    ///
    ///     assert_eq!(
    ///         format!("{:.1}", dev.to_matrix()),
    ///         "┌                ┐\n\
    ///          │ -4.0  2.0  3.0 │\n\
    ///          │  4.0  0.0  6.0 │\n\
    ///          │  7.0  8.0  4.0 │\n\
    ///          └                ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn deviator(&self, dev: &mut Tensor2) -> Result<(), StrError> {
        vec_copy(&mut dev.vec, &self.vec)?;
        let m = (self.vec[0] + self.vec[1] + self.vec[2]) / 3.0;
        dev.vec[0] -= m;
        dev.vec[1] -= m;
        dev.vec[2] -= m;
        Ok(())
    }

    /// Calculates the norm of the deviator tensor
    ///
    /// ```text
    /// || σ - ⅓ tr(σ) I ||
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use russell_chk::approx_eq;
    /// use russell_tensor::{Mandel, Tensor2, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let a = Tensor2::from_matrix(&[
    ///         [6.0,  1.0,  2.0],
    ///         [3.0, 12.0,  4.0],
    ///         [5.0,  6.0, 15.0],
    ///     ], Mandel::General)?;
    ///
    ///     let mut dev = Tensor2::new(Mandel::General);
    ///     a.deviator(&mut dev).unwrap();
    ///     approx_eq(dev.trace(), 0.0, 1e-15);
    ///
    ///     assert_eq!(
    ///         format!("{:.1}", dev.to_matrix()),
    ///         "┌                ┐\n\
    ///          │ -5.0  1.0  2.0 │\n\
    ///          │  3.0  1.0  4.0 │\n\
    ///          │  5.0  6.0  4.0 │\n\
    ///          └                ┘"
    ///     );
    ///
    ///     approx_eq(dev.norm(), f64::sqrt(133.0), 1e-15);
    ///     approx_eq(a.deviator_norm(), f64::sqrt(133.0), 1e-15);
    ///     Ok(())
    /// }
    /// ```
    pub fn deviator_norm(&self) -> f64 {
        let mut sm = self.vec[3] * self.vec[3]
            + (self.vec[0] - self.vec[1]) * (self.vec[0] - self.vec[1]) / 3.0
            + (self.vec[1] - self.vec[2]) * (self.vec[1] - self.vec[2]) / 3.0
            + (self.vec[2] - self.vec[0]) * (self.vec[2] - self.vec[0]) / 3.0;
        let dim = self.vec.dim();
        if dim > 4 {
            sm += self.vec[4] * self.vec[4] + self.vec[5] * self.vec[5];
        }
        if dim > 6 {
            sm += self.vec[6] * self.vec[6] + self.vec[7] * self.vec[7] + self.vec[8] * self.vec[8];
        }
        f64::sqrt(sm)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Tensor2, SQRT_2};
    use crate::Mandel;
    use russell_chk::{approx_eq, vec_approx_eq};
    use serde::{Deserialize, Serialize};

    #[test]
    fn new_works() {
        // general
        let tt = Tensor2::new(Mandel::General);
        let correct = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(tt.vec.as_data(), correct);
        assert_eq!(tt.symmetric(), false);
        assert_eq!(tt.two_dim(), false);

        // symmetric 3D
        let tt = Tensor2::new(Mandel::Symmetric);
        let correct = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(tt.vec.as_data(), correct);
        assert_eq!(tt.symmetric(), true);
        assert_eq!(tt.two_dim(), false);

        // symmetric 2D
        let tt = Tensor2::new(Mandel::Symmetric2D);
        let correct = &[0.0, 0.0, 0.0, 0.0];
        assert_eq!(tt.vec.as_data(), correct);
        assert_eq!(tt.symmetric(), true);
        assert_eq!(tt.two_dim(), true);
    }

    #[test]
    fn from_matrix_works() {
        // general
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
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
        vec_approx_eq(tt.vec.as_data(), correct, 1e-15);

        // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        let correct = &[1.0, 2.0, 3.0, 4.0 * SQRT_2, 5.0 * SQRT_2, 6.0 * SQRT_2];
        vec_approx_eq(tt.vec.as_data(), correct, 1e-14);

        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        let correct = &[1.0, 2.0, 3.0, 4.0 * SQRT_2];
        vec_approx_eq(tt.vec.as_data(), correct, 1e-14);
    }

    #[test]
    fn from_matrix_captures_errors() {
        // symmetric 3D
        let eps = 1e-15;
        #[rustfmt::skip]
        let comps_std_10 = &[
            [1.0, 4.0, 6.0],
            [4.0+eps, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        #[rustfmt::skip]
        let comps_std_20 = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0+eps, 5.0, 3.0],
        ];
        #[rustfmt::skip]
        let comps_std_21 = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0+eps, 3.0],
        ];
        assert_eq!(
            Tensor2::from_matrix(comps_std_10, Mandel::Symmetric).err(),
            Some("symmetric Tensor2 does not pass symmetry check")
        );
        assert_eq!(
            Tensor2::from_matrix(comps_std_20, Mandel::Symmetric).err(),
            Some("symmetric Tensor2 does not pass symmetry check")
        );
        assert_eq!(
            Tensor2::from_matrix(comps_std_21, Mandel::Symmetric).err(),
            Some("symmetric Tensor2 does not pass symmetry check")
        );

        // symmetric 2D
        let eps = 1e-15;
        #[rustfmt::skip]
        let comps_std_12 = &[
            [1.0,     4.0, 0.0+eps],
            [4.0,     2.0, 0.0],
            [0.0+eps, 0.0, 3.0],
        ];
        #[rustfmt::skip]
        let comps_std_02 = &[
            [1.0, 4.0,     0.0],
            [4.0, 2.0,     0.0+eps],
            [0.0, 0.0+eps, 3.0],
        ];
        assert_eq!(
            Tensor2::from_matrix(comps_std_12, Mandel::Symmetric2D).err(),
            Some("cannot define 2D Tensor2 due to non-zero off-diagonal values")
        );
        assert_eq!(
            Tensor2::from_matrix(comps_std_02, Mandel::Symmetric2D).err(),
            Some("cannot define 2D Tensor2 due to non-zero off-diagonal values")
        );
    }

    #[test]
    fn get_works() {
        // general
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                approx_eq(tt.get(i, j), comps_std[i][j], 1e-14);
            }
        }

        // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                approx_eq(tt.get(i, j), comps_std[i][j], 1e-14);
            }
        }

        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                approx_eq(tt.get(i, j), comps_std[i][j], 1e-14);
            }
        }
    }

    #[test]
    fn to_matrix_works() {
        // general
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        let res = tt.to_matrix();
        for i in 0..3 {
            for j in 0..3 {
                approx_eq(res.get(i, j), comps_std[i][j], 1e-14);
            }
        }

        // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        let res = tt.to_matrix();
        for i in 0..3 {
            for j in 0..3 {
                approx_eq(res.get(i, j), comps_std[i][j], 1e-14);
            }
        }

        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        let res = tt.to_matrix();
        for i in 0..3 {
            for j in 0..3 {
                approx_eq(res.get(i, j), comps_std[i][j], 1e-14);
            }
        }
    }

    #[test]
    #[should_panic]
    fn to_matrix_2d_panics_on_3d_case() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        tt.to_matrix_2d();
    }

    #[test]
    fn to_matrix_2d_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        let (t22, res) = tt.to_matrix_2d();
        assert_eq!(t22, 3.0);
        assert_eq!(
            format!("{:.1}", res),
            "┌         ┐\n\
             │ 1.0 4.0 │\n\
             │ 4.0 2.0 │\n\
             └         ┘"
        );

        #[rustfmt::skip]
        let data = &[
            [1.0, 2.0, 0.0],
            [2.0, 3.0, 0.0],
            [0.0, 0.0, 4.0],
        ];
        let tt = Tensor2::from_matrix(data, Mandel::Symmetric2D).unwrap();
        let (t22, a) = tt.to_matrix_2d();
        assert_eq!(t22, 4.0);
        assert_eq!(
            format!("{:.1}", a),
            "┌         ┐\n\
             │ 1.0 2.0 │\n\
             │ 2.0 3.0 │\n\
             └         ┘"
        );
    }

    #[test]
    fn sym_set_works() {
        let mut a = Tensor2::new(Mandel::Symmetric);
        a.sym_set(0, 0, 1.0);
        a.sym_set(1, 1, 2.0);
        a.sym_set(2, 2, 3.0);
        a.sym_set(0, 1, 4.0);
        a.sym_set(1, 0, 4.0);
        a.sym_set(2, 0, 5.0);
        let out = a.to_matrix();
        assert_eq!(
            format!("{:.1}", out),
            "┌             ┐\n\
             │ 1.0 4.0 5.0 │\n\
             │ 4.0 2.0 0.0 │\n\
             │ 5.0 0.0 3.0 │\n\
             └             ┘"
        );
    }

    #[test]
    fn clear_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let mut a = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        a.clear();
        assert_eq!(a.vec.as_data(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[should_panic]
    fn sym_add_panics_on_non_sym() {
        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let mut a = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        a.sym_add(0, 0, 1.0, 1.0);
    }

    #[test]
    #[should_panic]
    fn sym_add_panics_on_i_greater_than_j() {
        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let mut a = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        a.sym_add(1, 0, 1.0, 1.0);
    }

    #[test]
    fn sym_add_works() {
        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let mut a = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        a.sym_add(0, 0, 10.0, 10.0);
        a.sym_add(1, 1, 10.0, 10.0);
        a.sym_add(2, 2, 10.0, 10.0);
        a.sym_add(0, 1, 10.0, 10.0); // must not do (1,0)
        let out = a.to_matrix();
        assert_eq!(
            format!("{:.1}", out),
            "┌                   ┐\n\
             │ 101.0 104.0   0.0 │\n\
             │ 104.0 102.0   0.0 │\n\
             │   0.0   0.0 103.0 │\n\
             └                   ┘"
        );

        // // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let mut a = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        a.sym_add(0, 0, 10.0, 10.0);
        a.sym_add(1, 1, 10.0, 10.0);
        a.sym_add(2, 2, 10.0, 10.0);
        a.sym_add(0, 1, 10.0, 10.0); // must nod do (1,0)
        a.sym_add(0, 2, 10.0, 10.0); // must not do (2,0)
        a.sym_add(1, 2, 10.0, 10.0); // must not do (2,1)
        let out = a.to_matrix();
        assert_eq!(
            format!("{:.1}", out),
            "┌                   ┐\n\
             │ 101.0 104.0 106.0 │\n\
             │ 104.0 102.0 105.0 │\n\
             │ 106.0 105.0 103.0 │\n\
             └                   ┘"
        );
    }

    #[test]
    #[should_panic]
    fn sym_add_panics_on_lower_diagonal() {
        let mut a = Tensor2::new(Mandel::Symmetric2D);
        a.sym_add(1, 0, 1.0, 0.0);
    }

    #[test]
    fn set_and_add_work() {
        let mut a = Tensor2::new(Mandel::General);
        #[rustfmt::skip]
        let b = Tensor2::from_matrix(&[
            [1.0, 3.0, 1.0], 
            [2.0, 2.0, 2.0], 
            [3.0, 1.0, 3.0],
        ],
        Mandel::General).unwrap();
        let c = Tensor2::from_matrix(
            &[[100.0, 100.0, 100.0], [100.0, 100.0, 100.0], [100.0, 100.0, 100.0]],
            Mandel::General,
        )
        .unwrap();
        a.set(&b).unwrap();
        a.add(10.0, &c).unwrap();
        let out = a.to_matrix();
        assert_eq!(
            format!("{:.1}", out),
            "┌                      ┐\n\
             │ 1001.0 1003.0 1001.0 │\n\
             │ 1002.0 1002.0 1002.0 │\n\
             │ 1003.0 1001.0 1003.0 │\n\
             └                      ┘"
        );
    }

    #[test]
    fn clone_and_serialize_work() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        // clone
        let mut cloned = tt.clone();
        cloned.vec[0] = -1.0;
        assert_eq!(
            format!("{:.1}", tt.to_matrix()),
            "┌             ┐\n\
             │ 1.0 2.0 3.0 │\n\
             │ 4.0 5.0 6.0 │\n\
             │ 7.0 8.0 9.0 │\n\
             └             ┘"
        );
        assert_eq!(
            format!("{:.1}", cloned.to_matrix()),
            "┌                ┐\n\
             │ -1.0  2.0  3.0 │\n\
             │  4.0  5.0  6.0 │\n\
             │  7.0  8.0  9.0 │\n\
             └                ┘"
        );
        // serialize
        let mut serialized = Vec::new();
        let mut serializer = rmp_serde::Serializer::new(&mut serialized);
        tt.serialize(&mut serializer)
            .map_err(|_| "tensor serialize failed")
            .unwrap();
        assert!(serialized.len() > 0);
        // deserialize
        let mut deserializer = rmp_serde::Deserializer::new(&serialized[..]);
        let ss: Tensor2 = Deserialize::deserialize(&mut deserializer)
            .map_err(|_| "cannot deserialize tensor data")
            .unwrap();
        assert_eq!(
            format!("{:.1}", ss.to_matrix()),
            "┌             ┐\n\
             │ 1.0 2.0 3.0 │\n\
             │ 4.0 5.0 6.0 │\n\
             │ 7.0 8.0 9.0 │\n\
             └             ┘"
        );
    }

    #[test]
    fn debug_works() {
        let tt = Tensor2::new(Mandel::General);
        assert!(format!("{:?}", tt).len() > 0);
    }

    #[test]
    fn det_works() {
        // general
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        approx_eq(tt.determinant(), 0.0, 1e-13);

        // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 6.0],
            [4.0, 2.0, 5.0],
            [6.0, 5.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        approx_eq(tt.determinant(), 101.0, 1e-13);

        // symmetric 3D (another test)
        #[rustfmt::skip]
        let comps_std = &[
            [ 1.0, -3.0, 4.0],
            [-3.0, -6.0, 1.0],
            [ 4.0,  1.0, 5.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        approx_eq(tt.determinant(), -4.0, 1e-13);

        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        approx_eq(tt.determinant(), -42.0, 1e-13);
    }

    #[test]
    fn trace_works() {
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        approx_eq(tt.trace(), 15.0, 1e-15);
    }

    #[test]
    fn norm_works() {
        // general
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        approx_eq(tt.norm(), f64::sqrt(285.0), 1e-15);

        // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [ 2.0, -3.0, 4.0],
            [-3.0, -5.0, 1.0],
            [ 4.0,  1.0, 6.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        approx_eq(tt.norm(), f64::sqrt(117.0), 1e-15);

        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        approx_eq(tt.norm(), f64::sqrt(46.0), 1e-15);
    }

    #[test]
    fn deviator_catches_errors() {
        let tt = Tensor2::new(Mandel::General);
        let mut dev = Tensor2::new(Mandel::Symmetric);
        assert_eq!(tt.deviator(&mut dev).err(), Some("vectors are incompatible"));
    }

    #[test]
    fn deviator_and_norm_work() {
        // general
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::General).unwrap();
        let mut dev = Tensor2::new(Mandel::General);
        tt.deviator(&mut dev).unwrap();
        approx_eq(dev.trace(), 0.0, 1e-15);
        assert_eq!(
            format!("{:.1}", dev.to_matrix()),
            "┌                ┐\n\
             │ -4.0  2.0  3.0 │\n\
             │  4.0  0.0  6.0 │\n\
             │  7.0  8.0  4.0 │\n\
             └                ┘"
        );
        approx_eq(dev.norm(), tt.deviator_norm(), 1e-15);

        // symmetric 3D
        #[rustfmt::skip]
        let comps_std = &[
            [ 2.0, -3.0, 4.0],
            [-3.0, -5.0, 1.0],
            [ 4.0,  1.0, 6.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric).unwrap();
        let mut dev = Tensor2::new(Mandel::Symmetric);
        tt.deviator(&mut dev).unwrap();
        approx_eq(dev.trace(), 0.0, 1e-15);
        assert_eq!(
            format!("{:.1}", dev.to_matrix()),
            "┌                ┐\n\
             │  1.0 -3.0  4.0 │\n\
             │ -3.0 -6.0  1.0 │\n\
             │  4.0  1.0  5.0 │\n\
             └                ┘"
        );
        approx_eq(dev.norm(), tt.deviator_norm(), 1e-14);

        // symmetric 2D
        #[rustfmt::skip]
        let comps_std = &[
            [1.0, 4.0, 0.0],
            [4.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let tt = Tensor2::from_matrix(comps_std, Mandel::Symmetric2D).unwrap();
        let mut dev = Tensor2::new(Mandel::Symmetric2D);
        tt.deviator(&mut dev).unwrap();
        approx_eq(dev.trace(), 0.0, 1e-15);
        assert_eq!(
            format!("{:.1}", dev.to_matrix()),
            "┌                ┐\n\
             │ -1.0  4.0  0.0 │\n\
             │  4.0  0.0  0.0 │\n\
             │  0.0  0.0  1.0 │\n\
             └                ┘"
        );
        approx_eq(dev.norm(), tt.deviator_norm(), 1e-15);
    }
}
