mod app;
mod container;
mod map;

use app::App;
use container::Container;
use structopt::StructOpt;

fn main()
{
	App::from_args().run().unwrap();
}
