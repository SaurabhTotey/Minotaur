use std::net::{TcpListener, TcpStream};
use crate::network::{PORT, NetworkManager};
use crate::labyrinth::Labyrinth;
use crate::network::Action::Action;

pub struct MinotaurManager {
	tcpListener: TcpListener,
	heroStream: TcpStream,
	labyrinth: Labyrinth
}
impl MinotaurManager {
}
impl NetworkManager for MinotaurManager {

	fn new() -> MinotaurManager {
		let listener = TcpListener::bind("0.0.0.0:".to_string() + &*PORT.to_string()).unwrap();
		let stream = listener.accept().unwrap().0;
		return MinotaurManager { tcpListener: listener, heroStream: stream, labyrinth: Labyrinth::new() };
	}

	fn handleInput(&mut self, input: Action) -> bool {
		unimplemented!()
	}

	fn handleResponse(&mut self, response: String) -> bool {
		unimplemented!()
	}

	fn run(&mut self) {
		let mut isGameFinished = false;
		while !isGameFinished {

		}
	}

}
