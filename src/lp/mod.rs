use rulinalg::matrix::Matrix;
use std::collections::HashSet;
use rulinalg::matrix::{BaseMatrix, Matrix};
use std::fmt;

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

impl fmt::Display for Lp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	unsafe {
    		let opt_str = if self.optimization == Optimization::Max {"max"} else {"min"};
    		writeln!(f, "{} LP: {:>3} variables, {:>3} constraints\n", opt_str, self.c.len(), self.b.len());
    		for c in 0 .. self.A.cols() {
    			write!(f, "{:>5.1}  ", self.c[c]);
    		}

    		writeln!(f, "");

    		for c in 0 .. self.A.cols() {
    			write!(f, " - - - ");
    		}
    		writeln!(f, "");
    		

			for r in 0 .. self.A.rows() {
				for c in 0 .. self.A.cols() - 1{
					write!(f, "{:>5.1}, ", *self.A.get_unchecked([r, c]));
				}
				writeln!(f, "{:>5.1} | {:>5.1}", *self.A.get_unchecked([r, self.A.cols() -1]), self.b[r]);
			}

			write!(f, "")
    	}
    }
}
