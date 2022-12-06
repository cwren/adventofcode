use std::collections::{HashSet, VecDeque};
use std::{fs::File, io::Read};

fn all_different(v: &VecDeque<char>) -> bool {
    let mut hash = HashSet::with_capacity(v.len());
    for c in v.iter() {
        if hash.contains(c) {
            return false;
        }
        hash.insert(c);
    }
    true
}

fn find_marker(input: &str, w: usize) -> usize {
    let mut window = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        window.push_back(c);
        while window.len() > w {
            window.pop_front();
        }
        if window.len() == w && all_different(&window) {
            return i + 1;
        }
    }
    0
}

fn find_packet(input: &str) -> usize {
    find_marker(input, 4)
}

fn find_message(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let mut f = File::open("input/006.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    println!("first packet: {}", find_packet(&input));
    println!("first message: {}", find_message(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_find_packet() {
        assert_eq!(find_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_find_message() {
        assert_eq!(find_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
