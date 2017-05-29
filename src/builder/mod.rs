mod impl_builder;

pub use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Builder {
	variables: HashSet<String>,
	variable_indices: HashMap<String, usize>,
	constraints: Vec<Constraint>,
	objective: Option<Constraint>
}

#[derive(Debug, PartialEq)]
pub struct Variable {
	pub name: String,
	pub coefficient: f64,
	pub upper_bound: Option<f64>,
	pub lower_bound: Option<f64>
}

#[derive(Debug, PartialEq)]
pub struct Constraint {
	pub name: String,
	pub variables: Vec<Variable>,
	pub constant: f64,
	pub relation: Relation
}

#[derive(Debug, PartialEq)]
pub struct Objective {
	pub name: String,
	pub variables: Vec<Variable>,
	pub constant: f64,
}

#[derive(Debug, PartialEq)]
pub enum Relation {
	Equal,
	LessThanOrEqual,
	GreaterThanOrEqual
}