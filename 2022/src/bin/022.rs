use lazy_static::lazy_static;
use std::{fs, str::Lines, fmt};
use std::collections::HashMap;
use vecmath::{vec2_add, vec2_neg, Vector2};
use pest::{Parser, iterators::Pair};
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "bin/022.pest"]
struct ListParser;

type Coord = Vector2<i32>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Void,
    Open,
    Wall
}
use Cell::{Void, Open, Wall};

struct Map {
    m: Vec<Vec<Cell>>,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            ' ' => Void,
            '.' => Open,
            '#' => Wall,
            _ => panic!("illegal map character"),
        }
    }
}

impl Map {
    fn read(lines: &mut Lines) -> Self {
        let mut m = Vec::new();
        loop {
            let line = lines.next();
            if line.is_none() || line.unwrap().is_empty() {
                return Map {m};
            }
            m.push(line.unwrap().chars().map(Cell::from).collect());
        }
    }

    fn get(&self, coord: &Coord) -> Cell {
        if coord[1] < 0 || coord[0] < 0 { 
            return Void;
        }
        let i = coord[0] as usize;
        let j = coord[1] as usize;
        if j >= self.m.len() {
            return Void;
        }
        if i >= self.m[j].len() {
            return Void;
        }
        self.m[j][i]
    }

    fn find_start(&self) -> State {
        let mut x = [0, 0];
        let f = East;
        for (i, c) in self.m[0].iter().enumerate() {
            if *c == Open {
                x[0] = i as i32;
                return State {x, f};
            }
        }
        panic!("couldn't find an open spot on the top row!");
    }

    fn warp(&self, x: &Coord, dx: &Coord, wormholes: &Option<Edgemap>) -> Coord {
        let mut x1 = *x;
        loop {
            let x2 = vec2_add(x1, *dx);
            if self.get(&x2) == Void {
                return x1;
            }
            x1 = x2;
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.m.iter() {
            for cell in row.iter() {
                match cell {
                    Void => write!(f, " "),
                    Open => write!(f, "."),
                    Wall => write!(f, "#"),
                }?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Right,
    Left,
    Move(i32),
}
use Instruction::{Right, Left, Move};
type Instructions = Vec<Instruction>;

impl From<Pair<'_, Rule>> for Instruction {
    fn from(token: Pair<Rule>) -> Self {
        match token.as_rule() {
            Rule::number => return Move(token.as_str().parse().expect("parser says so")),
            Rule::left => Left, 
            Rule::right => Right, 
            Rule::list => panic!("no recursive lists!"), 
        }
    }
}

fn parse_instructions(s: &str) -> Instructions {
    let mut tokens = ListParser::parse(Rule::list, s).unwrap_or_else(|e| panic!("{}", e));
    let list = tokens.next().expect("at least one token per lline");
    list.into_inner().map(Instruction::from).collect()
}

fn parse_all(input: &str) -> (Map, Instructions) {
    let mut lines = input.lines(); 
    (Map::read(&mut lines), parse_instructions(lines.next().unwrap()))
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Facing { 
    North,
    South,
    East,
    West,
}
use Facing::{North, South, East, West};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    x: Coord,
    f: Facing,
}

impl State {
    fn score(&self) -> i32 { 
        // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
        // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
        let facing_score = match self.f {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        };
        let column_score = 4 * (self.x[0] + 1);
        let row_score = 1000 * (self.x[1] + 1);
        facing_score + column_score + row_score

    }
    fn go_walkies(&mut self, m: &Map, instructions: &Instructions, wormholes: &Option<Edgemap>) {
        for i in instructions.iter() {
            self.follow(m, i, wormholes);
        }
    }

    fn follow(&mut self, map: &Map, i: &Instruction, wormholes: &Option<Edgemap>) {
        match i {
            Right => self.turn_right(),
            Left =>  self.turn_left(),
            Move(n) => self.walk_forward(map, *n, wormholes),
        }
    }

    fn walk_forward(&mut self, map: &Map, n: i32, wormholes: &Option<Edgemap>) {
        let dx = match self.f {
            North => [0, -1],
            East => [1, 0],
            South => [0 ,1],
            West => [-1, 0],
        };
        for _ in 0..n {
            let x1 = vec2_add(self.x, dx);
            match map.get(&x1) {
                Open => self.x = x1,
                Wall => break,
                Void => {
                    let x1 = map.warp(&self.x, &vec2_neg(dx), wormholes);
                    if map.get(&x1) == Wall {
                        break;
                    }
                    self.x = x1;
                },
            }
        }
    }

    fn turn_left(&mut self) {
        self.f = match self.f {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn turn_right(&mut self) {
        self.f = match self.f {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}


type Warp = (i32, i32, Facing);

#[derive(Clone)]
struct Edgemap {
    n: i32,
    j: HashMap<Warp, Warp>,
}

fn main() {
    let input: &str = &fs::read_to_string("input/022.txt").expect("file read error");
    let (map, instructions) = parse_all(input);
    let mut state = map.find_start();
    state.go_walkies(&map, &instructions, &None);
    println!("the walk score is {}", state.score());
}
#[test]
fn test_score() {
    assert_eq!(State { x: [7, 5], f: East}.score(), 6032);
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    lazy_static! {
        static ref sample_edgemap: Edgemap = Edgemap {
            n: 4,
            j: vec![
                ((2, 0, West),  (1, 1, South)),
                ((2, 0, North), (0, 1, South)),
                ((2, 0, East),  (3, 2, West)),
                ((0, 1, North), (2, 0, South)),
                ((0, 1, West),  (3, 2, North)),
                ((0, 1, South), (2, 2, North)),
                ((1, 1, North), (2, 0, West)),
                ((1, 1, South), (2, 2, East)),
                ((2, 1, East),  (3, 2, South)),
                ((2, 3, West),  (1, 1, North)),
                ((2, 3, South), (0, 1, North)),
                ((3, 2, South), (0, 1, East)),
                ((3, 2, East),  (2, 0, West)),
                ((3, 2, North), (2, 1, West)),
            ].iter().cloned().collect(),
        };
    }
    
    #[test]
    fn test_parse_map() {
        let mut lines = SAMPLE.lines();
        Map::read(&mut lines);
        assert!(lines.next().is_some());
    }
    #[test]
    fn test_parse_instructions() {
        let line = SAMPLE.lines().last().unwrap();
        let instructions = parse_instructions(line);
        assert_eq!(instructions.len(), 13);
        assert_eq!(instructions, vec![
            // 10R5L5R10
            Move(10), Right, Move(5), Left, Move(5), Right, Move(10),
            // L4R5L5
            Left, Move(4), Right, Move(5), Left, Move(5)]);
    }
    #[test]
    fn test_parse_all() {
        let (map, instructions) = parse_all(SAMPLE);
        assert_eq!(instructions.len(), 13);
        assert_eq!(map.get(&[11, 3]), Open);
        assert_eq!(map.get(&[11, 4]), Wall);
        assert_eq!(map.get(&[4, 8]), Void);
    }
    #[test]
    fn test_find_start() {
        let (map, _) = parse_all(SAMPLE);
        assert_eq!(map.find_start(), State { x: [8, 0], f: East});

    }
    #[test]
    fn test_walkies() {
        let (map, instructions) = parse_all(SAMPLE);
        let mut state = map.find_start();
        state.go_walkies(&map, &instructions, &None);
        assert_eq!(state, State { x: [7, 5], f: East});
    }
    #[test]
    fn test_score() {
        assert_eq!(State { x: [7, 5], f: East}.score(), 6032);
    }
}
