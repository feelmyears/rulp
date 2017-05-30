use std::io::{Write};
use std::fs::File;
use std::env;

extern crate rulp;

use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SolverBase, SimplexSolver, Solution};
use rulp::lp::{Lp, Optimization};

fn main() {
	let args: Vec<String> = env::args().collect();
	let ref source_path = args[1];
	let mut source_file = File::open(source_path).unwrap();

	let mut builder = Builder::new();
	let lp = Parser::lp_from_file(&mut source_file, builder);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();

	// If there is an output path, write to it
	if args.len() >= 3 {
		let mut output_file = File::create(&args[2]).unwrap();
		output_file.write(format!("{:?}", solution).as_bytes()).expect("Failed to write to destination");
	} else {
		println!("{:?}", solution);
	}
}