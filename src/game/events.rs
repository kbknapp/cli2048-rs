use ncurses;
use game::player::Player;
use constants::*;

pub struct GameEvents(Player);

impl GameEvents {
    pub fn with_player(p: Player) -> GameEvents {
        GameEvents(p)
    }
}

impl Iterator for GameEvents {
    type Item = GameEvent;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match ncurses::getch() {
                K_DOWN  => return Some(GameEvent::Move(Direction::Down)), 
                K_W_LC  => return Some(GameEvent::Move(Direction::Down)),
                K_W_UC  => return Some(GameEvent::Move(Direction::Down)),
                K_UP    => return Some(GameEvent::Move(Direction::Up)),
                K_S_LC  => return Some(GameEvent::Move(Direction::Up)),
                K_S_UC  => return Some(GameEvent::Move(Direction::Up)),
                K_LEFT  => return Some(GameEvent::Move(Direction::Left)),
                K_A_LC  => return Some(GameEvent::Move(Direction::Left)),
                K_A_UC  => return Some(GameEvent::Move(Direction::Left)),
                K_RIGHT => return Some(GameEvent::Move(Direction::Right)),
                K_D_LC  => return Some(GameEvent::Move(Direction::Right)),
                K_D_UC  => return Some(GameEvent::Move(Direction::Right)),
                K_ESC   => return Some(GameEvent::Quit),
                K_Q_UC  => return Some(GameEvent::Quit),
                K_Q_LC  => return Some(GameEvent::Quit),
                K_H_UC  => return Some(GameEvent::Help),
                K_H_LC  => return Some(GameEvent::Help),
                K_N_UC  => return None,
                K_N_LC  => return None,
                K_M_UC  => continue,
                K_M_LC  => continue,
                _       => continue,

            }
        }
    }
}

pub enum GameEvent {
    Move(Direction),
    Win,
    Lose,
    Quit,
    Menu,
    Help
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
