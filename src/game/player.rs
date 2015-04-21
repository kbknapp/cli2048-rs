#[derive(Copy, Clone)]
pub struct Player {
    pub score: u64,
    pub high_score: u64
}

impl Player {
    pub fn new() -> Player {
        Player {
            score: 0,
            high_score: 0
        }
    }

    pub fn inc_score(&mut self, s: u64) {
        self.score += s;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
}