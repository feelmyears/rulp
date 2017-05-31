// #[allow(unused_imports)]
use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SolverBase, SimplexSolver};
// use rulp::lp::{Lp, Optimization};
use rulp::solver::Status;
use assert_approx_eq::*;

#[test]
fn simple_minimize_test() {
	let text_problem = "	
		# Radiation Example;

		var x_1;
		var x_2;

		minimize healthy_anatomy_exposure: 0.4*x_1 + 0.5*x_2;
		subject to critical_tissues: 0.3*x_1 + 0.1*x_2 <= 2.7;
		subject to tumor_region: 0.5*x_1 + 0.5*x_2 == 6;
		subject to tumor_center: 0.6*x_1 + 0.4*x_2 >= 6;
	";

	let builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
	print!("finished solving");
	print!("{:?}", &solution);

	let expected_sol = vec![7.5, 4.5];
	let sol = solution.values.unwrap(); // returning [7.5, 0.3, 0, 4.5]
	for i in 0 .. expected_sol.len() {
		assert_approx_eq!(expected_sol[i], sol[i]);
	}
	assert_approx_eq!(5.25, solution.objective.unwrap());
}

/*#[test]
fn simple_minimize_test_v2() {
	let text_problem = "	
		# Number of dose units from beam 1;
		var x_1 >= 0;
		# Number of dose units from beam 2;
		var x_2 >= 0;

		# Minimize radiation dosage units to healthy anatomy (in kilorads);
		minimize h_a_rad: 0.4*x_1 + 0.5*x_2;

		# Exposure for critical tissues should not exceed 2.7 kilorads;
		subject to critical_tissue_rad: 0.3*x_1 + 0.1*x_2 <= 2.7;
		# Exposure for center of tumor should be at least 6 kilorads;
		subject to tumor_center_rad: 0.6*x_1 + 0.4*x_2 >= 6;
		# Exposure for tumor region should be exactly 6 kilorads;
		subject to tumor_region_rad: 0.5*x_1 + 0.5*x_2 = 6;
		";

	let builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
	print!("finished solving");
	print!("{:?}", &solution);

	let expected_sol = vec![7.5, 4.5];
	let sol = solution.values.unwrap(); // unwraps a None right now
	for i in 0 .. expected_sol.len() {
		assert_approx_eq!(expected_sol[i], sol[i]);
	}
	assert_approx_eq!(5.25, solution.objective.unwrap());
}*/

#[test]
fn full_case_study_test () {
	let text_problem = "	
		# This is a problem;

		var television;
		var newspaper;
		var radio;

		maximize objective: 100000.*television + 40000.*newspaper + 18000.*radio;

		subject to constraint_1: 20.*television + 6.*newspaper + 3.*radio <= 182.;
		subject to constraint_2: newspaper <= 10.;
		subject to constraint_3: -1.*television + -1.*newspaper + radio <= 0.;
		subject to constraint_4: -9.*television + newspaper + radio <= 0.;

		# Nothing more to see here;
	";
	let builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let simplex = SimplexSolver::new(lp);
	let solution = simplex.solve();
	println!("{:?}", &solution);
	let res = solution.values.unwrap();
	let expected = vec![4., 10., 14.];
	assert_eq!(solution.status, Status::Optimal);
	for i in 0..expected.len() {
		assert_approx_eq!(res[i], expected[i]);
	}
	assert_eq!(solution.objective.unwrap(), 1052000.);
}	