mod error;

use {
	crate::map::{tileset::Tileset, Build, Map, ShortestPath},
	error::Result,
	std::{collections::HashSet, fs, path::PathBuf},
	structopt::StructOpt,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(
	name = "sanctum_solver",
	about = "A tool to find the most optimal layout for a Sanctum map"
)]
pub struct App {
	#[structopt(help = "The maximum number of blocks to place", long, short)]
	blocks: Option<usize>,

	#[structopt(help = "A JSON file containing the map layout")]
	map_json: PathBuf,

	#[structopt(
		help = "Where to save the output. If not specified, goes to `stdout`",
		long,
		short
	)]
	output: Option<PathBuf>,

	#[structopt(
		help = "Prioritize placing blocks for spawn regions with shorter paths to the core. Default: false",
		long,
		short
	)]
	prioritize: bool,
}

impl App {
	/// # Summary
	///
	/// Run the application and parse its provided arguments / flags.
	pub fn run(self) -> Result<()> {
		let mut map: Map = serde_json::from_slice(&fs::read(self.map_json)?)?;
		let mut tileset = Tileset::new(map.grid);

		let build = if self.prioritize {
			Build::from_entrances_to_any_core_with_priority(&tileset, self.blocks)
		} else {
			Build::from_entrances_to_any_core(&tileset, self.blocks)
		};
		build.apply_to(&mut tileset.grid);

		map.grid = tileset.grid;
		map.shortest_path_length = Some(
			ShortestPath::from_entrances_to_any_core(
				&Tileset::new(map.grid.clone()),
				Option::<&HashSet<_>>::None,
			)
			.into_iter()
			.map(|path| path.map(|p| p.len()))
			.collect(),
		);

		let map_json = serde_json::to_string_pretty(&map)?;
		if let Some(output) = self.output {
			fs::write(output, map_json)?;
		} else {
			println!("{}", map_json);
		}

		Ok(())
	}
}
