use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Integer = i64;
type Coord = (Integer, Integer);

fn find_galaxies(lines: Vec<String>) -> Vec<Coord> {
    let mut galaxies = Vec::new();
    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i as Integer, j as Integer));
            }
        }
    }
    galaxies
}

fn inflate(galaxies:  &Vec<Coord>, expansion_ratio: Integer) -> Vec<Coord> {
    // inflate in x direction
    let igroups: HashMap<Integer, Vec<Coord>> = galaxies
        .to_owned()
        .into_iter()
        .into_group_map_by(|a| a.0);
    let mut last = -1;
    let mut num_empty = 0;
    let mut inflated = Vec::new();
    for key in igroups.keys().sorted() {
        num_empty += key - last - 1;
        last = *key;
        for galaxy in igroups[&key].iter() {
            inflated.push((galaxy.0 + num_empty * expansion_ratio, galaxy.1));
        }
    }
    // inflate in y direction
    let jgroups: HashMap<Integer, Vec<Coord>> = inflated
        .to_owned()
        .into_iter()
        .into_group_map_by(|a| a.1);
    inflated.clear();
    let mut last = -1;
    let mut num_empty = 0;
    for key in jgroups.keys().sorted() {
        num_empty += key - last - 1;
        last = *key;
        for galaxy in jgroups[&key].iter() {
            inflated.push((galaxy.0, galaxy.1 + num_empty * expansion_ratio));
        }
    }
    inflated
}

fn compute_distances(galaxies:  &Vec<Coord>) -> Vec<Integer> {
    let mut distances = Vec::new();
    for (i , a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            distances.push((a.0-b.0).abs() + (a.1-b.1).abs());
        }
    }
    distances
}

fn main() {
    let f = File::open("input/011.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let galaxies = find_galaxies(lines);
    let inflated = inflate(&galaxies, 1);
    let distances = compute_distances(&inflated);
    println!("sum of intergalactic ditances is {}", distances.iter().sum::<Integer>());

    let inflated = inflate(&galaxies, 999999);
    let distances = compute_distances(&inflated);
    println!("sum of hyper-inflated ditances is {}", distances.iter().sum::<Integer>());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const CONTRACTED: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
    const EXPANDED: &str = r#"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"#;

    #[test]
    fn test_parse() {
        let lines = CONTRACTED.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let galaxies = find_galaxies(lines);
        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies, vec![
            (3, 0),
            (7, 1,),
            (0, 2),
            (6, 4),
            (1, 5),
            (9, 6),
            (7, 8),
            (0, 9),
            (4, 9),
        ]);
    }

    #[test]
    fn test_expand() {
        let lines = CONTRACTED.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let galaxies = find_galaxies(lines);
        let lines = EXPANDED.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let expected = find_galaxies(lines);
        let actual = inflate(&galaxies, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_distances() {
        let lines = CONTRACTED.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let galaxies = find_galaxies(lines);
        let inflated = inflate(&galaxies, 1);
        let distances = compute_distances(&inflated);
        assert_eq!(distances.len(), 36);
        assert_eq!(distances.iter().sum::<Integer>(), 374);
    }

    #[test]
    fn test_rapid_inflation() {
        let lines = CONTRACTED.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let galaxies = find_galaxies(lines);

        let inflated = inflate(&galaxies, 9);
        let distances = compute_distances(&inflated);
        assert_eq!(distances.iter().sum::<Integer>(), 1030);

        let inflated = inflate(&galaxies, 99);
        let distances = compute_distances(&inflated);
        assert_eq!(distances.iter().sum::<Integer>(), 8410);
    }
}
