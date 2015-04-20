use clap::App;

const MORE_HELP: &'static str = "Controls:
    UP      w, [UP ARROW]
    DOWN    s, [DOWN ARROW]
    LEFT    a, [LEFT ARROW]
    RIGHT   d, [RIGHT ARROW]

    HELP    h
    NEW GAME    r, n

    QUIT    q, [ESC]";

fn main() {
	// Handle basic arguments and free the memory since we aren't using the
    // matches...yet
    {
        App::new("cli2048")
				.about("A command-line implementation of the 2048 game")
				.version(&format!("v{}", crate_version!())[..])
				.author("Kevin K. <kbknapp@gmail.com>")
                .more_help(more_help)
				.get_matches();
    }

    game::run();

}
