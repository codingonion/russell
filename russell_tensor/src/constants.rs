/// sqrt(2) <https://oeis.org/A002193>
pub const SQRT_2: f64 =
    1.41421356237309504880168872420969807856967187537694807317667973799073247846210703885038753432764157f64;

/// sqrt(3) <https://oeis.org/A002194>
pub const SQRT_3: f64 = 1.7320508075688772935274463415058723669428052538103806280558069794519330169088000370811461867572485756756261414154f64;

/// sqrt(6) <https://oeis.org/A010464>
pub const SQRT_6: f64 =
    2.44948974278317809819728407470589139196594748065667012843269256725096037745731502653985943310464023f64;

/// sqrt(2/3) <https://oeis.org/A157697>
pub const SQRT_2_BY_3: f64 =
    0.816496580927726032732428024901963797321982493552223376144230855750320125819105008846619811034880078272864f64;

/// sqt(3/2) <https://oeis.org/A115754>
pub const SQRT_3_BY_2: f64 =
    1.22474487139158904909864203735294569598297374032833506421634628362548018872865751326992971655232011f64;

/// 1/3
pub const ONE_BY_3: f64 =
    0.33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333f64;

/// 2/3
pub const TWO_BY_3: f64 =
    0.66666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666f64;

/// Tolerance to avoid zero division with the J2 invariant
///
/// This constant is used, for instance, in the calculation of the Lode invariant using the following expression:
///
/// ```text
///        3 √3 J3
/// l = ─────────────
///     2 pow(J2,1.5)
/// ```
///
/// Note: pow(1e-9,1.5) = 3.16-14
pub const TOL_J2: f64 = 1e-9;

/// 1 (just so we can get highlighting in the following constants)
const ONE: f64 = 1.0;

/// Second-order identity tensor in Mandel basis (I)
///
/// ```text
/// Mandel vector:
///       ┌   ┐
///       │ 1 │
///       │ 1 │
///       │ 1 │
///       │ 0 │
/// [I] = │ 0 │
///       │ 0 │
///       │ 0 │
///       │ 0 │
///       │ 0 │
///       └   ┘
/// ```
pub const IDENTITY2: [f64; 9] = [ONE, ONE, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

/// Fourth-order identity tensor in Mandel basis (II)
///
/// **Note:** this tensor cannot be represented in reduced-dimension because it is not minor-symmetric.
///
/// ```text
/// Definition:
///        _
/// II = I ⊗ I
/// ```
///
/// ```text
/// Mandel matrix:
///        ┌                     ┐
///        │ 1 0 0  0 0 0  0 0 0 │
///        │ 0 1 0  0 0 0  0 0 0 │
///        │ 0 0 1  0 0 0  0 0 0 │
///        │ 0 0 0  1 0 0  0 0 0 │
/// [II] = │ 0 0 0  0 1 0  0 0 0 │
///        │ 0 0 0  0 0 1  0 0 0 │
///        │ 0 0 0  0 0 0  1 0 0 │
///        │ 0 0 0  0 0 0  0 1 0 │
///        │ 0 0 0  0 0 0  0 0 1 │
///        └                     ┘
/// ```
pub const IDENTITY4: [[f64; 9]; 9] = [
    [ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE],
];

/// Fourth-order transposition tensor in Mandel basis (TT)
///
/// **Note:** this tensor cannot be represented in reduced-dimension because it is not minor-symmetric.
///
/// ```text
/// Definition:
///
/// TT = I ⊗ I
///        ‾
/// ```
///
/// ```text
/// Mandel matrix:
///        ┌                        ┐
///        │ 1 0 0  0 0 0   0  0  0 │
///        │ 0 1 0  0 0 0   0  0  0 │
///        │ 0 0 1  0 0 0   0  0  0 │
///        │ 0 0 0  1 0 0   0  0  0 │
/// [TT] = │ 0 0 0  0 1 0   0  0  0 │
///        │ 0 0 0  0 0 1   0  0  0 │
///        │ 0 0 0  0 0 0  -1  0  0 │
///        │ 0 0 0  0 0 0   0 -1  0 │
///        │ 0 0 0  0 0 0   0  0 -1 │
///        └                        ┘
/// ```
pub const TRANSPOSITION: [[f64; 9]; 9] = [
    [ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -ONE, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -ONE, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -ONE],
];

/// Fourth-order trace-projection tensor (JJ)
///
/// Note: this tensor can be represented in reduced-dimension.
///
/// ```text
/// Definition:
///
/// JJ = I ⊗ I
/// ```
///
/// ```text
/// Mandel matrix:
///        ┌                     ┐
///        │ 1 1 1  0 0 0  0 0 0 │
///        │ 1 1 1  0 0 0  0 0 0 │
///        │ 1 1 1  0 0 0  0 0 0 │
///        │ 0 0 0  0 0 0  0 0 0 │
/// [JJ] = │ 0 0 0  0 0 0  0 0 0 │
///        │ 0 0 0  0 0 0  0 0 0 │
///        │ 0 0 0  0 0 0  0 0 0 │
///        │ 0 0 0  0 0 0  0 0 0 │
///        │ 0 0 0  0 0 0  0 0 0 │
///        └                     ┘
/// ```
pub const TRACE_PROJECTION: [[f64; 9]; 9] = [
    [ONE, ONE, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [ONE, ONE, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [ONE, ONE, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

/// Fourth-order isotropic making projector (Piso)
///
/// Note: this tensor can be represented in reduced-dimension.
///
/// ```text
/// Definition:
///
/// Piso = ⅓ I ⊗ I = ⅓ JJ
/// ```
///
/// ```text
/// Mandel matrix:
///          ┌                     ┐
///          │ ⅓ ⅓ ⅓  0 0 0  0 0 0 │
///          │ ⅓ ⅓ ⅓  0 0 0  0 0 0 │
///          │ ⅓ ⅓ ⅓  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
/// [Piso] = │ 0 0 0  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          └                     ┘
/// ```
pub const P_ISO: [[f64; 9]; 9] = [
    [ONE_BY_3, ONE_BY_3, ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [ONE_BY_3, ONE_BY_3, ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [ONE_BY_3, ONE_BY_3, ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

/// Fourth-order symmetric making projector (Psym)
///
/// Note: this tensor can be represented in reduced-dimension.
///
/// ```text
/// Definition:
///             _
/// Psym = ½ (I ⊗ I + I ⊗ I) = ½ (II + TT) = ½ ssd(I)
///                     ‾
/// ```
///
/// ```text
/// Mandel matrix:
///          ┌                     ┐
///          │ 1 0 0  0 0 0  0 0 0 │
///          │ 0 1 0  0 0 0  0 0 0 │
///          │ 0 0 1  0 0 0  0 0 0 │
///          │ 0 0 0  1 0 0  0 0 0 │
/// [Psym] = │ 0 0 0  0 1 0  0 0 0 │
///          │ 0 0 0  0 0 1  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          │ 0 0 0  0 0 0  0 0 0 │
///          └                     ┘
/// ```
pub const P_SYM: [[f64; 9]; 9] = [
    [ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

/// Fourth-order skew making projector (Pskew)
///
/// **Note:** this tensor cannot be represented in reduced-dimension because it is not minor-symmetric.
///
/// ```text
/// Definition:
///              _
/// Pskew = ½ (I ⊗ I - I ⊗ I) = ½ (II - TT)
///                      ‾
/// ```
///
/// ```text
/// Mandel matrix:
///           ┌                     ┐
///           │ 0 0 0  0 0 0  0 0 0 │
///           │ 0 0 0  0 0 0  0 0 0 │
///           │ 0 0 0  0 0 0  0 0 0 │
///           │ 0 0 0  0 0 0  0 0 0 │
/// [Pskew] = │ 0 0 0  0 0 0  0 0 0 │
///           │ 0 0 0  0 0 0  0 0 0 │
///           │ 0 0 0  0 0 0  1 0 0 │
///           │ 0 0 0  0 0 0  0 1 0 │
///           │ 0 0 0  0 0 0  0 0 1 │
///           └                     ┘
/// ```
pub const P_SKEW: [[f64; 9]; 9] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE],
];

/// Fourth-order deviatoric making projector (Pdev)
///
/// **Note:** this tensor cannot be represented in reduced-dimension because it is not minor-symmetric.
///
/// ```text
/// Definition:
///          _
/// Pdev = I ⊗ I - ⅓ I ⊗ I = II - Piso
/// ```
///
/// ```text
/// Mandel matrix:
///          ┌                        ┐
///          │  ⅔ -⅓ -⅓  0 0 0  0 0 0 │
///          │ -⅓  ⅔ -⅓  0 0 0  0 0 0 │
///          │ -⅓ -⅓  ⅔  0 0 0  0 0 0 │
///          │  0  0  0  1 0 0  0 0 0 │
/// [Pdev] = │  0  0  0  0 1 0  0 0 0 │
///          │  0  0  0  0 0 1  0 0 0 │
///          │  0  0  0  0 0 0  1 0 0 │
///          │  0  0  0  0 0 0  0 1 0 │
///          │  0  0  0  0 0 0  0 0 1 │
///          └                        ┘
/// ```
pub const P_DEV: [[f64; 9]; 9] = [
    [TWO_BY_3, -ONE_BY_3, -ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-ONE_BY_3, TWO_BY_3, -ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-ONE_BY_3, -ONE_BY_3, TWO_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ONE],
];

/// Fourth-order symmetric-deviatoric projector in Mandel basis
///
/// Note: this tensor can be represented in reduced-dimension.
///
/// ```text
/// Definition:
///                _
/// Psymdev = ½ (I ⊗ I + I ⊗ I) - ⅓ I ⊗ I = Psym - Piso
///                        ‾
/// ```
///
/// ```text
/// Mandel matrix:
///             ┌                        ┐
///             │  ⅔ -⅓ -⅓  0 0 0  0 0 0 │
///             │ -⅓  ⅔ -⅓  0 0 0  0 0 0 │
///             │ -⅓ -⅓  ⅔  0 0 0  0 0 0 │
///             │  0  0  0  1 0 0  0 0 0 │
/// [Psymdev] = │  0  0  0  0 1 0  0 0 0 │
///             │  0  0  0  0 0 1  0 0 0 │
///             │  0  0  0  0 0 0  0 0 0 │
///             │  0  0  0  0 0 0  0 0 0 │
///             │  0  0  0  0 0 0  0 0 0 │
///             └                        ┘
/// ```
pub const P_SYMDEV: [[f64; 9]; 9] = [
    [TWO_BY_3, -ONE_BY_3, -ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-ONE_BY_3, TWO_BY_3, -ONE_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [-ONE_BY_3, -ONE_BY_3, TWO_BY_3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, ONE, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

// --- maps -------------------------------------------------------------------------------------------------------------

/// Maps the m-th position in the vector representation to the index (i,j) of Tensor2
///
/// Diagonal goes first, then the upper diagonals, and, finally, the lower diagonals.
///
/// ```text
/// ┌   ┐    ┌    ┐
/// │ 0 │    │ 00 │
/// │ 1 │    │ 11 │
/// │ 2 │    │ 22 │    ┌          ┐
/// │ 3 │    │ 01 │    │ 00 01 02 │
/// │ 4 │ => │ 12 │ => │ 10 11 12 │
/// │ 5 │    │ 02 │    │ 20 21 22 │
/// │ 6 │    │ 10 │    └          ┘
/// │ 7 │    │ 21 │
/// │ 8 │    │ 20 │
/// └   ┘    └    ┘
/// ```
///
/// # Example
///
/// ```
/// use russell_tensor::M_TO_IJ;
/// assert_eq!(M_TO_IJ[3], (0,1));
/// ```
#[rustfmt::skip]
pub const M_TO_IJ: [(usize, usize); 9] = [
    // diagonal
    (0,0), // 0
    (1,1), // 1
    (2,2), // 2
    // upper-diagonal
    (0,1), // 3
    (1,2), // 4
    (0,2), // 5
    // lower-diagonal
    (1,0), // 6
    (2,1), // 7
    (2,0), // 8
];

/// Maps (i,j) of Tensor2 to the m-th position in the vector representation
///
/// Diagonal goes first, then the upper diagonals, and, finally, the lower diagonals.
///
/// ```text
///                 ┌    ┐    ┌   ┐
///                 │ 00 │    │ 0 │
///                 │ 11 │    │ 1 │
/// ┌          ┐    │ 22 │    │ 2 │
/// │ 00 01 02 │    │ 01 │    │ 3 │
/// │ 10 11 12 │ => │ 12 │ => │ 4 │
/// │ 20 21 22 │    │ 02 │    │ 5 │
/// └          ┘    │ 10 │    │ 6 │
///                 │ 21 │    │ 7 │
///                 │ 20 │    │ 8 │
///                 └    ┘    └   ┘
/// ```
///
/// # Example
///
/// ```
/// use russell_tensor::IJ_TO_M;
/// assert_eq!(IJ_TO_M[0][1], 3);
/// ```
#[rustfmt::skip]
pub const IJ_TO_M: [[usize; 3]; 3] = [
    [0, 3, 5],
    [6, 1, 4],
    [8, 7, 2],
];

/// Maps (i,j) of Tensor2 to the m-th position in the vector representation (symmetric version)
///
/// Diagonal goes first, then the upper diagonals, and, finally, the lower diagonals.
///
/// ```text
///                 ┌    ┐    ┌   ┐
/// ┌          ┐    │ 00 │    │ 0 │
/// │ 00 01 02 │    │ 11 │    │ 1 │
/// │ 01 11 12 │ => │ 22 │ => │ 2 │
/// │ 02 12 22 │    │ 01 │    │ 3 │
/// └          ┘    │ 12 │    │ 4 │
///                 │ 02 │    │ 5 │
///                 └    ┘    └   ┘
/// ```
///
/// # Example
///
/// ```
/// use russell_tensor::IJ_TO_M_SYM;
/// assert_eq!(IJ_TO_M_SYM[0][1], 3);
/// ```
#[rustfmt::skip]
pub const IJ_TO_M_SYM: [[usize; 3]; 3] = [
    [0, 3, 5],
    [3, 1, 4],
    [5, 4, 2],
];

/// Maps the (m,n)-th position in the matrix representation to (i,j,k,l) of Tensor4
///
/// ```text
///      0  0   0  1   0  2    0  3   0  4   0  5    0  6   0  7   0  8
///    ----------------------------------------------------------------
/// 0 │ 00_00  00_11  00_22   00_01  00_12  00_02   00_10  00_21  00_20
/// 1 │ 11_00  11_11  11_22   11_01  11_12  11_02   11_10  11_21  11_20
/// 2 │ 22_00  22_11  22_22   22_01  22_12  22_02   22_10  22_21  22_20
///   │
/// 3 │ 01_00  01_11  01_22   01_01  01_12  01_02   01_10  01_21  01_20
/// 4 │ 12_00  12_11  12_22   12_01  12_12  12_02   12_10  12_21  12_20
/// 5 │ 02_00  02_11  02_22   02_01  02_12  02_02   02_10  02_21  02_20
///   │
/// 6 │ 10_00  10_11  10_22   10_01  10_12  10_02   10_10  10_21  10_20
/// 7 │ 21_00  21_11  21_22   21_01  21_12  21_02   21_10  21_21  21_20
/// 8 │ 20_00  20_11  20_22   20_01  20_12  20_02   20_10  20_21  20_20
///    ----------------------------------------------------------------
///      8  0   8  1   8  2    8  3   8  4   8  5    8  6   8  7   8  8
/// ```
///
/// # Example
///
/// ```
/// use russell_tensor::MN_TO_IJKL;
/// assert_eq!(MN_TO_IJKL[3][3], (0,1,0,1));
/// ```
#[rustfmt::skip]
pub const MN_TO_IJKL: [[(usize,usize,usize,usize); 9]; 9] = [
    [(0,0,0,0), (0,0,1,1), (0,0,2,2), (0,0,0,1), (0,0,1,2), (0,0,0,2), (0,0,1,0), (0,0,2,1), (0,0,2,0)], // 0
    [(1,1,0,0), (1,1,1,1), (1,1,2,2), (1,1,0,1), (1,1,1,2), (1,1,0,2), (1,1,1,0), (1,1,2,1), (1,1,2,0)], // 1
    [(2,2,0,0), (2,2,1,1), (2,2,2,2), (2,2,0,1), (2,2,1,2), (2,2,0,2), (2,2,1,0), (2,2,2,1), (2,2,2,0)], // 2
    [(0,1,0,0), (0,1,1,1), (0,1,2,2), (0,1,0,1), (0,1,1,2), (0,1,0,2), (0,1,1,0), (0,1,2,1), (0,1,2,0)], // 3
    [(1,2,0,0), (1,2,1,1), (1,2,2,2), (1,2,0,1), (1,2,1,2), (1,2,0,2), (1,2,1,0), (1,2,2,1), (1,2,2,0)], // 4
    [(0,2,0,0), (0,2,1,1), (0,2,2,2), (0,2,0,1), (0,2,1,2), (0,2,0,2), (0,2,1,0), (0,2,2,1), (0,2,2,0)], // 5
    [(1,0,0,0), (1,0,1,1), (1,0,2,2), (1,0,0,1), (1,0,1,2), (1,0,0,2), (1,0,1,0), (1,0,2,1), (1,0,2,0)], // 6
    [(2,1,0,0), (2,1,1,1), (2,1,2,2), (2,1,0,1), (2,1,1,2), (2,1,0,2), (2,1,1,0), (2,1,2,1), (2,1,2,0)], // 7
    [(2,0,0,0), (2,0,1,1), (2,0,2,2), (2,0,0,1), (2,0,1,2), (2,0,0,2), (2,0,1,0), (2,0,2,1), (2,0,2,0)], // 8
];

/// Maps (i,j,k,l) of Tensor4 to the (m,n)-th position in the matrix representation
///
/// ```text
///      0  0   0  1   0  2    0  3   0  4   0  5    0  6   0  7   0  8
///    ----------------------------------------------------------------
/// 0 │ 00_00  00_11  00_22   00_01  00_12  00_02   00_10  00_21  00_20
/// 1 │ 11_00  11_11  11_22   11_01  11_12  11_02   11_10  11_21  11_20
/// 2 │ 22_00  22_11  22_22   22_01  22_12  22_02   22_10  22_21  22_20
///   │
/// 3 │ 01_00  01_11  01_22   01_01  01_12  01_02   01_10  01_21  01_20
/// 4 │ 12_00  12_11  12_22   12_01  12_12  12_02   12_10  12_21  12_20
/// 5 │ 02_00  02_11  02_22   02_01  02_12  02_02   02_10  02_21  02_20
///   │
/// 6 │ 10_00  10_11  10_22   10_01  10_12  10_02   10_10  10_21  10_20
/// 7 │ 21_00  21_11  21_22   21_01  21_12  21_02   21_10  21_21  21_20
/// 8 │ 20_00  20_11  20_22   20_01  20_12  20_02   20_10  20_21  20_20
///    ----------------------------------------------------------------
///      8  0   8  1   8  2    8  3   8  4   8  5    8  6   8  7   8  8
/// ```
///
/// # Example
///
/// ```
/// use russell_tensor::IJKL_TO_MN;
/// assert_eq!(IJKL_TO_MN[0][1][0][1], (3,3));
/// ```
#[rustfmt::skip]
pub const IJKL_TO_MN: [[[[(usize, usize); 3]; 3]; 3]; 3] = [
    [
        [[(0,0), (0,3), (0,5)], [(0,6), (0,1), (0,4)], [(0,8), (0,7), (0,2)]], // [0][0][.][.]
        [[(3,0), (3,3), (3,5)], [(3,6), (3,1), (3,4)], [(3,8), (3,7), (3,2)]], // [0][1][.][.]
        [[(5,0), (5,3), (5,5)], [(5,6), (5,1), (5,4)], [(5,8), (5,7), (5,2)]], // [0][2][.][.]
    ],
    [
        [[(6,0), (6,3), (6,5)], [(6,6), (6,1), (6,4)], [(6,8), (6,7), (6,2)]], // [1][0][.][.]
        [[(1,0), (1,3), (1,5)], [(1,6), (1,1), (1,4)], [(1,8), (1,7), (1,2)]], // [1][1][.][.]
        [[(4,0), (4,3), (4,5)], [(4,6), (4,1), (4,4)], [(4,8), (4,7), (4,2)]], // [1][2][.][.]
    ],
    [
        [[(8,0), (8,3), (8,5)], [(8,6), (8,1), (8,4)], [(8,8), (8,7), (8,2)]], // [2][0][.][.]
        [[(7,0), (7,3), (7,5)], [(7,6), (7,1), (7,4)], [(7,8), (7,7), (7,2)]], // [2][1][.][.]
        [[(2,0), (2,3), (2,5)], [(2,6), (2,1), (2,4)], [(2,8), (2,7), (2,2)]], // [2][2][.][.]
    ],
];

/// Maps (i,j,k,l) of Tensor4 to the (m,n)-th position in the matrix representation (minor-symmetric version)
///
/// ```text
///      0  0   0  1   0  2    0  3   0  4   0  5
///    ------------------------------------------
/// 0 │ 00_00  00_11  00_22   00_01  00_12  00_02
/// 1 │ 11_00  11_11  11_22   11_01  11_12  11_02
/// 2 │ 22_00  22_11  22_22   22_01  22_12  22_02
///   │
/// 3 │ 01_00  01_11  01_22   01_01  01_12  01_02
/// 4 │ 12_00  12_11  12_22   12_01  12_12  12_02
/// 5 │ 02_00  02_11  02_22   02_01  02_12  02_02
///    ------------------------------------------
///      8  0   8  1   8  2    8  3   8  4   8  5
/// ```
///
/// # Example
///
/// ```
/// use russell_tensor::IJKL_TO_MN_SYM;
/// assert_eq!(IJKL_TO_MN_SYM[0][1][0][1], (3,3));
/// ```
#[rustfmt::skip]
pub const IJKL_TO_MN_SYM: [[[[(usize, usize); 3]; 3]; 3]; 3] = [
    [
        [[(0,0), (0,3), (0,5)], [(0,3), (0,1), (0,4)], [(0,5), (0,4), (0,2)]], // [0][0][.][.]
        [[(3,0), (3,3), (3,5)], [(3,3), (3,1), (3,4)], [(3,5), (3,4), (3,2)]], // [0][1][.][.]
        [[(5,0), (5,3), (5,5)], [(5,3), (5,1), (5,4)], [(5,5), (5,4), (5,2)]], // [0][2][.][.]
    ],
    [
        [[(3,0), (3,3), (3,5)], [(3,3), (3,1), (3,4)], [(3,5), (3,4), (3,2)]], // [1][0][.][.]
        [[(1,0), (1,3), (1,5)], [(1,3), (1,1), (1,4)], [(1,5), (1,4), (1,2)]], // [1][1][.][.]
        [[(4,0), (4,3), (4,5)], [(4,3), (4,1), (4,4)], [(4,5), (4,4), (4,2)]], // [1][2][.][.]
    ],
    [
        [[(5,0), (5,3), (5,5)], [(5,3), (5,1), (5,4)], [(5,5), (5,4), (5,2)]], // [2][0][.][.]
        [[(4,0), (4,3), (4,5)], [(4,3), (4,1), (4,4)], [(4,5), (4,4), (4,2)]], // [2][1][.][.]
        [[(2,0), (2,3), (2,5)], [(2,3), (2,1), (2,4)], [(2,5), (2,4), (2,2)]], // [2][2][.][.]
    ],
];

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{
        IJKL_TO_MN, IJKL_TO_MN_SYM, IJ_TO_M, IJ_TO_M_SYM, MN_TO_IJKL, M_TO_IJ, ONE_BY_3, SQRT_2, SQRT_2_BY_3, SQRT_3,
        SQRT_3_BY_2, SQRT_6, TWO_BY_3,
    };

    // #[test]
    // #[rustfmt::skip]
    // fn generate_mathematica_code() {
    //     print!("MNtoIJKL = {{");
    //     for m in 0..9 {
    //         if m > 0 { print!(","); } print!("{{");
    //         for n in 0..9 {
    //             if n > 0 { print!(","); }
    //             let (i, j, k, l) = MN_TO_IJKL[m][n];
    //             print!("{{{},{},{},{}}}", i + 1, j + 1, k + 1, l + 1);
    //         }
    //         print!("}}");
    //     }
    //     println!("}};");
    //     print!("IJKLtoMN = {{");
    //     for i in 0..3 {
    //         if i > 0 { print!(","); }
    //         print!("{{");
    //         for j in 0..3 {
    //             if j > 0 { print!(","); }
    //             print!("{{");
    //             for k in 0..3 {
    //                 if k > 0 { print!(","); } print!("{{");
    //                 for l in 0..3 {
    //                     if l > 0 {
    //                         print!(",");
    //                     }
    //                     let (m, n) = IJKL_TO_MN_SYM[i][j][k][l];
    //                     print!("{{{},{}}}", m + 1, n + 1);
    //                 }
    //                 print!("}}");
    //             }
    //             print!("}}");
    //         }
    //         print!("}}");
    //     }
    //     println!("}};");
    // }

    #[test]
    fn constants_are_correct() {
        assert_eq!(SQRT_2, 2_f64.sqrt());
        assert_eq!(SQRT_3, 3_f64.sqrt());
        assert_eq!(SQRT_6, 6_f64.sqrt());
        assert_eq!(SQRT_2_BY_3, (2_f64 / 3_f64).sqrt());
        assert_eq!(SQRT_3_BY_2, (3_f64 / 2_f64).sqrt());
        assert_eq!(ONE_BY_3, 1_f64 / 3_f64);
        assert_eq!(TWO_BY_3, 2_f64 / 3_f64);
    }

    #[test]
    fn maps_are_correct() {
        // M_TO_IJ => IJ_TO_M and IJ_TO_M_SYM
        for m in 0..9 {
            let (i, j) = M_TO_IJ[m];
            assert_eq!(IJ_TO_M[i][j], m);
            let m_sym = match m {
                6 => 3,
                7 => 4,
                8 => 5,
                _ => m,
            };
            assert_eq!(IJ_TO_M_SYM[i][j], m_sym);
        }

        // MN_TO_IJKL => IJKL_TO_MN and IJKL_TO_MN_SYM
        for m in 0..9 {
            let m_sym = match m {
                6 => 3,
                7 => 4,
                8 => 5,
                _ => m,
            };
            for n in 0..9 {
                let (i, j, k, l) = MN_TO_IJKL[m][n];
                assert_eq!(IJKL_TO_MN[i][j][k][l], (m, n));
                let n_sym = match n {
                    6 => 3,
                    7 => 4,
                    8 => 5,
                    _ => n,
                };
                assert_eq!(IJKL_TO_MN_SYM[i][j][k][l], (m_sym, n_sym))
            }
        }
    }
}
