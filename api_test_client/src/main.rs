#![allow(clippy::unused_io_amount)]
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process;

fn main() -> std::io::Result<()> {
	let mut oh = String::from("");
	loop {
		let mut stream = TcpStream::connect("127.0.0.1:7878")?;

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
