use std::{fs::File, io::Read};
use std::collections::VecDeque;

fn all_different (v: &VecDeque<char>) -> bool {
    for c in v.iter() {
        let n = v.iter().filter(|d| d == &c).count();
        if n > 1 {
            return false
        }
    }
    true
}

fn find_marker(input: &str) -> usize {
    let mut window = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        window.push_back(c);
        while window.len() > 4 {
            window.pop_front();
        }
        if window.len() == 4 && all_different(&window) {
            return i + 1
        }
    }
    0
}

fn main() {
    let mut f = File::open("input/006.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    println!("first start: {}", find_marker(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_start() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
