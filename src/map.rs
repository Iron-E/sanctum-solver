mod adjacent;
mod build;
mod coordinate;
mod shortest_path;
mod tile;
pub mod tileset;

pub use adjacent::Adjacent;
pub use build::Build;
pub use coordinate::Coordinate;
use serde::{Deserialize, Serialize};
pub use shortest_path::ShortestPath;
pub use tile::Tile;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Map
{
	pub name: String,
	pub grid: Vec<Vec<Tile>>,
	pub shortest_path_length: Option<Vec<Option<usize>>>,
}
