use crate::network::{NetworkManager, PORT, getNetworkResponse, getUserInput};
use std::net::TcpStream;
use crate::network::Action::Action;
use std::io::Write;

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
		let mut buffer = input.into();
		self.minotaurStream.write(&buffer);
		return false;
	}

	fn handleResponse(&mut self, response: String) -> bool {
		unimplemented!()
	}

	fn run(&mut self) {
		let mut isGameFinished = false;
		while !isGameFinished {
			let networkResponse = getNetworkResponse(&mut self.minotaurStream);
			isGameFinished = self.handleResponse(networkResponse);
			if isGameFinished {
				break;
			}
			isGameFinished = self.handleInput(getUserInput());
		}
	}

}
