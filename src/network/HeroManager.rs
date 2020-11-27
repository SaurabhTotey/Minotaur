use crate::network::{NetworkManager, PORT, getNetworkResponse};
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
		unimplemented!()
	}

	fn handleResponse(&mut self, response: String) -> bool {
		unimplemented!()
	}

	fn run(&mut self) {
		let mut isGameFinished = false;
		while !isGameFinished {
			//TODO: delete below
			println!("{}", getNetworkResponse(&mut self.minotaurStream));
			self.minotaurStream.write(b"Hello, from the hero!\n").unwrap();
			isGameFinished = true;
		}
	}

}
