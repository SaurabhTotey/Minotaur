pub enum Action {
	MOVE_LEFT,
	MOVE_RIGHT,
	MOVE_UP,
	MOVE_DOWN,
	PERFORM_ACTION
}
impl Into<[u8; 1]> for Action {
	fn into(self) -> [u8; 1] {
		return [match self {
			Action::MOVE_UP => 'w',
			Action::MOVE_LEFT => 'a',
			Action::MOVE_DOWN => 's',
			Action::MOVE_RIGHT => 'd',
			_ => 'r'
		} as u8; 1];
	}
}
impl From<[u8; 1]> for Action {
	fn from(character: [u8; 1]) -> Self {
		return match character[0] as char {
			'w' => Action::MOVE_UP,
			'a' => Action::MOVE_LEFT,
			's' => Action::MOVE_DOWN,
			'd' => Action::MOVE_RIGHT,
			_ => Action::PERFORM_ACTION
		};
	}
}
