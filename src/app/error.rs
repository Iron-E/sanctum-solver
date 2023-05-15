use {
	snafu::Snafu,
	std::{io, result::Result as StdResult},
};

#[derive(Debug, Snafu)]
pub enum Error {
	#[snafu(display("{}", err))]
	Io { err: io::Error },

	#[snafu(display("{}", err))]
	Json { err: serde_json::Error },
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Self::Io { err }
	}
}

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Self {
		Self::Json { err }
	}
}

pub type Result<T> = StdResult<T, Error>;
