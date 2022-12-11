use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

use Move::{Paper, Rock, Scissors};
impl Move {
    fn beats(&self, other: &Move) -> std::cmp::Ordering {
        if *self == *other {
            Equal
        } else if (*self == Rock && *other == Scissors)
            || (*self == Scissors && *other == Paper)
            || (*self == Paper && *other == Rock)
        {
            Greater
        } else {
            Less
        }
    }
    fn lose_to(other: Move) -> Move {
        match other {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
    }
    fn win_against(other: Move) -> Move {
        match other {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }
}

struct Game {
    them: Move,
    me: Move,
}

impl Game {
    fn bad_parse(line: String) -> Game {
        let parts: Vec<&str> = line.split(&[' '][..]).collect();
        let theirs = match parts[0] {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            unk => panic!("invalid move {unk}"),
        };
        let mine = match parts[1] {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            unk => panic!("invalid move {unk}"),
        };
        Game {
            them: theirs,
            me: mine,
        }
    }

    fn parse(line: String) -> Game {
        let parts: Vec<&str> = line.split(&[' '][..]).collect();
        let theirs = match parts[0] {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            unk => panic!("invalid move {unk}"),
        };
        let mine = match parts[1] {
            "X" => Move::lose_to(theirs),
            "Y" => theirs,
            "Z" => Move::win_against(theirs),
            unk => panic!("invalid move {unk}"),
        };
        Game {
            them: theirs,
            me: mine,
        }
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        match self.me {
            Rock => score += 1,
            Paper => score += 2,
            Scissors => score += 3,
        }
        match self.me.beats(&self.them) {
            Equal => score += 3,
            Greater => score += 6,
            Less => (),
        }
        score
    }
}

type Guide = Vec<Game>;

trait Score {
    fn score(&self) -> u32;
}

impl Score for Guide {
    fn score(&self) -> u32 {
        self.iter().map(|game| game.score()).sum()
    }
}

fn main() {
    let f = File::open("input/002.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let guide = parse_guide(Game::bad_parse, lines.clone());
    println!("follow the wrong guide for a score of: {}", guide.score());
    let guide = parse_guide(Game::parse, lines);
    println!("follow the secret guide for a score of: {}", guide.score());
}

fn parse_guide(parser: fn(String) -> Game, lines: Vec<String>) -> Guide {
    let mut guide = Vec::new();
    for summary in lines {
        guide.push(parser(summary));
    }
    guide
}

#[cfg(test)]
mod tests {
    use crate::parse_guide;
    use crate::Game;
    use crate::Score;
    #[test]
    fn test_bad_parse() {
        let guide = parse_guide(Game::bad_parse, ["A Y", "B X", "C Z"].map(String::from).to_vec());
        assert_eq!(guide.len(), 3);
        assert_eq!(guide[0].score(), 8);
        assert_eq!(guide.score(), 15);
    }
    #[test]
    fn test_good_parse() {
        let guide = parse_guide(Game::parse, ["A Y", "B X", "C Z"].map(String::from).to_vec());
        assert_eq!(guide.len(), 3);
        assert_eq!(guide[0].score(), 4);
        assert_eq!(guide.score(), 12);
    }
}
