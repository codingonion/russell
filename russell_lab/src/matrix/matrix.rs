use russell_openblas::*;
use std::cmp;
use std::convert::TryInto;
use std::fmt::{self, Write};

pub struct Matrix {
    pub(crate) nrow: usize,    // number of rows
    pub(crate) ncol: usize,    // number of columns
    pub(crate) data: Vec<f64>, // col-major => Fortran
}

/// Holds matrix components
///
/// # Note
///
/// Data is stored in col-major format
///
/// Example of col-major data:
///
/// ```text
///       _      _
///      |  0  3  |
///  A = |  1  4  |            ⇒     a = [0, 1, 2, 3, 4, 5]
///      |_ 2  5 _|(m x n)
///
///  a[i+j*m] = A[i][j]
/// ```
///
impl Matrix {
    /// Creates new (nrow x ncol) Matrix filled with zeros
    ///
    /// ```
    /// use russell_lab::*;
    /// let a = Matrix::new(3, 3);
    /// let correct = "┌       ┐\n\
    ///                │ 0 0 0 │\n\
    ///                │ 0 0 0 │\n\
    ///                │ 0 0 0 │\n\
    ///                └       ┘";
    /// assert_eq!(format!("{}", a), correct);
    /// ```
    pub fn new(nrow: usize, ncol: usize) -> Self {
        Matrix {
            nrow,
            ncol,
            data: vec![0.0; nrow * ncol],
        }
    }

    /// Creates new matrix from given data
    ///
    /// # Examples
    ///
    /// ```
    /// use russell_lab::*;
    /// let a = Matrix::from(&[
    ///     &[1.0, 2.0, 3.0],
    ///     &[4.0, 5.0, 6.0],
    ///     &[7.0, 8.0, 9.0],
    /// ]);
    /// let correct = "┌       ┐\n\
    ///                │ 1 2 3 │\n\
    ///                │ 4 5 6 │\n\
    ///                │ 7 8 9 │\n\
    ///                └       ┘";
    /// assert_eq!(format!("{}", a), correct);
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there are rows with different number of columns
    ///
    pub fn from(data: &[&[f64]]) -> Self {
        let nrow = data.len();
        if nrow == 0 {
            return Matrix {
                nrow: 0,
                ncol: 0,
                data: Vec::new(),
            };
        }
        let ncol = data[0].len();
        let mut matrix = Matrix {
            nrow,
            ncol,
            data: vec![0.0; nrow * ncol],
        };
        for i in 0..nrow {
            if data[i].len() != ncol {
                panic!("all rows must have the same number of columns");
            }
            for j in 0..ncol {
                matrix.data[i + j * nrow] = data[i][j];
            }
        }
        matrix
    }

    /// Returns the number of rows
    ///
    /// # Examples
    ///
    /// ```
    /// use russell_lab::*;
    /// let a = Matrix::new(4, 3);
    /// assert_eq!(a.nrow(), 4);
    /// ```
    pub fn nrow(&self) -> usize {
        self.nrow
    }

    /// Returns the number of columns
    ///
    /// # Examples
    ///
    /// ```
    /// use russell_lab::*;
    /// let a = Matrix::new(4, 3);
    /// assert_eq!(a.ncol(), 3);
    /// ```
    pub fn ncol(&self) -> usize {
        self.ncol
    }

    /// Returns the dimensions (nrow, ncol) of this matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use russell_lab::*;
    /// let a = Matrix::new(4, 3);
    /// assert_eq!(a.dims(), (4, 3));
    /// ```
    pub fn dims(&self) -> (usize, usize) {
        (self.nrow, self.ncol)
    }

    /// Scales this matrix
    ///
    /// ```text
    /// a := alpha * a
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use russell_lab::*;
    /// let mut a = Matrix::from(&[
    ///     &[1.0, 2.0, 3.0],
    ///     &[4.0, 5.0, 6.0],
    /// ]);
    /// a.scale(0.5);
    /// let correct = "┌             ┐\n\
    ///                │ 0.5   1 1.5 │\n\
    ///                │   2 2.5   3 │\n\
    ///                └             ┘";
    /// assert_eq!(format!("{}", a), correct);
    /// ```
    ///
    pub fn scale(&mut self, alpha: f64) {
        let n: i32 = self.data.len().try_into().unwrap();
        dscal(n, alpha, &mut self.data, 1);
    }
}

impl fmt::Display for Matrix {
    /// Implements the Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // find largest width
        let mut width = 0;
        let mut buf = String::new();
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let val = self.data[i + j * self.nrow];
                write!(&mut buf, "{}", val)?;
                width = cmp::max(buf.chars().count(), width);
                buf.clear();
            }
        }
        width += 1;
        write!(f, "┌{:1$}┐\n", " ", width * self.ncol + 1)?;
        for i in 0..self.nrow {
            if i > 0 {
                write!(f, " │\n")?;
            }
            for j in 0..self.ncol {
                if j == 0 {
                    write!(f, "│")?;
                }
                let val = self.data[i + j * self.nrow];
                write!(f, "{:>1$}", val, width)?;
            }
        }
        write!(f, " │\n")?;
        write!(f, "└{:1$}┘", " ", width * self.ncol + 1)?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use russell_chk::*;

    #[test]
    fn new_works() {
        let u = Matrix::new(3, 3);
        let correct = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_vec_approx_eq!(u.data, correct, 1e-15);
    }

    #[test]
    fn from_works() {
        #[rustfmt::skip]
        let a = Matrix::from(&[
            &[1.0, 2.0, 3.0],
            &[4.0, 5.0, 6.0],
            &[7.0, 8.0, 9.0],
        ]);
        let correct = &[1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0];
        assert_vec_approx_eq!(a.data, correct, 1e-15);
    }

    #[test]
    #[should_panic(expected = "all rows must have the same number of columns")]
    fn from_panics_on_wrong_columns() {
        #[rustfmt::skip]
         Matrix::from(&[
            &[1.0, 2.0, 3.0],
            &[4.0, 5.0],
            &[7.0, 8.0, 8.0],
        ]);
    }

    #[test]
    fn nrow_works() {
        let a = Matrix::new(4, 3);
        assert_eq!(a.nrow(), 4);
    }

    #[test]
    fn ncol_works() {
        let a = Matrix::new(4, 3);
        assert_eq!(a.ncol(), 3);
    }

    #[test]
    fn dims_works() {
        let a = Matrix::new(5, 4);
        assert_eq!(a.dims(), (5, 4));
    }

    #[test]
    fn display_trait_works() {
        #[rustfmt::skip]
        let a = Matrix::from(&[
            &[1.0, 2.0, 3.0],
            &[4.0, 5.0, 6.0],
            &[7.0, 8.0, 9.0],
        ]);
        let correct = "┌       ┐\n\
                            │ 1 2 3 │\n\
                            │ 4 5 6 │\n\
                            │ 7 8 9 │\n\
                            └       ┘";
        assert_eq!(format!("{}", a), correct);
    }

    #[test]
    fn scale_works() {
        #[rustfmt::skip]
        let mut a = Matrix::from(&[
            &[ 6.0,  9.0,  12.0],
            &[-6.0, -9.0, -12.0],
        ]);
        a.scale(1.0 / 3.0);
        #[rustfmt::skip]
        let correct = slice_to_colmajor(&[
            &[ 2.0,  3.0,  4.0],
            &[-2.0, -3.0, -4.0],
        ]);
        assert_vec_approx_eq!(a.data, correct, 1e-15);
    }
}