use std::net::{TcpListener, TcpStream};
use crate::network::{PORT, NetworkManager};

pub struct MinotaurManager {
	tcpListener: TcpListener,
	heroStream: TcpStream
}
impl MinotaurManager {
}
impl NetworkManager for MinotaurManager {

	fn new() -> MinotaurManager {
		let listener = TcpListener::bind("0.0.0.0:".to_string() + &*PORT.to_string()).unwrap();
		let stream = listener.accept().unwrap().0;
		return MinotaurManager { tcpListener: listener, heroStream: stream };
	}

}
