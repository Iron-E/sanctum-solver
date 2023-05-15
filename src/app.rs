mod error;

use
{
	error::Result,
	crate::Map,
	std::{fs, io, path::PathBuf},
	structopt::StructOpt,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="sanctum_solver", about="A tool to find the most optimal layout for a Sanctum map.")]
pub struct App
{
	#[structopt(help="A JSON file containing the map layout.")]
	map_json: PathBuf,
}

impl App
{
	/// # Summary
	///
	/// Run the application and parse its provided arguments / flags.
	pub fn run(self) -> Result<()>
	{
		let map: Map = serde_json::from_slice(&fs::read(self.map_json)?)?;

		todo!()
	}
}

