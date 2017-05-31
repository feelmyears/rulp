#![allow(unused_imports)]		// Allowed because compiler complains about/doesnt recognize macro imports only used in tests

#[macro_use]
extern crate rulinalg;
extern crate assert_approx_eq;
extern crate rulp;

pub mod solve_tests;