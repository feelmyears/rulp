use super::*;

#[test]
fn simple_minimize_test() {
	let text_problem = "	
		# This is a problem;

		var a;
		var b;

		minimize objective: -2.*a + 5.*b;

		subject to constraint_1: a + b >= 10.;
		subject to constraint_2: b >= 5.;
		subject to constraint_3: a <= 15;

		# Nothing more to see here;
	";

	let mut builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
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