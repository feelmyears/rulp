use lp::{Lp, Optimization};
use rulinalg::matrix::{Matrix, BaseMatrix};
use super::*;

impl BuilderBase for Builder {
	/// Constructor for Builder struct.
	fn new() -> Self {
		Self {
			variables: HashSet::new(),
			variable_indices: HashMap::new(),
			constraints: vec![],
			objective: None,
			var_names: vec![]
		}
	}

	/// Adds a new user-defined Variable struct to this Builder struct.
	fn add_variable(&mut self, variable: Variable) {
		if !(self.variables.contains(&variable.name)) {
			let num_variables = self.variables.len();
			self.variable_indices.insert(variable.name.clone(), num_variables);
			self.variables.insert(variable.name.clone());
			self.var_names.push(variable.name.clone());
		}
	}

	/// Adds a new user-defined Constraint struct to this Builder struct.
	fn add_constraint(&mut self, constraint: Constraint) {
		self.check_variables(&constraint.variables).expect("Unknown variable in constraint");
		self.constraints.push(constraint);
	}

	/// Adds a new user-defined Objective struct to this Builder struct.
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

	/// Constructor for Lp struct.
	///
	/// Requires this Builder struct as input.
	///
	/// Converts the user-defined parameters into standard form
	/// in the process.
	///
	/// # Examples
	/// ```
	/// # #[macro_use] extern crate rulinalg;
	/// # extern crate rulp;
	/// use rulp::lp::{Lp, Optimization};
	/// use rulp::builder::*;
	///
	/// # fn gen_var(name: String, coeff: f64) -> Variable {
	/// 	Variable {
	/// 		name: name.to_string(),
	/// 		coefficient: coeff
	/// 	}
	/// # }
	///
	/// # fn gen_constraint(variables: Vec<Variable>, constant: f64, relation: Relation) -> Constraint {
	/// 	Constraint {
	/// 		name: "foo".to_string(),
	/// 		variables: variables,
	/// 		constant: constant,
	/// 		relation: relation
	/// 	}
	/// # }
	///
	/// # fn gen_objective(variables: Vec<Variable>) -> Objective {
    /// 	Objective {
	///			name: "bar".to_string(),
	///			variables: variables,
	///			maximize: false,
	///		}
	/// # }
	///
	/// # fn main() {
	/// 	let mut lpb = Builder::new();
	///
	/// 	let vars: Vec<Variable> = vec![gen_var("a".to_string(), 1.), gen_var("b".to_string(), 1.), gen_var("c".to_string(), 1.), gen_var("d".to_string(), 1.)];
	///	
	///		let a = vec![gen_var("a".to_string(), 2.), gen_var("b".to_string(), -5.)];
	///		let b = vec![gen_var("c".to_string(), -3.), gen_var("d".to_string(), -5.)];
	///		let c = vec![gen_var("a".to_string(), 1.), gen_var("b".to_string(), 1.), gen_var("c".to_string(), 1.)];
	///
	/// 	let constraints = vec![
	///			gen_constraint(a, 10., Relation::LessThanOrEqual),
	///			gen_constraint(b, 15., Relation::GreaterThanOrEqual),
	///			gen_constraint(c, 33., Relation::Equal),
	/// 	];
	///
	///		let obj_vec = vec![gen_var("a".to_string(), 1.), gen_var("b".to_string(), 2.), gen_var("c".to_string(), 3.), gen_var("d".to_string(), 4.)];
	/// 	let objective = gen_objective(obj_vec);
	///
	/// 	for v in vars {
	/// 		lpb.add_variable(v);
	/// 	}
	///
	/// 	for c in constraints {
	/// 		lpb.add_constraint(c);
	/// 	}
	///
	///
	/// 	lpb.add_objective(objective);
	///
	/// 	let lp = lpb.build_lp();
	///
	/// 	let expected_A = matrix![
	/// 			2.0,  -5.0,   0.0,   0.0,   1.0,   0.0;
	///   			0.0,   0.0,  -3.0,  -5.0,   0.0,  -1.0;
	///   			1.0,   1.0,   1.0,   0.0,   0.0,   0.0
	/// 	];
	///
	/// 	let expected_b = vec![10., 15., 33.];
	/// 	let expected_c = vec![1., 2., 3., 4., 0., 0.];
	///
	/// 	assert_matrix_eq!(lp.A, expected_A);
	/// 	assert_eq!(lp.b, expected_b);
	/// 	assert_eq!(lp.c, expected_c);
	/// 	assert_eq!(lp.optimization, Optimization::Min);
	/// # }
	/// ```
	fn build_lp(&mut self) -> Lp {
		//self.convert_to_standard_form();
		// println!("{:?}", self);
		let num_artificial_vars = self.convert_to_standard_form();
		// println!("{:?}", self);
		let num_variables = self.variables.len();
		let num_constraints = self.constraints.len();
		let A = self.generate_A();
		let b = self.generate_b();
		let (c, opt) = self.generate_c();

		Lp {
			A: A,
			b: b,
			c: c,
			optimization: opt,
			vars: self.var_names.clone(),
			// num_artificial_vars: 0
			num_artificial_vars: num_artificial_vars

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
		let mut c = vec![0.; num_variables];

		let opt = match self.objective {
			None => {
				panic!("No objective function!");
			},
			Some(ref obj) => {
				for ref var in &obj.variables {
					c[self.variable_indices[&var.name]] = var.coefficient;
				}

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

	fn convert_to_standard_form(&mut self) -> usize {
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

		let num_artificial_vars = needs_slack.len() + needs_excess.len();

		self.add_slack_variables(needs_slack);
		self.add_excess_variables(needs_excess);

		num_artificial_vars
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