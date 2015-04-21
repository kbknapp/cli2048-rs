mod display;
mod board;
mod player;
mod events;

use game::player::Player;
use game::display::Display;
use game::board::Board;
use game::events::{GameEvent, GameEvents};

use constants::*;
use ncurses;


pub fn run() {
    let disp = Display::new();

    let mut quit = false;
    while !quit {
        let p = Player::new();
        let mut b = Board::new();
        let g = GameEvents::with_player(p);
        disp.draw(p, &b);
        disp.refresh();
        for_match!{ g,
            GameEvent::Move(dir) => { 
                let (score, moves) = b.move_cells(dir); 
                if moves < 1 {
                    continue
                }
                if !b.new_cell() {
                    handle_lose();
                } 
                p.inc_score(score);
                disp.draw(p, &b);
                disp.refresh();
            },
            GameEvent::Win  => { quit = handle_win(); break; },
            GameEvent::Lose => { quit = handle_lose(); break; },
            GameEvent::Quit => { quit = true; break; },
            GameEvent::Help => { handle_help(); },
            GameEvent::Menu => { handle_menu(); }
        }
    }
}

fn handle_help() {
    ncurses::printw(MORE_HELP);
    ncurses::refresh();
}

fn handle_menu() {
}

fn handle_win() -> bool {
    false
}

fn handle_lose() -> bool {
    false
}