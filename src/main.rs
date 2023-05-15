mod app;
mod map;

use
{
	app::App,
	map::Map,
	structopt::StructOpt,
};

fn main() {
	App::from_args().run().unwrap();
}
