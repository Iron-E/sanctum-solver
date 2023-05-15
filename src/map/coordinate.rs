use {
	super::Tile,
	crate::Container,
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// A __(__`x`__,__ `y`__)__ tuple which refers to coordinates in a two-dimensional array.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
	/// # Summary
	///
	/// Calculate the distance between this [`Coordinate`] and the `other`.
	///
	/// # Remarks
	///
	/// This does not take into account any barriers which may exist between the [`Coordinate`]s.
	pub fn distance_from(&self, other: &Self) -> usize {
		((self.0 as i128 - other.0 as i128).abs() + (self.1 as i128 - other.1 as i128).abs()) as usize
	}

	/// # Summary
	///
	/// Retrieve the `T` value stored at the [`Coordinate`] in array.
	pub fn get_from<T>(&self, grid: &[impl AsRef<[T]>]) -> Option<T>
	where
		T: Copy,
	{
		if let Some(inner) = grid.get(self.1) {
			if let Some(value) = inner.as_ref().get(self.0) {
				return Some(*value);
			}
		}

		None
	}

	/// # Summary
	///
	/// Get some [`Coordinate`] from a `grid`, but only if it is not present in the `build`.
	///
	/// # Returns
	///
	/// * If `build` is [`None`], [`Self::get_from`] `grid`.
	/// * If `build` is [`Some`], [`Tile::Block`].
	pub fn get_from_with_build(
		&self,
		grid: &[impl AsRef<[Tile]>],
		build: Option<&impl Container<Coordinate>>,
	) -> Option<Tile> {
		if let Some(b) = build {
			if b.contains(self) {
				return Some(Tile::Block);
			}
		}
		self.get_from(grid)
	}

	/// # Summary
	///
	/// Set the `T` value stored at the [`Coordinate`] in array.
	///
	/// # Panics
	///
	/// If `array[self.1][self.0]` is out of bounds.
	pub fn set<T>(&self, grid: &mut [impl AsMut<[T]>], value: T) {
		if let Some(inner) = grid.get_mut(self.1) {
			inner.as_mut()[self.0] = value
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Coordinate;

	const ARRAY: [[usize; 5]; 5] = [
		[1, 2, 3, 4, 5],
		[6, 7, 8, 9, 10],
		[11, 12, 13, 14, 15],
		[16, 17, 18, 19, 20],
		[21, 22, 23, 24, 25],
	];

	#[test]
	fn test_get_from() {
		assert_eq!(Coordinate(2, 2).get_from(&ARRAY), Some(13));
		assert_eq!(Coordinate(0, 1).get_from(&ARRAY), Some(6));
		assert_eq!(Coordinate(100, 1).get_from(&ARRAY), None);
	}
}
