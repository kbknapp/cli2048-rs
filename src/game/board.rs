struct Board<'r> {
    board: [u16; 16],
    rows: [[u8; 4]; 4],
    cols: [[u8; 4]; 4],
    rng: Generator<'r, u16>,
    c_score: u64,
    h_score: u64
}

impl<'r> Board<'r> {
    fn with_player(p: &Player) -> Board<'r> {
        let b = Board {
            board: [0; 16],
            rows: [[0; 4]; 4],
            cols: [[0; 4]; 4],
            c_score: 0,
            rng: u16::rand()
            h_score: p.high_score
        };

        let mut r = 0;
        let mut c = 0;
        for i in (0..4) {
            for j in (0..4) {
                b.rows[i][j] = r;
                r += 1;
                b.cols[i][j] = (j * 4) + c;
            }
            c += 1;
        }

        b.new_cell();
        b.new_cell();
        b
 
    }

    fn new_cell(&mut self) -> bool {
        if self.is_full() { return true }

        rand.Seed(time.Now().Unix())
    for {
        i = rand.Intn(len(gb.M))
        if gb.M[i] == 0 {
            break
        }
    }
    num := 2
    for {
        num = rand.Intn(5)
        if num%2 == 0 && num != 0 {
            if num == 4 {
                w := rand.Intn(4)
                if (w % 2) != 0 {
                    break
                }
            } else {
                break
            }
        }
    }

    gb.M[i] = num

    return gb.movesLeft()

    }

    fn move(&mut self, d: Direction) {
        match d {
            Direction::Up    => self.move_up(),
            Direction::Down  => self.move_down(),
            Direction::Left  => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn move_up(&mut self) {

    }

    fn move_down(&mut self) {
        
    }

    fn move_left(&mut self) {
        
    }

    fn move_right(&mut self) {
        
    }
}