//! The parser module.
//!
//! This code provides an alternative method for
//! inputing linear program problems, either by
//! text input or by reading a file.
extern crate regex;
use self::regex::Regex;

use lp::Lp;
use std::fs::File;
use builder::{Variable, Constraint, Objective, Builder, BuilderBase};

mod impl_parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Components {
	pub variables: Vec<Variable>,
	pub constraints: Vec<Constraint>,
	pub objective: Objective
}

pub trait ParserBase {
	fn parse_components_from_text(text: &str) -> Components;
	fn parse_components_from_file(file: &mut File) -> Components;

	fn lp_from_text<B: BuilderBase>(text: &str, mut builder: B) -> Lp;
	fn lp_from_file<B: BuilderBase>(file: &mut File, mut builder: B) -> Lp;
}

#[derive(Debug)]
pub struct Parser {
	variable_declaration_regex: Regex,
	variable_regex: Regex,
	constraint_regex: Regex,
	equation_component_regex: Regex,
	objective_regex: Regex,
}

// #[cfg(test)]
// mod LPParser_tests {
// 	use super::*;

// 	#[test]
// 	fn parse_line_test() {

// 	}

// 	#[test]
// 	fn line_type_test() {
// 		let p = LPParser::new();

// 		let comment = "# This is a comment";
// 		let variable = "var a;";
// 		let min_objective= "minimize obj: 3*a;";
// 		let max_objective= "maximize obj: 3*a;";
// 		let constraint = "subject to foo_constraint: a == 10;";

// 		assert_eq!(p.line_type(comment), Comment);
// 		assert_eq!(p.line_type(variable), Variable);
// 		assert_eq!(p.line_type(min_objective), Objective);
// 		assert_eq!(p.line_type(max_objective), Objective);
// 		assert_eq!(p.line_type(constraint), Constraint);
// 	}


// 	#[test]
// 	fn parse_variable_declaration_test() {
// 		let p = LPParser::new();

// 		let variable = "var a;";
// 		let expected = Variable {
// 			name: "a".to_string(),
// 			coefficient: 0.,
// 			upper_bound: None,
// 			lower_bound: None
// 		};

// 		assert_eq!(p.parse_variable_declaration(variable), expected);
// 	}

// 	#[test]
// 	fn parse_objective_vars_test() {
// 		let p = LPParser::new();

// 		let data = "3.5*a + 1.5*b + -0.5*c";
// 		let expected = vec![
// 			generate_var("a".to_string(), 3.5),
// 			generate_var("b".to_string(), 1.5),
// 			generate_var("c".to_string(), -0.5),
// 		];

// 		assert_eq!(p.parse_objective_vars(data), expected);
// 	}

// 	fn generate_var(name: String, coeff: f64) -> Variable {
// 		Variable {
// 			name: name,
// 			coefficient: coeff,
// 			upper_bound: None,
// 			lower_bound: None
// 		}
// 	}

// 	// #[test]
// 	fn get_declarations_test() {
// 		let text = "
// 			#Comment here;

// 			var a;
// 			var b;
// 			var c;

// 			subject to constraint_1: a + b >= 1;
// 			subject to constraint_2: b + a <= 5;
// 			subject to constraint_3: c + b + a >= 0;
// 			subject to constraint_4: a + c == 10;

// 		";

// 		let p = LPParser::new();
// 		println!("{:?}", p.get_declarations(text));
// 		// assert!(false);
// 	}

// 	// #[test]
// 	fn parse_test () {
// 		let text = "
// 			#Comment here;

// 			var a;
// 			var b;
// 			var c;
// 			var d;

// 			maximize profit: 5.5*a + -3.5*b;

// 			subject to constraint_1: a + b >= 1;
// 			subject to constraint_2: b + a <= 5;
// 			subject to constraint_3: c + b + a >= 0;
// 			subject to constraint_4: a + c == 10;
// 			subject to constraint_5: d == 10;

// 		";
// 		let lp = LPParser::parse_text(text);
// 		print_matrix(&lp.A);

// 		println!("{:?}", lp);
// 		// assert!(false);
// 	}
// }