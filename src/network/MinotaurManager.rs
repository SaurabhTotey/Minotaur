use std::net::{TcpListener, TcpStream};
use crate::network::{PORT, NetworkManager, getNetworkResponse, getUserInput};
use crate::labyrinth::Labyrinth;
use crate::network::Action::Action;
use crate::network::MinotaurMessage::{MinotaurMessage, MINOTAUR_MESSAGE_SIZE};
use crate::labyrinth::tile::Tile;
use std::io::Write;

pub struct MinotaurManager {
	tcpListener: TcpListener,
	heroStream: TcpStream,
	labyrinth: Labyrinth
}
impl MinotaurManager {

	/**
	 * Gets a new location after applying the given action to the given initial location
	 */
	fn locationAfterActionOn(self, initialLocation: (usize, usize), action: Action) -> (usize, usize) {
		let newLocation = match action {
			Action::PERFORM_ACTION => initialLocation,
			Action::MOVE_UP => (initialLocation.0 - 1, initialLocation.1),
			Action::MOVE_DOWN => (initialLocation.0 + 1, initialLocation.1),
			Action::MOVE_LEFT => (initialLocation.0, initialLocation.1 - 1),
			Action::MOVE_RIGHT => (initialLocation.0, initialLocation.1 + 1)
		};
		if !self.labyrinth.isWalkable(newLocation) {
			return initialLocation;
		}
		return newLocation;
	}

	/**
	 * Sends the game state as a MinotaurMessage to the hero
	 */
	fn sendState(&mut self) {
		let winner = self.labyrinth.getWinner();
		let message = MinotaurMessage {
			isGameFinished: winner.is_some(),
			isWinnerMinotaur: winner.contains(&Tile::MINOTAUR),
			map: self.labyrinth.viewFrom(self.labyrinth.heroCoordinates.0)
		};
		let sendableMessage: [u8; MINOTAUR_MESSAGE_SIZE] = message.into();
		self.heroStream.write(&sendableMessage);
	}

}
impl NetworkManager for MinotaurManager {

	fn new() -> MinotaurManager {
		let listener = TcpListener::bind("0.0.0.0:".to_string() + &*PORT.to_string()).unwrap();
		let stream = listener.accept().unwrap().0;
		return MinotaurManager { tcpListener: listener, heroStream: stream, labyrinth: Labyrinth::new() };
	}

	fn handleInput(&mut self, input: Action) -> bool {
		//TODO: change game state based on minotaur's input
		self.sendState();
		if self.labyrinth.getWinner().is_some() {
			println!("{}", "You have caught the hero!");
			return true;
		}
		return false;
	}

	fn handleResponse(&mut self) -> bool {
		let networkResponse = getNetworkResponse::<Action, 1>(&mut self.heroStream);
		//TODO: change game state based on hero's input
		self.sendState();
		if self.labyrinth.getWinner().is_some() {
			println!("{}", "The hero has escaped the labyrinth!");
			return true;
		}
		return false;
	}

	fn run(&mut self) {
		let mut isGameFinished = false;
		while !isGameFinished {
			isGameFinished = self.handleInput(getUserInput());
			if isGameFinished {
				break;
			}
			isGameFinished = self.handleResponse();
		}
	}

}
