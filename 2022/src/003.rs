use std::panic;
use std::{fs::File, io::Read};
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack { 
    all: String,
    front: String,
    back: String,
}

impl Rucksack {
    pub fn new(value: &str) -> Self {
        let n = value.len();
        Self {
            all: value.to_string(),
            front: value[0..n/2].to_string(),
            back: value[n/2..].to_string(),
        }
    }

    fn common(&self) -> char {
        let mut front_set = HashSet::new();
        for c in self.front.chars() {
            front_set.insert(c);
        }
        for c in self.back.chars() {
            if front_set.contains(&c) { return c }
        }
        panic!("none")
    }
}

fn priority(c: char) -> u32 {
    match c {
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        _ => panic!(r#"invalid item"#)
    }
}

fn total_priority(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().map(|s| priority(s.common())).sum()
}

fn parse_rucksacks(input: &str) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in input.lines() {
        if line.len() %2 != 0 {
            panic!("Line is not an even length: {line}");
        } else {
            rucksacks.push(Rucksack::new(line));
        }
    }
    rucksacks
}

fn find_badge(groups: &[Rucksack]) -> char {
    let mut sets: Vec<HashSet<char>> = Vec::new();
    for group in groups.iter() {
        sets.push(group.all.chars().collect::<HashSet<char>>());
    }
    let mut acc = sets[0].clone();
    for set in sets.iter().skip(1) {
        acc = &acc & set;
    }
    *acc.iter().next().expect("missing badge")
}

fn compute_group_priorities(rucksacks: &[Rucksack]) -> u32 {
    let mut badges = Vec::new();
    for i in (0..rucksacks.len()).step_by(3) {
        badges.push(find_badge(&rucksacks[i..i+3]));
    }
    let mut total = 0;
    for badge in badges {
        total += priority(badge);
    }
    total
}

fn main() {
    let mut f = File::open("input/003.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");
    let rucksacks = parse_rucksacks(&input);
    let total: u32 = total_priority(&rucksacks);
    println!("total priority: {total}");
    let group_total: u32 = compute_group_priorities(&rucksacks);
    println!("total group priority: {group_total}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
    
    #[test]
    fn test_parse() {
        let rucksacks = parse_rucksacks(SAMPLE);
        assert_eq!(rucksacks.len(), 6);
        assert_eq!(rucksacks[0].front, "vJrwpWtwJgWr");
        assert_eq!(rucksacks[0].back, "hcsFMMfFFhFp");
        assert_eq!(rucksacks[1].front, "jqHRNqRjqzjGDLGL");
        assert_eq!(rucksacks[1].back, "rsFMfFZSrLrFZsSL");
        assert_eq!(rucksacks[2].front, "PmmdzqPrV");
        assert_eq!(rucksacks[2].back, "vPwwTWBwg");
    }

    #[test]
    fn test_compare() {
        let rucksacks = parse_rucksacks(SAMPLE);
        assert_eq!(rucksacks[0].common(), 'p');
        assert_eq!(rucksacks[1].common(), 'L');
        assert_eq!(rucksacks[2].common(), 'P');
        assert_eq!(rucksacks[3].common(), 'v');
        assert_eq!(rucksacks[4].common(), 't');
        assert_eq!(rucksacks[5].common(), 's');
    }

    #[test]
    fn test_missing_item() {
        let sack = Rucksack::new("abcdef");
        let result = panic::catch_unwind(|| {
            sack.common();
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_priorotiy_map() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('s'), 19);
    }

    #[test]
    fn test_total_p() {
        let rucksacks = parse_rucksacks(SAMPLE);
        assert_eq!(total_priority(&rucksacks), 157);
    }

    #[test]
    fn test_find_badge() {
        let rucksacks = parse_rucksacks(SAMPLE);
        assert_eq!(find_badge(&rucksacks[0..3]), 'r');
        assert_eq!(find_badge(&rucksacks[3..6]), 'Z');
    }

    #[test]
    fn test_compute_group_priorities() {
        let rucksacks = parse_rucksacks(SAMPLE);
        assert_eq!(compute_group_priorities(&rucksacks), 70);
    }

}
