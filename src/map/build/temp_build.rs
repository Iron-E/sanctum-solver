use {super::Coordinate, crate::Container, std::collections::HashSet};

/// # Summary
///
/// A temporary [`Build`][build] holding some `blocks` and `temp_block` for purposes of
/// checking before insertion into the main [`Build`][build].
///
/// [build]: super::Build
pub(super) struct TempBuild<'blocks> {
	pub(super) blocks: &'blocks HashSet<Coordinate>,
	pub(super) temp_block: Coordinate,
}

impl Container<Coordinate> for TempBuild<'_> {
	fn contains(&self, some: &Coordinate) -> bool {
		self.blocks.contains(some) || self.temp_block == *some
	}
}
