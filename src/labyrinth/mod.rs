mod tile;
use tile::Tile;
use std::fmt::{Debug, Formatter, Result};
extern crate rand;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

const WIDTH: usize = 30;
const HEIGHT: usize = 15;

pub struct Labyrinth {
	tiles: [[Tile; WIDTH]; HEIGHT], // all the tiles in the labyrinth
	minotaurCoordinates: (usize, usize),
	heroCoordinates: (usize, usize),
	commonKnowledge: Vec<(usize, usize)> // what tiles are visible to both players
}
impl Labyrinth {

	/**
	 * Creates a new labyrinth
	 */
	pub fn new() -> Labyrinth {
		let mut rng = thread_rng();
		let mut tiles = [[Tile::WALL; WIDTH]; HEIGHT];

		let mut branchCandidates: Vec<(usize, usize)> = Vec::new();
		branchCandidates.push((rng.gen_range(1, HEIGHT), rng.gen_range(1, WIDTH)));
		while !branchCandidates.is_empty() {
			let branchCandidateIndex = rng.gen_range(0, branchCandidates.len());
			let branchCandidate = branchCandidates.remove(branchCandidateIndex);
			tiles[branchCandidate.0][branchCandidate.1] = Tile::WALKABLE;

			let movementDirections = [(1 as isize, 0), (0, 1), (-1, 0), (0, -1)];
			let mut locationCandidates = movementDirections.iter().map(|movementDirection|
				(branchCandidate.0 as isize + movementDirection.0, branchCandidate.1 as isize + movementDirection.1)
			).filter(|location|
				location.0 > 0 && location.1 > 0 && location.0 < HEIGHT as isize - 1 && location.1 < WIDTH as isize - 1
					&& tiles[location.0 as usize][location.1 as usize] == Tile::WALL
					&& movementDirections.iter().map(|innerMovementDirection|
						(location.0 + innerMovementDirection.0, location.1 + innerMovementDirection.1)
					).filter(|innerLocation|
						innerLocation.0 >= 0 && innerLocation.1 >= 0 && innerLocation.0 < HEIGHT as isize && innerLocation.1 < WIDTH as isize
							&& tiles[innerLocation.0 as usize][innerLocation.1 as usize] == Tile::WALL
					).count() >= 3
			).map(|location|
				(location.0 as usize, location.1 as usize)
			).collect::<Vec<_>>();

			if locationCandidates.is_empty() {
				continue;
			}

			let location: (usize, usize) = *locationCandidates.choose(&mut rng).unwrap();
			tiles[location.0][location.1] = Tile::WALKABLE;
			branchCandidates.push(location);

			if locationCandidates.len() > 1 {
				branchCandidates.push(branchCandidate);
			}
		}

		//TODO: place hero, minotaur, and exit on walkable tiles

		return Labyrinth { tiles, minotaurCoordinates: (0, 0), heroCoordinates: (0, 0), commonKnowledge: Vec::new() };
	}

	/**
	 * Returns whether the given coordinates are legal
	 * Legal coordinates are those that can access the tiles value of a Labyrinth struct
	 * Is similar to isInsideLabyrinthBounds, but this method also returns true for the edges
	 */
	fn isLegalCoordinate(location: (usize, usize)) -> bool {
		return location.0 >= 0 && location.1 >= 0 && location.0 < HEIGHT && location.1 < WIDTH
	}

	/**
	 * Returns the legal neighbor coordinates of a given location
	 */
	fn neighborsOf(location: (usize, usize)) -> Vec<(usize, usize)> {
		let movementDirections = [(1 as isize, 0), (0, 1), (-1, 0), (0, -1)];
		return movementDirections.iter()
			.map(|movementDirection| ((location.0 as isize + movementDirection.0) as usize, (location.1 as isize + movementDirection.1) as usize))
			.filter(|neighbor| Labyrinth::isLegalCoordinate(*neighbor)).collect();
	}

	/**
	 * Returns whether the given coordinates are within the labyrinth bounds
	 * Notably excludes the very edges as those should always be WALLs
	 */
	fn isInsideLabyrinthBounds(location: (usize, usize)) -> bool {
		return location.0 > 0 && location.1 > 0 && location.0 < (HEIGHT as isize - 1) as usize && location.1 < (WIDTH as isize - 1) as usize
	}

	//TODO: make methods to make a displayable labyrinth for the player and the minotaur

}
impl Debug for Labyrinth {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			self.tiles.iter()
				.map(|row|
					row.iter().map(|tile| tile.representation()).collect::<Vec<char>>().into_iter().collect()
				)
				.collect::<Vec<String>>().join("\n")
		)
	}
}
