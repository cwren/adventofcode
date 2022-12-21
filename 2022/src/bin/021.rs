use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


lazy_static! {
    static ref NUMBER_MOKEY: regex::Regex = Regex::new(r"^([a-z]+): (\d+)$").unwrap();
    static ref OPERATOR_MOKEY: regex::Regex = Regex::new(r"^([a-z]+): ([a-z]+) (.) ([a-z]+)$").unwrap();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operator {
    Yell(i64),
    Add(u64, u64),
    Sub(u64, u64),
    Mul(u64, u64),
    Div(u64, u64),
    Equal(u64, u64),
}
use Operator::{Yell, Add, Sub, Mul, Div, Equal};


#[derive(Debug, Copy, Clone)]
struct Monkey {
    id: u64,
    op: Operator,
}

type Troop = HashMap<u64, Monkey>;

impl Monkey {
    fn name_to_id(name: &str) -> u64 {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        s.finish()
    }
    fn new(name: &str, op: Operator) -> Self {
        let name = String::from(name);
        let id = Self::name_to_id(&name);
        Monkey{ id, op}
    }
    fn find<'troop>(troop: &'troop Troop, name: &str) -> Option<&'troop Monkey> {
        troop.get(&Monkey::name_to_id(name))
    }
    fn evaluate(troop: &Troop) -> i64 {
        let root = Monkey::find(&troop, "root").expect("there is no root!");
        root.eval(troop)
    }
    fn eval(&self, troop: &Troop) -> i64 {
        match self.op {
            Yell(n) => n,
            Add(a, b) => troop.get(&a).unwrap().eval(troop) + troop.get(&b).unwrap().eval(troop),
            Sub(a, b) => troop.get(&a).unwrap().eval(troop) - troop.get(&b).unwrap().eval(troop),
            Mul(a, b) => troop.get(&a).unwrap().eval(troop) * troop.get(&b).unwrap().eval(troop),
            Div(a, b) => troop.get(&a).unwrap().eval(troop) / troop.get(&b).unwrap().eval(troop),
            Equal(_,_) => panic!("only root should hold an equality operator"),
        }
    }
    fn equality(troop: &Troop) -> Ordering {
        let root = Monkey::find(&troop, "root").expect("there is no root!");
        if let Equal(a, b) = root.op {
            let a = troop.get(&a).unwrap().eval(troop);
            let b = troop.get(&b).unwrap().eval(troop);
            return a.cmp(&b);
        }
        panic!("root is not an equaltiy monkey");
    }
    fn fix_human(troop: &mut HashMap<u64, Monkey>, value: i64) {
        let mut human = *Monkey::find(&*troop, "humn").unwrap();
        troop.remove(&human.id);
        human.op = Yell(value);
        troop.insert(human.id, human);
    }
    
    fn fix_root(troop: &mut HashMap<u64, Monkey>) {
        let mut root = *Monkey::find(&*troop, "root").unwrap();
        troop.remove(&root.id);
        let (a, b) = match root.op {
            Yell(_) => panic!("expected root to have a binary operator"),
            Add(a, b) => (a, b),
            Sub(a, b) => (a, b),
            Mul(a, b) => (a, b),
            Div(a, b) => (a, b),
            Equal(a, b) => (a, b),
        };
        root.op = Equal(a, b);
        troop.insert(root.id, root);
    }
    fn find_equality(troop: &mut Troop) -> i64 {
        Monkey::fix_root(troop);

        let mut lower_value = 1;
        Monkey::fix_human(troop, lower_value);
        let lower_order = Monkey::equality(troop);
        println!("lower order is {:?}", lower_order);
        
        let mut upper_value = 2;
        loop {
            println!("searching for upper: {upper_value}");
            Monkey::fix_human(troop, upper_value);
            if lower_order != Monkey::equality(troop) { break; }
            upper_value *= 2;
        }

        loop {
            let probe = lower_value + (upper_value - lower_value) / 2;
            println!("testing {probe}");
            Monkey::fix_human(troop, probe);
            let result = Monkey::equality(troop);
            if result == Ordering::Equal { return probe; }
            if result == lower_order {
                lower_value = probe;
            } else {
                upper_value = probe;
            }
        }
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        match NUMBER_MOKEY.captures(s) {
            Some(cap) => {
                // number monkey
                let name = cap.get(1).expect("missing name").as_str();
                let operand = cap.get(2).expect("missing number").as_str().parse::<i64>().expect("not a number");
                Monkey::new(name, Yell(operand))
            }
            None => 
            match OPERATOR_MOKEY.captures(s) {
                // operator monkey
                Some(cap) => {
                    let name = cap.get(1).expect("missing name").as_str();
                    let a = Monkey::name_to_id(cap.get(2).expect("missing operand a").as_str());
                    let op = cap.get(3).expect("missing operand a").as_str();
                    let b = Monkey::name_to_id(cap.get(4).expect("missing operand b").as_str());
                    let op = match op {
                        "+" => Add(a, b),
                        "-" => Sub(a, b),
                        "*" => Mul(a, b),
                        "/" => Div(a, b),
                        _ => panic!("unknown operaotr")
                    };
                    Monkey::new(name, op)
                }
                None => panic!("unparsable Monkey {s}"),      
            }      
        }
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/021.txt").expect("file read error");
    let mut troop: Troop = input.lines().map(Monkey::from).map(|m| (m.id, m)).collect();
    println!("there are {} monkeys", troop.len());
    println!("the troop will yell {}", Monkey::evaluate(&troop));
    println!("human yells {}", Monkey::find_equality(&mut troop));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    #[test]
    fn test_parse_troop() {
        let troop: Troop = SAMPLE.lines().map(Monkey::from).map(|m| (m.id, m)).collect();
        assert_eq!(troop.len(), 15);
        assert!(Monkey::find(&troop, "root").is_some());
        assert_eq!(Monkey::find(&troop, "root").unwrap().op, Add(Monkey::name_to_id("pppw"), Monkey::name_to_id("sjmn")));
        assert_eq!(Monkey::find(&troop, "sllz").unwrap().op, Yell(4));
    }

    #[test]
    fn test_execute() {
        let troop: Troop = SAMPLE.lines().map(Monkey::from).map(|m| (m.id, m)).collect();
        assert_eq!(Monkey::evaluate(&troop), 152);
    }

    #[test]
    fn test_equality() {
        let mut troop: Troop = SAMPLE.lines().map(Monkey::from).map(|m| (m.id, m)).collect();
        Monkey::fix_root(&mut troop);
        Monkey::fix_human(&mut troop, 301);
        assert_eq!(Monkey::equality(&troop), Ordering::Equal);
        Monkey::fix_human(&mut troop, 302);
        assert_eq!(Monkey::equality(&troop), Ordering::Equal); // !!
    }

    #[test]
    fn find_equality() {
        let mut troop: Troop = SAMPLE.lines().map(Monkey::from).map(|m| (m.id, m)).collect();
        assert!([301, 302].contains(&Monkey::find_equality(&mut troop)));
    }
}
