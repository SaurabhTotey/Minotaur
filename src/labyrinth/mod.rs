mod tile;
use tile::Tile;
use std::fmt::{Debug, Formatter, Result};
extern crate rand;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use std::cmp::min;

const WIDTH: usize = 30;
const HEIGHT: usize = 15;
const MINIMUM_DISTANCE_BETWEEN_STARTING_POSITIONS: i32 = 10;

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
		branchCandidates.push((rng.gen_range(1, HEIGHT - 1), rng.gen_range(1, WIDTH - 1)));
		let mut walkableTiles: Vec<(usize, usize)> = Vec::new();
		walkableTiles.push(branchCandidates[0]);

		while !branchCandidates.is_empty() {
			let branchCandidate = branchCandidates.remove(rng.gen_range(0, branchCandidates.len()));
			tiles[branchCandidate.0][branchCandidate.1] = Tile::WALKABLE;

			let locationCandidates = Labyrinth::neighborsOf(branchCandidate).into_iter()
				.filter(|location|
					Labyrinth::isInsideLabyrinthBounds(*location) && tiles[location.0][location.1] == Tile::WALL
						&& Labyrinth::neighborsOf(*location).into_iter().filter(|innerLocation| tiles[innerLocation.0][innerLocation.1] == Tile::WALL).count() >= 3
				).collect::<Vec<_>>();

			if locationCandidates.is_empty() {
				continue;
			}

			let location: (usize, usize) = *locationCandidates.choose(&mut rng).unwrap();
			tiles[location.0][location.1] = Tile::WALKABLE;
			branchCandidates.push(location);
			walkableTiles.push(location);

			if locationCandidates.len() > 1 {
				branchCandidates.push(branchCandidate);
			}
		}

		let exitLocation = *walkableTiles.choose(&mut rng).unwrap();
		tiles[exitLocation.0][exitLocation.1] = Tile::INVISIBLE_EXIT;

		let mut minotaurLocation = exitLocation;
		while Labyrinth::distanceBetweeen(tiles, exitLocation, minotaurLocation) <= MINIMUM_DISTANCE_BETWEEN_STARTING_POSITIONS {
			minotaurLocation = *walkableTiles.choose(&mut rng).unwrap();
		}
		tiles[minotaurLocation.0][minotaurLocation.1] = Tile::MINOTAUR;

		let mut heroLocation = exitLocation;
		while min(Labyrinth::distanceBetweeen(tiles, exitLocation, heroLocation), Labyrinth::distanceBetweeen(tiles, minotaurLocation, heroLocation)) <= MINIMUM_DISTANCE_BETWEEN_STARTING_POSITIONS {
			heroLocation = *walkableTiles.choose(&mut rng).unwrap();
		}
		tiles[heroLocation.0][heroLocation.1] = Tile::HERO;

		return Labyrinth { tiles, minotaurCoordinates: minotaurLocation, heroCoordinates: heroLocation, commonKnowledge: Vec::new() };
	}

	/**
	 * Returns whether the given coordinates are legal
	 * Legal coordinates are those that can access the tiles value of a Labyrinth struct
	 * Is similar to isInsideLabyrinthBounds, but this method also returns true for the edges
	 */
	fn isLegalCoordinate(location: (usize, usize)) -> bool {
		return location.0 < HEIGHT && location.1 < WIDTH
	}

	/**
	 * Returns the legal neighbor coordinates of a given location
	 * Assumes that overflow and underflow won't be an issue
	 */
	fn neighborsOf(location: (usize, usize)) -> Vec<(usize, usize)> {
		let movementDirections = [(1 as isize, 0), (0, 1), (-1, 0), (0, -1)];
		return movementDirections.iter()
			.map(|movementDirection| ((location.0 as isize + movementDirection.0) as usize, (location.1 as isize + movementDirection.1) as usize))
			.filter(|neighbor| Labyrinth::isLegalCoordinate(*neighbor)).collect();
	}

	/**
	 * Returns the neighbors of the given location that are 'walkable'
	 * Walkable tiles are all tiles that aren't Tile::WALL
	 * Walkable means the hero or minotaur can traverse over those tiles
	 */
	fn walkableNeighborsOf(tiles: [[Tile; WIDTH]; HEIGHT], location: (usize, usize)) -> Vec<(usize, usize)> {
		return Labyrinth::neighborsOf(location).into_iter().filter(|neighbor| tiles[neighbor.0][neighbor.1] != Tile::WALL).collect()
	}

	/**
	 * Returns whether the given coordinates are within the labyrinth bounds
	 * Notably excludes the very edges as those should always be WALLs
	 */
	fn isInsideLabyrinthBounds(location: (usize, usize)) -> bool {
		return location.0 > 0 && location.1 > 0 && location.0 < (HEIGHT as isize - 1) as usize && location.1 < (WIDTH as isize - 1) as usize
	}

	/**
	 * Gets the effective distance between two locations when only traversing over walkable tiles
	 * Uses a breadth-first search and assumes that there will always be a path from location0 to location1
	 */
	fn distanceBetweeen(tiles: [[Tile; WIDTH]; HEIGHT], location0: (usize, usize), location1: (usize, usize)) -> i32 {
		if location0 == location1 {
			return 0;
		}
		return MINIMUM_DISTANCE_BETWEEN_STARTING_POSITIONS + 1; //TODO: implement
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
