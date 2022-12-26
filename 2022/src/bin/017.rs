use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs;
use vecmath::{vec2_add, Vector2};

type Int = i64;
type Coord = Vector2<Int>;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Move {
    Left,
    Right,
}
use Move::{Left, Right};

struct Moves {
    q: VecDeque<Move>,
    n: usize,
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        Moves {
            q: s.trim().chars().map(Move::from).collect::<VecDeque<Move>>(),
            n: 0,
        }
    }
}

impl Moves {
    fn next(&mut self) -> Move {
        self.n = (self.n + 1) % self.q.len();
        let m = self.q.pop_front();
        match m {
            Some(m) => {
                self.q.push_back(m);
                m
            }
            None => panic!("we should not ever consume moves!"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Piece {
    Bar,
    Cross,
    Ell,
    Stick,
    Square,
}

#[derive(Debug)]
struct Sprite {
    w: Int,
    blocks: Vec<Coord>,
}
use Piece::{Bar, Cross, Ell, Square, Stick};

impl Sprite {
    fn get(piece: &Piece) -> Self {
        match piece {
            Bar => Sprite {
                w: 4,
                blocks: vec![[0, 0], [1, 0], [2, 0], [3, 0]],
            },
            Cross => Sprite {
                w: 3,
                blocks: vec![[0, 1], [1, 0], [1, 1], [1, 2], [2, 1]],
            },
            Ell => Sprite {
                w: 3,
                blocks: vec![[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
            },
            Stick => Sprite {
                w: 1,
                blocks: vec![[0, 0], [0, 1], [0, 2], [0, 3]],
            },
            Square => Sprite {
                w: 2,
                blocks: vec![[0, 0], [0, 1], [1, 0], [1, 1]],
            },
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '<' => Left,
            '>' => Right,
            _ => panic!("unknkwon move {c}"),
        }
    }
}

fn next_piece(prev: &Piece) -> Piece {
    match prev {
        Bar => Cross,
        Cross => Ell,
        Ell => Stick,
        Stick => Square,
        Square => Bar,
    }
}

#[derive(Debug)]
struct Board {
    top: Int,
    n: Int,
    occupied: HashSet<Coord>,
    piece: Piece,
    pos: Coord,
    w: Int,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut top = self.top;
        let sprite = Sprite::get(&self.piece);
        let mut sprite_blocks = HashSet::new();
        for block in sprite.blocks.iter() {
            let coord = vec2_add(self.pos, *block);
            sprite_blocks.insert(coord);
            top = top.max(coord[1]);
        }
        top += 1;
        for i in 0..top {
            let y = top - i - 1;
            let mut row = String::with_capacity(self.w as usize + 2);
            row.push('|');
            for x in 0..self.w {
                if sprite_blocks.contains(&[x, y]) {
                    row.push('@');
                } else {
                    row.push(match self.occupied(&[x, y]) {
                        true => '#',
                        false => '.',
                    });
                }
            }
            row.push('|');
            writeln!(f, "{row}")?;
        }
        let bottom = "-".repeat(self.w as usize + 2);
        writeln!(f, "{bottom}")?;
        writeln!(f, "top: {}", self.top)?;
        writeln!(f, "piece: {:?}", self.piece)
    }
}

impl Board {
    fn new() -> Self {
        Board {
            top: 0,
            occupied: HashSet::new(),
            piece: Bar,
            pos: [2, 3],
            w: 7,
            n: 0,
        }
    }

    fn execute(&mut self, m: Move) {
        let sprite = Sprite::get(&self.piece);
        match m {
            Left => {
                if self.pos[0] > 0 && !self.collide(&sprite, &vec2_add(self.pos, [-1, 0])) {
                    self.pos[0] -= 1;
                }
            }
            Right => {
                if (self.pos[0] + sprite.w) < self.w
                    && !self.collide(&sprite, &vec2_add(self.pos, [1, 0]))
                {
                    self.pos[0] += 1;
                }
            }
        }
        if self.pos[1] > 0 && !self.collide(&sprite, &vec2_add(self.pos, [0, -1])) {
            self.pos = vec2_add(self.pos, [0, -1]);
        } else {
            self.cement(&sprite);
            self.piece = next_piece(&self.piece);
            self.pos = [2, self.top + 3];
        }
    }

    fn collide(&self, sprite: &Sprite, pos: &Coord) -> bool {
        sprite
            .blocks
            .iter()
            .any(|block| self.occupied(&vec2_add(*pos, *block)))
    }

    fn occupied(&self, p: &Coord) -> bool {
        self.occupied.contains(p)
    }

    fn cement(&mut self, sprite: &Sprite) {
        for block in sprite.blocks.iter() {
            let p = vec2_add(self.pos, *block);
            self.top = self.top.max(p[1] + 1);
            self.occupied.insert(p);
        }
        self.n += 1;
    }

    fn drop(&mut self, moves: &mut Moves) {
        let start = self.piece;
        while self.piece == start {
            self.execute(moves.next());
        }
        self.clean();
    }

    fn clean(&mut self) {
        let mut defragged = HashSet::new();
        for cell in self.occupied.drain() {
            if cell[1] > self.top - 100 {
                defragged.insert(cell);
            }
        }
        self.occupied = defragged;
    }

    fn find_repeat(&mut self, moves: &mut Moves) -> (i64, i64) {
        let mut memory = HashMap::new();
        loop {
            let fingerprint = Fingerprint::new(self, moves);
            if let Some((rock, top)) = memory.get(&fingerprint) {
                return (*rock, *top);
            }
            memory.insert(fingerprint, (self.n, self.top));
            self.drop(moves);
        }
    }

    fn power_drop(&mut self, moves: &mut Moves, goal: Int) -> Int {
        let (preamble_n, preamble_top) = self.find_repeat(moves);
        let loop_length = self.n - preamble_n;
        println!("found a loop of {loop_length} starting at {preamble_n}");
        let loop_gain = self.top - preamble_top;
        let remaining = goal - preamble_n;
        let loops = remaining / loop_length;
        let loops_gain = loops * loop_gain;
        let remaining = remaining % loop_length;
        let remain_base = self.top;
        for _ in 0..remaining {
            self.drop(moves);
        }
        preamble_top + loops_gain + (self.top - remain_base)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Fingerprint {
    p: Piece,
    m: usize,
    t: Vec<Int>,
}

impl Fingerprint {
    fn new(board: &Board, moves: &Moves) -> Self {
        let p = board.piece;
        let m = moves.n;
        let mut t = Vec::new();
        for i in 0..board.w {
            let mut j = board.top;
            while !board.occupied(&[i, j]) && j >= 0 {
                j -= 1;
            }
            t.push(board.top - j);
        }
        Self { p, m, t }
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/017.txt").expect("file read error");
    let mut moves = Moves::from(input);
    println!("there are {} moves", moves.q.len());
    let mut board = Board::new();
    for _ in 0..2022 {
        board.drop(&mut moves);
    }
    println!("top of structure after 2022 blocks is {}", board.top);

    let mut moves = Moves::from(input);
    let mut board = Board::new();
    let top = board.power_drop(&mut moves, 1_000_000_000_000_i64);
    println!("top of structure after 1T blocks is {top}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#;

    #[test]
    fn test_parse() {
        let moves: VecDeque<Move> = SAMPLE.trim().chars().map(Move::from).collect();
        assert_eq!(moves.len(), 40);
        assert_eq!(moves[0], Right);
        assert_eq!(moves[1], Right);
        assert_eq!(moves[2], Right);
        assert_eq!(moves[3], Left);
        assert_eq!(moves[39], Right);
    }

    #[test]
    fn test_next_move() {
        let mut moves = Moves::from(SAMPLE);
        assert_eq!(moves.next(), Right);
        assert_eq!(moves.next(), Right);
        assert_eq!(moves.next(), Right);
        assert_eq!(moves.next(), Left);
        for _ in 4..40 {
            moves.next();
        }
        assert_eq!(moves.next(), Right);
        assert_eq!(moves.next(), Right);
        assert_eq!(moves.next(), Right);
        assert_eq!(moves.next(), Left);
    }

    #[test]
    fn test_move() {
        let mut moves = Moves::from(SAMPLE);
        // Bar, Cross, Ell, Stick, Square
        let mut board = Board::new();
        assert_eq!(board.piece, Bar);
        assert_eq!(board.pos, [2, 3]);

        board.execute(moves.next()); // Right
        assert_eq!(board.piece, Bar);
        assert_eq!(board.pos, [3, 2]);

        board.execute(moves.next()); // Right
        assert_eq!(board.piece, Bar);
        assert_eq!(board.pos, [3, 1]);

        board.execute(moves.next()); // Right
        assert_eq!(board.piece, Bar);
        assert_eq!(board.pos, [3, 0]);

        board.execute(moves.next()); // Left
        assert!(!board.occupied(&[1, 0])); // cement the bar
        assert!(!board.occupied(&[1, 1]));
        assert!(board.occupied(&[2, 0]));
        assert!(board.occupied(&[5, 0]));
        assert!(!board.occupied(&[6, 0]));
        assert_eq!(board.piece, Cross);
        assert_eq!(board.top, 1);
        assert_eq!(board.pos, [2, 4]);

        board.execute(moves.next()); // Left
        assert_eq!(board.piece, Cross);
        assert_eq!(board.top, 1);
        assert_eq!(board.pos, [1, 3]);

        board.execute(moves.next()); // Right
        assert_eq!(board.piece, Cross);
        assert_eq!(board.top, 1);
        assert_eq!(board.pos, [2, 2]);

        board.execute(moves.next()); // Left
        assert_eq!(board.piece, Cross);
        assert_eq!(board.top, 1);
        assert_eq!(board.pos, [1, 1]);

        board.execute(moves.next()); // Right
        assert!(board.occupied(&[3, 3])); // cement the cross
        assert_eq!(board.piece, Ell);
        assert_eq!(board.top, 4);
        assert_eq!(board.pos, [2, 7]);

        while board.piece == Ell {
            board.execute(moves.next());
        }
        assert_eq!(board.top, 6);
        while board.piece == Stick {
            board.execute(moves.next());
        }
        assert_eq!(board.top, 7);
        while board.piece == Square {
            board.execute(moves.next());
        }
        assert_eq!(board.top, 9);
        while board.piece != Square {
            board.execute(moves.next());
        }
        while board.piece == Square {
            println!("{board}");
            board.execute(moves.next());
        }
        println!("{board}");
        assert_eq!(board.top, 17);
    }

    #[test]
    fn test_drop() {
        let mut moves = Moves::from(SAMPLE);
        let mut board = Board::new();
        for _ in 0..10 {
            board.drop(&mut moves);
        }
        println!("{board}");
        assert_eq!(board.top, 17);
    }

    #[test]
    fn test_drop_all_the_things() {
        let mut moves = Moves::from(SAMPLE);
        let mut board = Board::new();
        for _ in 0..2022 {
            board.drop(&mut moves);
        }
        println!("{board}");
        assert_eq!(board.top, 3068);
    }

    #[test]
    fn test_fingerprint() {
        let mut moves = Moves::from(SAMPLE);
        let mut board = Board::new();
        let mut memory = HashSet::new();
        memory.insert(Fingerprint::new(&board, &moves));
        for _ in 0..28 {
            board.drop(&mut moves);
        }
        let f28 = Fingerprint::new(&board, &moves);
        assert!(!memory.contains(&f28));
        memory.insert(f28);
        for _ in 28..63 {
            board.drop(&mut moves);
        }
        let f63 = Fingerprint::new(&board, &moves);
        assert!(memory.contains(&f63));
        for _ in 63..98 {
            board.drop(&mut moves);
        }
        let f98 = Fingerprint::new(&board, &moves);
        assert!(memory.contains(&f98));

        // let mut memory = HashSet::new();
        // let fb = Fingerprint {};
        // let fb = Fingerprint {};
    }

    #[test]
    fn test_find_loops() {
        let mut moves = Moves::from(SAMPLE);
        let mut board = Board::new();
        assert_eq!(board.find_repeat(&mut moves), (28, 49));
        assert_eq!(board.n, 63);
        assert_eq!(board.top, 102);
    }

    #[test]
    fn test_terra_drop() {
        let mut moves = Moves::from(SAMPLE);
        let mut board = Board::new();
        let top = board.power_drop(&mut moves, 1_000_000_000_000_i64);
        assert_eq!(top, 1_514_285_714_288_i64);
    }
}
