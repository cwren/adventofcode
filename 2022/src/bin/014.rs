use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::ops;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,   
}

struct Cave {
    rock: HashSet<Pos>,
    bottom: i32,
    limitless: bool
}

struct Path {
    intersections: Vec<Pos>
}

impl Pos {
    fn from(x: i32, y: i32) -> Self {
        Pos {x, y}
    }
}

impl ops::Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Pos {
        Pos::from(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl From<&str> for Path {
    fn from(line: &str) -> Self {
        let mut intersections = Vec::new();
        let points = line.split(" -> ");
        for point in points {
            let mut coords = point.split(',');
            intersections.push(Pos::from(
                coords.next().expect("too few coordinates in a point").parse().expect("unable to parse integer"),
                coords.next().expect("too few coordinates in a point").parse().expect("unable to parse integer"),
            ));
        }
        Path { intersections }
    }
}

impl Cave {
    fn from(paths: &Vec<Path>) -> Self {
        let mut bottom = 0;
        let mut rock = HashSet::new();
        for path in paths {
            for (start, end) in path.intersections.iter().tuple_windows() {
                let delta = Pos::from(
                    (end.x - start.x).signum(),
                    (end.y - start.y).signum(),
                );
                if delta.x.abs() == 1 && delta.y.abs() == 1 {
                    panic!("diagonal path");
                }
                let mut p = *start;
                while p != *end {
                    rock.insert(p);
                    p += delta;
                    bottom = bottom.max(p.y);
                }
                rock.insert(p);
                bottom = bottom.max(p.y);
            }
        }
        Cave { rock, bottom, limitless: true}
    }

    fn assume_hard_floor(&mut self, gap: i32) {
        self.bottom += gap - 1;
        self.limitless = false;
    }

    fn fill(&mut self, start: &Pos) -> usize {
        let mut n = 0;
        while self.drop_grain(start).is_some() {
            n += 1;
        }
        n
    }

    fn drop_grain(&mut self, start: &Pos) -> Option<Pos> {
        if self.rock.contains(start) {
            return None;
        }
        let mut p = *start;
        while p.y < self.bottom {
            for delta in [Some(Pos::from(0, 1)), Some(Pos::from(-1, 1)), Some(Pos::from(1, 1)), None].iter() {
                match delta {
                    Some(d) => {
                        let q = p + d;
                        if !self.rock.contains(&q) {
                            p = q;
                            break;
                        }
                    },
                    None => {
                        self.rock.insert(p);
                        return Some(p);
                    },
                }
            }
        }
        if self.limitless {
            None
        } else {
            self.rock.insert(p);
            Some(p)
        }
    }
}


fn main() {
    let input = fs::read_to_string("input/014.txt").expect("file read error");
    let scan: Vec<Path> = input.lines().map(Path::from).collect();
    println!("there are {} scans", scan.len());
    let mut cave = Cave::from(&scan);
    println!("the infinite cave held {} grains of sand", cave.fill(&Pos::from(500, 0)));

    let mut cave = Cave::from(&scan);
    cave.assume_hard_floor(2);
    println!("the finite cave held {} grains of sand", cave.fill(&Pos::from(500, 0)));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_from() {
        let scan: Vec<Path> = SAMPLE.lines().map(Path::from).collect();
        assert_eq!(scan.len(), 2);
        assert_eq!(scan[0].intersections.len(), 3);
        assert_eq!(scan[1].intersections.len(), 4);
        assert_eq!(scan[1].intersections[2], Pos::from(502, 9));
    }

    #[test]
    fn test_build_cave() {
        let scan: Vec<Path> = SAMPLE.lines().map(Path::from).collect();
        let cave = Cave::from(&scan);
        assert!(cave.rock.contains(&Pos::from(496, 6)));
        assert!(cave.rock.contains(&Pos::from(498, 4)));
        assert!(cave.rock.contains(&Pos::from(502, 7)));
        assert!(cave.rock.contains(&Pos::from(503, 4)));
        assert!(!cave.rock.contains(&Pos::from(500, 6)));
        assert_eq!(cave.rock.len(), 20);
        assert_eq!(cave.bottom, 9);
    }

    #[test]
    fn test_drop_grain() {
        let scan: Vec<Path> = SAMPLE.lines().map(Path::from).collect();
        let mut cave = Cave::from(&scan);
        assert_eq!(cave.drop_grain(&Pos::from(500, 0)), Some(Pos::from(500, 8)));
        assert_eq!(cave.drop_grain(&Pos::from(500, 0)), Some(Pos::from(499, 8)));
        for _ in 2..4 { cave.drop_grain(&Pos::from(500, 0)); }
        assert_eq!(cave.drop_grain(&Pos::from(500, 0)), Some(Pos::from(498, 8)));
        for _ in 5..21 { cave.drop_grain(&Pos::from(500, 0)); }
        assert_eq!(cave.drop_grain(&Pos::from(500, 0)), Some(Pos::from(500, 2)));
        assert!(cave.drop_grain(&Pos::from(500, 0)).is_some());
        assert!(cave.drop_grain(&Pos::from(500, 0)).is_some());
        assert!(cave.drop_grain(&Pos::from(500, 0)).is_none());
    }

    #[test]
    fn test_fill_cave() {
        let scan: Vec<Path> = SAMPLE.lines().map(Path::from).collect();
        let mut cave = Cave::from(&scan);
        assert_eq!(cave.fill(&Pos::from(500, 0)), 24);
    }

    #[test]
    fn test_fill_finite_cave() {
        let scan: Vec<Path> = SAMPLE.lines().map(Path::from).collect();
        let mut cave = Cave::from(&scan);
        cave.assume_hard_floor(2);
        assert_eq!(cave.fill(&Pos::from(500, 0)), 93);
    }
}
