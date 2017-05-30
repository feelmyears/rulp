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

fn create_dummy_lp() -> Lp {
	let A = matrix![2., 1., 1., 0.;
					1., 2., 0., 1.];
	let b = vec![4., 3.];
	let c = vec![-1., -1., 0., 0.];
	let mut vars = HashSet::new();
	vars.insert("x1".to_string());
	vars.insert("x2".to_string());
	vars.insert("x3".to_string());
	vars.insert("x4".to_string());
	Lp {
			A: A,
			b: b,
			c: c,
			optimization: Optimization::Max,
			vars: vars,
	}
}

// #[test]
// fn to_tableau_test () {
// 	let expected = matrix![
// 				1., -1., -1., 0., 0., 0.;
//     			0.,  2.,  1., 1., 0., 4.;
//     			0.,  1.,  2., 0., 1., 3.];
// 	let lp = create_dummy_lp();
// 	assert_matrix_eq!(SimplexSolver::convert_lp_to_tableau(&lp), expected);
// }

// #[test]
// fn is_optimal_test() {
// 	let A = matrix![1., 0., 3., 1., 0.;
//                     3., 1., 3., 0., 1.];
// 	let b = vec![6., 9.];
// 	let c = vec![-4., -1., 1., 0., 0.]; // not optimal
// 	let c2 = vec![4., 1., 1., 0., 0.]; // optimal
// 	let mut vars = HashSet::new();
// 	vars.insert("x1".to_string());
// 	vars.insert("x2".to_string());
// 	vars.insert("x3".to_string());
// 	vars.insert("x4".to_string());
// 	let Lp1 = Lp {
// 			A: A.clone(),
// 			b: b.clone(),
// 			c: c,
// 			optimization: Optimization::Max,
// 			vars: vars.clone(),
// 	};
// 	let Lp2 = Lp {
// 			A: A,
// 			b: b,
// 			c: c2,
// 			optimization: Optimization::Max,
// 			vars: vars,
// 	};
// 	let not_optimal = SimplexSolver::new(Lp1);
// 	let optimal = SimplexSolver::new(Lp2);
//     assert_eq!(not_optimal.is_optimal(), false);
//     assert_eq!(optimal.is_optimal(), true);
// }

// #[test]
// fn is_basic_test() {
//     let lp = create_dummy_lp();
// 	let simplex_1 = SimplexSolver::new(lp);

//     assert!(!simplex_1.is_basic(1));
//     assert!(!simplex_1.is_basic(2));
//     assert!(simplex_1.is_basic(3));
//     assert!(simplex_1.is_basic(4));
// }

// #[test]
// #[should_panic]
// fn is_basic_z_test() {
//     let lp = create_dummy_lp();
// 	let simplex_1 = SimplexSolver::new(lp);

//     assert!(simplex_1.is_basic(0));
// }

// #[test]
// #[should_panic]
// fn is_basic_rhs_test() {
//     let lp = create_dummy_lp();
// 	let simplex = SimplexSolver::new(lp);

//     assert!(simplex.is_basic(5));
// }

// #[test]
// fn get_basic_feasible_solution_test() {
//     let lp = create_dummy_lp();
// 	let simplex = SimplexSolver::new(lp);
//     let bfs = vec![0., 0., 4., 3.];
//     assert_eq!(simplex.get_basic_feasible_solution(), bfs);
// }

// #[test]
// // TODO: Add test cases for None cases (when pivot element coeff <= 0)
// fn calc_pivot_ratio_test() {
//     let lp = create_dummy_lp();
// 	let simplex = SimplexSolver::new(lp);

//    	assert_eq!(simplex.calc_pivot_ratio(1, 1).unwrap(), 2.);
//    	assert_eq!(simplex.calc_pivot_ratio(2, 1).unwrap(), 3.);
// }

// #[test]
// fn choose_pivot_col_test() {
//     let A = matrix![1., 0., 3., 1., 0.;
//                     3., 1., 3., 0., 1.];
// 	let b = vec![6., 9.];
// 	let c = vec![-4., -1., 1., 0., 0.];
// 	let mut vars = HashSet::new();
// 	vars.insert("x1".to_string());
// 	vars.insert("x2".to_string());
// 	vars.insert("x3".to_string());
// 	vars.insert("x4".to_string());
// 	let lp = Lp {
// 			A: A.clone(),
// 			b: b.clone(),
// 			c: c,
// 			optimization: Optimization::Max,
// 			vars: vars.clone(),
// 	};
// 	let simplex = SimplexSolver::new(lp);

//    	assert_eq!(simplex.choose_pivot_col(), 1);
// }

// #[test]
// fn choose_pivot_row_test() {
//     let lp = create_dummy_lp();
// 	let simplex = SimplexSolver::new(lp);

//    	assert_eq!(simplex.choose_pivot_row(1), 1);
// }

// #[test]
// fn normalize_pivot_test() {
//     let lp = create_dummy_lp();
// 	let mut simplex = SimplexSolver::new(lp);

//     let expected_no_change = matrix![
//     							1., -1., -1., 0., 0., 0.;
//     							0.,  2.,  1., 1., 0., 4.;
//     							0.,  1.,  2., 0., 1., 3.
//     						];

//     let expected_change = matrix![
//     							1., -1.,  -1.,  0., 0., 0.;
//     							0.,  1.,  0.5, 0.5, 0., 2.;
//     							0.,  1.,   2.,  0., 1., 3.
//     						];

//     simplex.normalize_pivot(2, 1);
//     assert_matrix_eq!(simplex.tableau, expected_no_change);

//     simplex.normalize_pivot(1, 1);
// 	assert_matrix_eq!(simplex.tableau, expected_change);    	
// }

// #[test]
// fn eliminate_row_test() {
//     let lp = create_dummy_lp();
// 	let mut simplex = SimplexSolver::new(lp.clone());

//     let expected_1 = matrix![
//     						1.,  0.,  1., 0., 1., 3.;
// 							0.,  2.,  1., 1., 0., 4.;
// 							0.,  1.,  2., 0., 1., 3.
//     					];

//     simplex.eliminate_row(2, 1, 0);
//     assert_matrix_eq!(simplex.tableau, expected_1);

// 	simplex = SimplexSolver::new(lp);
//     let expected_2 = matrix![
//     						1., -1., -1., 0., 0., 0.;
// 							0.,  2.,  1., 1., 0., 4.;
// 							0.,  0.,  1.5, -0.5, 1., 1.
//     					];

// 	simplex.eliminate_row(1, 1, 2);
//     assert_matrix_eq!(simplex.tableau, expected_2);
// }

// #[test]
// fn pivot_test() {
// 	let lp = create_dummy_lp();
// 	let mut simplex = SimplexSolver::new(lp);

// 	let expected = matrix![
// 							1., 0., -0.5, 0.5, 0., 2.;
// 							0., 1.,  0.5,   0.5, 0., 2.;
// 							0., 0.,  1.5,  -0.5, 1., 1.
// 						];

// 	simplex.pivot(1, 1);
// 	assert_matrix_eq!(simplex.tableau, expected);
// 	assert_eq!(simplex.choose_pivot_row(2), 2);
// }

// #[test]
// fn solve_test() {
// 	let lp = create_dummy_lp();
// 	let mut simplex = SimplexSolver::new(lp);
// 	let expected = vec![5./3., 2./3., 0., 0.];
// 	let solution = simplex.solve();
// 	assert_eq!(solution.status, Status::Optimal);
// 	assert_eq!(solution.values.unwrap(), expected);
// 	assert_eq!(solution.objective.unwrap(), 7./3.);
// }

// // http://college.cengage.com/mathematics/larson/elementary_linear/4e/shared/downloads/c09s3.pdf
// // Example 5
// #[test]
// fn case_study_test () {
// 	let A = matrix![20., 6., 3.;
// 					0., 1., 0.;
// 					-1., -1., 1.;
// 					-9., 1., 1.];
// 	let b = vec![182., 10., 0., 0.];
// 	let c = vec![100000., 40000., 18000.];
// 	let mut vars = HashSet::new();
// 	vars.insert("x1".to_string());
// 	vars.insert("x2".to_string());
// 	vars.insert("x3".to_string());
// 	let lp = Lp {
// 			A: A,
// 			b: b,
// 			c: c,
// 			optimization: Optimization::Max,
// 			vars: vars,
// 	};
// 	let simplex = SimplexSolver::new(lp);
// 	let solution = simplex.solve();
// 	let res = solution.values.unwrap();
// 	let expected = vec![4., 10., 14.];
// 	assert_eq!(solution.status, Status::Optimal);
// 	for i in 0..res.len() {
// 		assert_approx_eq!(res[i], expected[i]);
// 	}
// 	//assert_approx_eq!(solution.values.unwrap(), expected);
// 	assert_eq!(solution.objective.unwrap(), 1052000.);
// }