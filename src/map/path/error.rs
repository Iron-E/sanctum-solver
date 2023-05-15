use {
	super::{Coordinate, Tile},
	snafu::Snafu,
	std::result::Result as StdResult,
};

#[derive(Debug, Snafu)]
pub enum Error {
	#[snafu(display("Tried to pass through a non-passable tile {:?}.", tile))]
	CannotPass { tile: Tile },

	#[snafu(display(
		"Tried to create a path between {:?} to some {:?}, and none was found.",
		start,
		end
	))]
	NoPath { start: Coordinate, end: Tile },
}

pub type Result<T> = StdResult<T, Error>;
