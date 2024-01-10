use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use lazy_static::lazy_static;

type Coord = (usize, usize);

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
    static ref TILE_MAP: HashMap<char, Tile> =
    HashMap::from([
        ('|', Tile::NS),
        ('-', Tile::EW),
        ('L', Tile::NE),
        ('J', Tile::NW),
        ('F', Tile::SE),
        ('7', Tile::SW),
        ('S', Tile::START),
        ('.', Tile::NONE),
    ]);
    static ref SOUTHERLY: Vec<Tile> = Vec::from(vec![Tile::SW, Tile::SE, Tile::NS]);
    static ref NORTHERLY: Vec<Tile> = Vec::from(vec![Tile::NW, Tile::NE, Tile::NS]);
    static ref EASTERLY: Vec<Tile> = Vec::from(vec![Tile::NE, Tile::SE, Tile::EW]);
    static ref WESTERLY: Vec<Tile> = Vec::from(vec![Tile::SW, Tile::NW, Tile::EW]);
}
struct Map { 
    tiles: Vec<Vec<Tile>>,
    start: Coord,
    size: Coord,
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
        let mut start = (0, 0);
        let mut size = (0, 0);
        for (j,  row) in tiles.iter().enumerate() {
            size.1 = size.1.max(j + 1);
            for (i,  t) in row.iter().enumerate() {
                size.0 = size.0.max(i + 1);
                if *t == Tile::START {
                    start = (i, j);
                }
            }
        }
        Map { tiles, start, size }
    }
}

impl Map {
    fn find_start(&self) -> Option<(Dir, Coord)> {
        let here = self.start;
        // Go North
        if here.1 > 0 && SOUTHERLY.contains(&self.tiles[here.1 - 1][here.0]) {
            return Some((Dir::SOUTH, (here.0, here.1 - 1)));
        }
        // Go East
        if here.0 < self.size.0 && WESTERLY.contains(&self.tiles[here.1][here.0 + 1]) {
            return Some((Dir::WEST, (here.0 + 1, here.1)));
        }
        // Go South
        if here.1 < self.size.1 && NORTHERLY.contains(&self.tiles[here.1 + 1][here.0]) {
            return Some((Dir::NORTH, (here.0, here.1 + 1)));
        }
        // Go West
        if here.0 > 0 && EASTERLY.contains(&self.tiles[here.1][here.0 - 1]) {
            return Some((Dir::EAST, (here.0 - 1, here.1)));
        }
        None
    }

    fn walk_loop(self) -> Vec<Coord> {
        let mut path = Vec::new();
        path.push(self.start);
        let (mut from, mut here) = self.find_start().expect("there was no path out of start");
        while here != self.start {
            path.push(here);
            let tile = self.tiles[here.1][here.0];
            (from, here) = match from {
                Dir::NORTH => { 
                    match tile {
                        Tile::NE => (Dir::WEST, (here.0 + 1, here.1)),
                        Tile::NS => (Dir::NORTH, (here.0, here.1 + 1)),
                        Tile::NW => (Dir::EAST, (here.0 - 1, here.1)),
                        _ => panic!("pipes do not match"),
                    }
                },
                Dir::SOUTH => { 
                    match tile {
                        Tile::SE => (Dir::WEST, (here.0 + 1, here.1)),
                        Tile::NS => (Dir::SOUTH, (here.0, here.1 - 1)),
                        Tile::SW => (Dir::EAST, (here.0 - 1, here.1)),
                        _ => panic!("pipes do not match"),
                    }
                },
                Dir::EAST => { 
                    match tile {
                        Tile::NE => (Dir::SOUTH, (here.0, here.1 - 1)),
                        Tile::EW => (Dir::EAST, (here.0 - 1, here.1)),
                        Tile::SE => (Dir::NORTH, (here.0, here.1 + 1)),
                        _ => panic!("pipes do not match"),
                    }
                },
                Dir::WEST => { 
                    match tile {
                        Tile::NW => (Dir::SOUTH, (here.0, here.1 - 1)),
                        Tile::EW => (Dir::WEST, (here.0 + 1, here.1)),
                        Tile::SW => (Dir::NORTH, (here.0, here.1 + 1)),
                        _ => panic!("pipes do not match"),
                    }
                },
            }
        }
        path
    }
}
fn main() {
    let f = File::open("input/010.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let map = Map::from(lines);
    let path = map.walk_loop();
    println!("half the loop is {}", path.len() / 2)
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
}
