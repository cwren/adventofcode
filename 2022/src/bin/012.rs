use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;
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
            let row: Vec<usize> = line
                .chars()
                .enumerate()
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
        Map {
            start,
            end,
            heights,
            w,
            h,
        }
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

    fn dist(&self, p: &Pos, q: &Pos) -> usize {
        ((p.0 - q.0).abs() + (p.1 - q.1).abs()).try_into().unwrap()
    }

    fn h(&self, p: &Pos) -> usize {
        self.dist(&self.end, p)
    }
}

fn scenic_path(map: &Map) -> usize {
    let mut candidates = DoublePriorityQueue::new();
    for (j, row) in map.heights.iter().enumerate() {
        for (i, h) in row.iter().enumerate() {
            if *h == 0 {
                let p = (i as i32, j as i32);
                candidates.push(p, shortest_path_from_pos(map, p));
            }
        }
    }
    candidates.pop_min().expect("found no paths at all").1
}

fn shortest_path(map: &Map) -> usize {
    shortest_path_from_pos(map, map.start)
}

fn shortest_path_from_pos(map: &Map, start: Pos) -> usize {
    // https://en.wikipedia.org/wiki/A*_search_algorithm
    let longest = map.longest();
    let mut open = DoublePriorityQueue::new();
    open.push(start, map.h(&start));

    let mut g_score = HashMap::new();
    g_score.insert(start, 0usize);

    let mut from = HashMap::new();

    while !open.is_empty() {
        let (current, _) = open.pop_min().expect("while says it's not empty");
        if current == map.end {
            // unwind
            let mut path = Vec::new();
            let mut p = current;
            while let Some(q) = from.get(&p) {
                path.push(*q);
                p = *q;
            }
            path.reverse();
            return path.len();
        }
        let current_height = map.get_height(&current);
        for offset in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
            if let Some(neighbor) = map.pos_add(&current, &offset) {
                let tentative_g_score = if map.get_height(&neighbor) <= (current_height + 1) {
                    1 + g_score.get(&current).unwrap_or(&longest)
                } else {
                    usize::MAX
                };
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&longest) {
                    from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    open.push(neighbor, tentative_g_score + map.h(&neighbor));
                }
            }
        }
    }
    usize::MAX
}

fn main() {
    let input = fs::read_to_string("input/012.txt").expect("file read error");
    let map = Map::parse(input.lines());

    println!("there map is {} units high", map.heights.len());
    println!("the shortest path is {}", shortest_path(&map));
    println!("the scenic path is {}", scenic_path(&map));
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
        assert_eq!(shortest_path(&map), 31);
    }

    #[test]
    fn test_scenic_path() {
        let map: Map = Map::parse(SAMPLE.lines());
        assert_eq!(scenic_path(&map), 29);
    }
}
