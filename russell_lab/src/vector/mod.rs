mod add_vectors;
mod add_vectors_simd;
mod inner;
mod vector;
pub use crate::vector::add_vectors::*;
use crate::vector::add_vectors_simd::*;
pub use crate::vector::inner::*;
pub use crate::vector::vector::*;
