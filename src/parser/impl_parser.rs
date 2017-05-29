use lp::Lp;
use builder::Builder;

#[derive(Debug, PartialEq)]
enum LineType {
	Variable,
	Constraint,
	Objective,
	Comment
}

#[derive(Debug)]
enum ParseResult {
	VariableDec(Variable),
	ConstraintDec(Constraint),
	ObjectiveDec(Constraint),
	CommentDec(String)
}

use self::LineType::*;
use self::ParseResult::*;

impl Parser {
	pub fn parse_text(text: &str) -> Lp {
		let mut parser = Self::new();
		let declarations = parser.get_declarations(text);

		for dec in declarations {
			match dec {
			    VariableDec(variable) => {
			    	parser.builder.add_variable(&variable.name);
			    }
				ConstraintDec(constraint) => {
					parser.builder.add_constraint(constraint);
				}
				ObjectiveDec(constraint) => {
					parser.builder.add_objective(constraint)
				}
				CommentDec(String) => {}
			}
		}

		parser.builder.generate_lp()
	}

	pub fn new() -> Self {
		LPParser {
			builder: LPBuilder::new(),
			variable_declaration_regex: Regex::new(r"var\s+(?P<name>\w+)\s*").unwrap(),
			variable_regex: Regex::new(r"((?:\s*(?P<sign>-)?\s*)(?P<coeff>\d+\.?\d*)\s*\*\s*)?(?P<name>\w+)").unwrap(),
			objective_regex: Regex::new(r"(?P<type>minimize|maximize)\s+(?P<name>\w+)\s*:\s*(?P<equation>[^;]*)").unwrap(),
			equation_component_regex: Regex::new(r"^(?P<vars>[\w\s\*\.\+-]*)\s*((?P<type>==|<=|>=)\s*(?P<constant>\d+\.?\d*)\s*)?$").unwrap(),
			constraint_regex: Regex::new(r"subject to (?P<name>\w*):\s*(?P<terms>[^=><]+?)\s*(?P<type>==|<=|>=)\s*?(?P<constant>\d+\.?\d*)\s*?").unwrap()
		}
	}

	fn get_declarations(&self, text: &str) -> Vec<ParseResult> {
		text.split(';').map(|line| line.trim()).filter(|line| line.len() > 0).map(|line| self.parse_line(line)).collect()
	}

	fn parse_line(&self, line: &str) -> ParseResult {
		match self.line_type(line) {
			Variable => {
				VariableDec(self.parse_variable_declaration(line))
			}, 
			Constraint => {
				ConstraintDec(self.parse_constraint(line))
			},
			Objective => {
				ObjectiveDec(self.parse_objective(line))
			},
			Comment => {
				CommentDec(line.to_string())
			}
		}
	}

	fn line_type(&self, line: &str) -> LineType {
		if line.contains("#") {
			return Comment;
		} else if line.contains("var") {
			return Variable;
		} else if line.contains("minimize") || line.contains("maximize") {
			return Objective;
		} else if line.contains("subject to") {
			return Constraint;
		} 

		panic!("Unknown line type for \"{:?}\"", line);
	}

	fn parse_variable_declaration(&self, data: &str) -> Variable {
		println!("{:?}", data);
		let caps = self.variable_declaration_regex.captures(data).unwrap();
		return Variable {
			name: caps["name"].to_string(),
			coefficient: 0.,
			lower_bound: None,
			upper_bound: None
		}	
	}

	fn parse_constraint(&self, data: &str) -> Constraint {

		let caps = self.constraint_regex.captures(data).unwrap();
		println!("{:?}", caps);
		let name = caps["name"].to_string();
		let form = if caps["type"].contains("<") {
			ConstraintForm::LessThanOrEqual
		} else if caps["type"].contains(">") {
			ConstraintForm::GreaterThanOrEqual
		} else {
			ConstraintForm::Equal
		};
			
		let constant = caps["constant"].parse::<f64>().unwrap();
		let variables = self.parse_objective_vars(&caps["terms"]);

		Constraint {
			variables: variables,
			form: form,
			constant: constant
		}

	}

	fn parse_objective(&self, data: &str) -> Constraint {
		let caps = self.objective_regex.captures(data).expect("Invalid objective!");
		let goal = caps["type"].to_string();
		let name = caps["name"].to_string();
		let form = if goal.contains("maximize") {ConstraintForm::GreaterThanOrEqual} else {ConstraintForm::LessThanOrEqual};

		Constraint {
			variables: self.parse_objective_vars(&caps["equation"]),
			constant: 0.,
			form: form
		}
	}

	fn parse_objective_vars(&self, data: &str) -> Vec<Variable> {
		data.split('+').map(|s| s.trim()).map(|var| self.parse_variable(var)).collect()
	}

	fn parse_variable(&self, data: &str) -> Variable {
		let caps = self.variable_regex.captures(data).unwrap();
		let name = caps["name"].to_string();
		let sign = match caps.name("sign") {
			None => {
				1.
			},
			Some(_) => {
				-1.
			}
		};

		let coefficient = match caps.name("coeff") {
			None => {
				1.
			}, 
			Some(coeff) => {
				coeff.as_str().parse::<f64>().unwrap()
			}
		};

		Variable {
				name: name,
				coefficient: coefficient * sign,
				lower_bound: None,
				upper_bound: None
		}
	}
}

#[cfg(test)]
mod LPParser_tests {
	use super::*;

	#[test]
	fn parse_line_test() {

	}

	#[test]
	fn line_type_test() {
		let p = LPParser::new();

		let comment = "# This is a comment";
		let variable = "var a;";
		let min_objective= "minimize obj: 3*a;";
		let max_objective= "maximize obj: 3*a;";
		let constraint = "subject to foo_constraint: a == 10;";

		assert_eq!(p.line_type(comment), Comment);
		assert_eq!(p.line_type(variable), Variable);
		assert_eq!(p.line_type(min_objective), Objective);
		assert_eq!(p.line_type(max_objective), Objective);
		assert_eq!(p.line_type(constraint), Constraint);
	}


	#[test]
	fn parse_variable_declaration_test() {
		let p = LPParser::new();

		let variable = "var a;";
		let expected = Variable {
			name: "a".to_string(),
			coefficient: 0.,
			upper_bound: None,
			lower_bound: None
		};

		assert_eq!(p.parse_variable_declaration(variable), expected);
	}

	#[test]
	fn parse_objective_vars_test() {
		let p = LPParser::new();

		let data = "3.5*a + 1.5*b + -0.5*c";
		let expected = vec![
			generate_var("a".to_string(), 3.5),
			generate_var("b".to_string(), 1.5),
			generate_var("c".to_string(), -0.5),
		];

		assert_eq!(p.parse_objective_vars(data), expected);
	}

	fn generate_var(name: String, coeff: f64) -> Variable {
		Variable {
			name: name,
			coefficient: coeff,
			upper_bound: None,
			lower_bound: None
		}
	}

	// #[test]
	fn get_declarations_test() {
		let text = "
			#Comment here;

			var a;
			var b;
			var c;

			subject to constraint_1: a + b >= 1;
			subject to constraint_2: b + a <= 5;
			subject to constraint_3: c + b + a >= 0;
			subject to constraint_4: a + c == 10;

		";

		let p = LPParser::new();
		println!("{:?}", p.get_declarations(text));
		// assert!(false);
	}

	// #[test]
	fn parse_test () {
		let text = "
			#Comment here;

			var a;
			var b;
			var c;
			var d;

			maximize profit: 5.5*a + -3.5*b;

			subject to constraint_1: a + b >= 1;
			subject to constraint_2: b + a <= 5;
			subject to constraint_3: c + b + a >= 0;
			subject to constraint_4: a + c == 10;
			subject to constraint_5: d == 10;

		";
		let lp = LPParser::parse_text(text);
		print_matrix(&lp.A);

		println!("{:?}", lp);
		// assert!(false);
	}
}