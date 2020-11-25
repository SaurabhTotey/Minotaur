mod tile;
use tile::Tile;
use std::fmt::{Display, Formatter, Result};

const WIDTH: usize = 100;
const HEIGHT: usize = 20;

pub struct Labyrinth {
	tiles: [[Tile; WIDTH]; HEIGHT]
}
impl Labyrinth {

	/**
	 * Creates a new labyrinth
	 */
	pub fn new() -> Labyrinth {
		let labyrinth = Labyrinth { tiles: [[Tile::WALL; WIDTH]; HEIGHT] };
		//TODO: maze algo
		return labyrinth;
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
