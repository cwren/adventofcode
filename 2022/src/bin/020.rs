use std::fs;

#[derive(Debug)]
struct Node { 
    id: usize,
    v: i32,
}

fn string_nodes(ring: Vec<i32>) -> Vec<Node> {
    let mut nodes = Vec::new();
    for (id, value) in ring.iter().enumerate() {
        nodes.push(Node { id, v: *value });
    }
    nodes
}

fn find_node(ring: &Vec<Node>, id: usize) -> Option<usize> {
    for (p, n) in ring.iter().enumerate() {
        if n.id == id {
            return Some(p);
        }
    }
    None
}
fn find_value(ring: &Vec<Node>, value: i32) -> Option<usize> {
    for (p, n) in ring.iter().enumerate() {
        if n.v == value {
            return Some(p);
        }
    }
    None
}

fn wrap(i: i32, n: i32) -> i32 {
    let mut i = i;
    while i < 0 { i += n; }
    while i > n { i -= n; }
    i
}

fn move_node(ring: &mut Vec<Node>, id: usize) {
    let from = find_node(&ring, id).expect("unknwon node!");
    let n = ring.len() as i32;
    let node = ring.remove(from);
    let from = from as i32;
    let mut to = from + node.v;
    to = wrap(to, n - 1);
    if node.v < 0 && to == 0 { to = n - 1 }
    if node.v > 0 && to == n { to = 0 }
    ring.insert(to as usize, node);
}

fn move_all(ring: &mut Vec<Node>) {
    for i in 0..ring.len() {
        move_node(ring, i);
    }
}
fn score(ring: &Vec<Node>) -> i32 {
    let zero = find_value(ring, 0).expect("no node with value 0") as i32;
    let n = ring.len() as i32;
    let a = wrap(zero + 1000, n) as usize;
    let b = wrap(zero + 2000, n) as usize;
    let c = wrap(zero + 3000, n) as usize;
    ring.get(a).unwrap().v + ring.get(b).unwrap().v + ring.get(c).unwrap().v
}

fn main() {
    let input: &str = &fs::read_to_string("input/020.txt").expect("file read error");
    let ring = input.lines().map(|s| s.parse::<i32>().expect("not a number")).collect::<Vec<i32>>();
    let mut ring = string_nodes(ring);
    println!("there are {} nodes", ring.len());
    move_all(&mut ring);
    println!("grove coordiante is {}", score(&ring));   
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn test_parse_input() {
        let ring = SAMPLE.lines().map(|s| s.parse::<i32>().expect("not a number")).collect::<Vec<i32>>();
        let ring = string_nodes(ring);
        assert_eq!(ring.len(), 7);
        assert_eq!(ring.first().unwrap().id, 0);
        assert_eq!(ring.first().unwrap().v, 1);
        assert_eq!(ring.last().unwrap().id, 6);
        assert_eq!(ring.last().unwrap().v, 4);
    }
    
    #[test]
    fn test_find_node() {
        let ring = SAMPLE.lines().map(|s| s.parse::<i32>().expect("not a number")).collect::<Vec<i32>>();
        let ring = string_nodes(ring);
        assert_eq!(find_node(&ring, 0), Some(0));
        assert_eq!(find_node(&ring, 1), Some(1));
        assert_eq!(find_node(&ring, 6), Some(6));
        assert_eq!(find_node(&ring, 7), None);
    }
    
    #[test]
    fn test_move_node() {
        let ring = SAMPLE.lines().map(|s| s.parse::<i32>().expect("not a number")).collect::<Vec<i32>>();
        let mut ring = string_nodes(ring);
        // 0, 1, 2, 3, 4, 5, 6
        // 1, 2,-3, 3,-2, 0, 4
        move_node(&mut ring, 0);
        // 1 moves between 2 and -3:
        // 2, 1, -3, 3, -2, 0, 4
        assert_eq!(find_node(&ring, 1), Some(0));
        assert_eq!(find_node(&ring, 0), Some(1));
        assert_eq!(find_node(&ring, 2), Some(2));

        move_node(&mut ring, 1);
        // 2 moves between -3 and 3:
        // 1, -3, 2, 3, -2, 0, 4
        assert_eq!(find_node(&ring, 2), Some(1));
        assert_eq!(find_node(&ring, 1), Some(2));
        assert_eq!(find_node(&ring, 3), Some(3));

        move_node(&mut ring, 2);
        // -3 moves between -2 and 0:
        // 1, 2, 3, -2, -3, 0, 4
        assert_eq!(find_node(&ring, 4), Some(3));
        assert_eq!(find_node(&ring, 2), Some(4));
        assert_eq!(find_node(&ring, 5), Some(5));

        move_node(&mut ring, 3);
        // 3 moves between 0 and 4:
        // 1, 2, -2, -3, 0, 3, 4
        assert_eq!(find_node(&ring, 5), Some(4));
        assert_eq!(find_node(&ring, 3), Some(5));
        assert_eq!(find_node(&ring, 6), Some(6));

        move_node(&mut ring, 4);
        // -2 moves between 4 and 1:
        // 1, 2, -3, 0, 3, 4, -2
        assert_eq!(find_node(&ring, 6), Some(5));
        assert_eq!(find_node(&ring, 4), Some(6));
        assert_eq!(find_node(&ring, 0), Some(0));

        move_node(&mut ring, 5);
        // 0 does not move:
        // 1, 2, -3, 0, 3, 4, -2
        assert_eq!(find_node(&ring, 2), Some(2));
        assert_eq!(find_node(&ring, 5), Some(3));
        assert_eq!(find_node(&ring, 3), Some(4));

        move_node(&mut ring, 6);
        // 4 moves between -3 and 0:
        // 1, 2, -3, 4, 0, 3, -2
        assert_eq!(find_node(&ring, 2), Some(2));
        assert_eq!(find_node(&ring, 6), Some(3));
        assert_eq!(find_node(&ring, 5), Some(4));

        ring = string_nodes(vec![0, -1, 0, 0]);
        move_node(&mut ring, 1);
        assert_eq!(find_node(&ring, 0), Some(0));
        assert_eq!(find_node(&ring, 1), Some(3));
        assert_eq!(find_node(&ring, 2), Some(1));
        assert_eq!(find_node(&ring, 3), Some(2));

        ring = string_nodes(vec![0, 0, -1, 0]);
        move_node(&mut ring, 2);
        assert_eq!(find_node(&ring, 0), Some(0));
        assert_eq!(find_node(&ring, 1), Some(2));
        assert_eq!(find_node(&ring, 2), Some(1));
        assert_eq!(find_node(&ring, 3), Some(3));

    }
    
    #[test]
    fn test_move_all() {
        let ring = SAMPLE.lines().map(|s| s.parse::<i32>().expect("not a number")).collect::<Vec<i32>>();
        let mut ring = string_nodes(ring);
        // 0, 1, 2, 3, 4, 5, 6
        // 1, 2,-3, 3,-2, 0, 4
        move_all(&mut ring);
        // 1, 2, -3, 4, 0, 3, -2
        assert_eq!(find_node(&ring, 0), Some(0));
        assert_eq!(find_node(&ring, 1), Some(1));
        assert_eq!(find_node(&ring, 2), Some(2));
        assert_eq!(find_node(&ring, 3), Some(5));
        assert_eq!(find_node(&ring, 4), Some(6));
        assert_eq!(find_node(&ring, 5), Some(4));
        assert_eq!(find_node(&ring, 6), Some(3));
    }
    #[test]
    fn test_score() {
        let ring = SAMPLE.lines().map(|s| s.parse::<i32>().expect("not a number")).collect::<Vec<i32>>();
        let mut ring = string_nodes(ring);
        move_all(&mut ring);
        assert_eq!(score(&ring), 3);   
    }
}
