use rulinalg::matrix::{BaseMatrix, Matrix};

pub fn print_matrix(mat: &Matrix<f64>) {
	unsafe{
		println!("");
		for r in 0 .. mat.rows() {
			for c in 0 .. mat.cols() - 1{
				print!("{:>5.1}, ", *mat.get_unchecked([r, c]));
			}
			println!("{:>5.1}", *mat.get_unchecked([r, mat.cols() -1]));
		}
		println!("");	
	}
}