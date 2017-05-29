use rulinalg::matrix::Matrix;

#[derive(Debug, PartialEq)]
pub struct Lp {
	pub A: Matrix<f64>,
	pub b: Vec<f64>,
	pub c: Vec<f64>,
	pub optimization: Optimization,
}

#[derive(Debug, PartialEq)]
pub enum Optimization {
	Min,
	Max,
}