use regex::Regex;
use std::{fs::File, io::Read};

const ELF_PAIR_RE: &str = r"^(\d+)-(\d+),(\d+)-(\d+)$";

type Job = (u32, u32);
type ElfPair = (Job, Job);

fn contained_in(a: &Job, b: &Job) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn overlapping(a: &Job, b: &Job) -> bool {
    (a.0 <= b.0 && a.1 >= b.0) || (a.0 <= b.1 && a.1 >= b.1) || (a.0 <= b.1 && a.1 >= b.0)
}

fn parse_jobs(input: &str) -> Vec<ElfPair> {
    let elf_pair_re: regex::Regex = Regex::new(ELF_PAIR_RE).unwrap();
    let mut jobs = Vec::new();

    for line in input.lines() {
        let caps = elf_pair_re.captures(line).unwrap();
        jobs.push((
            (
                caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            ),
            (
                caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            ),
        ))
    }
    jobs
}

fn main() {
    let mut f = File::open("input/004.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    let jobs = parse_jobs(&input);
    let count = jobs
        .iter()
        .map(|j| contained_in(&j.0, &j.1) || contained_in(&j.1, &j.0))
        .filter(|b| *b)
        .count();
    println!("contains: {:?}", count);

    let count = jobs
        .iter()
        .map(|j| overlapping(&j.0, &j.1))
        .filter(|b| *b)
        .count();
    println!("overlaps: {:?}", count);
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_parse() {
        let jobs = parse_jobs(SAMPLE);
        assert_eq!(jobs.len(), 6);
        assert_eq!(jobs[0].0 .0, 2);
        assert_eq!(jobs[0].0 .1, 4);
        assert_eq!(jobs[0].1 .0, 6);
        assert_eq!(jobs[0].1 .1, 8);
        assert_eq!(jobs[4].0 .0, 6);
        assert_eq!(jobs[4].0 .1, 6);
        assert_eq!(jobs[4].1 .0, 4);
        assert_eq!(jobs[4].1 .1, 6);
    }

    #[test]
    fn test_contains() {
        let jobs = parse_jobs(SAMPLE);
        assert_eq!(contained_in(&jobs[0].1, &jobs[0].0), false);
        assert_eq!(contained_in(&jobs[3].0, &jobs[3].1), false);
        assert_eq!(contained_in(&jobs[3].1, &jobs[3].0), true);
    }

    #[test]
    fn test_count_contains() {
        let jobs = parse_jobs(SAMPLE);
        let count = jobs
            .iter()
            .map(|j| contained_in(&j.0, &j.1) || contained_in(&j.1, &j.0))
            .filter(|b| *b)
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_overlapping() {
        let jobs = parse_jobs(SAMPLE);
        assert_eq!(overlapping(&jobs[0].1, &jobs[0].0), false);
        assert_eq!(overlapping(&jobs[1].0, &jobs[1].1), false);
        assert_eq!(overlapping(&jobs[2].1, &jobs[2].0), true);
        assert_eq!(overlapping(&jobs[3].1, &jobs[3].0), true);
        assert_eq!(overlapping(&jobs[4].1, &jobs[4].0), true);
        assert_eq!(overlapping(&jobs[5].1, &jobs[5].0), true);
    }

    #[test]
    fn test_count_overlaps() {
        let jobs = parse_jobs(SAMPLE);
        let count = jobs
            .iter()
            .map(|j| overlapping(&j.0, &j.1))
            .filter(|b| *b)
            .count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_regex_match() {
        let elf_pair_re: regex::Regex = Regex::new(ELF_PAIR_RE).unwrap();
        assert!(elf_pair_re.is_match("5-7,6-9"));
        assert!(elf_pair_re.is_match("522-794,62302-954322"));
    }

    #[test]
    fn test_regex_groups() {
        let elf_pair_re: regex::Regex = Regex::new(ELF_PAIR_RE).unwrap();
        let caps = elf_pair_re.captures("54-78,62-91").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), 54);
        assert_eq!(caps.get(2).unwrap().as_str().parse::<u32>().unwrap(), 78);
        assert_eq!(caps.get(3).unwrap().as_str().parse::<u32>().unwrap(), 62);
        assert_eq!(caps.get(4).unwrap().as_str().parse::<u32>().unwrap(), 91);
    }
}
