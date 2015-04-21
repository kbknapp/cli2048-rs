use rand::{ChaChaRng, Rng};
use rand::SeedableRng;

use time;

use game::events::Direction;

const ROWS: [[u8; 4]; 4] = [[0, 1, 2, 3], 
                            [4, 5, 6, 7], 
                            [8, 9, 10, 11], 
                            [12, 13, 14, 15]];
const COLS: [[u8; 4]; 4] = [[0, 4, 8, 12],
                            [1, 5, 9, 13],
                            [2, 6, 10, 14],
                            [3, 7, 11, 15]];
#[derive(Copy, Clone)]
pub struct Board {
    board: [u16; 16],
    rng: ChaChaRng,
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            board: [0; 16],
            // rows: [[0; 4]; 4],
            // cols: [[0; 4]; 4],
            rng: ChaChaRng::from_seed(&[time::precise_time_s() as u32]),
        };

        // let mut r: u8 = 0;
        // let mut c: usize = 0;
        // for i in (0..4) {
        //     for j in (0..4) {
        //         b.rows[i][j] = r;
        //         r += 1;
        //         b.cols[i][j] = ((j * 4) + c) as u8;
        //     }
        //     c += 1;
        // }

        b.new_cell();
        b.new_cell();
        b
 
    }

    pub fn new_cell(&mut self) -> bool {
        if self.is_full() { return self.moves_remain() }
        let mut i;
        loop {
            i = (self.rng.next_u32() % 16) as usize;
            if self.board[i] == 0 {
                break;
            }

        }
        let mut num;
        loop {
            num = self.rng.next_u32() % 5;
            if num % 2 == 0 && num != 0 {
                if num == 4 {
                    let w = self.rng.next_u32() % 4;
                    if (w % 2) != 0 {
                        break
                    }
                } else {
                    break
                }
            } 
        }

        self.board[i] = num as u16;

        return self.moves_remain()

    }

    fn moves_remain(&self) -> bool {
        let mut a;
        let mut b = self.board[1];
        for i in (0..self.board.len()) {
            if (i+1)% 4 == 0 {
                b = self.board[i+2];
                continue
            }
            a = self.board[i];
            if a == b {
                return true
            }
            if i >= self.board.len()-2 {
                break
            }
            b = self.board[i+2];
        }
        b = self.board[self.board.len()-1];
        for (i, col) in COLS.iter().enumerate() {
            for (j, cell) in col.iter().enumerate() {
                a = self.board[*cell as usize];
                if a == b {
                    return true
                }
                if j >= 2 {
                    if i < self.board.len()-1 {
                        b = self.board[COLS[i+1][1] as usize];
                    }
                    break
                }
                b = self.board[col[j+2] as usize];
            }
        }
        if self.is_full() {
            return false
        } 
        true
    }

    fn is_full(&self) -> bool {
        for i in (0..self.board.len()) {
            if self.board[i] == 0 {
                return false
            }
        }
        true
    }

    pub fn move_cells(&mut self, d: Direction) -> (u64, u64) {
        match d {
            Direction::Up    => {
                self.shift_cells(ROWS)
            },
            Direction::Down  => {
                self.shift_cells_rev(ROWS)
            },
            Direction::Left  => {
                self.shift_cells(COLS)
            },
            Direction::Right => {
                self.shift_cells_rev(COLS)
            },
        }
    }

    fn shift_cells_rev(&mut self, indices: [[u8; 4]; 4]) -> (u64, u64) {
        let mut done = false;
        let mut moves = 0;
        let mut p: u64 = 0;
        let mut off_limits = [false; 16]; 
        loop {
            for i in (4..0) {
                for j in (4..0) {
                    if i == 3 {
                        continue
                    }

                    let curr_num = self.board[indices[i][j] as usize];

                    if curr_num == 0 {
                        continue
                    }

                    let mut new_idx: usize = 0;
                    let mut pos_idx = None;
                    for ni in (i+1..4) {
                        new_idx = indices[ni][j] as usize;
                        if self.board[new_idx] == 0 {
                            pos_idx = Some(new_idx);
                            continue
                        } else {
                            break
                        }
                    }

                    if self.board[new_idx] == 0 {
                        self.board[new_idx] = curr_num;
                        self.board[indices[i][j] as usize] = 0;
                        done = false;
                        moves += 1;
                    } else if curr_num == self.board[new_idx] {
                        if !off_limits[indices[i][j] as usize] {
                            self.board[new_idx] = curr_num * 2;
                            p += self.board[new_idx] as u64;
                            off_limits[new_idx] = true;
                            self.board[indices[i][j] as usize] = 0;
                            done = false;
                            moves += 1;
                        }

                    } else {
                        if pos_idx.is_none() {
                            continue
                        }
                        self.board[pos_idx.unwrap() as usize] = curr_num;
                        self.board[indices[i][j] as usize] = 0;
                        done = false;
                        moves += 1;
                    }
                }
            }
            if done {
                break
            }
            done = true;
        }
        return (p, moves) 
    }

    fn shift_cells(&mut self, indices: [[u8; 4]; 4]) -> (u64, u64) {
        let mut done = false;
        let mut moves = 0;
        let mut p: u64 = 0;
        let mut off_limits = [false; 16];
        loop {
            for (i, seq) in indices.iter().enumerate() {
                for (j, cell) in seq.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    let curr_num = self.board[*cell as usize];

                    if curr_num == 0 {
                        continue
                    }

                    let mut new_idx: usize = 0;
                    let mut pos_idx = None; 
                    for ni in (4..0) {
                        new_idx = indices[ni][j] as usize;
                        if self.board[new_idx] == 0 {
                            pos_idx = Some(new_idx);
                            continue
                        } else {
                            break
                        }
                    }

                    if self.board[new_idx] == 0 {
                        self.board[new_idx] = curr_num;
                        self.board[*cell as usize] = 0;
                        done = false;
                        moves += 1;
                    } else if curr_num == self.board[new_idx as usize] {
                        if !off_limits[*cell as usize] {
                            self.board[new_idx] = curr_num * 2;
                            p += self.board[new_idx] as u64;
                            off_limits[new_idx] = true;
                            self.board[*cell as usize] = 0;
                            done = false;
                            moves += 1;
                        }

                    } else {
                        if pos_idx.is_none() {
                            continue
                        }
                        self.board[pos_idx.unwrap()] = curr_num;
                        self.board[*cell as usize] = 0;
                        done = false;
                        moves += 1;
                    }
                }
            }
            if done {
                break
            }
            done = true;
        }
        return (p, moves)
    }

    pub fn draw(&self) -> String {
        let mut b = String::with_capacity(130);
        for r in (0..4) {
            b.push_str("____________________\n");
            b.push_str(&format!("|{}|{}|{}|{}|\n", self.format_cell(self.board[ROWS[r][0] as usize])
                                               , self.format_cell(self.board[ROWS[r][1] as usize]) 
                                               , self.format_cell(self.board[ROWS[r][2] as usize]) 
                                               , self.format_cell(self.board[ROWS[r][3] as usize]))[..]);
        }
        b.push_str("---------------------\n");
        b.push_str("h=HELP, [esc] | q=QUIT\n");
        b.shrink_to_fit();
        b
    }

    fn format_cell(&self, c: u16) -> String {
        match c {
            n if n == 0 || n < 10      => format!("   {}", n),
            n if n > 9 && n < 100     => format!("  {}", n),
            n if n > 99 && n < 1000   => format!(" {}", n),
            n if n > 999 && n < 10000 => format!("{}", n),
            _                         => format!(" err")
        }
    }
}