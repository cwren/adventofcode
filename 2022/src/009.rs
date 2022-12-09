use std::collections::HashSet;
use std::io::Read;

enum Direction {
    U,
    D,
    R,
    L,
}
use crate::Direction::{D, L, R, U};

struct Op {
    d: Direction,
    n: i32,
}
type Position = (i32, i32); // x, y
type State = (Position, Position); // head, tail
type Program = Vec<Op>;
impl Op {
    fn new(d: Direction, n: i32) -> Self {
        Op { d, n }
    }
}

impl From<&str> for Direction {
    fn from(line: &str) -> Self {
        match line {
            "U" => U,
            "D" => D,
            "R" => R,
            "L" => L,
            _ => panic!("unrecognized direction"),
        }
    }
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        let mut parts = line.split(' ');
        let dir = Direction::from(parts.next().expect("empty operator"));
        let dist = parts
            .next()
            .expect("empty operand")
            .parse()
            .expect("non-numeric operand");
        Op { d: dir, n: dist }
    }
}

fn execute(s: &State, d: &Direction) -> State {
    let (h, t) = s;
    match d {
        U => ((h.0, h.1 + 1), *t),
        D => ((h.0, h.1 - 1), *t),
        R => ((h.0 + 1, h.1), *t),
        L => ((h.0 - 1, h.1), *t),
    }
}

fn follow(s: &State) -> State {
    let (h, t) = s;
    let d = (h.0 - t.0).abs().max((h.1 - t.1).abs());
    match d {
        0 | 1 => (*h, (t.0, t.1)),
        2 => (*h, (t.0 + (h.0 - t.0).signum(), t.1 + (h.1 - t.1).signum())),
        _ => panic!("how did the head get so far away!"),
    }
}

fn run_program(moves: &Program) -> usize {
    let mut trace = HashSet::new();
    let mut s = ((0, 0), (0, 0));
    trace.insert(s.1);
    for op in moves {
        for _ in 0..op.n {
            s = follow(&execute(&s, &op.d));
            trace.insert(s.1);
        }
    }
    trace.len()
}

fn main() {
    let mut f = std::fs::File::open("input/009.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    let program: Program = input.lines().map(|s| Op::from(s)).collect();
    println!("there are {} operations", program.len());
    println!("tail touched {} locations", run_program(&program));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn test_parse_moves() {
        let program: Program = SAMPLE.lines().map(|s| Op::from(s)).collect();
        assert_eq!(program.len(), 8);
    }

    #[test]
    fn test_move() {
        assert_eq!(execute(&((3, 4), (2, 1)), &U), ((3, 5), (2, 1)));
        assert_eq!(execute(&((3, 4), (2, 1)), &D), ((3, 3), (2, 1)));
        assert_eq!(execute(&((3, 4), (2, 1)), &R), ((4, 4), (2, 1)));
        assert_eq!(execute(&((3, 4), (2, 1)), &L), ((2, 4), (2, 1)));
        assert_eq!(execute(&((0, 4), (2, 1)), &L), ((-1, 4), (2, 1)));
    }

    #[test]
    fn test_follow() {
        assert_eq!(follow(&((3, 4), (3, 4))), ((3, 4), (3, 4)));
        assert_eq!(follow(&((3, 4), (3, 5))), ((3, 4), (3, 5)));
        assert_eq!(follow(&((3, 4), (4, 5))), ((3, 4), (4, 5)));
        assert_eq!(follow(&((3, 4), (2, 3))), ((3, 4), (2, 3)));

        assert_eq!(follow(&((3, 4), (5, 6))), ((3, 4), (4, 5)));
        assert_eq!(follow(&((3, 4), (1, 2))), ((3, 4), (2, 3)));

        assert_eq!(follow(&((3, 4), (4, 6))), ((3, 4), (3, 5)));
        assert_eq!(follow(&((3, 4), (2, 2))), ((3, 4), (3, 3)));
    }
    #[test]
    fn test_run_program() {
        let program: Program = SAMPLE.lines().map(|s| Op::from(s)).collect();
        assert_eq!(run_program(&program), 13);
    }
}
