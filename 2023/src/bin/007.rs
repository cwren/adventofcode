use lazy_static::lazy_static;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
struct Deck {
    hands: Vec<Hand>,
    wild: Option<char>,
}

#[derive(Eq, PartialEq, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    class: u8,
    wild: Option<char>,
}

impl From<&String> for Hand {
    fn from(line: &String) -> Self {
        if let Some((c, b)) = line.split_once(' ') {
            let bid = b.parse::<u32>().unwrap();
            let cards = c.chars().collect::<Vec<char>>();
            let class = Hand::classify(&cards);
            Hand {
                cards,
                bid,
                class,
                wild: None,
            }
        } else {
            panic!("bad hand {line}");
        }
    }
}
impl From<&Vec<String>> for Deck {
    fn from(lines: &Vec<String>) -> Self {
        Deck {
            hands: lines.iter().map(|l| Hand::from(l)).collect(),
            wild: None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.class == other.class {
            let n = min(self.cards.len(), other.cards.len());
            for i in 0..n {
                if self.cards[i] != other.cards[i] {
                    return Hand::compare_cards(&self.cards[i], &other.cards[i], self.wild);
                }
            }
            return Some(Ordering::Equal);
        }
        return Some(self.class.cmp(&other.class));
    }
}

lazy_static! {
    static ref FACE_VALUE: Vec<char> =
        vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
}

impl Hand {
    const HIGH: u8 = 1;
    const ONE: u8 = 2;
    const TWO: u8 = 3;
    const THREE: u8 = 4;
    const HOUSE: u8 = 5;
    const FOUR: u8 = 6;
    const FIVE: u8 = 7;

    fn classify(cards: &Vec<char>) -> u8 {
        Hand::classify_wild(cards, None)
    }

    fn classify_wild(cards: &Vec<char>, wild: Option<char>) -> u8 {
        let mut counter = HashMap::new();
        for card in cards {
            *counter.entry(card).or_insert(0) += 1;
        }
        let mut num_wild = 0;
        if let Some(w) = wild {
            match counter.get(&w) {
                Some(n) => {
                    num_wild = *n;
                    if num_wild == cards.len() as i32 {
                        return Hand::FIVE;
                    }
                    counter.remove(&w);
                }
                None => (),
            }
        }
        let mut counts: Vec<(char, i32)> =
            counter.iter().map(|kv| (**kv.0, *kv.1 as i32)).collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts[0].1 += num_wild;
        if counts[0].1 == 5 {
            return Hand::FIVE;
        }
        if counts[0].1 == 4 {
            return Hand::FOUR;
        }
        if counts[0].1 == 3 && counts[1].1 == 2 {
            return Hand::HOUSE;
        }
        if counts[0].1 == 3 {
            return Hand::THREE;
        }
        if counts[0].1 == 2 && counts[1].1 == 2 {
            return Hand::TWO;
        }
        if counts[0].1 == 2 {
            return Hand::ONE;
        }
        return Hand::HIGH;
    }

    fn set_wild(mut self, wild: Option<char>) -> Self {
        self.class = Hand::classify_wild(&self.cards, wild);
        self.wild = wild;
        self
    }

    fn compare_cards(a: &char, b: &char, wild: Option<char>) -> Option<Ordering> {
        if a.is_ascii_digit() && b.is_ascii_digit() {
            return a.partial_cmp(b);
        }
        let mut values = FACE_VALUE.clone();
        if let Some(w) = wild {
            values = values
                .iter()
                .map(|c| *c)
                .filter(|c| *c != w)
                .collect::<Vec<char>>();
            values.insert(0, w);
        }
        let a_value = values.iter().position(|&r| r == *a);
        let b_value = values.iter().position(|&r| r == *b);
        if a_value.is_none() || b_value.is_none() {
            return None;
        }
        return a_value.partial_cmp(&b_value);
    }
}

impl Deck {
    fn judge(&self) -> Self {
        let mut sorted = self.clone();
        sorted.hands.sort();
        sorted
    }

    fn score(&self) -> u32 {
        let sorted = self.judge();
        sorted
            .hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u32 * h.bid)
            .sum()
    }

    fn set_wild(mut self, wild: Option<char>) -> Self {
        self.wild = wild;
        self.hands = self
            .hands
            .iter()
            .map(|h| h.clone().set_wild(self.wild))
            .collect();
        self
    }
}
fn main() {
    let f = File::open("input/007.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let deck = Deck::from(&lines);
    println!("score for this play is {}", deck.score());
    let wild_deck = deck.set_wild(Some('J'));
    println!("Jack's wild: {}", wild_deck.score());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_parse() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let deck = Deck::from(&lines);
        assert_eq!(deck.hands.len(), 5);
        assert_eq!(deck.hands[0].bid, 765);
        assert_eq!(deck.hands[1].bid, 684);
        assert_eq!(deck.hands[2].bid, 28);
        assert_eq!(deck.hands[3].bid, 220);
        assert_eq!(deck.hands[4].bid, 483);
        assert_eq!(deck.hands[2].cards[0], 'K');
        assert_eq!(deck.hands[2].cards[1], 'K');
        assert_eq!(deck.hands[2].cards[2], '6');
        assert_eq!(deck.hands[2].cards[3], '7');
        assert_eq!(deck.hands[2].cards[4], '7');
    }

    #[test]
    fn test_classify() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let deck = Deck::from(&lines);
        assert_eq!(Hand::classify(&vec!['1', '2', '3', '4', '5']), Hand::HIGH);
        assert_eq!(Hand::classify(&vec!['4', '4', '3', '3', '4']), Hand::HOUSE);
        assert_eq!(Hand::classify(&vec!['4', '4', '3', '4', '4']), Hand::FOUR);
        assert_eq!(Hand::classify(&vec!['A', 'A', 'A', 'A', 'A']), Hand::FIVE);
        assert_eq!(Hand::classify(&deck.hands[0].cards), Hand::ONE); // 32T3K
        assert_eq!(Hand::classify(&deck.hands[2].cards), Hand::TWO); // KK677
        assert_eq!(Hand::classify(&deck.hands[3].cards), Hand::TWO); // KTJJT
        assert_eq!(Hand::classify(&deck.hands[1].cards), Hand::THREE); // T55J5
        assert_eq!(Hand::classify(&deck.hands[4].cards), Hand::THREE); // QQQJA
    }

    #[test]
    fn test_classify_wild() {
        assert_eq!(
            Hand::classify_wild(&vec!['J', 'J', 'J', 'J', 'J'], Some('J')),
            Hand::FIVE
        );
        assert_eq!(
            Hand::classify_wild(&vec!['J', 'J', 'K', 'K', 'K'], Some('J')),
            Hand::FIVE
        );
        assert_eq!(
            Hand::classify_wild(&vec!['3', 'J', '3', '4', '4'], Some('J')),
            Hand::HOUSE
        );
        assert_eq!(
            Hand::classify_wild(&vec!['3', 'J', '3', 'J', '4'], Some('J')),
            Hand::FOUR
        );
        assert_eq!(
            Hand::classify_wild(&vec!['3', 'J', '3', '7', '4'], Some('J')),
            Hand::THREE
        );
        assert_eq!(
            Hand::classify_wild(&vec!['3', 'J', 'K', '7', '4'], Some('J')),
            Hand::ONE
        );
    }

    #[test]
    fn test_cmp_card() {
        assert_eq!(Hand::compare_cards(&'2', &'3', None), Some(Ordering::Less));
        assert_eq!(Hand::compare_cards(&'2', &'Y', None), None);
        assert_eq!(Hand::compare_cards(&'W', &'Y', None), None);
        assert_eq!(
            Hand::compare_cards(&'Q', &'5', None),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Hand::compare_cards(&'Q', &'T', None),
            Some(Ordering::Greater)
        );
        assert_eq!(Hand::compare_cards(&'2', &'A', None), Some(Ordering::Less));
        assert_eq!(Hand::compare_cards(&'2', &'J', None), Some(Ordering::Less));
        assert_eq!(
            Hand::compare_cards(&'2', &'J', Some('J')),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn test_cmp_hand() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let deck = Deck::from(&lines);
        assert!(deck.hands[0] == deck.hands[0]);
        assert!(deck.hands[1] == deck.hands[1]);
        assert!(deck.hands[2] == deck.hands[2]);
        assert!(deck.hands[3] == deck.hands[3]);
        assert!(deck.hands[4] == deck.hands[4]);
        assert!(deck.hands[0] < deck.hands[2]); // 32T3K < KK677
        assert!(deck.hands[2] > deck.hands[3]); // KK677 > KTJJT
        assert!(deck.hands[4] > deck.hands[2]); // QQQJA > KK677
        assert!(deck.hands[4] > deck.hands[1]); // QQQJA > T55J5
        let a = Hand::from(&"KTJJT 220".to_string());
        let b = Hand::from(&"KJTJT 220".to_string());
        assert!(a < b); // KTJJT < KJTJT
    }

    #[test]
    fn test_cmp_wild_hand() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let deck = Deck::from(&lines).set_wild(Some('J'));
        assert!(deck.hands[0] == deck.hands[0]);
        assert!(deck.hands[1] == deck.hands[1]);
        assert!(deck.hands[2] == deck.hands[2]);
        assert!(deck.hands[3] == deck.hands[3]);
        assert!(deck.hands[4] == deck.hands[4]);
        assert!(deck.hands[0] < deck.hands[2]); // 32T3K < KK677
        assert!(deck.hands[2] < deck.hands[3]); // KK677 < KTJJT (KTTTT)
        assert!(deck.hands[4] > deck.hands[2]); // QQQJA > KK677
        assert!(deck.hands[4] > deck.hands[1]); // QQQJA > T55J5
        let a = Hand::from(&"KTJJT 220".to_string()).set_wild(Some('J'));
        let b = Hand::from(&"KJTJT 220".to_string()).set_wild(Some('J'));
        assert!(a > b); // KTJJT > KJTJT
    }

    #[test]
    fn test_judge() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let deck = Deck::from(&lines);
        let deck = deck.judge();
        assert_eq!(deck.hands[0].bid, 765);
        assert_eq!(deck.hands[1].bid, 220);
        assert_eq!(deck.hands[2].bid, 28);
        assert_eq!(deck.hands[3].bid, 684);
        assert_eq!(deck.hands[4].bid, 483);
    }

    #[test]
    fn test_round() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let deck = Deck::from(&lines);
        assert_eq!(deck.score(), 6440);
    }

    #[test]
    fn test_wild_round() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let wild_deck = Deck::from(&lines).set_wild(Some('J'));
        assert_eq!(wild_deck.score(), 5905);
    }
}
