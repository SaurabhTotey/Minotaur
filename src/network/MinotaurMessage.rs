use crate::labyrinth::{WIDTH, HEIGHT};

pub struct MinotaurMessage {
	isGameFinished: bool,
	isWinnerMinotaur: bool, // doesn't matter unless isGameFinished
	map: [[char; WIDTH]; HEIGHT]
}
impl Into<[u8; 2 + WIDTH * HEIGHT]> for MinotaurMessage {
	fn into(self) -> [u8; 2 + WIDTH * HEIGHT] {
		let mut buffer = [0u8; 2 + WIDTH * HEIGHT];
		buffer[0] = self.isGameFinished.into();
		buffer[1] = self.isWinnerMinotaur.into();
		for r in 0 .. HEIGHT {
			for c in 0 .. WIDTH {
				buffer[2 + r * WIDTH + c] = self.map[r][c] as u8;
			}
		}
		return buffer;
	}
}
impl From<[u8; 2 + WIDTH * HEIGHT]> for MinotaurMessage {
	fn from(buffer: [u8; 2 + WIDTH * HEIGHT]) -> Self {
		let mut map = [[' '; WIDTH]; HEIGHT];
		for r in 0 .. HEIGHT {
			for c in 0 .. WIDTH {
				map[r][c] = buffer[2 + r * WIDTH + c] as char;
			}
		}
		return MinotaurMessage {
			isGameFinished: buffer[0] != 0,
			isWinnerMinotaur: buffer[1] != 0,
			map
		};
	}
}
