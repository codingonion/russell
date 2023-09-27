use super::{to_i32, LinSolParams, LinSolTrait, SparseMatrix, Symmetry};
use crate::StrError;
use russell_lab::Vector;

/// Opaque struct holding a C-pointer to InterfaceIntelDSS
///
/// Reference: <https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs>
#[repr(C)]
struct InterfaceIntelDSS {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    fn solver_intel_dss_new() -> *mut InterfaceIntelDSS;
    fn solver_intel_dss_drop(solver: *mut InterfaceIntelDSS);
    fn solver_intel_dss_factorize(
        solver: *mut InterfaceIntelDSS,
        // output
        determinant_coefficient: *mut f64,
        determinant_exponent: *mut f64,
        // requests
        compute_determinant: i32,
        // matrix config
        general_symmetric: i32,
        positive_definite: i32,
        ndim: i32,
        // matrix
        row_pointers: *const i32,
        col_indices: *const i32,
        values: *const f64,
    ) -> i32;
    fn solver_intel_dss_solve(solver: *mut InterfaceIntelDSS, x: *mut f64, rhs: *const f64) -> i32;
}

/// Wraps the IntelDSS solver for sparse linear systems
///
/// **Warning:** This solver does not check whether the matrix is singular or not;
/// thus it may return **incorrect results** if a singular matrix is given to factorize.
///
/// **Warning:** This solver may fail with large matrices (e.g., ATandT/pre2) and
/// may return **incorrect results**.
pub struct SolverIntelDSS {
    /// Holds a pointer to the C interface to IntelDSS
    solver: *mut InterfaceIntelDSS,

    /// Indicates whether the sparse matrix has been factorized or not
    factorized: bool,

    /// Holds the symmetry type used in the first call to factorize
    factorized_symmetry: Option<Symmetry>,

    /// Holds the matrix dimension saved in the first call to factorize
    factorized_ndim: usize,

    /// Holds the number of non-zeros saved in the first call to factorize
    factorized_nnz: usize,

    /// Holds the determinant coefficient: det = coefficient * pow(10, exponent)
    determinant_coefficient: f64,

    /// Holds the determinant exponent: det = coefficient * pow(10, exponent)
    determinant_exponent: f64,
}

impl Drop for SolverIntelDSS {
    /// Tells the c-code to release memory
    fn drop(&mut self) {
        unsafe {
            solver_intel_dss_drop(self.solver);
        }
    }
}

impl SolverIntelDSS {
    /// Allocates a new instance
    ///
    /// **Warning:** This solver does not check whether the matrix is singular or not;
    /// thus it may return **incorrect results** if a singular matrix is given to factorize.
    ///
    /// **Warning:** This solver may fail with large matrices (e.g., ATandT/pre2) and
    /// may return **incorrect results**.
    pub fn new() -> Result<Self, StrError> {
        if !cfg!(with_intel_dss) {
            return Err("This code has not been compiled with Intel DSS");
        }
        unsafe {
            let solver = solver_intel_dss_new();
            if solver.is_null() {
                return Err("c-code failed to allocate the IntelDSS solver");
            }
            Ok(SolverIntelDSS {
                solver,
                factorized: false,
                factorized_symmetry: None,
                factorized_ndim: 0,
                factorized_nnz: 0,
                determinant_coefficient: 0.0,
                determinant_exponent: 0.0,
            })
        }
    }
}

impl LinSolTrait for SolverIntelDSS {
    /// Performs the factorization (and analysis/initialization if needed)
    ///
    /// # Input
    ///
    /// * `mat` -- the coefficient matrix A (**COO** or **CSR**, but not CSC).
    ///   Also, the matrix must be square (`nrow = ncol`) and, if symmetric,
    ///   the symmetry/storage must [crate::Storage::Upper].
    /// * `params` -- configuration parameters; None => use default
    ///
    /// # Notes
    ///
    /// 1. The structure of the matrix (nrow, ncol, nnz, symmetry) must be
    ///    exactly the same among multiple calls to `factorize`. The values may differ
    ///    from call to call, nonetheless.
    /// 2. The first call to `factorize` will define the structure which must be
    ///    kept the same for the next calls.
    /// 3. If the structure of the matrix needs to be changed, the solver must
    ///    be "dropped" and a new solver allocated.
    /// 4. For symmetric matrices, `DSS` requires that the symmetry/storage be [crate::Storage::Upper].
    ///
    /// **Warning:** This solver does not check whether the matrix is singular or not;
    /// thus it may return **incorrect results** if a singular matrix is given to factorize.
    ///
    /// **Warning:** This solver may fail with large matrices (e.g., ATandT/pre2) and
    /// may return **incorrect results**.
    fn factorize(&mut self, mat: &mut SparseMatrix, params: Option<LinSolParams>) -> Result<(), StrError> {
        // get CSR matrix
        let csr = mat.get_csr_or_from_coo()?;

        // check CSR matrix
        if csr.nrow != csr.ncol {
            return Err("the matrix must be square");
        }
        csr.check_dimensions()?;

        // check already factorized data
        if self.factorized == true {
            if csr.symmetry != self.factorized_symmetry {
                return Err("subsequent factorizations must use the same matrix (symmetry differs)");
            }
            if csr.nrow != self.factorized_ndim {
                return Err("subsequent factorizations must use the same matrix (ndim differs)");
            }
            if (csr.row_pointers[csr.nrow] as usize) != self.factorized_nnz {
                return Err("subsequent factorizations must use the same matrix (nnz differs)");
            }
        } else {
            self.factorized_symmetry = csr.symmetry;
            self.factorized_ndim = csr.nrow;
            self.factorized_nnz = csr.row_pointers[csr.nrow] as usize;
            if self.factorized_nnz < self.factorized_ndim {
                return Err("for Intel DSS, nnz = row_pointers[nrow] must be ≥ nrow");
            }
        }

        // configuration parameters
        let par = if let Some(p) = params { p } else { LinSolParams::new() };

        // requests
        let calc_det = if par.compute_determinant { 1 } else { 0 };

        // extract the symmetry flags and check the storage type
        let (general_symmetric, positive_definite) = match csr.symmetry {
            Some(symmetry) => symmetry.status(false, true)?,
            None => (0, 0),
        };

        // matrix config
        let ndim = to_i32(csr.nrow)?;

        // call Intel DSS factorize
        let nnz = self.factorized_nnz;
        unsafe {
            let status = solver_intel_dss_factorize(
                self.solver,
                // output
                &mut self.determinant_coefficient,
                &mut self.determinant_exponent,
                // requests
                calc_det,
                // matrix config
                general_symmetric,
                positive_definite,
                ndim,
                // matrix
                csr.row_pointers.as_ptr(),
                csr.col_indices[0..nnz].as_ptr(),
                csr.values[0..nnz].as_ptr(),
            );
            if status != MKL_DSS_SUCCESS {
                return Err(handle_intel_dss_error_code(status));
            }
        }

        // done
        self.factorized = true;
        Ok(())
    }

    /// Computes the solution of the linear system
    ///
    /// Solves the linear system:
    ///
    /// ```text
    ///   A   · x = rhs
    /// (m,m)  (m)  (m)
    /// ```
    ///
    /// # Output
    ///
    /// * `x` -- the vector of unknown values with dimension equal to mat.nrow
    ///
    /// # Input
    ///
    /// * `mat` -- the coefficient matrix A; must be square and, if symmetric, [crate::Storage::Upper].
    /// * `rhs` -- the right-hand side vector with know values an dimension equal to mat.nrow
    /// * `_verbose` -- not used
    ///
    /// **Warning:** the matrix must be same one used in `factorize`.
    fn solve(&mut self, x: &mut Vector, mat: &SparseMatrix, rhs: &Vector, _verbose: bool) -> Result<(), StrError> {
        // check already factorized data
        if self.factorized == true {
            let (nrow, ncol, nnz, symmetry) = mat.get_info();
            if symmetry != self.factorized_symmetry {
                return Err("solve must use the same matrix (symmetry differs)");
            }
            if nrow != self.factorized_ndim || ncol != self.factorized_ndim {
                return Err("solve must use the same matrix (ndim differs)");
            }
            if nnz != self.factorized_nnz {
                return Err("solve must use the same matrix (nnz differs)");
            }
        } else {
            return Err("the function factorize must be called before solve");
        }

        // check vectors
        if x.dim() != self.factorized_ndim {
            return Err("the dimension of the vector of unknown values x is incorrect");
        }
        if rhs.dim() != self.factorized_ndim {
            return Err("the dimension of the right-hand side vector is incorrect");
        }

        // call Intel DSS solve
        unsafe {
            let status = solver_intel_dss_solve(self.solver, x.as_mut_data().as_mut_ptr(), rhs.as_data().as_ptr());
            if status != MKL_DSS_SUCCESS {
                return Err(handle_intel_dss_error_code(status));
            }
        }
        Ok(())
    }

    /// Returns the determinant
    ///
    /// Returns the three values `(mantissa, 10.0, exponent)`, such that the determinant is calculated by:
    ///
    /// ```text
    /// determinant = mantissa · pow(10.0, exponent)
    /// ```
    ///
    /// **Note:** This is only available if compute_determinant was requested.
    fn get_determinant(&self) -> (f64, f64, f64) {
        (self.determinant_coefficient, 10.0, self.determinant_exponent)
    }

    /// Returns the ordering effectively used by the solver (NOT AVAILABLE)
    fn get_effective_ordering(&self) -> String {
        "Unknown".to_string()
    }

    /// Returns the scaling effectively used by the solver (NOT AVAILABLE)
    fn get_effective_scaling(&self) -> String {
        "Unknown".to_string()
    }

    /// Returns the strategy (concerning symmetry) effectively used by the solver
    fn get_effective_strategy(&self) -> String {
        "Unknown".to_string()
    }

    /// Returns the name of this solver
    ///
    /// # Output
    ///
    /// * `IntelDSS` -- if the default system IntelDSS has been used
    /// * `IntelDSS-local` -- if the locally compiled IntelDSS has be used
    fn get_name(&self) -> String {
        if cfg!(with_intel_dss) {
            "IntelDSS".to_string()
        } else {
            "INTEL_DSS_IS_NOT_AVAILABLE".to_string()
        }
    }
}

/// Handles Intel DSS error code
pub(crate) fn handle_intel_dss_error_code(err: i32) -> StrError {
    match err {
        -1 => return "MKL_DSS_ZERO_PIVOT",
        -2 => return "MKL_DSS_OUT_OF_MEMORY",
        -3 => return "MKL_DSS_FAILURE",
        -4 => return "MKL_DSS_ROW_ERR",
        -5 => return "MKL_DSS_COL_ERR",
        -6 => return "MKL_DSS_TOO_FEW_VALUES",
        -7 => return "MKL_DSS_TOO_MANY_VALUES",
        -8 => return "MKL_DSS_NOT_SQUARE",
        -9 => return "MKL_DSS_STATE_ERR",
        -10 => return "MKL_DSS_INVALID_OPTION",
        -11 => return "MKL_DSS_OPTION_CONFLICT",
        -12 => return "MKL_DSS_MSG_LVL_ERR",
        -13 => return "MKL_DSS_TERM_LVL_ERR",
        -14 => return "MKL_DSS_STRUCTURE_ERR",
        -15 => return "MKL_DSS_REORDER_ERR",
        -16 => return "MKL_DSS_VALUES_ERR",
        17 => return "MKL_DSS_STATISTICS_INVALID_MATRIX",
        18 => return "MKL_DSS_STATISTICS_INVALID_STATE",
        19 => return "MKL_DSS_STATISTICS_INVALID_STRING",
        20 => return "MKL_DSS_REORDER1_ERR",
        21 => return "MKL_DSS_PREORDER_ERR",
        22 => return "MKL_DSS_DIAG_ERR",
        23 => return "MKL_DSS_I32BIT_ERR",
        24 => return "MKL_DSS_OOC_MEM_ERR",
        25 => return "MKL_DSS_OOC_OC_ERR",
        26 => return "MKL_DSS_OOC_RW_ERR",
        100000 => return "Error: c-code returned null pointer (IntelDSS)",
        200000 => return "Error: c-code failed to allocate memory (IntelDSS)",
        400000 => return "This code has not been compiled with Intel DSS",
        _ => return "Error: unknown error returned by c-code (IntelDSS)",
    }
}

const MKL_DSS_SUCCESS: i32 = 0;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
#[cfg(with_intel_dss)]
mod tests {
    use super::{handle_intel_dss_error_code, SolverIntelDSS};
    use crate::{CooMatrix, LinSolParams, LinSolTrait, Samples, SparseMatrix};
    use russell_chk::{approx_eq, vec_approx_eq};
    use russell_lab::{Matrix, Vector};

    #[test]
    fn new_and_drop_work() {
        // you may debug into the C-code to see that drop is working
        let solver = SolverIntelDSS::new().unwrap();
        assert!(!solver.factorized);
    }

    #[test]
    fn factorize_handles_errors() {
        let mut solver = SolverIntelDSS::new().unwrap();
        assert!(!solver.factorized);
        let (coo, _, _, _) = Samples::rectangular_1x7();
        let mut mat = SparseMatrix::from_coo(coo);
        assert_eq!(
            solver.factorize(&mut mat, None).err(),
            Some("the matrix must be square")
        );
        let coo = CooMatrix::new(1, 1, 1, None, false).unwrap();
        let mut mat = SparseMatrix::from_coo(coo);
        assert_eq!(
            solver.factorize(&mut mat, None).err(),
            Some("COO matrix: pos = nnz must be ≥ 1")
        );
        let (coo, _, _, _) = Samples::mkl_symmetric_5x5_lower(false, false, false);
        let mut mat = SparseMatrix::from_coo(coo);
        assert_eq!(
            solver.factorize(&mut mat, None).err(),
            Some("if the matrix is general symmetric, the required storage is upper triangular")
        );
    }

    #[test]
    fn factorize_works() {
        let mut solver = SolverIntelDSS::new().unwrap();
        assert!(!solver.factorized);
        let (coo, _, _, _) = Samples::umfpack_unsymmetric_5x5(false);
        let mut mat = SparseMatrix::from_coo(coo);
        let mut params = LinSolParams::new();

        let (nrow, ncol, _, _) = mat.get_info();
        let mut a = Matrix::new(nrow, ncol);
        mat.to_dense(&mut a).unwrap();
        println!("{}", a);

        params.compute_determinant = true;

        solver.factorize(&mut mat, Some(params)).unwrap();
        assert!(solver.factorized);

        assert_eq!(solver.get_effective_ordering(), "Unknown");
        assert_eq!(solver.get_effective_scaling(), "Unknown");

        let (a, b, c) = solver.get_determinant();
        let det = a * f64::powf(b, c);
        approx_eq(det, 114.0, 1e-13);

        // calling factorize again works
        solver.factorize(&mut mat, Some(params)).unwrap();
        let (a, b, c) = solver.get_determinant();
        let det = a * f64::powf(b, c);
        approx_eq(det, 114.0, 1e-13);
    }

    #[test]
    fn factorize_fails_on_singular_matrix() {
        let mut solver = SolverIntelDSS::new().unwrap();
        let mut coo = CooMatrix::new(2, 2, 2, None, false).unwrap();
        coo.put(0, 0, 1.0).unwrap();
        coo.put(1, 1, 0.0).unwrap();
        let mut mat = SparseMatrix::from_coo(coo);
        println!("Warning: Intel DSS does not detect singular matrices");
        assert_eq!(solver.factorize(&mut mat, None).err(), None);
    }

    #[test]
    fn solve_handles_errors() {
        let (coo, _, _, _) = Samples::tiny_1x1(false);
        let mut mat = SparseMatrix::from_coo(coo);
        let mut solver = SolverIntelDSS::new().unwrap();
        assert!(!solver.factorized);
        let mut x = Vector::new(2);
        let rhs = Vector::new(1);
        assert_eq!(
            solver.solve(&mut x, &mut mat, &rhs, false),
            Err("the function factorize must be called before solve")
        );
        solver.factorize(&mut mat, None).unwrap();
        assert_eq!(
            solver.solve(&mut x, &mut mat, &rhs, false),
            Err("the dimension of the vector of unknown values x is incorrect")
        );
        let mut x = Vector::new(1);
        let rhs = Vector::new(2);
        assert_eq!(
            solver.solve(&mut x, &mut mat, &rhs, false),
            Err("the dimension of the right-hand side vector is incorrect")
        );
    }

    #[test]
    fn solve_works() {
        let mut solver = SolverIntelDSS::new().unwrap();
        let (coo, _, _, _) = Samples::umfpack_unsymmetric_5x5(false);
        let mut mat = SparseMatrix::from_coo(coo);
        let mut x = Vector::new(5);
        let rhs = Vector::from(&[8.0, 45.0, -3.0, 3.0, 19.0]);
        let x_correct = &[1.0, 2.0, 3.0, 4.0, 5.0];
        solver.factorize(&mut mat, None).unwrap();
        solver.solve(&mut x, &mut mat, &rhs, false).unwrap();
        vec_approx_eq(x.as_data(), x_correct, 1e-14);

        // calling solve again works
        let mut x_again = Vector::new(5);
        solver.solve(&mut x_again, &mut mat, &rhs, false).unwrap();
        vec_approx_eq(x_again.as_data(), x_correct, 1e-14);
    }

    #[test]
    fn handle_intel_dss_error_code_works() {
        let default = "Error: unknown error returned by c-code (IntelDSS)";
        for i in 1..17 {
            let res = handle_intel_dss_error_code(-i);
            assert!(res.len() > 0);
            assert_ne!(res, default);
        }
        for i in 17..27 {
            let res = handle_intel_dss_error_code(i);
            assert!(res.len() > 0);
            assert_ne!(res, default);
        }
        assert_eq!(
            handle_intel_dss_error_code(100000),
            "Error: c-code returned null pointer (IntelDSS)"
        );
        assert_eq!(
            handle_intel_dss_error_code(200000),
            "Error: c-code failed to allocate memory (IntelDSS)"
        );
        assert_eq!(
            handle_intel_dss_error_code(400000),
            "This code has not been compiled with Intel DSS"
        );
        assert_eq!(handle_intel_dss_error_code(123), default);
    }
}
