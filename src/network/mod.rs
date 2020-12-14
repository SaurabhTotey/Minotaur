use std::net::TcpStream;
use std::io::Read;

pub mod HeroManager;
pub mod MinotaurManager;
pub mod Action;
pub mod MinotaurMessage;

const PORT: i32 = 6669;

/**
 * Blocks and gets input from stdin
 */
fn getUserInput() -> Action::Action {
	let mut buffer = String::new();
	let stdin = std::io::stdin();
	stdin.read_line(&mut buffer).unwrap();
	return Action::Action::from([buffer.as_bytes()[0]; 1]);
}

/**
 * Blocks and gets a response from the given TcpStream
 */
fn getNetworkResponse<T: From<[u8; N]>, const N: usize>(stream: &mut TcpStream) -> T {
	let mut buffer = [0u8; N];
	stream.read_exact(&mut buffer);
	return buffer.into();
}

pub trait NetworkManager {

	/**
	 * Creates a NetworkManager
	 */
	fn new() -> Self where Self: Sized;

	/**
	 * Should handle an input from the user
	 * Doesn't need to handle actually taking the input, just responding to it
	 * Should return whether the program is done
	 */
	fn handleInput(&mut self, input: Action::Action) -> bool;

	/**
	 * Should handle a response from the other player over the network
	 * Network manager should take in the network response itself in this method
	 * Should return whether the program is done
	 */
	fn handleResponse(&mut self) -> bool;

	/**
	 * Should actually run the game to completion
	 */
	fn run(&mut self);

}
