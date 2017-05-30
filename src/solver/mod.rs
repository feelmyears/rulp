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

pub struct SimplexSolver {
	lp: Lp,
	pub tableau: Matrix<f64>
}