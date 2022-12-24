use std::fs;
use vecmath::{vec2_add, vec2_neg, Vector2};

type Coord = Vector2<i32>;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction { 
    North,
    South,
    East,
    West,
}
use Direction::{North, South, East, West};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Blizzard {
    pos: Coord,
    dir: Direction,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    blizzards: Vec<Blizzard>,
    valley: Coord,
    start: Coord,
    end: Coord,
}

impl From<&str> for State {
    fn from(input: &str) -> Self {
        let mut blizzards = Vec::new();
        let mut start = [0, 0];
        let mut end = [0, 0];
        let mut valley = [0, 1];
        let mut lines = input.lines();

        // north wall
        let top = lines.next().expect("must have at least one line in the map");
        valley[0] = top.len() as i32;
        start[0] = top.find('.').expect("there should be a door in the north wall") as i32;
        for line in lines {
            let j = valley[1];
            let bytes = line.to_string().into_bytes();
            if bytes.iter().filter(|b| **b == '#' as u8).count() > 2 {
                // south wall
                end[1] = valley[1];
                end[0] = line.find('.').expect("there should be a door in the south wall") as i32;
            } else {
                for i in 1..(valley[0] as usize - 1) {
                    match bytes[i] as char {
                        '.' => (),
                        '#' => panic!("found a wall inside the valley"),
                        '^' => blizzards.push(Blizzard{ pos: [i as i32, j as i32], dir: North }),
                        '>' => blizzards.push(Blizzard{ pos: [i as i32, j as i32], dir: East }),
                        'v' => blizzards.push(Blizzard{ pos: [i as i32, j as i32], dir: South }),
                        '<' => blizzards.push(Blizzard{ pos: [i as i32, j as i32], dir: West }),
                        _ => panic!("illegal map character"),
                    }
                }
            }
            valley[1] += 1;
        }
        State { blizzards, valley, start, end }
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/024.txt").expect("file read error");
    let state = State::from(input);
    println!("start at {:?}", state.start);
    println!("end at {:?}", state.end);
    println!("valley is {:?} size", state.valley);
    println!("there are {:?} blizzards", state.blizzards.len());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    #[test]
    fn test_parse_input() {
        let state = State::from(SAMPLE);
        assert_eq!(state.start, [1, 0] ,"wrong start");
        assert_eq!(state.end, [6, 5], "wrong end");
        assert_eq!(state.valley, [8, 6], "wrong valley size");
        assert_eq!(state.blizzards.len(), 19);
        assert_eq!(state.blizzards[0].pos, [1, 1]);
        assert_eq!(state.blizzards[0].dir, East);
        assert_eq!(state.blizzards[9].pos, [2, 3]);
        assert_eq!(state.blizzards[9].dir, South);
        assert_eq!(state.blizzards[2].pos, [4, 1]);
        assert_eq!(state.blizzards[2].dir, West);
        assert_eq!(state.blizzards[3].pos, [5, 1]);
        assert_eq!(state.blizzards[3].dir, North);
        assert_eq!(state.blizzards[18].pos, [6, 4]);
        assert_eq!(state.blizzards[18].dir, East);
    }
}
