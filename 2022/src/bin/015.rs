use regex::Regex;
use lazy_static::lazy_static;
use std::fs;
use vecmath::Vector2;
use itertools::Itertools;

lazy_static! {
    static ref RE: regex::Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

type Coord = Vector2<i32>;

#[derive(Debug)]
struct Sensor {
    p: Coord,
    b: Coord,
    r: i32,
}

fn manhattan(a: Coord, b: Coord) -> i32 {
    (a[0]-b[0]).abs() + (a[1]-b[1]).abs()
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        match RE.captures(s) {
            Some(cap) => {
                let p = [ 
                    cap.get(1).expect("too few numbers").as_str().parse::<i32>().expect("not a number"),
                    cap.get(2).expect("too few numbers").as_str().parse::<i32>().expect("not a number")
                ];
                let b = [
                    cap.get(3).expect("too few numbers").as_str().parse::<i32>().expect("not a number"),
                    cap.get(4).expect("too few numbers").as_str().parse::<i32>().expect("not a number")
                ];
                Sensor { p, b, r: manhattan(p, b) }
            }
            None => panic!("unparsable input: {}", s),
        }
    }
}

impl Sensor {
    fn coverage_at(&self, row: i32) -> Option<Span> {
        let h = (self.p[1] - row).abs();
        if h > self.r {
            return None
        }
        Some(Span {
            s: self.p[0] - (self.r - h),
            e: self.p[0] + (self.r - h),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Span {
    s: i32,
    e: i32,
}

impl Span {
    fn overlaps(&self, o: &Span) -> bool {
        (self.s <= o.s && self.e >= o.s) ||
        (self.s <= o.e && self.e >= o.e) | 
        (self.s <= o.e && self.e >= o.s)
    }

    fn union(&self, other: &Span) -> Result<Span, &str> {
        match self.overlaps(other) {
            true => Ok(
                Span {
                    s: self.s.min(other.s),
                    e: self.e.max(other.e),
                }
            ),
            false => Err("spans do not overlap"),
        }
    }
    fn intersect(&self, other: &Span) -> Option<Span> {
        match self.overlaps(other) {
            true => Some(
                Span {
                    s: self.s.max(other.s),
                    e: self.e.min(other.e),
                }
            ),
            false =>None,
        }
    }

    fn len(&self) -> usize {
        (self.e - self.s + 1) as usize
    }
}

fn simplify(mut spans: Vec<Span>) -> Vec<Span> {
    loop {
        let mut a_idx = 0;
        let mut b_idx = 0;
        let mut found = false;
        'outer_for: for (i, a) in spans.iter().enumerate() {
            for (j, b) in spans.iter().enumerate().skip(i + 1) {
                if a.overlaps(b) {
                    a_idx = i;
                    b_idx = j;
                    found = true;
                    break 'outer_for;
                }
            }
        }
        if !found {
            break;
        } else {
            let b = spans.remove(b_idx);
            let a = spans.remove(a_idx);
            spans.push(a.union(&b).expect("checked for overlap above"));
        }
    }
    spans.sort();
    spans
}

fn spans_at(sensors: &[Sensor], row: i32) -> Vec<Span> {
    let mut spans = Vec::new();
    for sensor in sensors {
        if let Some(span) = sensor.coverage_at(row) {
            spans.push(span);
        }
    }
    simplify(spans)
}

fn covered_area_at(sensors: &[Sensor], row: i32) -> usize {
    let spans = spans_at(sensors, row);
    let total_area = spans.iter().map(Span::len).sum::<usize>();
    let beacons_on_row = sensors.iter().filter(|s| s.b[1] == row).map(|s| s.b).unique().count();
    total_area - beacons_on_row
}

fn holes_on_row(sensors: &[Sensor], row: i32, window: &Span) -> Option<Coord> {
    let spans = spans_at(sensors, row);
    let mut windowed = Vec::new();
    for span in spans.iter() {
        if let Some(intersection) = window.intersect(span) {
            windowed.push(intersection);
        }
    }
    windowed = simplify(windowed);
    if windowed.len() == 1 {
        let span = spans.get(0).expect("windowed.len() > 1");
        if span.s > window.s { 
            return Some([window.s, row]);
        } else if span.e < window.e {
            return Some([span.e + 1, row]);
        } else {
            return None;
        }    
    }
    let mut spans = windowed.iter();
    let a = spans.next().expect("windowed.len() > 1");
    Some([a.e + 1, row])
}

fn find_frequency(sensors: &[Sensor], window: &Span) -> Option<i64> {
    for y in window.s..(window.e + 1) {
        if let Some(hole) = holes_on_row(sensors, y, window) {
            return Some(4_000_000 * hole[0] as i64 + hole[1] as i64);
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input/015.txt").expect("file read error");
    let sensors: Vec<Sensor> = input.lines().map(Sensor::from).collect();
    println!("there are {} sensors", sensors.len());
    
    println!("there are {} excluded locations", covered_area_at(&sensors, 2_000_000));
    println!("The tuning frequency is {}", find_frequency(&sensors, &Span {s:0, e:4_000_000}).unwrap_or(0));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

#[test]
fn test_regex() {
    let mut input = SAMPLE.lines();
    assert_eq!(RE.is_match("$ cd /"), false);
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let cap = RE
        .captures(input.next().unwrap())
        .unwrap();
    assert_eq!(cap.get(1).unwrap().as_str().parse::<i32>().unwrap(), 2);
    assert_eq!(cap.get(2).unwrap().as_str().parse::<i32>().unwrap(), 18);
    assert_eq!(cap.get(3).unwrap().as_str().parse::<i32>().unwrap(), -2);
    assert_eq!(cap.get(4).unwrap().as_str().parse::<i32>().unwrap(), 15);
    }

    #[test]
    fn test_parse() {
        let sensors: Vec<Sensor> = SAMPLE.lines().map(Sensor::from).collect();
        assert_eq!(sensors[0].p, [2, 18]);
        assert_eq!(sensors[0].b, [-2, 15]);
        assert_eq!(sensors[1].p, [9, 16]);
        assert_eq!(sensors[1].b, [10, 16]);
        assert_eq!(sensors[6].p, [8, 7]);
        assert_eq!(sensors[6].r, 9);
    }

    #[test]
    fn test_coverage_at() {
        let sensors: Vec<Sensor> = SAMPLE.lines().map(Sensor::from).collect();
        assert_eq!(sensors[6].coverage_at(10), Some(Span{ s:2, e: 14 }));
        assert_eq!(sensors[6].coverage_at(16), Some(Span{ s:8, e: 8 }));
        assert_eq!(sensors[6].coverage_at(17), None);
        assert_eq!(sensors[6].coverage_at(4), Some(Span{ s:2, e: 14 }));
        assert_eq!(sensors[6].coverage_at(-2), Some(Span{ s:8, e: 8 }));
        assert_eq!(sensors[6].coverage_at(-3), None);
    }

    #[test]
    fn test_union() {
        assert!(Span{ s:2, e:14 }.union(&Span {s: 20, e:24}).is_err());
        assert_eq!(Span{ s:2, e:14 }.union(&Span {s: 8, e:24}), Ok(Span{ s:2, e:24 }));
        assert_eq!(Span{ s:8, e:24 }.union(&Span {s: 2, e:14}), Ok(Span{ s:2, e:24 }));
    }

    #[test]
    fn test_intersect() {
        assert!(Span{ s:2, e:14 }.intersect(&Span {s: 20, e:24}).is_none());
        assert_eq!(Span{ s:2, e:14 }.intersect(&Span {s: 8, e:24}), Some(Span{ s:8, e:14 }));
        assert_eq!(Span{ s:8, e:24 }.intersect(&Span {s: 2, e:14}), Some(Span{ s:8, e:14 }));
    }

    #[test]
    fn test_simplify() {
        assert_eq!(simplify(vec![Span{ s:2, e:14 }, Span {s: 20, e:24}]), vec![Span{ s:2, e:14 }, Span {s: 20, e:24}]);
        assert_eq!(simplify(vec![Span{ s:2, e:14 }, Span {s: 8, e:24}]), vec![Span{ s:2, e:24 }]);
        assert_eq!(simplify(vec![Span{ s:8, e:24 }, Span {s: 2, e:14}]), vec![Span{ s:2, e:24}]);
    }
    
    #[test]
    fn test_covered_area_at() {
        let sensors: Vec<Sensor> = SAMPLE.lines().map(Sensor::from).collect();
        assert_eq!(covered_area_at(&sensors, 10), 26);
    }
    
    #[test]
    fn test_holes_on_row() {
        let sensors: Vec<Sensor> = SAMPLE.lines().map(Sensor::from).collect();
        assert_eq!(holes_on_row(&sensors, 9, &Span {s:0, e:20}), None);
        assert_eq!(holes_on_row(&sensors, 10, &Span {s:0, e:20}), None);
        assert_eq!(holes_on_row(&sensors, 11, &Span {s:0, e:20}), Some([14, 11]));
        assert_eq!(holes_on_row(&sensors, 10, &Span {s:0, e:25}), Some([25, 10]));
        assert_eq!(holes_on_row(&sensors, 9, &Span {s:0, e:25}), Some([24, 9]));
    }
    
    #[test]
    fn test_find_frequency() {
        let sensors: Vec<Sensor> = SAMPLE.lines().map(Sensor::from).collect();
        assert_eq!(find_frequency(&sensors, &Span {s:0, e:20}), Some(56000011));
    }
}