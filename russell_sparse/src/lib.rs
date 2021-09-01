//! Russell - Rust Scientific Library
//!
//! **sparse**: Sparse matrix tools and solvers

/// Returns package description
pub fn desc() -> String {
    "Sparse matrix tools and solvers".to_string()
}

mod enums;
mod solver_mumps;
mod sparse_triplet;
mod to_i32;
pub use crate::enums::*;
pub use crate::solver_mumps::*;
pub use crate::sparse_triplet::*;
pub use crate::to_i32::*;
