use regex::Regex;
use std::{fs::File, io::Read};

type Crate = Vec::<char>;
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

fn parse_crates(input: &str) -> (Vec<Crate>, usize) {
    let mut n = 0;
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            n = i;
            break;
        }
    }
    let labels = input.lines().nth(n - 1)
        .expect("not enough lines of input");
    let num_columns = labels.trim().split(' ')
        .filter(|s| !s.is_empty())
        .count();
    
    let mut crates = Vec::new();
    for _ in 0..num_columns {
        crates.push(Vec::new());
    }

    for line in input.lines() {
        if line.contains('[') {
            for (i, stack) in crates.iter_mut().enumerate() {
                if let Some(c) = line.chars().nth(4 * i + 1) {
                    if c != ' ' {
                        stack.push(c);
                    }
                }
            }
        } else {
            break;
        }
    }

    for stack in crates.iter_mut() {
        stack.reverse();
    }
    (crates, n + 1)
}

fn parse_moves(input: &str, n: usize) -> Vec<Move> {
    let mut moves = Vec::new();
    // move 2 from 2 to 1
    let move_re: regex::Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in input.lines().skip(n) {
        let caps = move_re.captures(line).unwrap();
        moves.push(Move {
            n: caps.get(1).unwrap().as_str().parse().unwrap(),
            from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        });
    }
    moves
}

fn parse_crates_and_moves(input: &str) -> (Vec<Crate>, Vec<Move>) {
    let (crates, n) = parse_crates(input);
    let moves = parse_moves(input, n);
    (crates, moves)
}

fn execute_9000(moves: &Vec<Move>, crates: &mut [Crate]) {
    for m in moves {
        for _ in 0..m.n {
            let c = crates[m.from].pop().unwrap();
            crates[m.to].push(c);
        }
    }
}

fn execute_9001(moves: &Vec<Move>, crates: &mut [Crate]) {
    for m in moves {
        let n = crates[m.from].len();
        let cargo = crates[m.from].split_off(n - m.n);
        for c in cargo {
            crates[m.to].push(c);
        }
    }
}

fn main() {
    let mut f = File::open("input/005.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    let (mut crates, moves) = parse_crates_and_moves(&input);
    println!("moves: {:?}", moves.len());

    let mut saved_crates = crates.clone();

    execute_9000(&moves, &mut crates);
    let mut output = String::new();
    for stack in crates {
        output.push(*stack.last().unwrap());
    }
    println!("9000 top of stacks: {output}");

    execute_9001(&moves, &mut saved_crates);
    let mut output = String::new();
    for stack in saved_crates {
        output.push(*stack.last().unwrap());
    }
    println!("9001 top of stacks: {output}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_parse_crates() {
        let (crates, n) = parse_crates(SAMPLE);
        assert_eq!(n, 5);
        assert_eq!(crates.len(), 3);
        assert_eq!(crates[0], Vec::from(['Z', 'N']));
        assert_eq!(crates[1], Vec::from(['M', 'C', 'D']));
        assert_eq!(crates[2], Vec::from(['P']));
    }

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves(SAMPLE, 5);
        assert_eq!(moves.len(), 4);
        assert_eq!(moves[1].n, 3);
        assert_eq!(moves[1].from, 0);
        assert_eq!(moves[1].to, 2);
        assert_eq!(moves[2].n, 2);
        assert_eq!(moves[2].from, 1);
        assert_eq!(moves[2].to, 0);
    }

    #[test]
    fn test_execute_9000() {
        let (mut crates, moves) = parse_crates_and_moves(SAMPLE);
        execute_9000(&moves, &mut crates);
        assert_eq!(crates[0].last().unwrap(), &'C');
        assert_eq!(crates[1].last().unwrap(), &'M');
        assert_eq!(crates[2].last().unwrap(), &'Z');
    }

    #[test]
    fn test_execute_9001() {
        let (mut crates, moves) = parse_crates_and_moves(SAMPLE);
        execute_9001(&moves, &mut crates);
        assert_eq!(crates[0].last().unwrap(), &'M');
        assert_eq!(crates[1].last().unwrap(), &'C');
        assert_eq!(crates[2].last().unwrap(), &'D');
    }
}
