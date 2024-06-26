use super::ComplexVector;
use crate::{to_i32, Complex64, StrError};

extern "C" {
    // Computes constant times a vector plus a vector
    // <https://www.netlib.org/lapack/explore-html/d7/db2/zaxpy_8f.html>
    fn cblas_zaxpy(n: i32, alpha: *const Complex64, x: *const Complex64, incx: i32, y: *mut Complex64, incy: i32);
}

/// (zaxpy) Updates vector based on another vector
///
/// ```text
/// v += α⋅u
/// ```
///
/// See also: <https://www.netlib.org/lapack/explore-html/d7/db2/zaxpy_8f.html>
///
/// # Examples
///
/// ```
/// use russell_lab::*;
///
/// fn main() -> Result<(), StrError> {
///     let u = ComplexVector::from(&[10.0, 20.0, 30.0]);
///     let mut v = ComplexVector::from(&[10.0, 20.0, 30.0]);
///     complex_vec_update(&mut v, cpx!(0.1, 0.0), &u)?;
///     let correct = "┌       ┐\n\
///                    │ 11+0i │\n\
///                    │ 22+0i │\n\
///                    │ 33+0i │\n\
///                    └       ┘";
///     assert_eq!(format!("{}", v), correct);
///     Ok(())
/// }
/// ```
pub fn complex_vec_update(v: &mut ComplexVector, alpha: Complex64, u: &ComplexVector) -> Result<(), StrError> {
    let n = v.dim();
    if u.dim() != n {
        return Err("vectors are incompatible");
    }
    let n_i32 = to_i32(n);
    unsafe {
        cblas_zaxpy(n_i32, &alpha, u.as_data().as_ptr(), 1, v.as_mut_data().as_mut_ptr(), 1);
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::complex_vec_update;
    use crate::{complex_vec_approx_eq, cpx, Complex64, ComplexVector};

    #[test]
    fn complex_vec_update_fails_on_wrong_dims() {
        let u = ComplexVector::new(4);
        let mut v = ComplexVector::new(3);
        assert_eq!(
            complex_vec_update(&mut v, cpx!(2.0, 0.0), &u),
            Err("vectors are incompatible")
        );
    }

    #[test]
    fn complex_vec_update_works() {
        // real only
        let u = ComplexVector::from(&[cpx!(10.0, 0.0), cpx!(20.0, 0.0), cpx!(30.0, 0.0)]);
        let mut v = ComplexVector::from(&[cpx!(100.0, 0.0), cpx!(200.0, 0.0), cpx!(300.0, 0.0)]);
        complex_vec_update(&mut v, cpx!(2.0, 0.0), &u).unwrap();
        let correct = &[cpx!(120.0, 0.0), cpx!(240.0, 0.0), cpx!(360.0, 0.0)];
        complex_vec_approx_eq(&v, correct, 1e-15);

        // real and imag
        let u = ComplexVector::from(&[cpx!(10.0, 3.0), cpx!(20.0, 2.0), cpx!(30.0, 1.0)]);
        let mut v = ComplexVector::from(&[cpx!(100.0, 30.0), cpx!(200.0, 20.0), cpx!(300.0, 10.0)]);
        complex_vec_update(&mut v, cpx!(2.0, -2.0), &u).unwrap();
        let correct = &[cpx!(126.0, 16.0), cpx!(244.0, -16.0), cpx!(362.0, -48.0)];
        complex_vec_approx_eq(&v, correct, 1e-15);
    }
}
