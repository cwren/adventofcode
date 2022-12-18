use std::arch::aarch64::int32x2_t;
use std::fs;
use std::collections::{HashMap};
use vecmath::Vector3;

type int = i8;
type Key = u64;
type Coord = Vector3<int>;
const CARDINALS: [[int; 3]; 6]= [
    [-1, 0, 0],
    [ 1, 0, 0],
    [ 0,-1, 0],
    [ 0, 1, 0],
    [ 0, 0,-1],
    [ 0, 0, 1],
];
#[derive(Debug, PartialEq)]
struct Cube {
    pos: Coord,
    key: Key,
}

struct Cubes {
    store: HashMap<Key, Cube>,
}
impl Cube {
    fn key(c: &Coord) -> Key {
        ((c[0] as Key) << 16) + 
        ((c[1] as Key) << 8) +
        (c[2] as Key)
    }

    fn valid(c: &Coord) -> bool {
        c[0] >= 0 && c[0] <= int::MAX &&
        c[1] >= 0 && c[1] <= int::MAX &&
        c[2] >= 0 && c[2] <= int::MAX
    }
}
impl From<&str> for Cube {
    fn from(input: &str) -> Self {
        let pos = Coord::from(
            input.split(',')
            .map(str::parse::<int>)
            .map(Result::unwrap)
            .collect::<Vec<int>>()
            .try_into()
            .unwrap());
        let key = Cube::key(&pos);
        Cube { pos, key }
    }
}

impl From<&str> for Cubes {
    fn from(input: &str) -> Self {
        let mut store = HashMap::new();
        store.extend(input.lines().map(Cube::from).map(|c| (c.key, c)));
        Cubes { store }
    }
}

impl Cubes {
    fn len(&self) -> usize {
        self.store.len()
    }

    fn contains(&self, c: &Coord) -> bool {
        self.store.contains_key(&Cube::key(c))
    }

    fn count_exposed(&self) -> usize {
        let mut total = 0;
        for (_, cube) in self.store.iter() {
            for delta in CARDINALS {
                let probe = vecmath::vec3_add(cube.pos, delta);
                if !Cube::valid(&probe) || !self.store.contains_key(&Cube::key(&probe)) {
                    total += 1;
                }
            }
        }
        total
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/018.txt").expect("file read error");
    let cubes: Cubes = Cubes::from(input);
    println!("there are {} cubes", cubes.len());
    println!("there are {} expose sides", cubes.count_exposed());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn test_parse_cube() {
        let cubes: Vec<Cube> = SAMPLE.lines().map(Cube::from).collect();
        assert_eq!(cubes.len(), 13);
        assert_eq!(cubes[0].pos, [2, 2, 2]);
    }

    #[test]
    fn test_parse() {
        let cubes: Cubes = Cubes::from(SAMPLE);
        assert_eq!(cubes.len(), 13);
        assert!(cubes.contains(&[2, 2, 2]));
        assert!(cubes.contains(&[2, 3, 5]));
        assert!(!cubes.contains(&[2, 3, 6]));
    }

    #[test]
    fn test_count_exposed() {
        let cubes: Cubes = Cubes::from(SAMPLE);
        assert_eq!(cubes.count_exposed(), 64);
    }
}