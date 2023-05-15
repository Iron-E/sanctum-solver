mod app;
mod map;

use {app::App, structopt::StructOpt};

fn main() {
	App::from_args().run().unwrap();
}
