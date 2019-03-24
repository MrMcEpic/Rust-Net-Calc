#![allow(clippy::unused_io_amount)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};

use lib::get_nums;

fn main() {
	runner().unwrap_or_else(|x| {
		panic!("{}", x);
	});
}

fn runner() -> Result<(), Box<dyn Error>> {
	let mut f = File::open("ip.txt")?;
	let mut ip = String::new();
	f.read_to_string(&mut ip)?;
	let listener = TcpListener::bind(&ip)?;

	for stream in listener.incoming() {
		let stream = stream?;
		handle_connection(stream).unwrap_or_else(|x| {
			println!("Connection Closed {}", x);
		});
	}
	Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	let mut buffer = [0; 512];
	stream.read(&mut buffer)?;

	let val = String::from_utf8_lossy(&buffer[..]);
	let val = val.trim_end_matches(char::from(0));

	println!("{}", val);

	let data = get_nums(val)?;
	stream.write(format!("{}", data).as_bytes())?;
	stream.flush()?;

	println!("{}", data);
	Ok(())
}
