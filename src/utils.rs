use rulinalg::matrix::{BaseMatrix, Matrix};

use std::fs::File;
use std::io::prelude::*;

/// Prints out a matrix into a well spaced, easy to read form
pub fn print_matrix(mat: &Matrix<f64>) {
	unsafe{
		println!("");
		for r in 0 .. mat.rows() {
			for c in 0 .. mat.cols() - 1{
				print!("{:>6.3}, ", *mat.get_unchecked([r, c]));
			}
			println!("{:>6.3}", *mat.get_unchecked([r, mat.cols() -1]));
		}
		println!("");	
	}
}

/// Reads the contents of a file and returns the contents in a String
pub fn read_file_contents(file: &mut File) -> String {
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Failed to read file!");
	contents
}