use priority_queue::DoublePriorityQueue;
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::{Hash, Hasher},
};
use vecmath::{vec3_add, Vector2};

type Int = i32;
type SCoord = Vector2<Int>;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::{East, North, South, West};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Blizzard {
    pos: SCoord,
    dir: Direction,
}
impl Blizzard {
    fn wrap(&mut self, valley: [i32; 2]) {
        // #......# 1..valley-1
        if self.pos[0] < 1 {
            self.pos[0] = valley[0] - 2;
        }
        if self.pos[0] >= valley[0] - 1 {
            self.pos[0] = 1;
        }
        if self.pos[1] < 1 {
            self.pos[1] = valley[1] - 2;
        }
        if self.pos[1] >= valley[1] - 1 {
            self.pos[1] = 1;
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    occupied: HashSet<SCoord>,
    blizzards: Vec<Blizzard>,
    t: i32,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state);
    }
}

#[derive(Clone, Debug)]
struct Map {
    blizzards: Vec<Blizzard>,
    valley: SCoord,
    start: SCoord,
    end: SCoord,
    states: HashMap<i32, State>,
}
impl Map {
    fn longest(&self) -> usize {
        usize::MAX / 2
    }

    fn at_time(&mut self, t: i32) -> &State {
        if !self.states.contains_key(&t) {
            let state = self.tick(
                self.states
                    .get(&(t - 1))
                    .expect("search tried to step twice?"),
            );
            self.states.insert(t, state);
        }
        self.states.get(&t).unwrap()
    }

    fn tick(&self, t0: &State) -> State {
        let mut next = Vec::new();
        let mut occ = HashSet::new();
        let t = t0.t + 1;
        for b in t0.blizzards.iter() {
            let mut b = *b;
            match b.dir {
                North => b.pos[1] -= 1,
                South => b.pos[1] += 1,
                East => b.pos[0] += 1,
                West => b.pos[0] -= 1,
            }
            b.wrap(self.valley);
            next.push(b);
        }
        for b in next.iter() {
            occ.insert(b.pos);
        }
        State {
            blizzards: next,
            occupied: occ,
            t,
        }
    }

    fn h(&self, p: [i32; 3]) -> usize {
        ((self.end[0] - p[0]).abs() + (self.end[1] - p[1]).abs() + p[2])
            .try_into()
            .unwrap()
    }

    fn can_step(&mut self, current: [i32; 3], offset: [i32; 3]) -> Option<[i32; 3]> {
        let next = vec3_add(current, offset);
        if self.end == [next[0], next[1]] || self.start == [next[0], next[1]] {
            return Some(next);
        }
        if next[0] < 1
            || next[0] > self.valley[0] - 2
            || next[1] < 1
            || next[1] > self.valley[1] - 2
        {
            return None;
        }
        let state = self.at_time(next[2]);
        if state.occupied.contains(&[next[0], next[1]]) {
            return None;
        }
        Some(next)
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut blizzards = Vec::new();
        let mut states = HashMap::new();
        let mut start = [0, 0];
        let mut end = [0, 0];
        let mut valley = [0, 1];
        let mut lines = input.lines();

        // north wall
        let top = lines
            .next()
            .expect("must have at least one line in the map");
        valley[0] = top.len() as i32;
        start[0] = top
            .find('.')
            .expect("there should be a door in the north wall") as i32;
        for line in lines {
            let j = valley[1];
            let bytes = line.to_string().into_bytes();
            if bytes.iter().filter(|b| **b == b'#').count() > 2 {
                // south wall
                end[1] = valley[1];
                end[0] = line
                    .find('.')
                    .expect("there should be a door in the south wall")
                    as i32;
            } else {
                for (i, b) in bytes.iter().enumerate().take(valley[0] as usize - 1).skip(1) {
                    match *b as char {
                        '.' => (),
                        '#' => panic!("found a wall inside the valley"),
                        '^' => blizzards.push(Blizzard {
                            pos: [i as i32, j],
                            dir: North,
                        }),
                        '>' => blizzards.push(Blizzard {
                            pos: [i as i32, j],
                            dir: East,
                        }),
                        'v' => blizzards.push(Blizzard {
                            pos: [i as i32, j],
                            dir: South,
                        }),
                        '<' => blizzards.push(Blizzard {
                            pos: [i as i32, j],
                            dir: West,
                        }),
                        _ => panic!("illegal map character"),
                    }
                }
            }
            valley[1] += 1;
        }
        let mut occ = HashSet::new();
        for b in blizzards.iter() {
            occ.insert(b.pos);
        }
        states.insert(
            0,
            State {
                blizzards: blizzards.clone(),
                occupied: occ,
                t: 0,
            },
        );
        Map {
            blizzards,
            valley,
            start,
            end,
            states,
        }
    }
}

fn shortest_path_through_spacetime(map: &mut Map, goals: &[SCoord]) -> usize {
    // https://en.wikipedia.org/wiki/A*_search_algorithm
    let longest = map.longest();
    let mut open = DoublePriorityQueue::new();
    let current = [map.start[0], map.start[1], 0];
    let mut current_goal = 0;
    if current_goal < goals.len() {
        map.end = goals[current_goal];
    }
    open.push(current, map.h(current));

    let mut g_score = HashMap::new();
    g_score.insert(current, 0usize);

    let mut from = HashMap::new();
    while !open.is_empty() {
        let (current, _) = open.pop_min().expect("while says it's not empty");
        if [current[0], current[1]] == map.end {
            current_goal += 1;
            if current_goal >= goals.len() {
                // unwind
                let mut path = Vec::new();
                let mut p = current;
                while let Some(q) = from.get(&p) {
                    path.push(*q);
                    p = *q;
                }
                path.reverse();
                println!("best path: {path:?}");
                return path.len();
            } else {
                // the pricess is in another castle
                map.start = map.end;
                map.end = goals[current_goal];

                g_score = HashMap::new();
                g_score.insert(current, 0usize);
                open = DoublePriorityQueue::new();
            }
        }
        for offset in [[1, 0, 1], [0, 1, 1], [0, -1, 1], [-1, 0, 1], [0, 0, 1]] {
            if let Some(neighbor) = map.can_step(current, offset) {
                let tentative_g_score = 1 + g_score.get(&current).unwrap_or(&longest);
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&longest) {
                    from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    open.push(neighbor, tentative_g_score + map.h(neighbor));
                }
            }
        }
    }
    usize::MAX
}

fn main() {
    let input: &str = &fs::read_to_string("input/024.txt").expect("file read error");
    let mut map = Map::from(input);
    println!("start at {:?}", map.start);
    println!("end at {:?}", map.end);
    println!("valley is {:?} size", map.valley);
    println!("there are {:?} blizzards", map.blizzards.len());
    println!(
        "shortest path is: {}",
        shortest_path_through_spacetime(&mut map, &[])
    );
    let mut map = Map::from(input);
    let goals = [map.end, map.start, map.end];
    println!(
        "there and back again: {}",
        shortest_path_through_spacetime(&mut map, &goals)
    );
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
        let map = Map::from(SAMPLE);
        assert_eq!(map.start, [1, 0], "wrong start");
        assert_eq!(map.end, [6, 5], "wrong end");
        assert_eq!(map.valley, [8, 6], "wrong valley size");
        assert_eq!(map.blizzards.len(), 19);
        assert_eq!(map.blizzards[0].pos, [1, 1]);
        assert_eq!(map.blizzards[0].dir, East);
        assert_eq!(map.blizzards[9].pos, [2, 3]);
        assert_eq!(map.blizzards[9].dir, South);
        assert_eq!(map.blizzards[2].pos, [4, 1]);
        assert_eq!(map.blizzards[2].dir, West);
        assert_eq!(map.blizzards[3].pos, [5, 1]);
        assert_eq!(map.blizzards[3].dir, North);
        assert_eq!(map.blizzards[18].pos, [6, 4]);
        assert_eq!(map.blizzards[18].dir, East);
    }
    #[test]
    fn test_at_t_0() {
        let mut map = Map::from(SAMPLE);
        let state = map.at_time(0);
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
    #[test]
    fn test_at_t_1() {
        let mut map = Map::from(SAMPLE);
        let state = map.at_time(1);
        assert_eq!(state.blizzards.len(), 19);
        // straight
        assert_eq!(state.blizzards[0].pos, [2, 1]); // East
        assert_eq!(state.blizzards[9].pos, [2, 4]); // South
        assert_eq!(state.blizzards[2].pos, [3, 1]); // West
        assert_eq!(state.blizzards[14].pos, [2, 3]); // North
                                                     // wrapped
        assert_eq!(state.blizzards[12].pos, [1, 3]); // East
        assert_eq!(state.blizzards[15].pos, [3, 1]); // South
        assert_eq!(state.blizzards[13].pos, [6, 4]); // West
        assert_eq!(state.blizzards[3].pos, [5, 4]); // North
    }
    #[test]
    fn test_at_t_5() {
        let mut map = Map::from(SAMPLE);
        for t in 0..5 {
            map.at_time(t);
        }
        let state = map.at_time(5);
        assert!(state.occupied.contains(&[1, 1]));
    }
    #[test]
    fn test_path_finder() {
        let mut map = Map::from(SAMPLE);
        assert_eq!(shortest_path_through_spacetime(&mut map, &[]), 18);
    }
    #[test]
    fn test_multipath() {
        let mut map = Map::from(SAMPLE);
        let goals = [map.end, map.start, map.end];
        assert_eq!(shortest_path_through_spacetime(&mut map, &goals), 54);
    }
}
