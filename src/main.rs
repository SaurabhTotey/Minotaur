#![allow(nonstandard_style)]

use crate::network::NetworkManager;

mod network;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 || !["minotaur".to_string(), "hero".to_string()].contains(&args[1]) {
		panic!("Program must be run with argument of \"minotaur\" or \"hero\".")
	}
	let manager: Box<dyn NetworkManager> = if args[1] == "minotaur" {
		 Box::new(network::MinotaurManager::MinotaurManager::new())
	}
	else {
		Box::new(network::HeroManager::HeroManager::new())
	};
}
