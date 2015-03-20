extern crate clap;

use clap::{App};

fn main() {
	let matches = App::new("cli2048")
						.about("A command-line implementation of the 2048 game")
						.version("0.0.1")
						.author("Kevin K. <kbknapp@gmail.com>")
						.get_matches();

}
