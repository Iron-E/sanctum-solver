use {super::Adjacent, std::array::IntoIter};

impl<T> IntoIterator for Adjacent<T> {
	type Item = Option<T>;
	type IntoIter = IntoIter<Option<T>, 4>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new([self.up, self.right, self.down, self.left])
	}
}
