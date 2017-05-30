use super::*;
use lp::{Lp, Optimization};
use rulinalg::matrix::{Matrix, BaseMatrixMut};
use std::f64::INFINITY;

const OBJ_COEFFS_ROW_INDEX: usize = 0;

impl SolverBase for SimplexSolver {
	fn new(lp: Lp) -> Self {
		SimplexSolver {
			tableau: SimplexSolver::convert_lp_to_tableau(&lp),
			lp: lp
		}	
	}

	fn solve(&self) -> Solution {
		let mut local = SimplexSolver::new(self.lp.clone());

		match local.find_unspanned_rows() {
		    Some(rows) => {
		    	// unspanned rows = no clear basic feasible solution
		    	let mut phase_one = local.generate_phase_one(&rows);
		    	phase_one.optimize();
		    	if phase_one.get_objective() != 0. {
		    		return Solution {
						lp: self.lp.clone(),
		    			values: None,
		    			objective: None,
		    			status: Status::Infeasible,
		    		};
		    	} else {
		    		local.convert_to_phase_two(&phase_one);
		    	}

		    },
	    	None => {} // Do nothing
		} 

		// Local has a basic feasible solution so we can optimize
		let iterations = local.optimize();
		if local.is_optimal() {
			return Solution {
						lp: self.lp.clone(),
		    			values: Some(local.get_basic_feasible_solution()),
		    			objective: Some(local.get_objective()),
		    			status: Status::Optimal
    				};
		} else {
			return Solution {
						lp: self.lp.clone(),
		    			values: None,
		    			objective: None,
		    			status: Status::Degenerate
		    		};
		}
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
				mat_builder.push(lp.b[row]);
			}
		}
		
		println!("{:}", lp);
		println!("{:?}", mat_builder);
		println!("{:?}", mat_builder.len());

		Matrix::new(&lp.A.rows()+1, &lp.A.cols()+2, mat_builder)
	}

	pub fn is_optimal(&self) -> bool {
		let obj_coeffs = &self.tableau.row(OBJ_COEFFS_ROW_INDEX);
		for &coeff in obj_coeffs.iter() {
			if coeff < 0. {
				return false;
			}
		}
		return true
	}

	pub fn get_basic_feasible_solution(&self) -> Vec<f64> {
		let mut bfs = vec![];
		let mut basic_ct = 0;
		let rhs_index = self.tableau.cols() - 1;

		unsafe {
			for i in 1 .. self.tableau.cols() - 1 {
				if self.is_basic(i) {
					bfs.push(*self.tableau.get_unchecked([basic_ct + 1, rhs_index]));
					basic_ct += 1;
				} else {
					bfs.push(0.0);
				}
			}
		}
		bfs
	}

	pub fn is_basic(&self, col: usize) -> bool {
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

	pub fn calc_pivot_ratio(&self, row: usize, col: usize) -> Option<f64> {
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

	pub fn choose_pivot_row(&self, col: usize) -> usize {
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
		
		// min_row cannot be 0 because row 0 is not a constraint
		if min_row == 0 {
			panic!("No pivot row chosen!");
		}

		min_row
	}

	pub fn choose_pivot_col(&self) -> usize {
		unsafe {
			for i in 1 .. self.tableau.cols() - 1 {
				if *self.tableau.get_unchecked([0, i]) < 0. {
					return i;
				}
			}
		}

		panic!("No pivot var chosen because optimal solution!");
	}

	pub fn normalize_pivot(&mut self, row: usize, col: usize) {
		unsafe {
			let coeff = *self.tableau.get_unchecked([row, col]);
			for c in 1 .. self.tableau.cols() {
				*self.tableau.get_unchecked_mut([row, c]) /= coeff;
			}
			*self.tableau.get_unchecked_mut([row, col]) = 1.;
		}
	}

	pub fn eliminate_row(&mut self, pivot_row: usize, pivot_col: usize, row: usize) {
		unsafe {
			let mult_factor = *self.tableau.get_unchecked([row, pivot_col]) / *self.tableau.get_unchecked([pivot_row, pivot_col]) * -1.0;
			for c in 1 .. self.tableau.cols() {
				let add_factor = *self.tableau.get_unchecked([pivot_row, c]) * mult_factor;
				*self.tableau.get_unchecked_mut([row, c]) += add_factor;
			}
			*self.tableau.get_unchecked_mut([row, pivot_col]) = 0.;
		}
	}

	pub fn pivot(&mut self, row: usize, col:usize) {
		self.normalize_pivot(row, col);

		for r in 0 .. self.tableau.rows() {
			if r == row {
				continue;
			}

			self.eliminate_row(row, col, r);
		}
	}

	fn generate_phase_one(&self, unspanned_rows: &Vec<usize>) -> Self {
		unsafe {
			let new_rows = self.tableau.rows(); 								// Same as original
			let new_cols = self.tableau.cols() + unspanned_rows.len(); 				// Adding n more columns for n new artificial vars (for phase I)
			println!("Attempting phase one");
			let mut phase_one = Matrix::new(new_rows, new_cols, vec![0.; new_rows * new_cols]);
			println!("Got past phase one");

			// Transferring original data
			for row in 1 .. self.tableau.rows() { 								// Skipping first row (will be for new objective function)
				for col in 0 .. self.tableau.cols() - 1 { 						// Skipping RHS
					*phase_one.get_unchecked_mut([row, col]) = *self.tableau.get_unchecked([row, col]);
				}

																				
				*phase_one.get_unchecked_mut([row, new_cols - 1]) = 			// Now copying RHS
					*self.tableau.get_unchecked([row, self.tableau.cols() - 1]);
			}
			
			// Introducing Phase I vars
			let mut next_p1_var_index = self.tableau.cols() - 1;
			for &nb in unspanned_rows {
				// Making var basic
				*phase_one.get_unchecked_mut([nb, next_p1_var_index]) = 1.;

				// Writing objective function in terms of non-basic vars
				for col in 1 .. phase_one.cols() {
					if col != next_p1_var_index {
						*phase_one.get_unchecked_mut([0, col]) -= *phase_one.get_unchecked([nb, col]);
					}
				}

				next_p1_var_index += 1;
			}
			*phase_one.get_unchecked_mut([0, 0]) = 1.;							// w = 1

			//Simplex::from_tableau(phase_one)

			SimplexSolver {
				tableau: phase_one,
				lp: self.lp.clone(),
			}	
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

	fn convert_to_phase_two(&mut self, phase_one: &Self) {
		unsafe {
			let rhs_index = self.tableau.cols() - 1;

			// Updating values from current bfs
			for row in 1 .. self.tableau.rows() {
				for col in 1 .. self.tableau.cols() - 1{
					*self.tableau.get_unchecked_mut([row, col]) = 
						*phase_one.tableau.get_unchecked([row, col]);
				}

				*self.tableau.get_unchecked_mut([row, rhs_index]) = 
					*phase_one.tableau.get_unchecked([row, phase_one.tableau.cols() - 1]);
			}


			// Writing objective funciton in terms of non-basic vars and resetting the objective pub fn
			let mut obj_function = Vec::with_capacity(self.tableau.cols() - 2);
			for col in 1 .. self.tableau.cols() - 1 {
				obj_function.push(*self.tableau.get_unchecked([0, col]));
				*self.tableau.get_unchecked_mut([0, col]) = 0.;
			}

			for i in 0 .. obj_function.len() {
				let obj_coeff = obj_function[i];
				let col = i + 1;
				if !(self.is_basic(col)) {	
					// Var at col is non-basic so it equals itself
					*self.tableau.get_unchecked_mut([0, col]) -= obj_coeff;
				} else {
					// Var at col index is basic so must represent it in terms of non-basic vars

					// Finding the row that the basic var spans
					let mut row = 0;
					for r in 1 .. self.tableau.rows() {
						if *self.tableau.get_unchecked([r, col]) == 1. {
							row = r; 
							break;
						}
					}

					if row == 0 {
						panic!("Row cannot be 0");
					}

					for c in 1 .. self.tableau.cols() - 1 {
						if !(self.is_basic(c)) {
							let coeff = *self.tableau.get_unchecked([row, c]);
							*self.tableau.get_unchecked_mut([0, c]) += coeff * obj_coeff;	
						}
					}

					*self.tableau.get_unchecked_mut([0, rhs_index]) += 
						*self.tableau.get_unchecked([row, rhs_index]) * obj_coeff;
				}
			}
		}
	}

	fn optimize(&mut self) -> usize {
		let mut iterations = 0;
		while !(self.is_optimal() || self.check_degenerate()) {
			let pivot_col = self.choose_pivot_col();
			let pivot_row = self.choose_pivot_row(pivot_col);
			self.pivot(pivot_row, pivot_col);
			iterations += 1;
		}

		iterations
	}

	fn get_objective(&self) -> f64 {
		unsafe {
			*self.tableau.get_unchecked([0, self.tableau.cols() - 1])
		}
	}

	fn check_degenerate(&self) -> bool {
		let mut basics: Vec<usize> = vec![];
		for i in 1 .. self.tableau.cols() - 1 {
			if self.is_basic(i) {
				basics.push(i);
			}
		}
		let basic_soln = self.get_basic_feasible_solution();
		for basic in basics {
			if basic_soln[basic-1] == 0. { // subtract one to account for z in tableau, not in basic soln
				return true;
			}
		}
		return false;
	}
}