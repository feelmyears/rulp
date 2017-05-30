use super::*;
use builder::Relation;
use utils::read_file_contents;

#[derive(Debug, PartialEq)]
enum LineType {
	Variable,
	Constraint,
	Objective,
	Comment
}

#[derive(Debug, PartialEq)]
enum Component {
	Variable(Variable),
	Constraint(Constraint),
	Objective(Objective),
	Comment
}


impl ParserBase for Parser {
	/// Constructor for Components struct.
	///
	/// Takes a string input to be parsed.
	fn parse_components_from_text(text: &str) -> Components {
		let p = Parser::new();
		p.get_components(text)
	}

	/// Constructor for Components struct.
	/// 
	/// Takes a file input to be read.
	fn parse_components_from_file(file: &mut File) -> Components {
		Self::parse_components_from_text(&read_file_contents(file))
	}

	/// Constructor for Lp struct.
	///
	/// Takes a string input to be parsed and a Builder struct.
	fn lp_from_text<B: BuilderBase>(text: &str, mut builder: B) -> Lp {
		let components = Self::parse_components_from_text(text);

		for v in components.variables {
			builder.add_variable(v);
		}

		for c in components.constraints {
			builder.add_constraint(c);
		}

		builder.add_objective(components.objective);

		builder.build_lp()
	}

	/// Constructor for Lp struct.
	///
	/// Takes a file input to be read and a Builder struct.
	fn lp_from_file<B: BuilderBase>(file: &mut File, builder: B) -> Lp {
		Self::lp_from_text(&read_file_contents(file), builder)
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

	fn get_components(&self, text: &str) -> Components {
		let components: Vec<Component> = text
			.split(';')
			.map(|line| line.trim())
			.filter(|line| line.len() > 0)
			.map(|line| self.component_from_line(line))
			.filter(|component| *component != Component::Comment)
			.collect();

		let mut variables = vec![];
		let mut constraints = vec![];
		let mut objective = None;

		for c in components {
			match c {
				Component::Variable(var) => {
					variables.push(var);
				},
				Component::Constraint(con) => {
					constraints.push(con);
				},
				Component::Objective(obj) => {
					objective = Some(obj);
				},
				Component::Comment => {}
			}
		}

		Components {
			variables: variables,
			constraints: constraints,
			objective: objective.expect("No objective function provided!")
		}
	}

	fn component_from_line(&self, line: &str) -> Component {
		match self.get_line_type(line) {
			LineType::Variable => {
				Component::Variable(self.parse_variable_declaration(line))
			},
			LineType::Constraint => {
				Component::Constraint(self.parse_constraint(line))
			},
			LineType::Objective => {
				Component::Objective(self.parse_objective(line))
			},
			LineType::Comment => Component::Comment,
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
		let caps = self.variable_declaration_regex.captures(data).unwrap();
		return Variable {
			name: caps["name"].to_string(),
			coefficient: 0.,
		}	
	}

	fn parse_constraint(&self, data: &str) -> Constraint {
		let caps = self.constraint_regex.captures(data).unwrap();
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
		}
	}
}