use lib::*;

fn main() {
	let input = get_input().unwrap_or_else(|err| {
		println!("There was a fatal error! {}", err);
		process::exit(1);
	});

	let data = get_nums(&input).unwrap_or_else(|err| {
		println!("There was a fatal error! {}", err);
		process::exit(1);
	});
	println!("{}", data);
}
