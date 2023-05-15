use {super::Coordinate, crate::Container};

/// # Summary
///
/// A temporary [`Build`][build] holding some `blocks` and `temp_block` for purposes of
/// checking before insertion into the main [`Build`][build].
///
/// [build]: super::Build
pub(super) struct TempBuild<'blocks, C>
where
	C: Container<Coordinate>,
{
	pub(super) blocks: &'blocks C,
	pub(super) temp_block: Coordinate,
}

impl<C> Container<Coordinate> for TempBuild<'_, C>
where
	C: Container<Coordinate>,
{
	fn contains(&self, some: &Coordinate) -> bool {
		self.blocks.contains(some) || self.temp_block == *some
	}
}
