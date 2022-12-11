use std::cell::RefCell;
use std::fs;
use std::str::Lines;

type Id = usize;

struct Monkey {
    items: Vec<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    modulus: i32,
    accept: Id,
    reject: Id,
    looks: usize,
}

type Troop = Vec<RefCell<Monkey>>;

fn parse_troop<'paw>(input: Lines) -> Troop {
    let mut troop = Vec::new();
    let mut items = None;
    let mut op: Option<Box<dyn Fn(i32) -> i32>> = None;
    let mut modulus = None;
    let mut accept = None;
    let mut started = false;
    for line in input {
        if line.starts_with("Monkey ") {
            assert!(!started, "started a new monkey in te middle");
            started = true;
        } else if line.starts_with("  Starting items:") {
            items = Some(line.trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect());
        } else if line.starts_with("  Operation:") {
            match line.split(' ').last() {
                Some("old") => {
                    op = Some(Box::new(move |x| x * x));
                },
                Some(arg) => {
                    match arg.parse::<i32>() {
                        Ok(operand) => {
                            if line.contains("+") {
                                op = Some(Box::new(move |x| x + operand));
                            } else if line.contains("*") {
                                op = Some(Box::new(move |x| x * operand));
                            } else {
                                panic!("unknown operator on the monkey");
                            }
                        },
                        Err(_) => todo!(),
                    }
                }
                None => panic!("we already checked for a known good prefix")
            }
        } else if line.starts_with("  Test:") {
            modulus = Some(line.split(' ')
                .last()
                .expect("known good prefix at least")
                .parse::<i32>()
                .expect("couldn't parse modulus"));
        } else if line.starts_with("    If true:") {
            accept = Some(line.split(' ')
                .last()
                .expect("known good prefix at least")
                .parse::<usize>()
                .expect("couldn't parse accept"));

        } else if line.starts_with("    If false:") {
            let reject = Some(line.split(' ')
                .last()
                .expect("known good prefix at least")
                .parse::<usize>()
                .expect("couldn't parse reject"));

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

fn round<'round>(troop: &'round mut Troop) {
    for monkey_ref in troop.iter(){
        let mut monkey = monkey_ref.borrow_mut();
        for item in monkey.items.iter() {
            let updated_value = (monkey.op)(*item) / 3;
            let mut target = monkey.reject;
            if updated_value% monkey.modulus == 0 {
                target = monkey.accept;
            }
            troop[target].borrow_mut().items.push(updated_value);
        }
        monkey.looks += monkey.items.len();
        monkey.items.clear();
    }
}

fn monkey_business(troop: &Troop) -> usize {
    let mut looks = troop.iter().map(|m| m.borrow().looks).collect::<Vec<usize>>();
    looks.sort();
    looks.reverse();
    assert!(looks.len() >= 2, "too few monkeys to comtemplate any monkey business");
    looks[0] * looks[1]
}

fn main() {
    let input = fs::read_to_string("input/011.txt").expect("file read error");
    let mut troop = &mut parse_troop(input.lines());

    println!("there are {} monkeys", troop.len());
    for _ in 0..20 {
        round(&mut troop);
    }
    println!("the monkey business goes to {}", monkey_business(&troop));
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
        round(&mut troop);
        assert_eq!(troop[0].borrow().items, Vec::from([20, 23, 27, 26]));
        assert_eq!(troop[1].borrow().items, Vec::from([2080, 25, 167, 207, 401, 1046]));
        assert!(troop[2].borrow().items.is_empty());
        assert!(troop[3].borrow().items.is_empty());
    }

    #[test]
    fn test_monkey_business() {
        let mut troop = &mut parse_troop(SAMPLE.lines());
        for _ in 0..20 {
            round(&mut troop);
        }
        assert_eq!(troop[0].borrow().items, Vec::from([10, 12, 14, 26, 34]));
        assert_eq!(troop[1].borrow().items, Vec::from([245, 93, 53, 199, 115]));
        assert!(troop[2].borrow().items.is_empty());
        assert!(troop[3].borrow().items.is_empty());
        assert_eq!(monkey_business(troop), 10605);
    }
}
