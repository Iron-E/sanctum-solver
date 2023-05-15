use super::{Coordinate, Tile};

/// # Summary
///
/// Types which are adjacent to some other type.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Adjacent<T> {
	pub up: Option<T>,
	pub right: Option<T>,
	pub down: Option<T>,
	pub left: Option<T>,
}

impl<T> Adjacent<T> {
	/// # Summary
	///
	/// Run some `f`unction on each [`Some`] value.
	pub fn for_each(self, mut f: impl FnMut(T)) {
		if let Some(adjacent) = self.up {
			f(adjacent);
		}
		if let Some(adjacent) = self.right {
			f(adjacent);
		}
		if let Some(adjacent) = self.down {
			f(adjacent);
		}
		if let Some(adjacent) = self.left {
			f(adjacent);
		}
	}
}

impl Adjacent<Coordinate> {
	/// # Summary
	///
	/// Get the adjacent [`Coordinate`]s to a `coordinate` on an `array`.
	pub fn from_array_coordinate<T>(array: &[impl AsRef<[T]>], coord: &Coordinate) -> Self {
		Self {
			up: if coord.1 > 0 {
				Some(Coordinate(coord.0, coord.1 - 1))
			} else {
				None
			},

			right: if coord.0 < array[coord.1].as_ref().len() - 1 {
				Some(Coordinate(coord.0 + 1, coord.1))
			} else {
				None
			},

			down: if coord.1 < array.len() - 1 {
				Some(Coordinate(coord.0, coord.1 + 1))
			} else {
				None
			},

			left: if coord.0 > 0 {
				Some(Coordinate(coord.0 - 1, coord.1))
			} else {
				None
			},
		}
	}

	/// # Summary
	///
	/// Get each [`Tile`] from the `array` corresponding to the [`Adjacent`] [`Coordinate`]s.
	pub fn into_tiles(&self, array: &[impl AsRef<[Tile]>]) -> Adjacent<Tile> {
		Adjacent {
			up: self.up.and_then(|t| t.get_from(array)),
			right: self.right.and_then(|r| r.get_from(array)),
			down: self.down.and_then(|d| d.get_from(array)),
			left: self.left.and_then(|l| l.get_from(array)),
		}
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Adjacent, Coordinate},
		crate::map::{Tile, Tile::*},
	};

	#[rustfmt::skip]
	const ARRAY: [[Tile; 5]; 5] = [
		[Impass, Impass, Impass, Impass, Impass],
		[Spawn,  Pass,   Empty,  Core,   Impass],
		[Spawn,  Pass,   Empty,  Core,   Impass],
		[Spawn,  Pass,   Empty,  Core,   Impass],
		[Impass, Impass, Impass, Impass, Impass],
	];

	#[test]
	fn from_array_coordinate() {
		// Normal adjacency; no special cases
		assert_eq!(
			Adjacent::<Coordinate>::from_array_coordinate(&ARRAY, &Coordinate(2, 2)),
			Adjacent {
				up: Some(Coordinate(2, 1)),
				right: Some(Coordinate(3, 2)),
				down: Some(Coordinate(2, 3)),
				left: Some(Coordinate(1, 2)),
			}
		);

		// Nothing to the top.
		assert_eq!(
			Adjacent::<Coordinate>::from_array_coordinate(&ARRAY, &Coordinate(2, 0)),
			Adjacent {
				up: None,
				down: Some(Coordinate(2, 1)),
				right: Some(Coordinate(3, 0)),
				left: Some(Coordinate(1, 0)),
			}
		);

		// Nothing to the right.
		assert_eq!(
			Adjacent::<Coordinate>::from_array_coordinate(&ARRAY, &Coordinate(4, 3)),
			Adjacent {
				up: Some(Coordinate(4, 2)),
				right: None,
				down: Some(Coordinate(4, 4)),
				left: Some(Coordinate(3, 3)),
			}
		);

		// Nothing to the bottom.
		assert_eq!(
			Adjacent::<Coordinate>::from_array_coordinate(&ARRAY, &Coordinate(3, 4)),
			Adjacent {
				up: Some(Coordinate(3, 3)),
				right: Some(Coordinate(4, 4)),
				down: None,
				left: Some(Coordinate(2, 4)),
			}
		);

		// Nothing to the left.
		assert_eq!(
			Adjacent::<Coordinate>::from_array_coordinate(&ARRAY, &Coordinate(0, 2)),
			Adjacent {
				up: Some(Coordinate(0, 1)),
				right: Some(Coordinate(1, 2)),
				down: Some(Coordinate(0, 3)),
				left: None,
			}
		);
	}
}
