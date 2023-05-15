use serde::{Deserialize, Serialize};

/// # Summary
///
/// A square on a [`Map`](super::Map).
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Tile {
	/// # Summary
	///
	/// An [`Impass`](Self::Impass) space which used to be [`Empty`](Self::Empty)
	/// but now has a block on it.
	Block,

	/// # Summary
	///
	/// An [`Impass`](super::Impass) which serves as an __exit point__ for enemies.
	Core,

	/// # Summary
	///
	/// A [`Pass`](Self::Pass) which may have blocks placed on top of it.
	Empty,

	/// # Summary
	///
	/// Opposite of [`Pass`](Self::Pass).
	Impass,

	/// # Summary
	///
	/// A [`Tile`] which the player can walk over.
	Pass,

	/// # Summary
	///
	/// An [`Impass`](Self::Impass) where enemies may come from. Serves as a __starting point__.
	Spawn,
}

impl Tile {
	/// # Summary
	///
	/// Whether or not some [`Tile`] can be moved through.
	pub fn is_passable(&self) -> bool {
		matches!(self, Tile::Empty | Tile::Pass)
	}

	/// # Summary
	///
	/// Whether or not the [`Tile`] is region of space, wherein all tiles adjacent to each other of
	/// similar type count as one bigger tile.
	pub fn is_region(&self) -> bool {
		matches!(self, Tile::Core | Tile::Spawn)
	}
}
