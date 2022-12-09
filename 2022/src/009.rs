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
type State = Vec<Position>; // head, tail
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
    let mut s = s.clone();
    let mut h = s[0];
    match d {
        U => h.1 = h.1 + 1,
        D => h.1 = h.1 - 1,
        R => h.0 = h.0 + 1,
        L => h.0 = h.0 - 1,
    }
    s[0] = h;
    s
}

fn follow(s: &State) -> State {
    let mut next = State::new();
    let mut prev = None;
    for t in s {
        match prev {
            None => {
                prev = Some(t.clone());
                next.push(t.clone());
            },
            Some(h) => {
                let mut t = t.clone();
                if (h.0 - t.0).abs().max((h.1 - t.1).abs()) == 2 {
                    t.0 = t.0 + (h.0 - t.0).signum();
                    t.1 = t.1 + (h.1 - t.1).signum();
                }
                prev = Some(t.clone());
                next.push(t);
            }
        }
    }
    next
}

fn run_program(moves: &Program, num_knots: usize) -> usize {
    let mut trace = HashSet::new();
    let mut s = Vec::new();
    s.resize_with(num_knots + 1, || (0, 0));
    trace.insert(s[num_knots]);
    for op in moves {
        for _ in 0..op.n {
            s = follow(&execute(&s, &op.d));
            trace.insert(s[num_knots]);
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
    println!("1-tail touched {} locations", run_program(&program, 1));
    println!("9-tail touched {} locations", run_program(&program, 9));
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
const LONG_SAMPLE: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    #[test]
    fn test_parse_moves() {
        let program: Program = SAMPLE.lines().map(|s| Op::from(s)).collect();
        assert_eq!(program.len(), 8);
    }

    #[test]
    fn test_move() {
        let s = Vec::from([(3, 4), (2, 1)]);
        assert_eq!(execute(&s, &U), Vec::from([(3, 5), (2, 1)]));
        assert_eq!(execute(&s, &D), Vec::from([(3, 3), (2, 1)]));
        assert_eq!(execute(&s, &R), Vec::from([(4, 4), (2, 1)]));
        assert_eq!(execute(&s, &L), Vec::from([(2, 4), (2, 1)]));
        let s = Vec::from([(0, 4), (2, 1)]);
        assert_eq!(execute(&s, &L), Vec::from([(-1, 4), (2, 1)]));
    }

    #[test]
    fn test_snake_follow() {
        let mut s = Vec::from([(3, 4); 10]);
        for _ in 0..9 {
            s = follow(&execute(&s, &R));
        }
        assert_eq!(s[0], (3 + 9, 4));
        assert_eq!(s[1], (3 + 8, 4));
        assert_eq!(s[5], (3 + 4, 4));
        assert_eq!(s[9], (3 + 0, 4));
    }

    #[test]
    fn test_follow() {
        assert_eq!(follow(&Vec::from([(3, 4), (3, 4)])), Vec::from([(3, 4), (3, 4)]));
        assert_eq!(follow(&Vec::from([(3, 4), (3, 5)])), Vec::from([(3, 4), (3, 5)]));
        assert_eq!(follow(&Vec::from([(3, 4), (4, 5)])), Vec::from([(3, 4), (4, 5)]));
        assert_eq!(follow(&Vec::from([(3, 4), (2, 3)])), Vec::from([(3, 4), (2, 3)]));

        assert_eq!(follow(&Vec::from([(3, 4), (5, 6)])), Vec::from([(3, 4), (4, 5)]));
        assert_eq!(follow(&Vec::from([(3, 4), (1, 2)])), Vec::from([(3, 4), (2, 3)]));

        assert_eq!(follow(&Vec::from([(3, 4), (4, 6)])), Vec::from([(3, 4), (3, 5)]));
        assert_eq!(follow(&Vec::from([(3, 4), (2, 2)])), Vec::from([(3, 4), (3, 3)]));
    }

    #[test]
    fn test_run_short_tail() {
        let program: Program = SAMPLE.lines().map(|s| Op::from(s)).collect();
        assert_eq!(run_program(&program, 1), 13);
    }

    #[test]
    fn test_run_long_tail() {
        let program: Program = LONG_SAMPLE.lines().map(|s| Op::from(s)).collect();
        assert_eq!(run_program(&program, 9), 36);
    }
}
