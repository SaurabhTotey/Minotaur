mod tile;
use tile::Tile;
use std::fmt::{Display, Formatter, Result};
extern crate rand;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

const WIDTH: usize = 100;
const HEIGHT: usize = 20;

pub struct Labyrinth {
	tiles: [[Tile; WIDTH]; HEIGHT],
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
			let mut locationCandidates = [(1 as isize, 0), (0, 1), (-1, 0), (0, -1)].iter().map(|movementDirection| {
				(branchCandidate.0 as isize + movementDirection.0, branchCandidate.1 as isize + movementDirection.1)
			}).filter(|location| {
				location.0 > 0 && location.1 > 0 && location.0 < HEIGHT as isize - 1 && location.1 < WIDTH as isize - 1
			}).collect::<Vec<_>>();
			locationCandidates.shuffle(&mut rng);
		}

		return Labyrinth { tiles };
	}

}
impl Display for Labyrinth {
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
