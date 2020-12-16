use crate::labyrinth::{WIDTH, HEIGHT};

pub const MINOTAUR_MESSAGE_SIZE: usize = 2 + WIDTH * HEIGHT;

#[derive(Copy, Clone)]
pub struct MinotaurMessage {
	pub isGameFinished: bool,
	pub isWinnerMinotaur: bool, // doesn't matter unless isGameFinished
	pub map: [[char; WIDTH]; HEIGHT]
}
impl MinotaurMessage {
	pub fn toPrintableString(&self) -> String {
		return self.map.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
	}
}
impl Into<[u8; MINOTAUR_MESSAGE_SIZE]> for MinotaurMessage {
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
impl From<[u8; MINOTAUR_MESSAGE_SIZE]> for MinotaurMessage {
	fn from(buffer: [u8; MINOTAUR_MESSAGE_SIZE]) -> Self {
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
