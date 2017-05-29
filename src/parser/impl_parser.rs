use super::*;
use builder::Relation;

#[derive(Debug, PartialEq)]
enum LineType {
	Variable,
	Constraint,
	Objective,
	Comment
}

impl ParserBase for Parser {
	fn parse_components_from_text(text: &str) -> Components {
		unimplemented!();
	}

	fn parse_components_from_file(file: &File) -> Components {
		unimplemented!();
	}

	fn lp_from_text<B: BuilderBase>(text: &str, builder: B) -> Lp {
		unimplemented!();
	}

	fn lp_from_file<B: BuilderBase>(file: &File, builder: B) -> Lp {
		unimplemented!();
	}
}


impl Parser {
	fn new() -> Self {
		Parser {
			variable_declaration_regex: Regex::new(r"var\s+(?P<name>\w+)\s*").unwrap(),
			variable_regex: Regex::new(r"((?:\s*(?P<sign>-)?\s*)(?P<coeff>\d+\.?\d*)\s*\*\s*)?(?P<name>\w+)").unwrap(),
			objective_regex: Regex::new(r"(?P<type>minimize|maximize)\s+(?P<name>\w+)\s*:\s*(?P<equation>[^;]*)").unwrap(),
			equation_component_regex: Regex::new(r"^(?P<vars>[\w\s\*\.\+-]*)\s*((?P<type>==|<=|>=)\s*(?P<constant>\d+\.?\d*)\s*)?$").unwrap(),
			constraint_regex: Regex::new(r"subject to (?P<name>\w*):\s*(?P<terms>[^=><]+?)\s*(?P<type>==|<=|>=)\s*?(?P<constant>\d+\.?\d*)\s*?").unwrap()
		}
	}

	fn get_line_type(&self, line: &str) -> LineType {
		if line.contains("#") {
			return LineType::Comment;
		} else if line.contains("var") {
			return LineType::Variable;
		} else if line.contains("minimize") || line.contains("maximize") {
			return LineType::Objective;
		} else if line.contains("subject to") {
			return LineType::Constraint;
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
		let relation = if caps["type"].contains("<") {
			Relation::LessThanOrEqual
		} else if caps["type"].contains(">") {
			Relation::GreaterThanOrEqual
		} else {
			Relation::Equal
		};
			
		let constant = caps["constant"].parse::<f64>().unwrap();
		let variables = self.parse_objective_vars(&caps["terms"]);

		Constraint {
			name: name,
			variables: variables,
			constant: constant,
			relation: relation
		}

	}

	fn parse_objective(&self, data: &str) -> Objective {
		let caps = self.objective_regex.captures(data).expect("Invalid objective!");

		Objective {
			name: caps["name"].to_string(),
			variables: self.parse_objective_vars(&caps["equation"]),
			constant: 0.,
			maximize: caps["type"].contains("maximize")
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