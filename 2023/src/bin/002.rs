use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Sample>,
}

#[derive(Debug, PartialEq, Eq)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn possible(&self, probe: &Sample) -> bool {
        for turn in self.turns.iter() {
            if turn.red > probe.red {
                return false;
            };
            if turn.green > probe.green {
                return false;
            };
            if turn.blue > probe.blue {
                return false;
            };
        }
        true
    }
    fn minimum(&self) -> Sample {
        let mut mins = Sample {
            red: 0,
            blue: 0,
            green: 0,
        };
        for turn in self.turns.iter() {
            mins.red = max(mins.red, turn.red);
            mins.green = max(mins.green, turn.green);
            mins.blue = max(mins.blue, turn.blue);
        }
        mins
    }
}

impl Sample {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn score(games: &Vec<Game>, probe: &Sample) -> u32 {
    games
        .iter()
        .filter(|g| g.possible(probe))
        .map(|g| g.id)
        .sum::<u32>()
}

fn power_score(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|g| g.minimum())
        .map(|s| s.power())
        .sum::<u32>()
}
fn main() {
    let f = File::open("input/002.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let games = parse_games(lines);
    let probe = Sample {
        red: 12,
        green: 13,
        blue: 14,
    };
    println!("12/13/14 total is {:?}", score(&games, &probe));
    println!("12/13/14 total is {:?}", power_score(&games));
}

fn parse_game(input: &str) -> Game {
    let parts = input.split(':').collect::<Vec<_>>();
    let header = parts[0];
    let id = header.split(' ').collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();
    let body = parts[1];
    let turns_desc = body.split(';').collect::<Vec<_>>();
    let mut turns = Vec::new();
    for desc in turns_desc {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for count in desc.split(',') {
            let data = count.split(' ').collect::<Vec<_>>();
            let n = data[1].parse::<u32>().unwrap();
            match data[2] {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                &_ => panic!("unknown color:"),
            }
        }
        turns.push(Sample {
            red,
            green,
            blue,
        });
    }
    Game {
        id,
        turns,
    }
}

fn parse_games(lines: Vec<String>) -> Vec<Game> {
    let mut games = Vec::new();
    for line in lines {
        if !line.is_empty() {
            games.push(parse_game(&line));
        }
    }
    games
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_games() {
        let games = parse_games(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        let probe = Sample {
            red: 12,
            green: 13,
            blue: 14,
        };
        let total = score(&games, &probe);
        assert_eq!(total, 8);
    }

    #[test]
    fn test_parse_game() {
        let lines: Vec<_> = SAMPLE.lines().collect();
        let game = parse_game(lines[0]);
        assert_eq!(game.id, 1);
        assert_eq!(game.turns.len(), 3);

        assert_eq!(game.turns[0].red, 4);
        assert_eq!(game.turns[0].green, 0);
        assert_eq!(game.turns[0].blue, 3);

        assert_eq!(game.turns[1].red, 1);
        assert_eq!(game.turns[1].green, 2);
        assert_eq!(game.turns[1].blue, 6);

        assert_eq!(game.turns[2].red, 0);
        assert_eq!(game.turns[2].green, 2);
        assert_eq!(game.turns[2].blue, 0);
    }

    #[test]
    fn test_minimum() {
        let games = parse_games(SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(
            games[0].minimum(),
            Sample {
                red: 4,
                green: 2,
                blue: 6
            }
        );
        assert_eq!(
            games[1].minimum(),
            Sample {
                red: 1,
                green: 3,
                blue: 4
            }
        );
        assert_eq!(
            games[2].minimum(),
            Sample {
                red: 20,
                green: 13,
                blue: 6
            }
        );
        assert_eq!(
            games[3].minimum(),
            Sample {
                red: 14,
                green: 3,
                blue: 15
            }
        );
        assert_eq!(
            games[4].minimum(),
            Sample {
                red: 6,
                green: 3,
                blue: 2
            }
        );
    }

    #[test]
    fn test_power() {
        assert_eq!(
            Sample {
                red: 4,
                green: 2,
                blue: 6
            }
            .power(),
            48
        );
        assert_eq!(
            Sample {
                red: 1,
                green: 3,
                blue: 4
            }
            .power(),
            12
        );
        assert_eq!(
            Sample {
                red: 20,
                green: 13,
                blue: 6
            }
            .power(),
            1560
        );
        assert_eq!(
            Sample {
                red: 14,
                green: 3,
                blue: 15
            }
            .power(),
            630
        );
        assert_eq!(
            Sample {
                red: 6,
                green: 3,
                blue: 2
            }
            .power(),
            36
        );
    }
}
