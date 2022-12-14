use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use pest::iterators::Pair;
use pest::Parser;
use std::cmp::Ordering;
use std::fs;
use std::str::Lines;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "bin/013.pest"]
struct ListParser;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}
use crate::Item::{List, Number};

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Number(a) => match other {
                Number(b) => a.cmp(b),
                List(_) => {
                    let tmp = Vec::from([Number(*a)]);
                    List(tmp).cmp(other)
                }
            },
            List(l) => match other {
                Number(b) => {
                    let tmp = Vec::from([Number(*b)]);
                    self.cmp(&List(tmp))
                }
                List(m) => {
                    for pair in l.iter().zip_longest(m) {
                        match pair {
                            Left(_) => return Ordering::Greater,
                            Right(_) => return Ordering::Less,
                            Both(a, b) => match a.cmp(b) {
                                Ordering::Less => return Ordering::Less,
                                Ordering::Greater => return Ordering::Greater,
                                Ordering::Equal => (),
                            },
                        }
                    }
                    Ordering::Equal
                }
            },
        }
    }
}

fn unpack_token(token: Pair<Rule>) -> Item {
    match token.as_rule() {
        Rule::number => return Number(token.as_str().parse().expect("parser says so")),
        Rule::list => {
            let mut items = Vec::new();
            for inner_token in token.into_inner() {
                items.push(unpack_token(inner_token));
            }
            List(items)
        }
    }
}

fn parse_packets(lines: Lines) -> Vec<Item> {
    let mut items = Vec::new();
    for line in lines {
        if !line.is_empty() {
            let mut tokens =
                ListParser::parse(Rule::list, line).unwrap_or_else(|e| panic!("{}", e));
            let item = unpack_token(tokens.next().expect("at least one token per lline"));
            items.push(item);
        }
    }
    items
}

fn count_correct_orders(items: &[Item]) -> usize {
    items
        .iter()
        .tuple_windows()
        .step_by(2)
        .map(|(a, b)| a < b)
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(n, _)| n + 1)
        .sum()
}

fn find_decoder_key(items: &mut Vec<Item>) -> usize {
    let div1 = List(Vec::from([List(Vec::from([Number(2)]))]));
    let div2 = List(Vec::from([List(Vec::from([Number(6)]))]));
    items.push(div1.clone());
    items.push(div2.clone());
    items.sort();
    items
        .iter()
        .enumerate()
        .filter(|(_, a)| div1.eq(a) || div2.eq(a))
        .map(|(n, _)| n + 1)
        .product::<usize>()
}

fn main() {
    let input = fs::read_to_string("input/013.txt").expect("file read error");
    println!("there are {} lines", input.lines().count());
    let mut items: Vec<Item> = parse_packets(input.lines());
    println!("ordering score is {}", count_correct_orders(&items));

    println!("decoder key is {}", find_decoder_key(&mut items));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_parser() {
        let items: Vec<Item> = parse_packets(SAMPLE.lines());
        assert_eq!(
            items[0],
            List(Vec::from([
                Number(1),
                Number(1),
                Number(3),
                Number(1),
                Number(1)
            ]))
        );
    }

    #[test]
    fn test_compare() {
        let items: Vec<Item> = parse_packets(SAMPLE.lines());
        assert!(items[0] < items[1]);
        println!("{:?} < {:?}", items[2], items[3]);
        assert!(items[2] < items[3]);
        assert!(items[4] > items[5]);
        assert!(items[6] < items[7]);
        assert!(items[8] > items[9]);
        assert!(items[10] < items[11]);
        assert!(items[12] > items[13]);
        assert!(items[14] > items[15]);
    }

    #[test]
    fn test_count_correct_orders() {
        let items: Vec<Item> = parse_packets(SAMPLE.lines());
        assert_eq!(count_correct_orders(&items), 13);
    }

    #[test]
    fn test_find_decoder_key() {
        let mut items: Vec<Item> = parse_packets(SAMPLE.lines());
        assert_eq!(find_decoder_key(&mut items), 140);
    }
}
