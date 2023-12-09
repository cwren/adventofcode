use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
struct Card {
    id: u32,
    w: Vec<i32>,
    h: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(": ").ok_or(ParseCardError)?;
        let id = card.strip_prefix("Card ").ok_or(ParseCardError)?;
        let id = id
            .replace(" ", "")
            .parse::<u32>()
            .map_err(|_| ParseCardError)?;
        let (winners, holds) = numbers.split_once(" | ").ok_or(ParseCardError)?;
        let w = winners
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<i32>().expect("found a non-integer"))
            .collect::<Vec<i32>>();
        let h = holds
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<i32>().expect("found a non-integer"))
            .collect::<Vec<i32>>();
        Ok(Card { id, w, h })
    }
}

fn load_cards(lines: Vec<String>) -> Vec<Card> {
    lines
        .iter()
        .map(|l| l.parse::<Card>().expect("found a non-card"))
        .collect()
}

fn score_card(card: &Card) -> u32 {
    let match_count = count_matches(card);
    match match_count {
        0 => 0,
        _ => (2 as u32).pow((match_count - 1) as u32),
    }
}

fn count_matches(card: &Card) -> u32 {
    let mut w: HashSet<i32> = HashSet::new();
    w.extend(card.w.iter());
    card.h
        .iter()
        .map(|n| w.contains(n))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

fn score_deck(cards: &Vec<Card>) -> u32 {
    let mut inventory: Vec<u32> = vec![1; cards.len() + 1];
    inventory[0] = 0;
    for card in cards {
        let num_matches = count_matches(card);
        for i in 0..num_matches {
            let that_card = (card.id + 1 + i) as usize;
            if that_card < inventory.len() {
                inventory[that_card] += inventory[card.id as usize];
            }
        }
    }
    inventory.iter().sum()
}

fn main() {
    let f = File::open("input/004.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let cards = load_cards(lines);
    let total: u32 = cards.iter().map(|c| score_card(c)).sum();
    println!("total score is {total}");

    let card_count = score_deck(&cards);
    println!("total card count is {card_count}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_load() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let cards = load_cards(lines);
        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].id, 1);
        assert_eq!(cards[0].w.len(), 5);
        assert_eq!(cards[0].h.len(), 8);
        assert_eq!(cards[0].w, [41, 48, 83, 86, 17]);
        assert_eq!(cards[5].id, 6);
        assert_eq!(cards[5].w.len(), 5);
        assert_eq!(cards[5].h.len(), 8);
        assert_eq!(cards[5].h, [74, 77, 10, 23, 35, 67, 36, 11]);
        assert_eq!(cards[3].h, [59, 84, 76, 51, 58, 5, 54, 83]);
    }

    #[test]
    fn test_score_card() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let cards = load_cards(lines);
        assert_eq!(score_card(&cards[0]), 8);
        assert_eq!(score_card(&cards[1]), 2);
        assert_eq!(score_card(&cards[2]), 2);
        assert_eq!(score_card(&cards[3]), 1);
        assert_eq!(score_card(&cards[4]), 0);
        assert_eq!(score_card(&cards[5]), 0);
        let total: u32 = cards.iter().map(|c| score_card(c)).sum();
        assert_eq!(total, 13);
    }

    #[test]
    fn test_score_deck() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let cards = load_cards(lines);
        assert_eq!(score_deck(&cards), 30);
    }
}
