//! The tests module.
//!
//! This code provides a few unit tests to confirm
//! the functionality of rulp library uses.

#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(non_snake_case)]
use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SolverBase, SimplexSolver, Solution};
use rulp::lp::{Lp, Optimization};

#[cfg(test)]
mod simple_test;
