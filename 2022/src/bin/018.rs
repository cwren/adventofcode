use std::collections::{HashMap, HashSet};
use std::fs;
use vecmath::{vec3_add, Vector3};

type int = i8;
type Key = u64;
type Coord = Vector3<int>;
const CARDINALS: [[int; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];
#[derive(Debug, PartialEq, Clone)]
struct Cube {
    pos: Coord,
    key: Key,
}

struct Cubes {
    store: HashMap<Key, Cube>,
    known_outside: HashSet<Key>,
    lb: Coord,
    ub: Coord,
}
impl Cube {
    fn key(c: &Coord) -> Key {
        ((c[0] as Key) << 16) + ((c[1] as Key) << 8) + (c[2] as Key)
    }

    fn valid(c: &Coord) -> bool {
        c[0] >= 0
            && c[0] <= int::MAX
            && c[1] >= 0
            && c[1] <= int::MAX
            && c[2] >= 0
            && c[2] <= int::MAX
    }
}

impl From<&str> for Cube {
    fn from(input: &str) -> Self {
        let pos = Coord::from(
            input
                .split(',')
                .map(str::parse::<int>)
                .map(Result::unwrap)
                .collect::<Vec<int>>()
                .try_into()
                .unwrap(),
        );
        Cube::new(pos)
    }
}

impl Cube {
    fn new(pos: Coord) -> Self {
        Cube {
            pos,
            key: Self::key(&pos),
        }
    }
}

impl From<&str> for Cubes {
    fn from(input: &str) -> Self {
        let mut store = HashMap::new();
        store.extend(input.lines().map(Cube::from).map(|c| (c.key, c)));
        let mut ub = [0, 0, 0];
        let mut lb = [int::MAX, int::MAX, int::MAX];
        for (_, cube) in store.iter() {
            lb = min(&lb, &cube.pos);
            ub = max(&ub, &cube.pos);
        }
        Cubes {
            store,
            lb,
            ub,
            known_outside: HashSet::new(),
        }
    }
}

fn max(a: &Coord, b: &Coord) -> Coord {
    [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
}

fn min(a: &Coord, b: &Coord) -> Coord {
    [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]
}

impl Cubes {
    fn len(&self) -> usize {
        self.store.len()
    }

    fn insert(&mut self, c: Cube) {
        self.store.insert(c.key, c);
    }

    fn contains(&self, c: &Coord) -> bool {
        self.store.contains_key(&Cube::key(c))
    }

    fn count_exposed(&self) -> usize {
        let mut total = 0;
        for (_, cube) in self.store.iter() {
            for delta in CARDINALS {
                let probe = vec3_add(cube.pos, delta);
                if !Cube::valid(&probe) || !self.store.contains_key(&Cube::key(&probe)) {
                    total += 1;
                }
            }
        }
        total
    }

    fn fill_holes(&mut self) {
        let mut found_hole = true;
        while found_hole {
            found_hole = false;
            'outer: for x in self.lb[0]..self.ub[0] {
                for y in self.lb[1]..self.ub[1] {
                    for z in self.lb[2]..self.ub[2] {
                        let probe = Cube::new([x, y, z]);
                        if !self.contains(&probe.pos) {
                            if self.trivially_outside(&probe.pos) {
                                self.known_outside.insert(probe.key);
                            } else {
                                match self.flood(&probe) {
                                    Some(void) => {
                                        for q in void {
                                            self.insert(q);
                                        }
                                        found_hole = true;
                                        break 'outer;
                                    }
                                    None => self.known_outside.insert(probe.key),
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    fn flood(&self, seed: &Cube) -> Option<Vec<Cube>> {
        if self.contains(&seed.pos) {
            return None;
        }
        if self.trivially_outside(&seed.pos) {
            return None;
        }
        let mut void = Vec::new();
        void.push(seed.clone());
        let mut frontier = Vec::new();
        frontier.push(seed.clone());
        loop {
            let mut found = false;
            let mut candidates = Vec::new();
            candidates.extend(frontier.drain(..));
            for candidate in candidates.drain(..) {
                for delta in CARDINALS {
                    let probe = Cube::new(vec3_add(seed.pos, delta));
                    if !self.store.contains_key(&probe.key) && !void.contains(&probe) {
                        if self.trivially_outside(&probe.pos) {
                            return None;
                        } else {
                            frontier.push(probe.clone());
                            void.push(probe);
                            found = true;
                        }
                    }
                }
            }
            if !found {
                break;
            }
        }
        Some(void)
    }

    fn trivially_outside(&self, z: &Coord) -> bool {
        // trivially outside
        if self.known_outside.contains(&Cube::key(&z)) {
            return true;
        }
        if z[0] <= self.lb[0] || z[1] <= self.lb[1] || z[2] <= self.lb[2] {
            return true;
        }
        if z[0] >= self.ub[0] || z[1] >= self.ub[1] || z[2] >= self.ub[2] {
            return true;
        }
        for dim in 0..3 {
            let clear = false;
            let mut delta = [0, 0, 0];
            delta[dim] = -1;
            let mut probe = vec3_add(*z, delta);
            let mut clear_down = true;
            while probe[dim] >= self.lb[dim] {
                if self.contains(&probe) {
                    clear_down = false;
                }
                probe = vec3_add(probe, delta);
            }
            delta[dim] = 1;
            let mut probe = vec3_add(*z, delta);
            let mut clear_up = true;
            while probe[dim] <= self.ub[dim] {
                if self.contains(&probe) {
                    clear_up = false;
                }
                probe = vec3_add(probe, delta);
            }
            if clear_up || clear_down {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/018.txt").expect("file read error");
    let mut cubes: Cubes = Cubes::from(input);
    println!("there are {} cubes", cubes.len());
    println!("there are {} expose sides", cubes.count_exposed());
    cubes.fill_holes();
    println!(
        "there are {} expose sides, ignoring internal pockets",
        cubes.count_exposed()
    );
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

    #[test]
    fn test_cubes_bounds() {
        let cubes: Cubes = Cubes::from(SAMPLE);
        assert_eq!(cubes.lb, [1, 1, 1]);
        assert_eq!(cubes.ub, [3, 3, 6]);
    }

    #[test]
    fn test_cubes_outside() {
        let cubes: Cubes = Cubes::from(SAMPLE);
        assert!(cubes.trivially_outside(&[1, 1, 1]));
        assert!(!cubes.trivially_outside(&[2, 2, 5]));
        assert!(cubes.trivially_outside(&[2, 4, 5]));
    }

    #[test]
    fn test_cubes_flood() {
        let cubes: Cubes = Cubes::from(SAMPLE);
        assert!(cubes.flood(&Cube::new([1, 1, 1])).is_none());
        assert!(cubes.flood(&Cube::new([2, 4, 5])).is_none());
        assert!(cubes.flood(&Cube::new([2, 2, 5])).is_some());
        assert_eq!(
            cubes.flood(&Cube::new([2, 2, 5])).unwrap(),
            [Cube::new([2, 2, 5])]
        );
    }

    const BIG_VOID: &str = r#"1,1,1
2,1,2
1,2,2
2,2,1
3,2,2
2,2,3
1,3,2
2,3,1
2,4,2
3,3,2
2,3,3"#;

    #[test]
    fn test_cubes_big_flood() {
        let cubes: Cubes = Cubes::from(BIG_VOID);
        let res = cubes.flood(&Cube::new([2, 2, 2]));
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.len(), 2);
        assert!(res.contains(&Cube::new([2, 2, 2])));
        assert!(res.contains(&Cube::new([2, 3, 2])));
    }

    #[test]
    fn test_cubes_fill_holes() {
        let mut cubes: Cubes = Cubes::from(SAMPLE);
        assert!(!cubes.contains(&[2, 2, 5]), "this should be a hole");
        cubes.fill_holes();
        assert!(cubes.contains(&[2, 2, 5]), "this hole should be filled");
    }

    #[test]
    fn test_count_exposed_ignore_voids() {
        let mut cubes: Cubes = Cubes::from(SAMPLE);
        cubes.fill_holes();
        assert_eq!(cubes.count_exposed(), 58);
    }
}
