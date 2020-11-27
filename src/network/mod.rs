use std::net::TcpStream;
use std::io::Read;

pub mod HeroManager;
pub mod MinotaurManager;
pub mod Action;

const PORT: i32 = 6669;

/**
 * Blocks and gets input from stdin
 * TODO: interpret and make this return an Action
 */
fn getUserInput() -> String {
	let mut buffer = String::new();
	let mut stdin = std::io::stdin();
	stdin.read_line(&mut buffer).unwrap();
	return buffer;
}

/**
 * Blocks and gets a response from the given TcpStream
 */
fn getNetworkResponse(stream: &mut TcpStream) -> String {
	let mut buffer = String::new();
	stream.read_to_string(&mut buffer).unwrap();
	return buffer;
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
	fn handleInput(&self, input: Action::Action) -> bool;

	/**
	 * Should handle a response from the other player over the network
	 * Should return whether the program is done
	 */
	fn handleResponse(&self, response: String) -> bool;

	/**
	 * Should actually run the game to completion
	 */
	fn run();

}
