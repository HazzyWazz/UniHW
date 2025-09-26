#![allow(non_snake_case)]
use std::io::{self, Write, Result}; // import std::io
use std::cmp;

fn prompt() -> Result<String> {
	// ensure the above is outputted before reading input, returns the error if there's an error flushing
	io::stdout().flush()?;

	let mut input = String::new();
	// read line from stdin into input, returning the error if there's an error reading the line
	io::stdin().read_line(&mut input)?;

	// remove whitespace from the input
	Ok(input.trim().to_string())
}

fn main() {
	let x: i32 = prompt().expect("Invalid value!").parse().expect("Invalid value!");
	let y: i32 = prompt().expect("Invalid value!").parse().expect("Invalid value!");
	let z: i32 = prompt().expect("Invalid value!").parse().expect("Invalid value!");

	let x_abs = x.abs();
	let y_abs = y.abs();
	let z_abs = z.abs();

	let max = cmp::max(cmp::max(x_abs, y_abs), z_abs);

	if x_abs == y_abs || z_abs == y_abs || x_abs == z_abs {
		if max == x {
			print!("{}", x);
		} else if max == y {
			print!("{}", y);
		} else if max == z {
			print!("{}", z);
		}
	} else {
		if max == x_abs {
			print!("{}", x);
		} else if max == y_abs {
			print!("{}", y);
		} else if max == z_abs {
			print!("{}", z);
		}
	}
}
