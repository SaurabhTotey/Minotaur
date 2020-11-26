pub mod HeroManager;
pub mod MinotaurManager;

const PORT: i32 = 6669;

pub trait NetworkManager {

	fn new() -> Self;

}
