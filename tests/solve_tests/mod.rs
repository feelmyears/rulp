//! The tests module.
//!
//! This code provides a few unit tests to confirm
//! the functionality of rulp library uses.
use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SolverBase, SimplexSolver, Solution};
use rulp::lp::{Lp, Optimization};

mod simple_test;

use rulinalg::matrix::Matrix;
