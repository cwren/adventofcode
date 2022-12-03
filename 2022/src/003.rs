use std::{fs::File, io::Read};
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack { 
    front: String,
    back: String,
}

impl Rucksack {
    fn common(&self) -> Result<char, &str> {
        let mut front_set = HashSet::new();
        for c in self.front.chars() {
            front_set.insert(c);
        }
        for c in self.back.chars() {
            if front_set.contains(&c) { return Ok(c) }
        }
        Err("none")
    }
}

fn priority(c: char) -> Result<u32, &'static str> {
    match c {
        'A'..='Z' => Ok(c as u32 - 'A' as u32 + 27),
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 1),
        _ => Err(r#"invalid item"#)
    }
}

fn parse_rucksacks(input: &str) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in input.lines() {
        if line.len() %2 != 0 {
            panic!("Line is not an even length: {line}");
        } else {
            let items: Vec<char> = line.chars().collect();
            let n = items.len();
            rucksacks.push(Rucksack {
                front: line[0..n/2].to_string(),
                back: line[n/2..].to_string(),
            });
        }
    }
    rucksacks
}


fn main() {
    let mut f = File::open("input/003.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");
    let rucksacks = parse_rucksacks(&input);
    let total: u32 = rucksacks.iter().map(|s| priority(s.common().unwrap()).unwrap()).sum();
    println!("total priority: {total}");
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
        assert_eq!(rucksacks[0].common().unwrap(), 'p');
        assert_eq!(rucksacks[1].common().unwrap(), 'L');
        assert_eq!(rucksacks[2].common().unwrap(), 'P');
        assert_eq!(rucksacks[3].common().unwrap(), 'v');
        assert_eq!(rucksacks[4].common().unwrap(), 't');
        assert_eq!(rucksacks[5].common().unwrap(), 's');
    }

    #[test]
    fn test_missing_item() {
        let sack = Rucksack { front: String::from("abc"), back: String::from("def") };
        assert!(sack.common().is_err());
    }

    #[test]
    fn test_priorotiy_map() {
        assert_eq!(priority('p').unwrap(), 16);
        assert_eq!(priority('L').unwrap(), 38);
        assert_eq!(priority('P').unwrap(), 42);
        assert_eq!(priority('v').unwrap(), 22);
        assert_eq!(priority('t').unwrap(), 20);
        assert_eq!(priority('s').unwrap(), 19);
    }

    #[test]
    fn test_total_p() {
        let rucksacks = parse_rucksacks(SAMPLE);
        let total: u32 = rucksacks.iter().map(|s| priority(s.common().unwrap()).unwrap()).sum();
        assert_eq!(total, 157);
    }

}
