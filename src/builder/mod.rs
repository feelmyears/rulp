//! The builder module.
//!
//! This code handles most of the user-facing
//! functionality of constructing a new linear
//! programming problem.

pub use std::collections::{HashMap, HashSet};
pub use lp::Lp;

mod impl_builder;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
	pub name: String,
	pub coefficient: f64
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constraint {
	pub name: String,
	pub variables: Vec<Variable>,
	pub constant: f64,
	pub relation: Relation
}

#[derive(Debug, PartialEq, Clone)]
pub struct Objective {
	pub name: String,
	pub variables: Vec<Variable>,
	pub constant: f64,
	pub maximize: bool 
}

#[derive(Debug, PartialEq, Clone)]
pub enum Relation {
	Equal,
	LessThanOrEqual,
	GreaterThanOrEqual
}

pub trait BuilderBase {
	fn new() -> Self;
	fn add_variable(&mut self, variable: Variable);
	fn add_constraint(&mut self, constraint: Constraint);
	fn add_objective(&mut self, objective: Objective);
	fn build_lp(&mut self) -> Lp;
}

#[derive(Debug)]
pub struct Builder {
	variables: HashSet<String>,
	variable_indices: HashMap<String, usize>,
	constraints: Vec<Constraint>,
	objective: Option<Objective>
}