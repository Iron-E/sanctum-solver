mod app;
mod container;
mod map;

use {app::App, container::Container, structopt::StructOpt};

fn main() {
	App::from_args().run().unwrap();
}
