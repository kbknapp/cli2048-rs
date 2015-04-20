pub struct Display;

impl Display {
    fn new() -> Display {
        // Set up the display
        ncurses::initscr();
        ncruses::raw();

        // Allow extended keys
        ncurses::keypad(stdscr, true);
        ncurses::noecho();

        Display
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}