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
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();

	let expected_sol = vec![7.5, 4.5];
	let sol = solution.values.unwrap();
	for i in 0 .. expected_sol.len() {
		assert_approx_eq!(expected_sol[i], sol[i]);
	}
	assert_approx_eq!(5.25, solution.objective.unwrap());
}
```

## LP Syntax
rulp provides a simple syntax for modeling linear programs similar to that of AMPL. Whitespace is ignore and **all lines (even comments) must be terminated by a semicolon (;)**. Syntax is case sensitive and keywords (`var`, `minimze`, `maximize`, and `subject to`) must be lowercase. All coefficients must be numbers. Invalid syntax will cause the program to crash. 
### Variable Declaration
Variables are declared by `var var_name;` so to declare a variable called `foo` would be declared as `var foo;`. Variables are assumed to be non-negative. Other constraints on variable values must be declared as constraints (as opposed to the inline syntax that AMPL offers). Variable names must be unique and may contain any word characters a-z, A-Z, 0-9, and _ (underscore). Variables names must not follow the format `excess_#` or `slack_#` (e.g. `excess_0` or `slack_7`) as this may conflict with the library's underlying representation for slack and excess variables when converting to standard form. 
### Objective Declaration
Objectives are declared as `[minimize|maximize] obj_name: coeff_1*x_1 + coeff_2*x_2 + -coeff_3*x_3;`. Sample objectives may be `maximize profits: 5.*price_1 + 15.5*price_2 + -3*production_cost;` and `minimize time: duration_1 + 2*duration_2;`. All variables must be separated by `+` (variables negative objective coefficients are formatted as `(... +) -c*x (+ ...)`. Coefficients of `1` may be omitted. All variables contained in the objective function must be declared beforehand. Only one objective function is permitted per LP.
### Constraint Declaration
Objectives are declared as `subject to constraint_name: -coeff_1*var_1 + coeff_2*var_2+ -coeff_3*var_3 [==|<=|==] constant;`. A sample objective may look like `subject to production_minimum: 12*bagels + 14*doughnuts >= 66;`. As in the objective function, all variables must be separated by `+` (variables negative objective coefficients are formatted as `(... +) -c*x (+ ...)`. Coefficients of `1` may be omitted. All variables contained in a constraint function must be declared beforehand. Constraint names must be unique. 

A sample LP to maximize profits at a bakery follows:

```
# bakery.lp;

# Vars;
# bagels = number of bagels produced;
# doughnuts = number of doughuts produced;
var bagels;
var doughnuts;

# Objective;
maximize profits: 3*bagels + 1.25*doughnuts;

# Constraints;
subject to flour: 12*bagels + 6.5*doughnuts <= 400;
subject to milk: 1*bagels + .5*doughnuts <= 200;
subject to sugar: 2*doughnuts + 0.25*bagels <= 200;
subject to bagel_min: bagels >= 12;
subject to doughnut_min: doughnuts >= 14;
```

## Running rulp in the command line
rulp offers a simple CLI to optimize LPs declared in the above syntax. The CLI takes in 2 required inputs: `-i/--input` for the source file path and `-o/--output` for the destination file path. An optional flag `-d/--display` will also print the results of the solution to the command line. A sample command line interaction may be: 

```
~ ./rulp -i bakery.lp -o bakery.sol -d
  
Objective: 94.75
bagels: 25.75
doughnuts: 14
```

## Acknowledgements
We offer our thanks to [Prof. Jesse Tov](http://users.eecs.northwestern.edu/~jesse/) for teaching us Rust this quarter at Northwestern University. We also thank [Prof. Andreas Wächter](http://users.iems.northwestern.edu/~andreasw/) whose notes and sample problems from IEMS 313 were valuable in implementing and testing rulp. Our presentation on this project is available on [Google Slides](https://docs.google.com/presentation/d/1wrIj6-vqYLlUw0w4H3f_DWa6Of2Hpiv3eoH67ACDEBw/pub?start=false&loop=false&delayms=3000).
