# rulp
The crate is currently on version 0.1.0
Read the API Documentation to learn more.

## Summary
Rulp is a linear programming library written in Rust.
The goal of rulp is to provide simple and convenient methods for users to parse or build linear programming problems, and solve them.

## Implementation
This project is implemented using Rust.
Currently the library primarily makes use of the rulinalg library for matrix formatting.

## Usage
The library usage is well described in the API documentation, including example code.

### Installation
The library is most easily used with cargo. Simply include the following in your Cargo.toml file:
```
[dependencies]
rulp="0.1.0"
```
And then import the library using:
```
extern crate rulinalg;
```
Then import the modules and you're done!
```
fn minimize_text_test() {
	let text_problem = "	
		# Radiation Example;

		var x_1;
		var x_2;

		minimize healthy_anatomy_exposure: 0.4*x_1 + 0.5*x_2;
		subject to critical_tissues: 0.3*x_1 + 0.1*x_2 <= 2.7;
		subject to tumor_region: 0.5*x_1 + 0.5*x_2 == 6;
		subject to tumor_center: 0.6*x_1 + 0.4*x_2 >= 6;
	";

	let builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
	print!("finished solving");
	print!("{:?}", &solution);

	let expected_sol = vec![7.5, 4.5];
	let sol = solution.values.unwrap();
	for i in 0 .. expected_sol.len() {
		assert_approx_eq!(expected_sol[i], sol[i]);
	}
	assert_approx_eq!(5.25, solution.objective.unwrap());
}
```
