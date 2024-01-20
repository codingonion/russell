#![allow(unused)]

//! Russell - Rust Scientific Library
//!
//! `russell_ode`: Solvers for Ordinary Differential Equations

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

mod configuration;
mod constants;
mod enums;
mod explicit_runge_kutta;
mod function_types;
mod ode_output;
mod ode_solver;
mod runge_kutta_trait;
mod statistics;
mod workspace;
use crate::configuration::*;
use crate::constants::*;
use crate::enums::*;
use crate::explicit_runge_kutta::*;
use crate::function_types::*;
use crate::ode_output::*;
use crate::ode_solver::*;
use crate::runge_kutta_trait::*;
use crate::statistics::*;
use crate::workspace::*;

// run code from README file
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }
    external_doc_test!(include_str!("../README.md"));
}
