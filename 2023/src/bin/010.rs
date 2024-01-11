use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter;

type Coord = (i32, i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    NONE,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    START,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

lazy_static! {
    static ref TILE_MAP: HashMap<char, Tile> = HashMap::from([
        ('|', Tile::NS),
        ('-', Tile::EW),
        ('L', Tile::NE),
        ('J', Tile::NW),
        ('F', Tile::SE),
        ('7', Tile::SW),
        ('S', Tile::START),
        ('.', Tile::NONE),
    ]);
    static ref SOUTHERLY: Vec<Tile> = vec![Tile::SW, Tile::SE, Tile::NS];
    static ref NORTHERLY: Vec<Tile> = vec![Tile::NW, Tile::NE, Tile::NS];
    static ref EASTERLY: Vec<Tile> = vec![Tile::NE, Tile::SE, Tile::EW];
    static ref WESTERLY: Vec<Tile> = vec![Tile::SW, Tile::NW, Tile::EW];
}
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Coord,
    size: Coord,
}

trait Indexable<T> {
    fn get(&self, i: i32, j: i32) -> T;
    fn safe_get(&self, i: i32, j: i32) -> Option<T>;
    fn put(&mut self, i: i32, j: i32, v: T);
}

impl Indexable<Tile> for Vec<Vec<Tile>> {
    fn get(&self, i: i32, j: i32) -> Tile {
        self.safe_get(i, j).unwrap()
    }

    fn put(&mut self, i: i32, j: i32, v: Tile) {
        self[j as usize][i as usize] = v;
    }

    fn safe_get(&self, i: i32, j: i32) -> Option<Tile> {
        let i = i as usize;
        let j = j as usize;
        if j < self.len() {
            let row = &self[j];
            if i < row.len() {
                return Some(row[i]);
            }
        }
        None
    }
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let tiles: Vec<Vec<Tile>> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| *TILE_MAP.get(&c).expect("unrecognized letter"))
                    .collect()
            })
            .collect();
        let mut start = (0i32, 0i32);
        let mut size = (0i32, 0i32);
        for (j, row) in tiles.iter().enumerate() {
            size.1 = size.1.max(j as i32 + 1);
            for (i, t) in row.iter().enumerate() {
                size.0 = size.0.max(i as i32 + 1);
                if *t == Tile::START {
                    start = (i as i32, j as i32);
                }
            }
        }
        // assumed square
        assert!(tiles
            .iter()
            .map(|row| row.len() as i32 == size.0)
            .all(|b| b));
        Map { tiles, start, size }
    }
}

impl Map {
    fn find_start(&self) -> Option<(Dir, Coord)> {
        let here = self.start;
        // Go North
        if here.1 > 0 && SOUTHERLY.contains(&self.tiles.get(here.0, here.1 - 1)) {
            return Some((Dir::SOUTH, (here.0, here.1 - 1)));
        }
        // Go East
        if here.0 < self.size.0 && WESTERLY.contains(&self.tiles.get(here.0 + 1, here.1)) {
            return Some((Dir::WEST, (here.0 + 1, here.1)));
        }
        // Go South
        if here.1 < self.size.1 && NORTHERLY.contains(&self.tiles.get(here.0, here.1 + 1)) {
            return Some((Dir::NORTH, (here.0, here.1 + 1)));
        }
        // Go West
        if here.0 > 0 && EASTERLY.contains(&self.tiles.get(here.0 - 1, here.1)) {
            return Some((Dir::EAST, (here.0 - 1, here.1)));
        }
        None
    }

    fn walk_loop(&self) -> Vec<Coord> {
        let mut path = Vec::new();
        path.push(self.start);
        let (mut from, mut here) = self.find_start().expect("there was no path out of start");
        while here != self.start {
            path.push(here);
            let tile = self.tiles.get(here.0, here.1);
            (from, here) = match from {
                Dir::NORTH => match tile {
                    Tile::NE => (Dir::WEST, (here.0 + 1, here.1)),
                    Tile::NS => (Dir::NORTH, (here.0, here.1 + 1)),
                    Tile::NW => (Dir::EAST, (here.0 - 1, here.1)),
                    _ => panic!("pipes do not match"),
                },
                Dir::SOUTH => match tile {
                    Tile::SE => (Dir::WEST, (here.0 + 1, here.1)),
                    Tile::NS => (Dir::SOUTH, (here.0, here.1 - 1)),
                    Tile::SW => (Dir::EAST, (here.0 - 1, here.1)),
                    _ => panic!("pipes do not match"),
                },
                Dir::EAST => match tile {
                    Tile::NE => (Dir::SOUTH, (here.0, here.1 - 1)),
                    Tile::EW => (Dir::EAST, (here.0 - 1, here.1)),
                    Tile::SE => (Dir::NORTH, (here.0, here.1 + 1)),
                    _ => panic!("pipes do not match"),
                },
                Dir::WEST => match tile {
                    Tile::NW => (Dir::SOUTH, (here.0, here.1 - 1)),
                    Tile::EW => (Dir::WEST, (here.0 + 1, here.1)),
                    Tile::SW => (Dir::NORTH, (here.0, here.1 + 1)),
                    _ => panic!("pipes do not match"),
                },
            }
        }
        path
    }

    fn resolve_start(&mut self, path: &Vec<Coord>) {
        let first = path[1];
        let last = path.last().expect("path should be longer than 0");
        let a = (first.0 - self.start.0, first.1 - self.start.1);
        let b = (last.0 - self.start.0, last.1 - self.start.1);
        let start_tile: Tile;
        if a.0 == 0 && b.0 == 0 {
            start_tile = Tile::NS;
        } else if a.1 == 0 && b.1 == 0 {
            start_tile = Tile::EW;
        } else {
            let d = (a.0 + b.0, a.1 + b.1);
            start_tile = match d {
                (1, -1) => Tile::NE,
                (1, 1) => Tile::SE,
                (-1, 1) => Tile::SW,
                (-1, -1) => Tile::NW,
                _ => panic!("unknown pipe configuration at start"),
            }
        }
        self.tiles.put(self.start.0, self.start.1, start_tile);
    }

    fn score(tile: &Tile) -> usize {
        match tile {
            Tile::NONE => 0,
            Tile::NS => 1,
            Tile::EW => 1,
            Tile::NW => 1,
            Tile::SE => 1,
            Tile::NE => 2, // corner cut
            Tile::SW => 2, // corner cut
            Tile::START => panic!("start should have been cleared by this point"),
        }
    }

    fn contained_in(&self, path: Vec<Coord>) -> Vec<Coord> {
        let mut windings: Vec<Vec<usize>> = iter::repeat_with(|| vec![0; self.size.0 as usize])
            .take(self.size.1 as usize)
            .collect();
        for (j, row) in self.tiles.iter().enumerate() {
            for (i, t) in row.iter().enumerate() {
                windings[j][i] = Map::score(t);
            }
        }
        for i in 0..self.size.0 {
            let mut num_windings = 0;
            for j in 0..self.size.1 {
                let x = i + j;
                let y = j;
                if let Some(tile) = self.tiles.safe_get(x, y) {
                    if path.contains(&(x, y)) {
                        num_windings += Map::score(&tile);
                    }
                    windings[y as usize][x as usize] = num_windings;
                }
            }
        }
        for j in 0..self.size.1 {
            let mut num_windings = 0;
            for i in 0..self.size.0 {
                let x = i;
                let y = i + j;
                if let Some(tile) = self.tiles.safe_get(x, y) {
                    if path.contains(&(x, y)) {
                        num_windings += Map::score(&tile);
                    }
                    windings[y as usize][x as usize] = num_windings;
                }
            }
        }
        let mut contained = Vec::new();
        for j in 0..self.size.1 as usize {
            for i in 0..self.size.0 as usize {
                if windings[j][i] % 2 == 1 && !path.contains(&(i as i32, j as i32)) {
                    contained.push((i as i32, j as i32))
                }
            }
        }
        contained
    }
}
fn main() {
    let f = File::open("input/010.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let mut map = Map::from(lines);
    let path = map.walk_loop();
    println!("half the loop is {}", path.len() / 2);
    map.resolve_start(&path);
    let area = map.contained_in(path).len();
    println!("area contained by loop is {area}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE1: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;
    const SAMPLE2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"#;

    #[test]
    fn test_parse1() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        assert_eq!(map.tiles[0][0], Tile::EW);
        assert_eq!(map.tiles[4][4], Tile::SE);
        assert_eq!(map.tiles[1][1], Tile::START);
        assert_eq!(map.tiles[0][4], Tile::SW);
        assert_eq!(map.start, (1, 1));
        assert_eq!(map.size, (5, 5));
    }

    #[test]
    fn test_parse2() {
        let lines = SAMPLE2.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        assert_eq!(map.start, (0, 2));
        assert_eq!(map.size, (5, 5));
        assert_eq!(map.tiles[2][0], Tile::START);
    }

    #[test]
    fn test_find_start1() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        assert_eq!(map.find_start().unwrap().0, Dir::WEST);
        assert_eq!(map.find_start().unwrap().1, (2, 1));
    }

    #[test]
    fn test_find_start2() {
        let lines = SAMPLE2.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        assert_eq!(map.find_start().unwrap().0, Dir::WEST);
        assert_eq!(map.find_start().unwrap().1, (1, 2));
    }

    #[test]
    fn test_walk_loop1() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        let path = map.walk_loop();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_walk_loop2() {
        let lines = SAMPLE2.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let map = Map::from(lines);
        let path = map.walk_loop();
        assert_eq!(path.len(), 16);
    }

    const SAMPLE3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

    #[test]
    fn test_resolve_start_3() {
        let lines = SAMPLE3.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk_loop();
        map.resolve_start(&path);
        assert_eq!(map.tiles.get(map.start.0, map.start.1), Tile::SE);
    }

    #[test]
    fn test_contain_3() {
        let lines = SAMPLE3.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk_loop();
        map.resolve_start(&path);
        let area = map.contained_in(path).len();
        assert_eq!(area, 4);
    }

    const SAMPLE4: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

    #[test]
    fn test_resolve_start_4() {
        let lines = SAMPLE4.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk_loop();
        map.resolve_start(&path);
        assert_eq!(map.tiles.get(map.start.0, map.start.1), Tile::SE);
    }

    #[test]
    fn test_contain_4() {
        let lines = SAMPLE4.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk_loop();
        map.resolve_start(&path);
        let area = map.contained_in(path).len();
        assert_eq!(area, 8);
    }

    const SAMPLE5: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    #[test]
    fn test_resolve_start_5() {
        let lines = SAMPLE5.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk_loop();
        map.resolve_start(&path);
        assert_eq!(map.tiles.get(map.start.0, map.start.1), Tile::SW);
    }

    #[test]
    fn test_contain_5() {
        let lines = SAMPLE5.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut map = Map::from(lines);
        let path = map.walk_loop();
        map.resolve_start(&path);
        let area = map.contained_in(path).len();
        assert_eq!(area, 10);
    }
}
