/// # Summary
///
/// A __(__`x`__,__ `y`__)__ tuple which refers to coordinates in a two-dimensional array.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
}

#[cfg(test)]
mod tests {
	use super::Coordinate;

	#[test]
	fn test_get_from() {
		let array = vec![
			vec![1, 2, 3, 4, 5],
			vec![6, 7, 8, 9, 10],
			vec![11, 12, 13, 14, 15],
			vec![16, 17, 18, 19, 20],
			vec![21, 22, 23, 24, 25],
		];

		assert_eq!(Coordinate(2,2).get_from(&array), Some(13));
		assert_eq!(Coordinate(0,1).get_from(&array), Some(6));
		assert_eq!(Coordinate(100,1).get_from(&array), None);
	}
}
