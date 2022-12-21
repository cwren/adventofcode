use std::fmt::Display;
use std::fs;
use std::collections::{HashSet, VecDeque};
use vecmath::{vec2_add, Vector2};

type Int = i32;
type Coord = Vector2<Int>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Left,
    Right,
}
use Move::{Left, Right};

struct Moves {
    q: VecDeque<Move>,
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        Moves { q: s.trim().chars().map(Move::from).collect::<VecDeque<Move>>() }
    }
}

impl Moves {
    fn next(&mut self) -> Move {
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
use Piece::{Bar, Cross, Ell, Stick, Square};

impl Sprite {
    fn get(piece: &Piece) -> Self {
        match piece {
            Bar => Sprite { w: 4, blocks: vec![[0,0], [1,0], [2,0], [3,0]] },
            Cross => Sprite { w: 3, blocks: vec![[0,1], [1,0], [1,1], [1,2], [2,1]] },
            Ell => Sprite { w: 4, blocks: vec![[0,0], [1,0], [2,0], [2,1], [2,2]] },
            Stick => Sprite { w: 1, blocks: vec![[0,0], [0,1], [0,2], [0,3]] },
            Square => Sprite { w: 2, blocks: vec![[0,0], [0,1], [1,0], [1,1]] },
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
    occupied: HashSet<Coord>,
    piece: Piece,
    pos: Coord,
    w: Int,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.top {
            let y = self.top - i - 1;
            let mut row = String::with_capacity(self.w as usize + 2);
            row.push('|');
            for x in 0..self.w {
                row.push(match self.occupied(&[x, y]) {
                    true => '#',
                    false => '.',
                });
            }
            row.push('|');
            write!(f, "{row}\n")?;
        }
        let bottom = std::iter::repeat("-").take(self.w as usize + 2).collect::<String>();
        write!(f, "{bottom}\n")?;
        write!(f, "top: {}\n", self.top)?;
        write!(f, "piece: {:?}\n", self.piece)
    }
}

impl Board {
    fn new () -> Self {
        Board{ top: 0, occupied: HashSet::new(), piece: Bar, pos: [2, 3], w: 7 }
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
                if (self.pos[0] + sprite.w) < self.w && !self.collide(&sprite, &vec2_add(self.pos, [1, 0])) {
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
        sprite.blocks.iter().any(|block| self.occupied(&vec2_add(*pos, *block)))
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
    }
    
    fn drop(&mut self, moves: &mut Moves) {
        let start = self.piece;
        while self.piece == start {
            self.execute(moves.next());
        }
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
    println!("top of structure after 2022 blocks is {}", board.top - 1);
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
        assert_eq!(board.top - 1, 16);
    }

    #[test]
    fn test_drop_all_the_things() {
        let mut moves = Moves::from(SAMPLE);
        let mut board = Board::new();
        for _ in 0..2022 {
            board.drop(&mut moves);
        }
        println!("{board}");
        assert_eq!(board.top - 1, 3068);
    }
}
