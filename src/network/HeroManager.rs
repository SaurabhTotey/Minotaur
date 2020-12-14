use crate::network::{NetworkManager, PORT, getNetworkResponse, getUserInput};
use std::net::TcpStream;
use crate::network::Action::Action;
use std::io::Write;
use crate::network::MinotaurMessage::{MinotaurMessage, MINOTAUR_MESSAGE_SIZE};

pub struct HeroManager {
	minotaurStream: TcpStream
}
impl HeroManager {
}
impl NetworkManager for HeroManager {

	fn new() -> HeroManager {
		return HeroManager { minotaurStream: TcpStream::connect("127.0.0.1:".to_string() + &*PORT.to_string()).unwrap() };
	}

	fn handleInput(&mut self, input: Action) -> bool {
		let buffer: [u8; 1] = input.into();
		self.minotaurStream.write(&buffer);
		return false;
	}

	fn handleResponse(&mut self) -> bool {
		let networkResponse = getNetworkResponse::<MinotaurMessage, {MINOTAUR_MESSAGE_SIZE}>(&mut self.minotaurStream);
		println!("{}", networkResponse.toPrintableString());
		return networkResponse.isGameFinished;
	}

	fn run(&mut self) {
		let mut isGameFinished = false;
		while !isGameFinished {
			isGameFinished = self.handleResponse(); // minotaur's action
			if isGameFinished {
				break;
			}
			isGameFinished = self.handleInput(getUserInput()); // hero's action
			if isGameFinished {
				break;
			}
			isGameFinished = self.handleResponse(); // server/minotaur's response to hero's action
		}
	}

}
