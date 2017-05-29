use super::*;

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
	fn convert_lp_to_tableau(lp: &Lp) -> Matrix<f64> {
		unimplemented!();
	}

	
}