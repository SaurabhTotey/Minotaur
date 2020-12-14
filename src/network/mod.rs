use std::net::TcpStream;
use std::io::Read;
use std::mem::size_of;

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
 * TODO: look into why below definition doesn't compile
 */
fn getNetworkResponse<T: Sized>(stream: &mut TcpStream) -> T where T: Sized + From<[u8; size_of::<T>()]> {
	let mut buffer = [0u8; size_of::<T>()];
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
	 * Should return whether the program is done
	 */
	fn handleInput(&mut self, input: Action::Action) -> bool;

	/**
	 * Should handle a response from the other player over the network
	 * Should return whether the program is done
	 */
	fn handleResponse(&mut self, response: String) -> bool;

	/**
	 * Should actually run the game to completion
	 */
	fn run(&mut self);

}
