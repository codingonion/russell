use super::ComplexMatrix;
use crate::{to_i32, Complex64, StrError, CBLAS_COL_MAJOR, CBLAS_NO_TRANS};

extern "C" {
    // Performs the matrix-matrix multiplication
    // <https://www.netlib.org/lapack/explore-html/d7/d76/zgemm_8f.html>
    fn cblas_zgemm(
        layout: i32,
        transa: i32,
        transb: i32,
        m: i32,
        n: i32,
        k: i32,
        alpha: *const Complex64,
        a: *const Complex64,
        lda: i32,
        b: *const Complex64,
        ldb: i32,
        beta: *const Complex64,
        c: *mut Complex64,
        ldc: i32,
    );
}

/// (zgemm) Performs the matrix-matrix multiplication resulting in a matrix
///
/// ```text
///   c  :=  α   a   ⋅   b  +  β  c
/// (m,n)      (m,k)   (k,n)    (m,n)
/// ```
///
/// See also: <https://www.netlib.org/lapack/explore-html/d7/d76/zgemm_8f.html>
///
/// # Examples
///
/// ```
/// use russell_lab::*;
///
/// fn main() -> Result<(), StrError> {
///     let a = ComplexMatrix::from(&[
///         // 3 x 2
///         [1.0, 2.0],
///         [3.0, 4.0],
///         [5.0, 6.0],
///     ]);
///     let b = ComplexMatrix::from(&[
///         // 2 x 3
///         [-1.0, -2.0, -3.0],
///         [-4.0, -5.0, -6.0],
///     ]);
///     //   c  := α  a  ⋅  b
///     // (3,3)    (3,2) (2,3)
///     let mut c = ComplexMatrix::new(3, 3);
///     let alpha = cpx!(1.0, 0.0);
///     let beta = cpx!(0.0, 0.0);
///     complex_mat_mat_mul(&mut c, alpha, &a, &b, beta)?;
///     let correct = "┌                      ┐\n\
///                    │  -9+0i -12+0i -15+0i │\n\
///                    │ -19+0i -26+0i -33+0i │\n\
///                    │ -29+0i -40+0i -51+0i │\n\
///                    └                      ┘";
///     assert_eq!(format!("{}", c), correct);
///     Ok(())
/// }
/// ```
pub fn complex_mat_mat_mul(
    c: &mut ComplexMatrix,
    alpha: Complex64,
    a: &ComplexMatrix,
    b: &ComplexMatrix,
    beta: Complex64,
) -> Result<(), StrError> {
    let (m, n) = c.dims();
    let k = a.ncol();
    if a.nrow() != m || b.nrow() != k || b.ncol() != n {
        return Err("matrices are incompatible");
    }
    if m == 0 || n == 0 {
        return Ok(());
    }
    if k == 0 {
        let zero = Complex64::new(0.0, 0.0);
        c.fill(zero);
        return Ok(());
    }
    let m_i32: i32 = to_i32(m);
    let n_i32: i32 = to_i32(n);
    let k_i32: i32 = to_i32(k);
    let lda = m_i32;
    let ldb = k_i32;
    unsafe {
        cblas_zgemm(
            CBLAS_COL_MAJOR,
            CBLAS_NO_TRANS,
            CBLAS_NO_TRANS,
            m_i32,
            n_i32,
            k_i32,
            &alpha,
            a.as_data().as_ptr(),
            lda,
            b.as_data().as_ptr(),
            ldb,
            &beta,
            c.as_mut_data().as_mut_ptr(),
            m_i32,
        );
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::complex_mat_mat_mul;
    use crate::{complex_mat_approx_eq, cpx, Complex64, ComplexMatrix};

    #[test]
    fn mat_mat_mul_fails_on_wrong_dims() {
        let a_2x1 = ComplexMatrix::new(2, 1);
        let a_1x2 = ComplexMatrix::new(1, 2);
        let b_2x1 = ComplexMatrix::new(2, 1);
        let b_1x3 = ComplexMatrix::new(1, 3);
        let mut c_2x2 = ComplexMatrix::new(2, 2);
        let alpha = cpx!(1.0, 0.0);
        let beta = cpx!(0.0, 0.0);
        assert_eq!(
            complex_mat_mat_mul(&mut c_2x2, alpha, &a_2x1, &b_2x1, beta),
            Err("matrices are incompatible")
        );
        assert_eq!(
            complex_mat_mat_mul(&mut c_2x2, alpha, &a_1x2, &b_2x1, beta),
            Err("matrices are incompatible")
        );
        assert_eq!(
            complex_mat_mat_mul(&mut c_2x2, alpha, &a_2x1, &b_1x3, beta),
            Err("matrices are incompatible")
        );
    }

    #[test]
    fn mat_mat_mul_0x0_works() {
        let a = ComplexMatrix::new(0, 0);
        let b = ComplexMatrix::new(0, 0);
        let mut c = ComplexMatrix::new(0, 0);
        let alpha = cpx!(2.0, 0.0);
        let beta = cpx!(0.0, 0.0);
        complex_mat_mat_mul(&mut c, alpha, &a, &b, beta).unwrap();

        let a = ComplexMatrix::new(1, 0);
        let b = ComplexMatrix::new(0, 1);
        let mut c = ComplexMatrix::from(&[[cpx!(123.0, 456.0)]]);
        complex_mat_mat_mul(&mut c, alpha, &a, &b, beta).unwrap();
        let correct = &[
            [cpx!(0.0, 0.0)], //
        ];
        complex_mat_approx_eq(&c, correct, 1e-15);
    }

    #[test]
    fn mat_mat_mul_works_1() {
        let a = ComplexMatrix::from(&[
            // 2 x 3
            [1.0, 2.00, 3.0],
            [0.5, 0.75, 1.5],
        ]);
        let b = ComplexMatrix::from(&[
            // 3 x 4
            [0.1, 0.5, 0.5, 0.75],
            [0.2, 2.0, 2.0, 2.00],
            [0.3, 0.5, 0.5, 0.50],
        ]);
        let mut c = ComplexMatrix::new(2, 4);
        // c := 2⋅a⋅b
        let alpha = cpx!(2.0, 0.0);
        let beta = cpx!(0.0, 0.0);
        complex_mat_mat_mul(&mut c, alpha, &a, &b, beta).unwrap();
        #[rustfmt::skip]
        let correct = &[
            [cpx!(2.80,0.0), cpx!(12.0,0.0), cpx!(12.0,0.0), cpx!(12.50,0.0)],
            [cpx!(1.30,0.0), cpx!( 5.0,0.0), cpx!( 5.0,0.0), cpx!( 5.25,0.0)],
        ];
        complex_mat_approx_eq(&c, correct, 1e-15);
    }

    #[test]
    fn mat_mat_mul_works_2() {
        let a = ComplexMatrix::from(&[
            // 2 x 3
            [1.0, 2.00, 3.0],
            [0.5, 0.75, 1.5],
        ]);
        let b = ComplexMatrix::from(&[
            // 3 x 4
            [0.1, 0.5, 0.5, 0.75],
            [0.2, 2.0, 2.0, 2.00],
            [0.3, 0.5, 0.5, 0.50],
        ]);
        let mut c = ComplexMatrix::filled(2, 4, cpx!(100.0, 0.0));
        // c := 2 a⋅b + 10 c
        let alpha = cpx!(2.0, 0.0);
        let beta = cpx!(10.0, 0.0);
        complex_mat_mat_mul(&mut c, alpha, &a, &b, beta).unwrap();
        #[rustfmt::skip]
        let correct = &[
            [cpx!(1002.80,0.0), cpx!(1012.0,0.0), cpx!(1012.0,0.0), cpx!(1012.50,0.0)],
            [cpx!(1001.30,0.0), cpx!(1005.0,0.0), cpx!(1005.0,0.0), cpx!(1005.25,0.0)],
        ];
        complex_mat_approx_eq(&c, correct, 1e-15);
    }
}
