mod display;
mod board;
mod player;

struct GameEvents(Player)

impl GameEvents {
    fn with_player(p: &mut Player) -> GameEvents {
        GameEvents(p)
    }
}

impl Iterator for GameEvents {
    type Item = GameEvent;
    fn next(&mut self) -> Option<Item> {
        loop {
            return match ncurses::getch() {
                K_DOWN  | K_W_LC | K W_UC => Some(GameEvent::Move(Direction::Down)),
                K_UP    | K_S_LC | K_S_UC => Some(GameEvent::Move(Direction::Up)),
                K_LEFT  | K_A_LC | K_A_UC => Some(GameEvent::Move(Direction::Left)),
                K_RIGHT | K_D_LC | K_D_UC => Some(GameEvent::Move(Direction::Right)),
                K_ESC   | K_Q_UC | K_Q_LC => Some(GameEvent::Quit),
                K_H_UC | K_H_LC => Some(GameEvent::Help),
                K_N_UC | K_N_LC => None,
                K_M_UC | K_M_LC => continue,
                _               => continue,

            }
        }
    }
}

enum GameEvent {
    Move(Direction),
    Win,
    Lose,
    Quit,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn run() {
    let disp = Display::new();

    let mut quit = false;
    while !quit {
        let p = Player::new();
        let b = Board::with_player(&p);
        let g = GameEvents::with_player(&mut p);
        for_match!{ g,
            GameEvent::Move(dir) => { b.move(dir) },
            GameEvent::Win => {quit = handle_win(); break; },
            GameEvent::Lose => { quit = handle_lose(); break; },
            GameEvent::Quit => { quit = true; break; },
            GameEvent::Help => { handle_help(); }
        }
    }
}

fn handle_help() {
    ncurses::printw(MORE_HELP);
    ncurses::refresh();
}

fn handle_win() -> bool {
    false
}

fn handle_lose() -> bool {
    false
}