#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]


#[macro_use]
extern crate rulinalg;
extern crate assert_approx_eq;

// External
pub mod builder;
pub mod lp;
pub mod parser;
pub mod solver;

// Internal
mod utils;
