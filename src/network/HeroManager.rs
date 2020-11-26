use crate::network::{NetworkManager, PORT};
use std::net::TcpStream;

pub struct HeroManager {
	minotaurStream: TcpStream
}
impl HeroManager {
}
impl NetworkManager for HeroManager {

	fn new() -> HeroManager {
		return HeroManager { minotaurStream: TcpStream::connect("127.0.0.1:".to_string() + &*PORT.to_string()).unwrap() };
	}

	fn handleInput(&self, input: String) -> bool {
		unimplemented!()
	}

	fn handleResponse(&self, response: String) -> bool {
		unimplemented!()
	}

}
