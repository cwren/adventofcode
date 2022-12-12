use std::collections::HashSet;
use std::fs;
use std::str::Lines;

type Pos = (i32, i32);

#[derive(Debug)]
struct Map {
    start: Pos,
    end: Pos,
    heights: Vec<Vec<usize>>,
    w: i32,
    h: i32,
}


impl Map {
    fn parse(input: Lines) -> Self {
        let mut heights = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (j, line) in input.enumerate() {
            let row: Vec<usize> = line.chars().enumerate()
                .map(|(i, c)| match c {
                    'S' => {
                        start = (i as i32, j as i32);
                        0
                    }
                    'E' => {
                        end = (i as i32, j as i32);
                        'z' as usize - 'a' as usize
                    }
                    'a'..='z' => c as usize - 'a' as usize,
                    _ => panic!("unexpected character {c}"),
                })
                .collect();
            heights.push(row);
        }
        let w = heights[0].len() as i32;
        let h = heights.len() as i32;
        Map { start, end, heights, w, h}
    }

    fn get_height(&self, p: &Pos) -> usize {
        self.heights[p.1 as usize][p.0 as usize]
    }

    fn longest(&self) -> usize {
        (self.w * self.h) as usize
    }

    fn pos_add(&self, p: &Pos, delta: &Pos) -> Option<Pos> {
        let q = (p.0 + delta.0, p.1 + delta.1);
        if q.0 < 0 || q.1 < 0 || q.0 >= self.w || q.1 >= self.h {
            return None;
        }
        Some(q)
    }

    fn pos_to_index(&self, p: &Pos)-> i32 {
        p.0 * self.w + p.1
    }

}

fn shortest_path(map: Map) -> usize {
    let visited = HashSet::new();
    sp_worker(&map, &map.start, &visited, 0, map.longest())
}


fn sp_worker(map: &Map, p: &Pos, visited: &HashSet<i32>, depth: i32, too_much: usize) -> usize {
    if too_much == 0 {
        return map.longest();
    }
    if map.end == *p {
        println!("found the end!");
        return 0;
    }
    let indent = (0..depth).map(|_| ".").collect::<String>();
    let from = map.get_height(p);
    let mut visited = visited.clone();
    visited.insert(map.pos_to_index(&p));
    let mut shortest = too_much;
    for offset in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
        if let Some(q) = map.pos_add(p, &offset) {
            if !visited.contains(&map.pos_to_index(&q)) {
                if map.get_height(&q) <= (from + 1) {
                    if indent.len() < 50 {
                        println!("{indent}{:?}:{}", q, shortest);
                    }
                    shortest = shortest.min(sp_worker(&map, &q, &visited, depth + 1, shortest - 1));
                } else {
                    if indent.len() < 50 {
                        println!("{indent}{:?}:too high", q);
                    }
                }
            }
        }
    }
    shortest + 1
}

fn main() {
    let input = fs::read_to_string("input/012.txt").expect("file read error");
    let map = Map::parse(input.lines());

    println!("there map is {} units high", map.heights.len());
    println!("the shortest path is {}", shortest_path(map));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_parse_troop() {
        let map: Map = Map::parse(SAMPLE.lines());
        println!("{:?}", map);
        assert_eq!(map.heights.len(), 5);
        assert_eq!(map.heights[0].len(), 8);
        assert_eq!(map.h, 5);
        assert_eq!(map.w, 8);
        assert_eq!(map.start, (0, 0));
        assert_eq!(map.end, (5, 2));
    }

    #[test]
    fn test_shortest_path() {
        let map: Map = Map::parse(SAMPLE.lines());
        assert_eq!(shortest_path(map), 31);
    }
}
