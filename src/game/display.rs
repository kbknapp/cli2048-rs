use ncurses;
use game::player::Player;
use game::board::Board;

pub struct Display;

impl Display {
    pub fn new() -> Display {
        // Set up the display
        ncurses::initscr();
        ncurses::raw();

        // Allow extended keys
        ncurses::keypad(ncurses::stdscr, true);
        ncurses::noecho();
        // ncurses::attroff(ncurses::A_BLINK());
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        Display
    }

    pub fn draw(&self, p: Player, b: &Board) {
        ncurses::clear();
        ncurses::printw(format!("score: {}\nhigh score: {}\n", p.score, p.high_score).as_ref());
        ncurses::printw(b.draw().as_ref());
    }

    pub fn refresh(&self) {
        ncurses::refresh();
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}