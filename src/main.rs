#![allow(nonstandard_style)]

use std::net::{TcpStream, TcpListener};

mod labyrinth;

const PORT: i32 = 6669;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 || !["minotaur".to_string(), "hero".to_string()].contains(&args[1]) {
		panic!("Program must be run with argument of \"minotaur\" or \"hero\".")
	}
	else if args[1] == "minotaur" {
		let mut listener = TcpListener::bind("127.0.0.1:".to_string() + &*PORT.to_string()).unwrap();
		listener.accept();
	}
	else {
		let mut stream = TcpStream::connect("127.0.0.1:".to_string() + &*PORT.to_string()).unwrap();
	}
}
