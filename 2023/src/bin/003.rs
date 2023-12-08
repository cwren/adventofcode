use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::u32;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Number {
    v: u32,
    p: (usize, usize),
    n: usize,
}
trait Schematic {
    fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)>;
}

trait Cog {
    fn is_gear(&self, numbers: &Vec<Number>) -> Option<Gear>;
}

#[derive(Debug)]
struct Gear {
    pos: (usize, usize),
    pieces: HashSet<Number>,
}

impl Schematic for Vec<Vec<char>> {
    fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let i = p.0 as i32;
        let j = p.1 as i32;
        let h = self.len() as i32;
        let w = h; // assume full rectangle
        let mut neighbors = Vec::new();
        for di in -1..2 {
            for dj in -1..2 {
                if (di != 0 || dj != 0) && i + di >= 0 && i + di < h && j + dj >= 0 && j + dj < w {
                    neighbors.push(((i + di) as usize, (j + dj) as usize));
                }
            }
        }
        neighbors
    }
}

impl Number {
    fn adjacent(&self, schematic: &Vec<Vec<char>>) -> bool {
        let mut res = false;
        let i = self.p.0;
        for j in self.p.1..(self.p.1 + self.n) {
            for n in schematic.neighbors((i, j)) {
                let c = schematic[n.0][n.1];
                res |= c != '.' && !c.is_numeric();
            }
        }
        res
    }

    fn covers_any(&self, joins: &Vec<(usize, usize)>) -> bool {
        joins
            .iter()
            .any(|j| self.p.0 == j.0 && (j.1 >= self.p.1 && j.1 < (self.p.1 + self.n)))
    }
}

impl Cog for (usize, usize) {
    fn is_gear(&self, numbers: &Vec<Number>) -> Option<Gear> {
        let i = self.0 as i32;
        let j = self.1 as i32;
        let mut joins = Vec::new();
        for di in -1..2 {
            for dj in -1..2 {
                joins.push(((i + di) as usize, (j + dj) as usize));
            }
        }
        let mut pieces = HashSet::new();
        for piece in numbers {
            if piece.covers_any(&joins) {
                pieces.insert(piece.clone());
            }
        }
        if pieces.len() >= 2 {
            Some(Gear {
                pos: *self,
                pieces,
            })
        } else {
            None
        }
    }
}

fn load_schematic(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut schematic = Vec::new();
    for line in lines {
        let n = line.len();
        let items = line.split("");
        let values: Vec<char> = items
            .skip(1)
            .take(n)
            .map(|s| s.chars().next().expect("empty cell"))
            .collect();
        schematic.push(values);
    }
    schematic
}

fn find_numbers(schematic: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers = Vec::new();
    for i in 0..schematic.len() {
        let mut inhand = None;
        let len = schematic[i].len();
        for j in 0..(len + 1) {
            if j < len && schematic[i][j].is_ascii_digit() {
                if inhand.is_none() {
                    inhand = Some((i, j));
                }
            } else if let Some(p) = inhand {
                let s: String = schematic[i][p.1..j].iter().collect();
                let v = s.parse::<u32>().expect("found a non-integer");
                let n = j - p.1;
                numbers.push(Number { v, p, n });
                inhand = None;
            }
        }
    }
    numbers
}

fn find_cogs(schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut cogs = Vec::new();
    for i in 0..schematic.len() {
        let len = schematic[i].len();
        for j in 0..len {
            if schematic[i][j] == '*' {
                cogs.push((i, j));
            }
        }
    }
    cogs
}

fn main() {
    let f = File::open("input/003.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let schematic = load_schematic(lines);
    let numbers = find_numbers(&schematic);
    let sum = score_adjacent(&numbers, &schematic);
    println!("sum of adjacent parts is {sum}");
    // 528231 too low

    let cogs = find_cogs(&schematic);
    let gears: Vec<_> = cogs.iter().filter_map(|c| c.is_gear(&numbers)).collect();
    let gear_sum = score_gears(&gears);
    println!("sum of gear ratiosis {gear_sum}");
}

fn score_adjacent(numbers: &Vec<Number>, schematic: &Vec<Vec<char>>) -> u32 {
    numbers
        .iter()
        .filter(|n| n.adjacent(schematic))
        .map(|n| n.v)
        .sum::<u32>()
}

fn score_gears(gears: &Vec<Gear>) -> u32 {
    gears
        .iter()
        .map(|g| g.pieces.iter().map(|p| p.v).product::<u32>())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    const SAMPLE2: &str = r#"467..114"#;

    #[test]
    fn test_load() {
        let schematic = load_schematic(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(schematic.len(), 10);
        assert_eq!(schematic[0].len(), 10);
        assert_eq!(schematic[1][3], '*');
        assert_eq!(schematic[4][2], '7');
    }

    #[test]
    fn test_find() {
        let schematic = load_schematic(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let numbers = find_numbers(&schematic);
        assert_eq!(numbers.len(), 10);
        assert_eq!(
            numbers[0],
            Number {
                v: 467,
                p: (0, 0),
                n: 3
            }
        );
        assert_eq!(
            numbers[2],
            Number {
                v: 35,
                p: (2, 2),
                n: 2
            }
        );
        assert_eq!(
            numbers[6],
            Number {
                v: 592,
                p: (6, 2),
                n: 3
            }
        );
        assert_eq!(
            numbers[9],
            Number {
                v: 598,
                p: (9, 5),
                n: 3
            }
        );
    }

    #[test]
    fn test_find_eol() {
        let schematic = load_schematic(SAMPLE2.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let numbers = find_numbers(&schematic);
        assert_eq!(numbers.len(), 2);
        assert_eq!(
            numbers[0],
            Number {
                v: 467,
                p: (0, 0),
                n: 3
            }
        );
        assert_eq!(
            numbers[1],
            Number {
                v: 114,
                p: (0, 5),
                n: 3
            }
        );
    }

    #[test]
    fn test_adjacent() {
        let schematic = load_schematic(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(
            Number {
                v: 467,
                p: (0, 0),
                n: 3
            }
            .adjacent(&schematic),
            true
        );
        assert_eq!(
            Number {
                v: 35,
                p: (2, 2),
                n: 2
            }
            .adjacent(&schematic),
            true
        );
        assert_eq!(
            Number {
                v: 114,
                p: (0, 6),
                n: 3
            }
            .adjacent(&schematic),
            false
        );
        assert_eq!(
            Number {
                v: 58,
                p: (5, 7),
                n: 2
            }
            .adjacent(&schematic),
            false
        );
    }

    #[test]
    fn test_score_adjacent() {
        let schematic = load_schematic(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let numbers = find_numbers(&schematic);
        let sum = score_adjacent(&numbers, &schematic);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_find_cogs() {
        let schematic = load_schematic(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let cogs = find_cogs(&schematic);
        assert_eq!(cogs.len(), 3);
        assert_eq!(cogs[0], (1, 3));
        assert_eq!(cogs[1], (4, 3));
        assert_eq!(cogs[2], (8, 5));
    }

    #[test]
    fn test_score_gears() {
        let schematic = load_schematic(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let numbers = find_numbers(&schematic);
        let cogs = find_cogs(&schematic);
        let gears: Vec<_> = cogs.iter().filter_map(|c| c.is_gear(&numbers)).collect();
        assert_eq!(gears.len(), 2);
        let _sum = score_gears(&gears);
        assert_eq!(gears.len(), 2);
    }
}
