//! The solver module.
//!
//! This code provides the rulp library with
//! linear program-solving capabilities.
//! This is implemented via the Simplex Tableau
//! method.
use std::fmt;
use rulinalg::matrix::{Matrix};
use lp::Lp;

mod impl_solver;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
	Optimal,
	Infeasible,
	Unbounded
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
	lp: Lp,
	pub values: Option<Vec<f64>>, 
	pub objective: Option<f64>,
	pub status: Status
}

pub trait SolverBase {
	fn new(lp: Lp) -> Self;
	fn solve(&self) -> Solution;
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimplexSolver {
	lp: Lp,
	pub tableau: Matrix<f64>
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	writeln!(f, "").unwrap();
        match self.status {
            Status::Optimal => {
                match self.objective {
                    None => {

                    },
                    Some(ref obj) => {
                        writeln!(f, "Optimal objective: {:}", obj).unwrap();
                    }
                }
                match self.values {
                    None => {

                    },
                    Some(ref vals) => {
                        for i in 0 .. vals.len() - self.lp.num_artificial_vars {
                            writeln!(f, "{:}: {:?}", self.lp.vars[i],vals[i]).unwrap();
                        }
                    }
                }
            },  
            Status::Infeasible => {
                writeln!(f, "Infeasible").unwrap();
            },
            Status::Unbounded => {
                writeln!(f, "Unbounded").unwrap();
            }
        }

        write!(f, "")
    }
}