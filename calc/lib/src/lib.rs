pub use my_error::*;
pub use std::env;
pub use std::io;
pub use std::process;

pub fn get_input() -> Result<String, Box<dyn Error>> {
	println!("Input simple math");
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	input.trim();
	input = input.trim().to_string();
	Ok(input)
}

pub fn get_nums(input: &str) -> Result<f64, Box<dyn Error>> {
	let line: Vec<char> = input.chars().collect();
	let mut hold: Vec<f64> = vec![];
	let mut ops: Vec<char> = vec![];
	let mut n = String::new();

	for i in line {
		if i == ' ' {
			continue;
		} else if !i.is_digit(10) && i != '.' {
			let x: f64 = n.trim().parse()?;
			hold.push(x);
			ops.push(i);
			n = String::new();
		} else {
			n.push(i);
		}
	}
	let x: f64 = n.trim().parse()?;
	hold.push(x);

	ops.reverse();
	hold.reverse();
	let mut _val: f64 = 0.0;
	let ops_2 = ops.clone();
	if env::var("Debug").is_ok() {
		println!("{:?}", hold);
	}
	_val = hold.pop().unwrap();;

	for _ in ops_2 {
		let (mut _val_2, mut _op) = (0.0, 'x');
		match (hold.pop(), ops.pop()) {
			(Some(i), Some(o)) => {
				_val_2 = i;
				_op = o;
			}
			(Some(_), None) => return Err(MyError::new("Operator List Popping Error")),
			(None, Some(_)) => return Err(MyError::new("Number List Popping Error")),
			_ => return Err(MyError::new("Popping Error")),
		};
		_val = do_math(_val, _op, _val_2).unwrap_or_else(|err| {
			println!("There was a fatal error! {}", err);
			process::exit(1);
		})
	}

	Ok(_val)
}
pub fn do_math(num_1: f64, op: char, num_2: f64) -> Result<f64, Box<dyn Error>> {
	Ok(match op {
		'+' => num_1 + num_2,
		'-' => num_1 - num_2,
		'*' | 'x' => num_1 * num_2,
		'/' => num_1 / num_2,
		'^' => num_1.powf(num_2),
		_ => return Err(MyError::new("Decoding Error")),
	})
}
