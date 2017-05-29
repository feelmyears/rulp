use super::*;
use lp::{Lp, Optimization};
use rulinalg::matrix::Matrix;

impl SolverBase for SimplexSolver {
	fn new(lp: Lp) -> Self {
		SimplexSolver {
			tableau: SimplexSolver::convert_lp_to_tableau(&lp),
			lp: lp
		}	
	}

	fn solve(&self) -> Solution {
		unimplemented!();
	}
}

impl SimplexSolver {
	pub fn convert_lp_to_tableau(lp: &Lp) -> Matrix<f64> {
		// add [1 c 0]
		let mut mat_builder: Vec<f64> = vec![1.];
		for opt_coeff in &lp.c {
			match lp.optimization {
				Optimization::Min => {
					mat_builder.push(-1. * opt_coeff);
				},
				Optimization::Max => {
					mat_builder.push(*opt_coeff);
				},
			}
		}
		mat_builder.push(0.);
		// add [0 A b]
		unsafe {
			for row in 0 .. lp.A.rows() {
				mat_builder.push(0.);
				for col in 0 .. lp.A.cols() {
					mat_builder.push(*lp.A.get_unchecked([row, col]));
				}
				mat_builder.push(*lp.b.get(row).unwrap());
			}
		}
		return Matrix::new(&lp.A.rows()+1, &lp.A.cols()+2, mat_builder);
	}
}
/*
#[cfg(test)]
mod solver_tests {
	use super::*;
	//use SimplexSolver::convert_lp_to_tableau;

	#[test]
	fn to_tableau_test () {
		let expected = matrix![
				1., -1., -1., 0., 0., 0.;
    			0.,  2.,  1., 1., 0., 4.;
    			0.,  1.,  2., 0., 1., 3.];
		let A = matrix![2., 1., 1., 0.;
						1., 2., 0., 1.];
		let b = vec![4., 3.];
		let c = vec![-1., -1., 0., 0.];
		let lp = Lp {
			A: A,
			b: b,
			c: c,
			optimization: Optimization::Max,
		};
		assert_matrix_eq!(SimplexSolver::convert_lp_to_tableau(&lp), expected);
	}
}*/