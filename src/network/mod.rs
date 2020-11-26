pub mod HeroManager;
pub mod MinotaurManager;

const PORT: i32 = 6669;

pub trait NetworkManager {

	/**
	 * Creates a NetworkManager
	 */
	fn new() -> Self where Self: Sized;

	/**
	 * Blocks and gets user input from stdin
	 */
	fn awaitInput(&self) -> String {
		let mut buffer = String::new();
		let mut stdin = std::io::stdin();
		stdin.read_line(&mut buffer).unwrap();
		return buffer;
	}

	/*
	 * Should block and get a response from the other player over the network
	 */
	//fn awaitResponse(&self) -> String;

}
