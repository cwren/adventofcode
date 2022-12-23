#![feature(is_some_and)]
use std::{fs, fmt};
use std::collections::{HashMap, HashSet, VecDeque};
use vecmath::{vec2_add, vec2_sub, Vector2};

type Coord = Vector2<i32>;

struct Map {
    m: HashSet<Coord>,
    t: u32,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ll, ur) = self.rect();
        let ll = vec2_sub(ll, [2, 2]);
        let ur = vec2_add(ur, [2, 2]);
        for j in ll[1]..ur[1] {
            for i in ll[0]..ur[0] {
                match self.m.contains(&[i, j]) {
                    false => write!(f, "."),
                    true => write!(f, "#"),
                }?;
            }
            writeln!(f)?;
        }
        writeln!(f, "{:?}",  self.bound())
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut m = HashSet::new();
        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => { m.insert([i as i32, j as i32]); }
                    _ => panic!("unexpected map element"),
                };
            }
        }
        let t = 0;
        Map { m , t }
    }
}

const EIGHT: [[i32; 2]; 8] = [[1, 1], [1, 0], [1, -1], [0, -1], [-1, -1], [-1, 0], [-1, 1], [0, 1]];

impl Map {
    fn options(&self) -> VecDeque<(Coord, Coord, Coord)>{
        let mut options = VecDeque::from(vec![
            ([0, -1], [-1, -1], [1, -1]),
            ([0, 1], [-1, 1], [1, 1]),
            ([-1, 0], [-1, 1], [-1, -1]),
            ([1, 0], [1, -1], [1, 1]),
        ]);
        options.rotate_left((self.t % 4) as usize);
        options
    }
    
    fn tick(&mut self) -> bool {
        let mut destinations = HashSet::new();
        let mut moves = HashMap::new();
        let mut blocked = HashSet::new();
        let options = self.options();

        //look 
        'monkey: for monkey in self.m.iter() {
            let mut happy = true;
            for direction in EIGHT {
                let probe = vec2_add(*monkey, direction);
                if self.m.contains(&probe) {
                    happy = false;
                    break;
                }
            }
            if happy {
                continue 'monkey // don't move
            }
            for option in options.iter() {
                let destination = vec2_add(*monkey, option.0);
                if !self.m.contains(&destination) &&
                   !self.m.contains(&vec2_add(*monkey, option.1)) &&
                   !self.m.contains(&vec2_add(*monkey, option.2)) {
                    if destinations.contains(&destination) {
                        blocked.insert(destination);
                    } else {
                        destinations.insert(destination);
                        moves.insert(monkey, destination);
                    }
                    continue 'monkey;
                }
            }
        }

        // move
        let mut someone_moved = false;
        let mut next = HashSet::new();
        for monkey in self.m.iter() {
            let destination = moves.get(monkey);
            if destination.is_some_and(|d| !blocked.contains(d)) {
                next.insert(*destination.unwrap());
                someone_moved = true;
            } else {
                next.insert(*monkey);
            }
        }

        self.m = next;
        self.t += 1;

        someone_moved
    }

    fn rect(&self) -> (Coord, Coord) {
        let mut ll = [i32::MAX, i32::MAX];
        let mut ur = [0, 0];
        for monkey in self.m.iter() {
            ll[0] = ll[0].min(monkey[0]);
            ll[1] = ll[1].min(monkey[1]);
            ur[0] = ur[0].max(monkey[0]);
            ur[1] = ur[1].max(monkey[1]);
        }
        (ll, vec2_add(ur,[1, 1]))
    }
    
    fn bound(&self) -> Coord {
        let (ll, ur) = self.rect();
        vec2_sub(ur, ll)
    }
    fn empty_ground(&self) -> u32 {
        let r = self.bound();
        let n = (r[0] * r[1]) as u32;
        n - (self.m.len() as u32)
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/023.txt").expect("file read error");
    let mut map = Map::from(input);
    for _ in 0..10 {
        map.tick();
    }
    println!("{map:?}");
    println!("there are {} tiles of open ground after 10 ticks", map.empty_ground());
    while map.tick() {}
    println!("{map:?}");
    println!("it took {} ticks before all the elves were happy", map.t);
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#;
    const SMALL_SAMPLE: &str = r#".....
..##.
..#..
.....
..##.
....."#;
    #[test]
    fn test_parse_input() {
        assert_eq!(Map::from(SAMPLE).m.len(), 22);
    }

    #[test]
    fn test_tick() {
        let mut map = Map::from(SMALL_SAMPLE);
        println!("{:?}", map);
        assert_eq!(map.t, 0);
        assert_eq!(map.bound(), [2, 4]);
        map.tick();
        println!("{:?}", map);
        assert_eq!(map.t, 1);
        assert_eq!(map.bound(), [2, 5]);
    }

    #[test]
    fn test_big_tick() {
        let mut map = Map::from(SAMPLE);
        println!("{map:?}");
        assert_eq!(map.t, 0);
        assert_eq!(map.bound(), [7, 7]);
        map.tick();
        println!("{map:?}");
        assert_eq!(map.t, 1);
        assert_eq!(map.bound(), [9, 9]);
    }

    #[test]
    fn test_empty_ground() {
        let mut map = Map::from(SAMPLE);
        for _ in 0..10 {
            map.tick();
        }
        assert_eq!(map.t, 10);
        println!("{map:?}");
        assert_eq!(map.empty_ground(),110);
    }

    #[test]
    fn test_run_to_completion() {
        let mut map = Map::from(SAMPLE);
        for _ in 0..10 {
            map.tick();
        }
        println!("{map:?}");
        while map.tick() {}
        println!("{map:?}");
        assert_eq!(map.t, 20);
    }
}