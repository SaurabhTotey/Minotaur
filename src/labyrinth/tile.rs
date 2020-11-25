#[derive(Copy, Clone)]
pub enum Tile {
	WALKABLE, // can be walked on
	TORCH, // can be walked on and reveals the surrounding area
	VISIBLE_NON_EXIT, // can be walked on and is guaranteed to not be an exit
	WALL, // can not be walked on
	INVISIBLE_EXIT, // an exit that hasn't been revealed
	VISIBLE_EXIT, // an exit that both players can see
	HERO, // the hero player
	MINOTAUR // the minotaur player
}
impl Tile {

	/**
	 * Associates each tile with an ASCII character to visually represent it
	 */
	pub fn representation(&self) -> char {
		match *self {
			Tile::WALKABLE => ' ',
			Tile::TORCH => 'T',
			Tile::VISIBLE_NON_EXIT => 'N',
			Tile::WALL => '@',
			Tile::INVISIBLE_EXIT => ' ',
			Tile::VISIBLE_EXIT => 'E',
			Tile::HERO => 'H',
			Tile::MINOTAUR => 'M'
		}
	}

}
