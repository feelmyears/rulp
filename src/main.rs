extern crate clap;
use clap::{Arg, App};

extern crate rulp;
use rulp::builder::{Builder, BuilderBase};
use rulp::parser::{Parser, ParserBase};
use rulp::solver::{SolverBase, SimplexSolver};

use std::io::{Write};
use std::fs::File;

fn main() {
	let matches = App::new("myapp")
                      	.version("1.0")
                      	.author("Philip M. Meyers <philipmeyers2017@u.northwestern.edu> and Andy McConnell <andrewmcconnell2016@u.northwestern.edu>")
                      	.about("Linear program solver in Rust, Project for EECS 395 Spring 2017 course taught by Dr. Jesse Tov")
                      	.arg(Arg::with_name("input")
                           	.short("i")
                           	.long("input")
                           	.value_name("SOURCE")
                           	.help("source path of file with LP to optimize")
                           	.takes_value(true)
                      		.required(true))
                      	.arg(Arg::with_name("output")
                           	.short("o")
                           	.long("output")
                           	.value_name("DESTINATION")
                           	.help("destination path of LP solution")
                           	.takes_value(true)
                      		.required(true))
                      	.arg(Arg::with_name("display")
                           	.short("d")
                           	.long("display")
                           	.help("displays the LP solution in the console")
                      		.required(false))
                      	.get_matches();

    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();

    let mut input_file = File::open(input_path).unwrap();

	let builder = Builder::new();
	let lp = Parser::lp_from_file(&mut input_file, builder);

	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();

	let mut output_file = File::create(&output_path).unwrap();
	output_file.write(format!("{:}", &solution).as_bytes()).expect("Failed to write to destination file");

	if matches.is_present("display") {
		println!("{:}", &solution);
	}
}	