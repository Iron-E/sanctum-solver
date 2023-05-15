use serde::{Deserialize, Serialize};

/// # Summary
///
/// A __(__`x`__,__ `y`__)__ tuple which refers to coordinates in a two-dimensional array.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
	/// # Summary
	///
	/// Retrieve the `T` value stored at the [`Coordinate`] in array.
	pub fn get_from<T>(&self, array: &[impl AsRef<[T]>]) -> Option<T>
	where
		T: Copy,
	{
		if let Some(inner) = array.get(self.1) {
			if let Some(value) = inner.as_ref().get(self.0) {
				return Some(*value);
			}
		}

		None
	}

	/// # Summary
	///
	/// Set the `T` value stored at the [`Coordinate`] in array.
	///
	/// # Panics
	///
	/// If `array[self.1][self.0]` is out of bounds.
	pub fn set<T>(&self, array: &mut [impl AsMut<[T]>], value: T) {
		if let Some(inner) = array.get_mut(self.1) {
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
