pub struct Player {
    score: u64,
    high_score: u64
}

impl Player {
    fn new() -> Player {
        Player {
            score: 0,
            high_score: 0
        }
    }
}