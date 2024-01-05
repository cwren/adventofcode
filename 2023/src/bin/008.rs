use lazy_static::lazy_static;
use lcmx::lcmx;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    LEFT,
    RIGHT,
}
struct DanceCard {
    moves: VecDeque<Move>,
    orininal: Vec<Move>,
}

struct Map {
    card: DanceCard,
    nodes: HashMap<String, (String, String)>,
}

impl From<String> for DanceCard {
    fn from(line: String) -> Self {
        let v = line
            .chars()
            .filter(|c| *c == 'R' || *c == 'L')
            .map(|c| if c == 'L' { Move::LEFT } else { Move::RIGHT })
            .collect::<Vec<Move>>();
        DanceCard {
            moves: VecDeque::from(v.clone()),
            orininal: v,
        }
    }
}

lazy_static! {
    static ref NODE_RE: regex::Regex =
        Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let mut liter = lines.iter();
        let card = DanceCard::from(liter.next().expect("missing first line").to_owned());
        liter.next().expect("missing second line");
        let nodes = liter
            .map(|line| {
                if let Some(cap) = NODE_RE.captures(line) {
                    let key = cap.get(1).unwrap().as_str().to_string();
                    let lft = cap.get(2).unwrap().as_str().to_string();
                    let rgt = cap.get(3).unwrap().as_str().to_string();
                    (key, (lft, rgt))
                } else {
                    panic!("malformed line: {line}");
                }
            })
            .collect::<HashMap<String, (String, String)>>();
        Map { card, nodes }
    }
}
impl DanceCard {
    fn next(&mut self) -> Move {
        let m = self.moves.pop_front().expect("moves is empty!");
        self.moves.push_back(m);
        m
    }

    fn reset(&mut self) {
        self.moves = VecDeque::from(self.orininal.clone());
    }
}

impl Map {
    fn walk(&mut self) -> VecDeque<String> {
        self.do_walk("AAA")
    }

    fn do_walk(&mut self, start: &str) -> VecDeque<String> {
        let mut path = VecDeque::new();
        let mut here = start;
        path.push_back(here.to_owned());
        while !here.ends_with('Z') {
            let fork = self.nodes.get(here).unwrap();
            match self.card.next() {
                Move::LEFT => here = &fork.0,
                Move::RIGHT => here = &fork.1,
            }
            path.push_back(here.to_owned());
        }
        path
    }

    fn ghost_walk(&mut self) -> usize {
        let mut starts = Vec::new();
        for key in self.nodes.keys() {
            if key.ends_with('A') {
                starts.push(key.to_owned());
            }
        }
        let mut loop_lengths = Vec::new();
        for start in starts {
            self.card.reset();
            loop_lengths.push(self.do_walk(&start).len() - 1);
        }
        lcmx(&loop_lengths).unwrap()
    }
}
fn main() {
    let f = File::open("input/008.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let mut map = Map::from(lines);
    let path = map.walk();
    println!("{} steps tp get to ZZZ", path.len() - 1);
    println!("{} ghost deviations to get to the Zs", map.ghost_walk());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
    const SAMPLE2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)    
"#;
    const GHOSTMAP: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn test_parse() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        assert_eq!(map.card.moves.len(), 2);
        assert_eq!(map.nodes.len(), 7);
        assert_eq!(map.nodes.get("CCC").unwrap().0, "ZZZ");
        assert_eq!(map.nodes.get("CCC").unwrap().1, "GGG");
    }

    #[test]
    fn test_card() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        assert_eq!(map.card.next(), Move::RIGHT);
        assert_eq!(map.card.next(), Move::LEFT);
        assert_eq!(map.card.next(), Move::RIGHT);
        map.card.reset();
        assert_eq!(map.card.next(), Move::RIGHT);
    }

    #[test]
    fn test_walk1() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk();
        assert_eq!(path.len() - 1, 2);
    }

    #[test]
    fn test_walk2() {
        let lines = SAMPLE2.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk();
        assert_eq!(path.len() - 1, 6);
    }

    #[test]
    fn test_ghostwalk() {
        let lines = GHOSTMAP.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        assert_eq!(map.ghost_walk(), 6);
    }
}
