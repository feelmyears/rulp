//! The solver module.
//!
//! This code provides the rulp library with
//! linear program-solving capabilities.
//! This is implemented via the Simplex Tableau
//! method.
use rulinalg::matrix::{Matrix, BaseMatrix};
use assert_approx_eq::*;
use lp::Lp;

mod impl_solver;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
	Optimal,
	Infeasible,
	Degenerate,
	Unbounded
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
	lp: Lp,
	pub values: Option<Vec<f64>>, 
	pub objective: Option<f64>,
	pub status: Status
}

pub trait SolverBase {
	fn new(lp: Lp) -> Self;
	fn solve(&self) -> Solution;
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimplexSolver {
	lp: Lp,
	pub tableau: Matrix<f64>
}