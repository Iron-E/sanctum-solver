use std::result::Result as StdResult;

use snafu::Snafu;

use super::Tile;

#[derive(Debug, Snafu)]
pub enum Error
{
	#[snafu(display("Tried to make a region out of non-region tile {:?}", tile))]
	NotRegion
	{
		tile: Tile
	},
}

pub type Result<T> = StdResult<T, Error>;
