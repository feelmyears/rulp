#![allow(unused_imports)]		// Allowed because compiler complains about/doesnt recognize macro imports only used in tests
#![allow(non_snake_case)]		// Allowed to follow LP standard naming procedure

#[macro_use]
extern crate rulinalg;
extern crate assert_approx_eq;

pub mod builder;
pub mod lp;
pub mod parser;
pub mod solver;

#[allow(dead_code)]	// print_matrix mainly for debugging
mod utils;
