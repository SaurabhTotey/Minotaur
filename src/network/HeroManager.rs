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
		let mut buffer = [input.into(); 2];
		buffer[1] = '\n' as u8;
		self.minotaurStream.write(&buffer);
		return false;
	}

	fn handleResponse(&mut self, response: String) -> bool {
		unimplemented!()
	}

	fn run(&mut self) {
		let mut isGameFinished = false;
		while !isGameFinished {
			isGameFinished = self.handleResponse(getNetworkResponse(&mut self.minotaurStream));
			if isGameFinished {
				break;
			}
			isGameFinished = self.handleInput(getUserInput()).unwrap();
		}
	}

}
