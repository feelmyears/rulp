extern crate rulinalg;
extern crate assert_approx_eq;
extern crate rulp;

#[allow(unused_imports)]
use assert_approx_eq::*;
use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SolverBase, SimplexSolver};
use rulp::solver::Status;
use std::fs::File;

#[test]
fn minimize_text_test() {
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
	let sol = solution.values.unwrap();
	for i in 0 .. expected_sol.len() {
		assert_approx_eq!(expected_sol[i], sol[i]);
	}
	assert_approx_eq!(5.25, solution.objective.unwrap());
}


#[test]
fn maximize_text_test () {
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

#[test]
fn advertisement_file_test() {
	let builder = Builder::new();
	let mut input_file = File::open("./tests/test_files/advertisement_example.lp").unwrap();
	let lp = Parser::lp_from_file(&mut input_file, builder);

	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();

	let res = solution.values.unwrap();
	let expected = vec![4., 10., 14.];
	assert_eq!(solution.status, Status::Optimal);
	for i in 0..expected.len() {
		assert_approx_eq!(res[i], expected[i]);
	}
	assert_eq!(solution.objective.unwrap(), 1052000.);
}

#[test]
fn radiation_file_test() {
	let builder = Builder::new();
	let mut input_file = File::open("./tests/test_files/radiation_example.lp").unwrap();
	let lp = Parser::lp_from_file(&mut input_file, builder);

	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();

	let expected_sol = vec![7.5, 4.5];
	let sol = solution.values.unwrap(); // returning [7.5, 0.3, 0, 4.5]
	for i in 0 .. expected_sol.len() {
		assert_approx_eq!(expected_sol[i], sol[i]);
	}
	assert_approx_eq!(5.25, solution.objective.unwrap());
}


#[test]
fn unbounded_file_test() {
	let builder = Builder::new();
	let mut input_file = File::open("./tests/test_files/unbounded_example.lp").unwrap();
	let lp = Parser::lp_from_file(&mut input_file, builder);

	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();

	assert_eq!(solution.objective, None);
	assert_eq!(solution.values, None);
	assert_eq!(solution.status, Status::Unbounded);
}

#[test]
fn infeasible_file_test() {
	let builder = Builder::new();
	let mut input_file = File::open("./tests/test_files/infeasible_example.lp").unwrap();
	let lp = Parser::lp_from_file(&mut input_file, builder);

	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
	
	assert_eq!(solution.objective, None);
	assert_eq!(solution.values, None);
	assert_eq!(solution.status, Status::Infeasible);
}