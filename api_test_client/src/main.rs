#![allow(clippy::unused_io_amount)]
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process;

fn main() -> std::io::Result<()> {
	let mut f = File::open("ip.txt")?;
	let mut ip = String::new();
	f.read_to_string(&mut ip)?;
	let mut oh = String::from("");
	loop {
		let mut stream = TcpStream::connect(&ip)?;

		io::stdin().read_line(&mut oh)?;

		if oh.trim() == "" {
			println!("true");
			stream.shutdown(Shutdown::Both)?;
			break;
		} else {
			oh = oh.trim().to_string();

			stream.write(oh.as_bytes())?;
			stream.flush()?;

			let mut buffer = [0; 512];
			stream.read(&mut buffer)?;
			let result = String::from_utf8_lossy(&buffer[..]);
			let result = result.trim_end_matches(char::from(0));

			println!("Result: {}", result);

			oh = String::from("");
		}
	}
	process::exit(1);
}
