#![feature(rand)]

extern crate rand;
extern crate time;
extern crate ncurses;

#[macro_use]
extern crate clap;

#[macro_use]
mod macros;
mod game;
mod constants;

use clap::App;
use constants::MORE_HELP;

fn main() {
	// Handle basic arguments and free the memory since we aren't using the
    // matches...yet
    {
        App::new("cli2048")
				.about("A command-line implementation of the 2048 game")
				.version(&format!("v{}", crate_version!())[..])
				.author("Kevin K. <kbknapp@gmail.com>")
                .after_help(MORE_HELP)
				.get_matches();
    }

    game::run();

}
