//! The parser module.
//!
//! This code provides an alternative method for
//! inputing linear program problems, either by
//! text input or by reading a file.
extern crate regex;
use self::regex::Regex;

use lp::Lp;
use std::fs::File;
use builder::{Variable, Constraint, Objective, BuilderBase};


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

	fn lp_from_text<B: BuilderBase>(text: &str, builder: B) -> Lp;
	fn lp_from_file<B: BuilderBase>(file: &mut File, builder: B) -> Lp;
}

#[derive(Debug)]
pub struct Parser {
	variable_declaration_regex: Regex,
	variable_regex: Regex,
	constraint_regex: Regex,
	equation_component_regex: Regex,
	objective_regex: Regex,
}
