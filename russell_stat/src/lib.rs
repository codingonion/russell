//! Russell - Rust Scientific Library
//!
//! `russell_stat`: Statistics calculations and (engineering) probability distributions
//!
//! **Important:** This crate depends on external libraries (non-Rust). Thus, please check the [Installation Instructions on the GitHub Repository](https://github.com/cpmech/russell).
//!
//! This library assists in developing statistical computations and simulations aiming at engineering applications, such as reliability analyses. This library provides a light interface to [rand_distr](https://crates.io/crates/rand_distr) and implements extra functionality.
//!
//! Some essential distributions considered in this library are those classified as [Extreme Value Distribution](https://en.wikipedia.org/wiki/Generalized_extreme_value_distribution).

/// Defines the error output as a static string
pub type StrError = &'static str;

use rand::rngs::ThreadRng;

/// Returns the thread-local random number generator, seeded by the system
///
/// See more information here: [rand::thread_rng()]
#[inline]
pub fn get_rng() -> ThreadRng {
    // re-exported for convenience
    rand::thread_rng()
}

mod distribution_frechet;
mod distribution_gumbel;
mod distribution_lognormal;
mod distribution_normal;
mod distribution_uniform;
mod histogram;
mod probability_distribution;
mod statistics;
pub use crate::distribution_frechet::*;
pub use crate::distribution_gumbel::*;
pub use crate::distribution_lognormal::*;
pub use crate::distribution_normal::*;
pub use crate::distribution_uniform::*;
pub use crate::histogram::*;
pub use crate::probability_distribution::*;
pub use crate::statistics::*;

// run code from README file
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctest;
