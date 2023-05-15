/// # Summary
///
/// A __(__`y`__,__ `x`__)__ tuple which refers to coordinates in a two-dimensional array.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate
{
	/// # Summary
	///
	/// Retrieve the `T` value stored at the [`Coordinate`] in array.
	pub fn get<T>(&self, array: &[&[T]]) -> Option<T> where T : Copy
	{
		if let Some(inner) = array.get(self.0)
		{
			if let Some(value) = inner.get(self.1)
			{
				return Some(*value);
			}
		}

		None
	}
}
