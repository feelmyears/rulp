use rulinalg::matrix::Matrix;
use rulp::lp::{Lp, Optimization};
use rulp::solver::SimplexSolver;

#[test]
fn test() {
	assert!(false);
}

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