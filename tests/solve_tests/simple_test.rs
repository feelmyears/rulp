use super::*;
use std::collections::HashSet;
use rulp::solver::Status;
use assert_approx_eq::*;

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
fn full_case_study_test () {
	let text_problem = "	
		# This is a problem

		var television;
		var newspaper;
		var radio;

		maximize objective: 100000.*television + 40000.*newspaper + 18000.*radio;

		subject to constraint_1: 2000.*television + 600.*newspaper + 300.*radio <= 18200.;
		subject to constraint_2: newspaper <= 10.;
		subject to constraint_3: -1.*television + newspaper + radio <= 0.;
		subject to constraint_4: -9.*television + newspaper + radio <= 0.;

		# Nothing more to see here
	";
	let mut builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
	let res = solution.values.unwrap();
	let expected = vec![4., 10., 14.];
	assert_eq!(solution.status, Status::Optimal);
	for i in 0..res.len() {
		assert_approx_eq!(res[i], expected[i]);
	}
	assert_eq!(solution.objective.unwrap(), 1052000.);
}