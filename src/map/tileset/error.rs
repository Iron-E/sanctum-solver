use {super::Tile, snafu::Snafu, std::result::Result as StdResult};

#[derive(Debug, Snafu)]
pub enum Error {
	#[snafu(display("Tried to pass through a non-passable tile {:?}.", tile))]
	CannotPass { tile: Tile },

	#[snafu(display("Tried to make a region out of non-region tile {:?}", tile))]
	NotRegion { tile: Tile },
}

pub type Result<T> = StdResult<T, Error>;
