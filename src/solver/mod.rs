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
	values: Option<Vec<f64>>, 
	objective: Option<f64>,
	status: Status
}

pub trait SolverBase {
	fn new(lp: Lp) -> Self;
	fn solve(&self) -> Solution;
	fn is_optimal(&self) -> bool;
	fn get_basic_feasible_solution(&self) -> Vec<f64>;
	fn is_basic(&self, col: usize) -> bool;
	fn calc_pivot_ratio(&self, row: usize, col: usize) -> Option<f64>;
	fn choose_pivot_row(&self, col: usize) -> usize;
	fn choose_pivot_col(&self) -> usize;
	fn normalize_pivot(&mut self, row: usize, col: usize);
	fn eliminate_row(&mut self, pivot_row: usize, pivot_col: usize, row: usize);
	fn pivot(&mut self, row: usize, col:usize);
	fn generate_phase_one(&self, unspanned_rows: &Vec<usize>) -> Self;
	fn find_unspanned_rows(&self) -> Option<Vec<usize>>;
	fn convert_to_phase_two(&mut self, phase_one: &Self);
	fn optimize(&mut self) -> usize;
	fn get_objective(&self) -> f64;
	fn check_degenerate(&self) -> bool;
}

pub struct SimplexSolver {
	lp: Lp,
	tableau: Matrix<f64>
}