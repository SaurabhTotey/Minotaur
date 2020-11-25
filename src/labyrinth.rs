mod tile;
use tile::Tile;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

pub struct Labyrinth {
	tiles: [[Tile; WIDTH]; HEIGHT]
}
impl Labyrinth {

	/**
	 * Creates a new labyrinth
	 */
	pub fn new() -> Labyrinth {
		let labyrinth = Labyrinth { tiles: [[Tile::WALL; WIDTH]; HEIGHT] };
		return labyrinth;
	}

}
