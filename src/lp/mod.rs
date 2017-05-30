use rulinalg::matrix::Matrix;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub struct Lp {
	pub A: Matrix<f64>,
	pub b: Vec<f64>,
	pub c: Vec<f64>,
	pub optimization: Optimization,
	pub vars: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Optimization {
	Min,
	Max,
}