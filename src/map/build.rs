use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Coordinate, ShortestPath, Tile,
	},
	std::collections::HashSet,
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// A set of blocks for a [`Tileset`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Build {
	pub blocks: HashSet<Coordinate>,
}

impl Build {
	/// # Summary
	///
	/// Apply all of the `blocks` from the [`Build`] to a `tileset`.
	pub fn apply(&self, tileset: &mut Tileset) {
		self.blocks.iter().for_each(|coordinate| {
			coordinate.set(&mut tileset.grid, Tile::Block);
		})
	}

	/// # Summary
	///
	/// Get the longest build for a specific `tileset`.
	pub fn from_entrances_to_any_exit(tileset: &Tileset, max_blocks: Option<usize>) -> Self {
		let mut build = Build {
			blocks: HashSet::new(),
		};

		let mut current_entrance = 0;
		let mut switch_entrance = || {
			if current_entrance < tileset.entrances.len() - 1 {
				current_entrance += 1;
			} else {
				current_entrance = 0;
			}
			current_entrance
		};

		while match max_blocks {
			Some(max) => max > build.blocks.len(),
			_ => true,
		} {
			for coord in ShortestPath::from_any_to(
				&tileset,
				Some(&build),
				tileset.entrances[switch_entrance()].iter(),
				&tileset.exits,
			)
			.expect("Expected build to be valid")
			.coordinates()
			.into_iter()
			.rev()
			.filter(|coord| {
				coord.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET) == Tile::Empty
			}) {
				// TODO: check to see if any blocks can be removed from the build without removing any
				//       distance from the path.

				// TODO: check if no more blocks can be placed (i.e. the build is optimal) and exit
				//       early.

				if build.blocks.contains(&coord) { continue; }

				build.blocks.insert(coord);

				if build.is_valid_for(&tileset) {
					break;
				}

				build.blocks.remove(&coord);
			}
		}

		build
	}

	/// # Summary
	///
	/// Return whether or not the current [`Build`] prevents any entrance from reaching a core.
	fn is_valid_for(&self, tileset: &Tileset) -> bool {
		self.blocks
			.iter()
			.all(|coord| coord.get_from(&tileset.grid).expect(COORDINATE_ON_TILESET) == Tile::Empty)
			&& tileset.entrances.iter().all(|region| {
				region.iter().any(|entrance| {
					ShortestPath::from_coordinate_to(
						&tileset,
						Some(self),
						*entrance,
						&tileset.exits,
					)
					.is_some()
				})
			})
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Build, Coordinate, HashSet, Tileset},
		crate::map::tileset::tests::PARK_TWO_SPAWN,
		std::time::Instant,
	};

	#[test]
	fn from_entrances_to_any_exit() {
		let test_tileset = Tileset::new(
			PARK_TWO_SPAWN
				.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let build = Build::from_entrances_to_any_exit(&test_tileset, Some(10));
		println!(
			"Build::from_entrances_to_any_exit {}us",
			Instant::now().duration_since(start).as_micros()
		);

		// TODO: write assertions
	}

	#[test]
	fn is_valid() {
		let test_tileset = Tileset::new(
			PARK_TWO_SPAWN
				.iter()
				.map(|inner| inner.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();

		// Empty build should be valid for a valid Tileset.
		assert!(Build {
			blocks: HashSet::new()
		}
		.is_valid_for(&test_tileset));

		// Valid build for valid tileset.
		assert!(Build {
			blocks: [Coordinate(4, 1)].iter().copied().collect()
		}
		.is_valid_for(&test_tileset));

		// Invalid build for valid tileset (because of no path)
		assert!(!Build {
			blocks: [
				Coordinate(4, 1),
				Coordinate(5, 2),
				Coordinate(5, 3),
				Coordinate(5, 4),
				Coordinate(5, 5),
				Coordinate(4, 6),
			]
			.iter()
			.copied()
			.collect()
		}
		.is_valid_for(&test_tileset));

		// Invalid build for valid tileset (because of invalid block placement).
		assert!(!Build {
			blocks: [Coordinate(0, 1)].iter().copied().collect()
		}
		.is_valid_for(&test_tileset));

		println!(
			"Build::is_valid_for {}us",
			Instant::now().duration_since(start).as_micros() / 4
		);
	}
}
