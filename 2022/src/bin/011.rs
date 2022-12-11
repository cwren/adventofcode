use std::cell::RefCell;
use std::fs;
use std::str::Lines;

type Id = usize;

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    modulus: u64,
    accept: Id,
    reject: Id,
    looks: usize,
}

type Troop = Vec<RefCell<Monkey>>;

fn parse_troop(input: Lines) -> Troop {
    let mut troop = Vec::new();
    let mut items = None;
    let mut op: Option<Box<dyn Fn(u64) -> u64>> = None;
    let mut modulus = None;
    let mut accept = None;
    let mut started = false;
    for line in input {
        if line.starts_with("Monkey ") {
            assert!(!started, "started a new monkey in te middle");
            started = true;
        } else if line.starts_with("  Starting items:") {
            items = Some(
                line.trim_start_matches("  Starting items: ")
                    .split(", ")
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        } else if line.starts_with("  Operation:") {
            match line.split(' ').last() {
                Some("old") => {
                    op = Some(Box::new(move |x| x * x));
                }
                Some(arg) => match arg.parse::<u64>() {
                    Ok(operand) => {
                        if line.contains('+') {
                            op = Some(Box::new(move |x| x + operand));
                        } else if line.contains('*') {
                            op = Some(Box::new(move |x| x * operand));
                        } else {
                            panic!("unknown operator on the monkey");
                        }
                    }
                    Err(_) => todo!(),
                },
                None => panic!("we already checked for a known good prefix"),
            }
        } else if line.starts_with("  Test:") {
            modulus = Some(
                line.split(' ')
                    .last()
                    .expect("known good prefix at least")
                    .parse::<u64>()
                    .expect("couldn't parse modulus"),
            );
        } else if line.starts_with("    If true:") {
            accept = Some(
                line.split(' ')
                    .last()
                    .expect("known good prefix at least")
                    .parse::<usize>()
                    .expect("couldn't parse accept"),
            );
        } else if line.starts_with("    If false:") {
            let reject = Some(
                line.split(' ')
                    .last()
                    .expect("known good prefix at least")
                    .parse::<usize>()
                    .expect("couldn't parse reject"),
            );

            troop.push(RefCell::new(Monkey {
                items: items.expect("end of monkey with no items"),
                op: op.expect("end of monkey with no operation"),
                modulus: modulus.expect("end of monkey with no modulus"),
                accept: accept.expect("end of monkey with no accept target"),
                reject: reject.expect("end of monkey with no reject target"),
                looks: 0,
            }));
            items = None;
            op = None;
            modulus = None;
            accept = None;
            started = false;
        }
    }
    troop
}

fn round(troop: &mut Troop, fidget: &dyn Fn(u64) -> u64) {
    for monkey_ref in troop.iter() {
        let mut monkey = monkey_ref.borrow_mut();
        for value in monkey.items.iter() {
            let high_anxiety = (monkey.op)(*value);
            let updated_value = fidget(high_anxiety);
            let mut target = monkey.reject;
            if updated_value % monkey.modulus == 0 {
                target = monkey.accept;
            }
            troop[target].borrow_mut().items.push(updated_value);
        }
        monkey.looks += monkey.items.len();
        monkey.items.clear();
    }
}

fn monkey_business(troop: &Troop) -> usize {
    let mut looks = troop
        .iter()
        .map(|m| m.borrow().looks)
        .collect::<Vec<usize>>();
    looks.sort();
    looks.reverse();
    assert!(
        looks.len() >= 2,
        "too few monkeys to comtemplate any monkey business"
    );
    looks[0] * looks[1]
}

fn main() {
    let input = fs::read_to_string("input/011.txt").expect("file read error");
    let troop = &mut parse_troop(input.lines());

    println!("there are {} monkeys", troop.len());
    for _ in 0..20 {
        round(troop, &|w| w / 3);
    }
    println!("the monkey business goes to {}", monkey_business(troop));

    let troop = &mut parse_troop(input.lines());
    let lcm: u64 = troop
        .iter()
        .map(|m| m.borrow().modulus)
        .product();
    for _ in 0..10_000 {
        round(troop, &|w| w % lcm);
    }
    println!(
        "high anxiety monkey business goes to {}",
        monkey_business(troop)
    );
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = include_str!("../../input/011-sample.txt");

    #[test]
    fn test_parse_troop() {
        let troop: Troop = parse_troop(SAMPLE.lines());
        assert_eq!(troop.len(), 4);
    }

    #[test]
    fn test_round() {
        let mut troop = &mut parse_troop(SAMPLE.lines());
        round(&mut troop, &|w| w / 3);
        assert_eq!(troop[0].borrow().items, [20, 23, 27, 26]);
        assert_eq!(troop[1].borrow().items, [2080, 25, 167, 207, 401, 1046]);
        assert!(troop[2].borrow().items.is_empty());
        assert!(troop[3].borrow().items.is_empty());
    }

    #[test]
    fn test_monkey_business() {
        let mut troop = &mut parse_troop(SAMPLE.lines());
        for _ in 0..20 {
            round(&mut troop, &|w| w / 3);
        }
        assert_eq!(troop[0].borrow().items, [10, 12, 14, 26, 34]);
        assert_eq!(troop[1].borrow().items, [245, 93, 53, 199, 115]);
        assert!(troop[2].borrow().items.is_empty());
        assert!(troop[3].borrow().items.is_empty());
        assert_eq!(monkey_business(troop), 10605);
    }

    #[test]
    fn test_fidget() {
        let mut troop = &mut parse_troop(SAMPLE.lines());
        let lcm: u64 = troop
            .iter()
            .map(|m| m.borrow().modulus)
            .product();
        println!("{lcm}");
        let clocks = |w| w % lcm;
        round(&mut troop, &clocks);
        assert_eq!(troop[0].borrow().looks, 2);
        assert_eq!(troop[1].borrow().looks, 4);
        assert_eq!(troop[2].borrow().looks, 3);
        assert_eq!(troop[3].borrow().looks, 6);
        for _ in 1..20 {
            round(&mut troop, &clocks);
        }
        assert_eq!(troop[0].borrow().looks, 99);
        assert_eq!(troop[1].borrow().looks, 97);
        assert_eq!(troop[2].borrow().looks, 8);
        assert_eq!(troop[3].borrow().looks, 103);
        for _ in 20..1_000 {
            round(&mut troop, &clocks);
        }
        assert_eq!(troop[0].borrow().looks, 5204);
        assert_eq!(troop[1].borrow().looks, 4792);
        assert_eq!(troop[2].borrow().looks, 199);
        assert_eq!(troop[3].borrow().looks, 5192);
    }

    #[test]
    fn test_big_worres() {
        let mut troop = &mut parse_troop(SAMPLE.lines());
        let lcm: u64 = troop
            .iter()
            .map(|m| m.borrow().modulus)
            .product();
        for _ in 0..10_000 {
            round(&mut troop, &|w| w % lcm);
        }
        assert_eq!(monkey_business(troop), 2713310158);
    }
}
