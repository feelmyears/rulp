use rulinalg::matrix::{Matrix, BaseMatrix};
use lp::Lp;

mod impl_solver;

pub enum Status {
	Optimal,
	Infeasible,
	Degenerate,
	Unbounded
}

pub struct Solution {
	lp: Lp,
	values: Option<(String, f64)>, 
	objective: Option<f64>,
	status: Status
}

pub trait SolverBase {
	fn new(lp: Lp) -> Self;
	fn solve(&self) -> Solution;
}

pub struct SimplexSolver {
	lp: Lp,
	tableau: Matrix<f64>
}