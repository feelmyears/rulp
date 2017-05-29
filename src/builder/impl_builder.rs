use lp::{Lp, Optimization};
use rulinalg::matrix::{Matrix, BaseMatrix};
use super::*;

impl BuilderBase for Builder {
	fn new() -> Self {
		Self {
			variables: HashSet::new(),
			variable_indices: HashMap::new(),
			constraints: vec![],
			objective: None,
		}
	}

	fn add_variable(&mut self, variable: Variable) {
		if !(self.variables.contains(&variable.name)) {
			let num_variables = self.variables.len();
			self.variable_indices.insert(variable.name.clone(), num_variables);
			self.variables.insert(variable.name.clone());
		}
	}

	fn add_constraint(&mut self, constraint: Constraint) {
		self.check_variables(&constraint.variables).expect("Unknown variable in constraint");
		self.constraints.push(constraint);
	}

	fn add_objective(&mut self, objective: Objective) {
		self.check_variables(&objective.variables).expect("Unknown variable in objective");

		let curr_objective = self.objective.take();
		match curr_objective {
			Some(objective) => {
				panic!("Attempting to add a second objective!");
			},
			None => {
				self.objective = Some(objective);
			}
		}
	}

	fn build_lp(&mut self) -> Lp {
		self.convert_to_standard_form();
		let num_variables = self.variables.len();
		let num_constraints = self.constraints.len();
		let A = self.generate_A();
		let b = self.generate_b();
		let (c, opt) = self.generate_c();

		Lp {
			A: A,
			b: b,
			c: c,
			optimization: opt
		}
	}
}

impl Builder {
	fn generate_A(&self) -> Matrix<f64> {
		let num_variables = self.variables.len();
		let num_constraints = self.constraints.len();
		let mut A = vec![0.; num_constraints * num_variables];

		for row in 0 .. num_constraints {
			let constraint = &self.constraints[row];
			for ref var in &constraint.variables {
				let index = row * num_variables + self.variable_indices[&var.name];
				A[index] = var.coefficient;
			}
		}

		Matrix::new(num_constraints, num_variables, A)
	}

	fn generate_b(&self) -> Vec<f64> {
		let num_constraints = self.constraints.len();			
		let mut b = Vec::with_capacity(num_constraints);

		for row in 0 .. num_constraints {
			let constraint = &self.constraints[row];
			b.push(constraint.constant);
		}

		b

	}

	fn generate_c(&self) -> (Vec<f64>, Optimization) {
		let num_variables = self.variables.len();
		let mut c = vec![0.; num_variables + 1];

		let opt = match self.objective {
			None => {
				panic!("No objective function!");
			},
			Some(ref obj) => {
				for ref var in &obj.variables {
					c[self.variable_indices[&var.name]] = var.coefficient;
				}
				c[num_variables] = obj.constant;

				if obj.maximize {
					Optimization::Max
				} else {
					Optimization::Min
				}
			}
		};

		(c, opt)
	}

	fn check_variables(&self, variables: &Vec<Variable>) -> Option<()> {
		for ref var in variables {
			if !self.variables.contains(&var.name) {
				return None
			}
		}

		Some(())
	}

	fn convert_to_standard_form(&mut self) {
		let mut needs_slack = vec![];
		let mut needs_excess = vec![];

		for i in 0 .. self.constraints.len() {
			match self.constraints[i].relation {
			    Relation::Equal => {},
				Relation::LessThanOrEqual => {
					needs_slack.push(i);
				},
				Relation::GreaterThanOrEqual => {
					needs_excess.push(i);
				}
			}
		}

		self.add_slack_variables(needs_slack);
		self.add_excess_variables(needs_excess);
	}

	fn add_slack_variables(&mut self, constraints: Vec<usize>) {
		let mut slack_ct = 0;
		let mut vars_to_add = vec![];

		for i in constraints {
			let ref mut constraint = self.constraints[i];

			let slack = format!("slack_{}", slack_ct);
			slack_ct += 1;


			let var = Variable {
				name: slack,
				coefficient: 1.,
			};


			vars_to_add.push(var.clone());
			constraint.variables.push(var);
			constraint.relation = Relation::Equal;
		}

		for v in vars_to_add {
			self.add_variable(v);
		}
	}

	fn add_excess_variables(&mut self, constraints: Vec<usize>) {
		let mut excess_ct = 0;
		let mut vars_to_add = vec![];

		for i in constraints {
			let ref mut constraint = self.constraints[i];

			let excess = format!("excess_{}", excess_ct);
			excess_ct += 1;

			let var = Variable {
				name: excess,
				coefficient: -1.,
			};

			vars_to_add.push(var.clone());
			constraint.variables.push(var);
			constraint.relation = Relation::Equal;
		}

		for v in vars_to_add {
			self.add_variable(v);
		}
	}
}

// #[cfg(test)]
// mod builder_tests {
// 	use super::*;
// 	use lp::Optimization;

// 	#[test]
// 	fn generate_lp_test() {
// 		let mut builder = Builder::new();

// 		let vars = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()].iter().map(|v| gen_var(v, 1.)).collect();
		
// 		let constraints = vec![
// 			gen_constraint(vec![("a".to_string(), 2.), ("b".to_string(), -5.)], 10., Relation::LessThanOrEqual),
// 			gen_constraint(vec![("c".to_string(), -3.), ("d".to_string(), -5.)], 15., Relation::GreaterThanOrEqual),
// 			gen_constraint(vec![("a".to_string(), 1.), ("b".to_string(), 1.), ("c".to_string(), 1.)], 33., Relation::Equal),
// 		];

// 		let objective = gen_constraint(vec![("a".to_string(), 1.), ("b".to_string(), 2.), ("c".to_string(), 3.), ("d".to_string(), 4.)], 0., Relation::LessThanOrEqual);

// 		for &v in &vars {
// 			builder.add_variable(&v);
// 		}

// 		for c in constraints {
// 			lpb.add_constraint(c);
// 		}


// 		lpb.add_objective(objective);

// 		let lp = lpb.build_lp();

// 		let expected_A = matrix![
// 			2.0,  -5.0,   0.0,   0.0,   1.0,   0.0;
//   			0.0,   0.0,  -3.0,  -5.0,   0.0,  -1.0;
//   			1.0,   1.0,   1.0,   0.0,   0.0,   0.0
// 		];

// 		let expected_b = vec![10., 15., 33.];
// 		let expected_c = vec![1., 2., 3., 4., 0., 0., 0.];

// 		assert_matrix_eq!(lp.A, expected_A);
// 		assert_eq!(lp.b, expected_b);
// 		assert_eq!(lp.c, expected_c);
// 		assert_eq!(lp.optimization, Optimization::Min);
// 	}

// 	fn gen_constraint(variables: &Vec<Variable>, constant: f64, relation: Relation) -> Constraint {
// 		Constraint {
// 			name: "foo".to_string(),
// 			variables: variables.iter().map(|ref v| gen_var(v.0, v.1)).collect(),
// 			constant: constant,
// 			relation: relation
// 		}
// 	}

// 	fn gen_var(name: &str, coeff: f64) {
// 		Variable {
// 			name: name.to_string(),
// 			coefficient: coeff
// 		}
// 	}
// }