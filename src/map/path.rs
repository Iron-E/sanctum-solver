mod error;

pub use error::{Error, Result};

use {
	super::{
		tileset::{Tileset, COORDINATE_ON_TILESET},
		Adjacent, Coordinate, Tile,
	},
	serde::{Deserialize, Serialize},
	std::collections::HashMap,
};

pub const PATH_HAS_COORDINATE: &str = "Expected path to have at least one coordinate.";

/// # Summary
///
/// A two-dimensional array / grid of [`Tile`]s.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Path(pub Vec<Coordinate>);

impl Path {
	/// # Summary
	///
	/// Get the shortest [`Path`] to a [`Tile`] of `needle`'s type from some `start`ing [`Coordinate`] on a `tileset`.
	fn from_tileset_coordinate(tileset: &Tileset, start: Coordinate, needle: Tile) -> Result<Self> {
		let start_tile = start.get_from(&tileset.0).expect(COORDINATE_ON_TILESET);

		// We don't want to start the search on a tile which cannot be walked over.
		// This is to prevent accidentally crossing over the other side of a barrier.
		if !start_tile.is_passable() {
			return Err(Error::CannotPass { tile: start_tile });
		}

		let mut coordinate_distance_queue = vec![(start, vec![start])];
		let mut visited = HashMap::<Coordinate, Vec<Coordinate>>::new();

		while let Some((coord, current_path)) = coordinate_distance_queue.pop() {
			// If the current path is longer than the previous path (defaulting to `false` if there
			// is no previous path).
			if match visited.get(&coord) {
				Some(visited_path) => current_path.len() >= visited_path.len(),
				_ => false,
			} {
				continue;
			}

			let tile = coord.get_from(&tileset.0).expect(COORDINATE_ON_TILESET);

			// Only keep looking beyond a passable tile, and if the current tile is not what we're
			// searching for.
			if tile.is_passable() && tile != needle {
				Adjacent::<Coordinate>::from_array_coordinate(&tileset.0, &coord).for_each(
					|adjacent| {
						let mut new_path = current_path.clone();
						new_path.push(adjacent);

						coordinate_distance_queue.push((adjacent, new_path))
					},
				);
			}

			// Now that the current coordinate has been fully evaluated, mark it as visited.
			visited.insert(coord, current_path);
		}

		Ok(visited
			.into_iter()
			.filter(|(coord, _)| {
				// Only want Tiles of `needle`'s type
				coord.get_from(&tileset.0).expect(COORDINATE_ON_TILESET) == needle
			})
			.map(|(_, path)| Path(path))
			.reduce(Path::return_shorter)
			.expect(PATH_HAS_COORDINATE))
	}

	/// # Summary
	///
	/// Returns the shorter [`Path`].
	///
	/// # Remarks
	///
	/// If paths are equally long, the current path is preferred.
	fn return_shorter(self, other: Self) -> Self {
		if self.0.len() > other.0.len() {
			return other;
		}

		self
	}
}

impl From<&Tileset> for Result<Vec<Path>> {
	fn from(tileset: &Tileset) -> Self {
		let entrance_regions = tileset.entrances();
		let number_of_regions = entrance_regions.len();

		Ok(entrance_regions
			.into_iter()
			.try_fold(
				// Iterate over every region, and for each region, iterate over its entrances. For each
				// entrance, get their path to the nearest core.
				Vec::with_capacity(number_of_regions),
				|mut paths_by_region, region| {
					let number_of_entrances = region.len();

					paths_by_region.push(region.into_iter().try_fold(
						Vec::with_capacity(number_of_entrances),
						|mut paths_by_entrance, entrance| {
							paths_by_entrance.push(Path::from_tileset_coordinate(
								tileset,
								entrance,
								Tile::Core,
							)?);

							Ok(paths_by_entrance)
						},
					)?);

					Ok(paths_by_region)
				},
			)?
			// Iterate over the `paths_by_region`, and for each region, select only the shortest path.
			// This turns our Vec<Vec<Path>> (where outer Vec is region and inner Vec is entrance) into a Vec<Path> (where the vec is region and we only have the shortest path for each).
			.into_iter()
			.map(|region| {
				region
					.into_iter()
					.reduce(Path::return_shorter)
					.expect(PATH_HAS_COORDINATE)
			})
			.collect())
	}
}

#[cfg(test)]
mod tests {
	use {
		super::{Coordinate, Path, Result, Tile, Tileset},
		crate::map::tileset::tests::PARK_TWO_SPAWN,
		std::time::Instant,
	};

	#[test]
	fn from() {
		let test_tileset = Tileset(
			PARK_TWO_SPAWN
				.iter()
				.map(|row| row.iter().copied().collect())
				.collect(),
		);

		let start = Instant::now();
		let test_paths = Result::<Vec<Path>>::from(&test_tileset).unwrap();
		println!(
			"Result::<Vec<Path>>::from {}us",
			Instant::now().duration_since(start).as_micros()
		);

		// There should be two paths to the core since there are two spawn points.
		assert_eq!(test_paths.len(), 2);
	}
}
