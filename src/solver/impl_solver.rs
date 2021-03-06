use super::*;
use lp::{Lp, Optimization};
use rulinalg::matrix::{BaseMatrixMut, BaseMatrix};
use std::f64::INFINITY;
use utils::print_matrix;

impl SolverBase for SimplexSolver {
	/// Constructor for SolverBase struct.
	/// 
	/// Requires an input Lp struct.
	fn new(lp: Lp) -> Self {
		SimplexSolver {
			tableau: SimplexSolver::convert_lp_to_tableau(&lp),
			lp: lp
		}	
	}

	/// Solves the SimplexSolver.
	///
	/// Returns a Solution struct.
	///
	/// # Examples
	/// ```
	/// # extern crate rulinalg;
	/// # extern crate rulp;
	/// use rulp::solver::{SimplexSolver, Status, SolverBase};
	/// use std::collections::HashSet;
	/// use rulp::lp::{Lp, Optimization};
	/// use rulinalg::matrix::{Matrix, BaseMatrixMut};
	/// use std::f64::INFINITY;
	///
	/// # fn main() {
	/// let A = Matrix::new(2, 4, vec![2., 1., 1., 0.,
	///									1., 2., 0., 1.]);
	/// let b = vec![4., 3.];
	/// let c = vec![-1., -1., 0., 0.];
	/// let mut vars = vec![];
	/// vars.push("x1".to_string());
	/// vars.push("x2".to_string());
	/// vars.push("x3".to_string());
	/// vars.push("x4".to_string());
	/// let lp = Lp {
	/// 		A: A,
	/// 		b: b,
	/// 		c: c,
	/// 		optimization: Optimization::Max,
	/// 		vars: vars,
	///			num_artificial_vars: 0,
	/// };
	///
	/// let simplex = SimplexSolver::new(lp);
	/// let expected = vec![0., 0., 4., 3.];
	/// let solution = simplex.solve();
	/// assert_eq!(solution.status, Status::Optimal);
	/// assert_eq!(solution.values.unwrap(), expected);
	/// assert_eq!(solution.objective.unwrap(), 0.);
	/// # }
	/// ```
	fn solve(&self) -> Solution {
		// println!("Solver called");
		let mut local = SimplexSolver::new(self.lp.clone());
		let has_bfs = local.find_bfs();


		// print_matrix(&local.tableau);

		if !has_bfs {
			return Solution {
				lp: self.lp.clone(),
    			values: None,
    			objective: None,
    			status: Status::Infeasible
    		};
		}

		// Local has a basic feasible solution so we can optimize
		let bounded = local.optimize();

		if !bounded {
			return Solution {
				lp: self.lp.clone(),
    			values: None,
    			objective: None,
    			status: Status::Unbounded
    		};
		}

		let coeff;
		match &self.lp.optimization {
			&Optimization::Max => coeff = 1.,
			&Optimization::Min => coeff = -1.,
		}
		return Solution {
					lp: self.lp.clone(),
	    			values: Some(local.get_basic_feasible_solution()),
	    			objective: Some(local.get_objective() * coeff),
	    			status: Status::Optimal
		};
	}
}

impl SimplexSolver {
	fn convert_lp_to_tableau(lp: &Lp) -> Matrix<f64> {
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
		unsafe {
			for row in 0 .. lp.A.rows() {
				mat_builder.push(0.);
				for col in 0 .. lp.A.cols() {
					mat_builder.push(*lp.A.get_unchecked([row, col]));
				}
				mat_builder.push(lp.b[row]);
			}
		}
		
		// println!("{:}", lp);
		// println!("{:?}", mat_builder);
		// println!("{:?}", mat_builder.len());

		Matrix::new(&lp.A.rows()+1, &lp.A.cols()+2, mat_builder)
	}

	fn is_optimal(&self) -> bool {
		unsafe{
			for col in 1 .. self.tableau.cols() - 1{
				if *self.tableau.get_unchecked([0, col]) < 0. {
					return false;
				}
			}
		}

		return true
	}

	fn get_basic_feasible_solution(&self) -> Vec<f64> {
		let mut bfs = vec![];
		let rhs_index = self.tableau.cols() - 1;

		unsafe {
			for i in 1 .. self.tableau.cols() - 1 {
				if self.is_basic(i) {
					let row = self.get_basic_row(i);
					let val = *self.tableau.get_unchecked([row, rhs_index]);
					bfs.push(val);
				} else {
					bfs.push(0.0);
				}
			}
		}
		return bfs;
	}

	fn get_basic_row(&self, col: usize) -> usize {
		let mut ret = 1;
		unsafe {
			for row in 1 .. self.tableau.rows() {
				if *self.tableau.get_unchecked([row, col]) == 1. {
					ret = row;
				}
			}
		}
		return ret;
	}

	fn is_basic(&self, col: usize) -> bool {
		if col < 1 || col >= self.tableau.cols() {
			panic!("Invalid col index {} for basic variable", col);
		}
		unsafe {
			let mut one_ct = 0;
			let mut zero_ct = 0;
			for row in 1 .. self.tableau.rows() {
				let coeff = *self.tableau.get_unchecked([row, col]);
				if coeff == 1. {
					one_ct += 1;
				} else if coeff == 0. {
					zero_ct += 1;
				}
			}

			one_ct == 1 && zero_ct == (self.tableau.rows() - 2)
		}
	}

	fn calc_pivot_ratio(&self, row: usize, col: usize) -> Option<f64> {
		if self.is_basic(col) {
			panic!("Attempting to calculate pivot ratio on basic variable");
		} else if row == 0 || row >= self.tableau.rows() {
			panic!("Invalid constraint index {}", row);
		}
		unsafe {
			let coeff = *self.tableau.get_unchecked([row, col]);
			if coeff > 0. {
				let rhs_index = self.tableau.cols() - 1;
				let rhs_val = *self.tableau.get_unchecked([row, rhs_index]);
				Some(rhs_val / coeff)
			} else {
				None
			}
		}
	}

	fn choose_pivot_row(&self, col: usize) -> usize {
		let mut min_ratio = INFINITY;
		let mut min_row = 0;

		for row in 1 .. self.tableau.rows() {
			match self.calc_pivot_ratio(row, col) {
				Some(ratio) => {
					if ratio < min_ratio {
						min_ratio = ratio;
						min_row = row;
					}
				},
				_ => {}
			}
		}
		
		// // min_row cannot be 0 because row 0 is not a constraint
		// if min_row == 0 {
		// 	panic!("No pivot row chosen!");
		// }

		min_row
	}

	fn choose_pivot_col(&self) -> usize {
		unsafe {
			for i in 1 .. self.tableau.cols() - 1 {
				if *self.tableau.get_unchecked([0, i]) < 0. {
					return i;
				}
			}
		}

		panic!("No pivot var chosen because optimal solution!");
	}

	fn normalize_pivot(&mut self, row: usize, col: usize) {
		unsafe {
			let coeff = *self.tableau.get_unchecked([row, col]);
			for c in 1 .. self.tableau.cols() {
				*self.tableau.get_unchecked_mut([row, c]) /= coeff;
			}
			*self.tableau.get_unchecked_mut([row, col]) = 1.;
		}
	}

	fn eliminate_row(&mut self, pivot_row: usize, pivot_col: usize, row: usize) {
		unsafe {
			let mult_factor = *self.tableau.get_unchecked([row, pivot_col]) / *self.tableau.get_unchecked([pivot_row, pivot_col]) * -1.0;
			for c in 1 .. self.tableau.cols() {
				let add_factor = *self.tableau.get_unchecked([pivot_row, c]) * mult_factor;
				*self.tableau.get_unchecked_mut([row, c]) += add_factor;
			}
			*self.tableau.get_unchecked_mut([row, pivot_col]) = 0.;
		}
	}

	fn pivot(&mut self, row: usize, col:usize) {
		self.normalize_pivot(row, col);

		for r in 0 .. self.tableau.rows() {
			if r == row {
				continue;
			}

			self.eliminate_row(row, col, r);
		}
	}

	fn find_unspanned_rows(&self) -> Option<Vec<usize>> {
		unsafe {
			let mut no_basic = vec![];
			for row in 1 .. self.tableau.rows() {
				let mut has_basic = false;
				for col in 1 .. self.tableau.cols() - 1 {
					if *self.tableau.get_unchecked([row, col]) == 1. && self.is_basic(col) {
						has_basic = true;
					}
				}

				if !has_basic {
					no_basic.push(row);
				}
			}
			
			if no_basic.len() == 0 {
				return None;
			} else {
				return Some(no_basic);
			}
		}
	}

	// Can only be called once a BFS has been established
	fn optimize(&mut self) -> bool {
		// println!("Beginning optimization actually");
		// let mut iterations = 0;
		while !(self.is_optimal()) {
			// println!(">>> Iteration {}", iterations);
			// print_matrix(&self.tableau);
			let pivot_col = self.choose_pivot_col();
			// println!("Pivot column: {:?} ({} var entering)", pivot_col, pivot_col - 1);
			let pivot_row = self.choose_pivot_row(pivot_col);
			if pivot_row == 0 {			// Unbounded
				return false
			}
			// println!("Pivot row: {:?} ({} var leaving)", pivot_row, pivot_row - 1);
			self.pivot(pivot_row, pivot_col);
			// print_matrix(&self.tableau);
			// println!("<<< Iteration {}", iterations);
			// iterations += 1;
		}

		true
		// iterations
	}

	fn get_objective(&self) -> f64 {
		unsafe {
			return *self.tableau.get_unchecked([0, self.tableau.cols() - 1]);
		}
	}

	fn find_bfs(&mut self) -> bool {
		// println!("find_bfs");
		unsafe {
			match self.find_unspanned_rows() {
				None => {															// No unspanned rows, can proceed to look for bfs
					// println!("No unspanned rows found.");
				},	
				Some(unspanned_rows) => {											// Unspanned rows, need to create Phase I problem
					// println!("!! Unspanned rows found. Entering Phase I");
					// println!("{:?}", &unspanned_rows);
					let mut phase_one = self.generate_phase_one(&unspanned_rows);
					phase_one.find_bfs();
					let _ = phase_one.optimize();
					
					let phase_one_obj = phase_one.get_objective();					// If the objective of the optmized Phase I problem
					// print_matrix(&phase_one.tableau);
					if !(  relative_eq!(phase_one_obj,  0., epsilon = 0.0000001) 	// is non-zero, then no bfs exists (problem is infeasible)
						|| relative_eq!(phase_one_obj, -0., epsilon = 0.0000001)) { // kinda hacky way of testing due to f64 precision											
						return false
					} else {														// Bfs exists. Converting to Phase II by copying over
						for row in 1 .. self.tableau.rows() {						// new bfs
							for col in 0 .. self.tableau.cols() - 1 {
								*self.tableau.get_unchecked_mut([row, col]) =
									*phase_one.tableau.get_unchecked([row, col]);
							}
							let rhs_index = self.tableau.cols() - 1;
							*self.tableau.get_unchecked_mut([row, rhs_index]) =
									*phase_one.tableau.get_unchecked([row, phase_one.tableau.cols() - 1]);
						}
					}
					// println!("!! Leaving Phase I");
				}
			}

			// print_matrix(&self.tableau);
			self.write_obj_in_nb_vars();											// Tableau fully spanned, now want to write objective 
																					// function in terms of non-basic vars
			// print_matrix(&self.tableau);
			true
		}
	}

	fn generate_phase_one(&self, unspanned_rows: &Vec<usize>) -> Self {
		unsafe {
			let new_rows = self.tableau.rows(); 									// Phase I has same number of constraints
			let new_cols = self.tableau.cols() + unspanned_rows.len(); 				// Phase I has one more var for each unspanned row
			let mut phase_one = Matrix::new(new_rows, 								
											new_cols, 
											vec![0.; new_rows * new_cols]);

			*phase_one.get_unchecked_mut([0, 0]) = 1.;

			for row in 1 .. self.tableau.rows() { 									// Copying data from current tableau. RHS no longer in same
				for col in 0 .. self.tableau.cols() - 1 { 							// col so skipping for now
					*phase_one.get_unchecked_mut([row, col]) = *self.tableau.get_unchecked([row, col]);
				}

																				
				*phase_one.get_unchecked_mut([row, new_cols - 1]) = 				// Now copying RHS
					*self.tableau.get_unchecked([row, self.tableau.cols() - 1]);
			}

			let mut col = self.tableau.cols() - 1;									// Adding in Phase I artificial vars, starting at the first
			for &row in unspanned_rows {											// column specific to Phase I. Objective coeff for each Phase I
				*phase_one.get_unchecked_mut([row, col]) = 1.;						// artificial var is -1 b/c want to minimize them out of the
				*phase_one.get_unchecked_mut([0, col]) = -1.;						// basis
				col += 1;															
			}
			
			SimplexSolver {
				tableau: phase_one,
				lp: self.lp.clone()													// Don't really need LP here but makes it simpler to keep as
			}																		// SimplexSolver struct
		}
	}

	fn write_obj_in_nb_vars(&mut self) {
		// println!("Starting write_obj_in_nb_vars");
		unsafe {
			let mut obj_function = Vec::with_capacity(self.tableau.cols());			// Keeping same size as top row to make indexing simpler
																					// but first and last items are irrelevant 
			obj_function.push(1.);
			for col in 1 .. self.tableau.cols() {									
				obj_function.push(*self.tableau.get_unchecked([0, col]));			// Saving the value of the objective function for each var/col
				*self.tableau.get_unchecked_mut([0, col]) = 0.;						// Then setting the entry to 0
			}

			for col in 1 .. self.tableau.cols() - 1 {								// Can ignore first and last elements of obj row
				let obj_coeff = obj_function[col];
				if !self.is_basic(col) {
					*self.tableau.get_unchecked_mut([0, col]) -= obj_coeff;			// Basic vars are already written in terms of themselves
				} else {
					let mut row = 0;												// Finding the row that the basic var spans					
					for r in 1 .. self.tableau.rows() {
						if *self.tableau.get_unchecked([r, col]) == 1. {
							row = r; 
							break;
						}
					}

					if row == 0 {													// Row cannot be 0 since variable is non-basic
						panic!("Row cannot be 0");									// Should never happen, but cant hurt to check
					}

					for c in 1 .. self.tableau.cols() {							// Iterate through non-basic variables again and 
						if c == (self.tableau.cols() - 1) || !self.is_basic(c) {									// add the product of their coeff and the non-basic var coeff
							let coeff = *self.tableau.get_unchecked([row, c]);		// to the objective function 
							*self.tableau.get_unchecked_mut([0, c]) += coeff * obj_coeff;	
						}
					}


				}
			}
		}
	}
}

#[cfg(test)]
mod solve_tests {
	use super::*;
	use assert_approx_eq::*;

	#[test]
	fn to_tableau_test () {
		let expected = matrix![
					1., -1., -1., 0., 0., 0.;
	    			0.,  2.,  1., 1., 0., 4.;
	    			0.,  1.,  2., 0., 1., 3.];
		let lp = create_dummy_lp();
		assert_matrix_eq!(SimplexSolver::convert_lp_to_tableau(&lp), expected);
	}

	#[test]
	fn is_optimal_test() {
		let A = matrix![1., 0., 3., 1., 0.;
	                    3., 1., 3., 0., 1.];
		let b = vec![6., 9.];
		let c = vec![-4., -1., 1., 0., 0.]; // not optimal
		let c2 = vec![4., 1., 1., 0., 0.]; // optimal
		let mut vars = vec![];
		vars.push("x1".to_string());
		vars.push("x2".to_string());
		vars.push("x3".to_string());
		vars.push("x4".to_string());
		let Lp1 = Lp {
				A: A.clone(),
				b: b.clone(),
				c: c,
				optimization: Optimization::Max,
				vars: vars.clone(),
				num_artificial_vars: 0,
		};
		let Lp2 = Lp {
				A: A,
				b: b,
				c: c2,
				optimization: Optimization::Max,
				vars: vars,
				num_artificial_vars: 0
		};
		let not_optimal = SimplexSolver::new(Lp1);
		let optimal = SimplexSolver::new(Lp2);
	    assert_eq!(not_optimal.is_optimal(), false);
	    assert_eq!(optimal.is_optimal(), true);
	}

	#[test]
	fn is_basic_test() {
	    let lp = create_dummy_lp();
		let simplex_1 = SimplexSolver::new(lp);

	    assert!(!simplex_1.is_basic(1));
	    assert!(!simplex_1.is_basic(2));
	    assert!(simplex_1.is_basic(3));
	    assert!(simplex_1.is_basic(4));
	}

	#[test]
	#[should_panic]
	fn is_basic_z_test() {
	    let lp = create_dummy_lp();
		let simplex_1 = SimplexSolver::new(lp);

	    assert!(simplex_1.is_basic(0));
	}

	#[test]
	#[should_panic]
	fn is_basic_rhs_test() {
	    let lp = create_dummy_lp();
		let simplex = SimplexSolver::new(lp);

	    assert!(simplex.is_basic(5));
	}

	#[test]
	fn get_basic_feasible_solution_test() {
	    let lp = create_dummy_lp();
		let simplex = SimplexSolver::new(lp);
	    let bfs = vec![0., 0., 4., 3.];
	    assert_eq!(simplex.get_basic_feasible_solution(), bfs);
	}

	#[test]
	// TODO: Add test cases for None cases (when pivot element coeff <= 0)
	fn calc_pivot_ratio_test() {
	    let lp = create_dummy_lp();
		let simplex = SimplexSolver::new(lp);

	   	assert_eq!(simplex.calc_pivot_ratio(1, 1).unwrap(), 2.);
	   	assert_eq!(simplex.calc_pivot_ratio(2, 1).unwrap(), 3.);
	}

	#[test]
	fn choose_pivot_col_test() {
	    let A = matrix![1., 0., 3., 1., 0.;
	                    3., 1., 3., 0., 1.];
		let b = vec![6., 9.];
		let c = vec![-4., -1., 1., 0., 0.];
		let mut vars = vec![];
		vars.push("x1".to_string());
		vars.push("x2".to_string());
		vars.push("x3".to_string());
		vars.push("x4".to_string());
		let lp = Lp {
				A: A.clone(),
				b: b.clone(),
				c: c,
				optimization: Optimization::Max,
				vars: vars.clone(),
				num_artificial_vars: 0
		};
		let simplex = SimplexSolver::new(lp);

	   	assert_eq!(simplex.choose_pivot_col(), 1);
	}

	#[test]
	fn choose_pivot_row_test() {
	    let lp = create_dummy_lp();
		let simplex = SimplexSolver::new(lp);

	   	assert_eq!(simplex.choose_pivot_row(1), 1);
	}

	#[test]
	fn normalize_pivot_test() {
	    let lp = create_dummy_lp();
		let mut simplex = SimplexSolver::new(lp);

	    let expected_no_change = matrix![
	    							1., -1., -1., 0., 0., 0.;
	    							0.,  2.,  1., 1., 0., 4.;
	    							0.,  1.,  2., 0., 1., 3.
	    						];

	    let expected_change = matrix![
	    							1., -1.,  -1.,  0., 0., 0.;
	    							0.,  1.,  0.5, 0.5, 0., 2.;
	    							0.,  1.,   2.,  0., 1., 3.
	    						];

	    simplex.normalize_pivot(2, 1);
	    assert_matrix_eq!(simplex.tableau, expected_no_change);

	    simplex.normalize_pivot(1, 1);
		assert_matrix_eq!(simplex.tableau, expected_change);    	
	}

	#[test]
	fn eliminate_row_test() {
	    let lp = create_dummy_lp();
		let mut simplex = SimplexSolver::new(lp.clone());

	    let expected_1 = matrix![
	    						1.,  0.,  1., 0., 1., 3.;
								0.,  2.,  1., 1., 0., 4.;
								0.,  1.,  2., 0., 1., 3.
	    					];

	    simplex.eliminate_row(2, 1, 0);
	    assert_matrix_eq!(simplex.tableau, expected_1);

		simplex = SimplexSolver::new(lp);
	    let expected_2 = matrix![
	    						1., -1., -1., 0., 0., 0.;
								0.,  2.,  1., 1., 0., 4.;
								0.,  0.,  1.5, -0.5, 1., 1.
	    					];

		simplex.eliminate_row(1, 1, 2);
	    assert_matrix_eq!(simplex.tableau, expected_2);
	}

	#[test]
	fn pivot_test() {
		let lp = create_dummy_lp();
		let mut simplex = SimplexSolver::new(lp);

		let expected = matrix![
								1., 0., -0.5, 0.5, 0., 2.;
								0., 1.,  0.5,   0.5, 0., 2.;
								0., 0.,  1.5,  -0.5, 1., 1.
							];

		simplex.pivot(1, 1);
		assert_matrix_eq!(simplex.tableau, expected);
		assert_eq!(simplex.choose_pivot_row(2), 2);
	}

	#[test]
	fn solve_test() {
		let lp = create_dummy_lp();
		let simplex = SimplexSolver::new(lp);
		let expected = vec![0., 0., 4., 3.];
		let solution = simplex.solve();
		print!("{:}", &solution);
		assert_eq!(solution.status, Status::Optimal);
		assert_eq!(solution.values.unwrap(), expected);
		assert_eq!(solution.objective.unwrap(), 0.);
	}

	fn create_dummy_lp() -> Lp {
		let A = matrix![2., 1., 1., 0.;
						1., 2., 0., 1.];
		let b = vec![4., 3.];
		let c = vec![-1., -1., 0., 0.];
		let mut vars = vec![];
		vars.push("x1".to_string());
		vars.push("x2".to_string());
		vars.push("x3".to_string());
		vars.push("x4".to_string());
		Lp {
				A: A,
				b: b,
				c: c,
				optimization: Optimization::Max,
				vars: vars,
				num_artificial_vars: 0
		}
	}
	
	#[test]
	fn case_study_test () {
		// http://college.cengage.com/mathematics/larson/elementary_linear/4e/shared/downloads/c09s3.pdf
		// Example 5

		let A = matrix![20., 6., 3., 1., 0., 0., 0.;
						0., 1., 0., 0., 1., 0., 0.;
						-1., -1., 1., 0., 0., 1., 0.;
						-9., 1., 1., 0., 0., 0., 1.];
		let b = vec![182., 10., 0., 0.];
		let c = vec![100000., 40000., 18000., 0., 0., 0., 0.];
		let vars = vec![
			"x1".to_string(),
			"x2".to_string(),
			"x3".to_string(),
			"slack_1".to_string(),
			"slack_2".to_string(),
			"slack_3".to_string(),
			"slack_4".to_string(),
		];
		let lp = Lp {
				A: A,
				b: b,
				c: c,
				optimization: Optimization::Max,
				vars: vars,
				num_artificial_vars: 4
		};
		let simplex = SimplexSolver::new(lp);
		let solution = simplex.solve();
		let res = solution.values.unwrap();
		let expected = vec![4., 10., 14.];
		assert_eq!(solution.status, Status::Optimal);
		for i in 0..expected.len() {
			assert_approx_eq!(res[i], expected[i]);
		}
		assert_eq!(solution.objective.unwrap(), 1052000.);
	}
}