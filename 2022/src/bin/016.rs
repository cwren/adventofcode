use lazy_static::lazy_static;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
use priority_queue::PriorityQueue;

type Label = String;

#[derive(Debug)]
struct Valve {
    label: Label,
    rate: u32,
    tunnels: Vec<Label>,
    open: bool,
}

#[derive(Debug)]
struct Network {
    valves: HashMap<Label, Valve>,
}

enum Action {
     Open(String),
     Move(String)
}
use Action::{Open, Move};

type Plan = Vec<Action>;

lazy_static! {
    static ref RE: regex::Regex =
        Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)$")
            .unwrap();
}

impl From<&str> for Valve {
    fn from(s: &str) -> Self {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        match RE.captures(s) {
            Some(cap) => {
                let label = cap.get(1)
                    .expect("too few capture groups")
                    .as_str()
                    .to_string();
                let rate = cap.get(2)
                    .expect("too few capture groups")
                    .as_str()
                    .parse::<u32>()
                    .expect("not a capture groups");
                let tunnels = cap.get(3)
                    .expect("too few capture groups")
                    .as_str()
                    .split(", ")
                    .map(String::from)
                    .collect();
                Valve {
                    label,
                    rate,
                    tunnels,
                    open: false,
                }
            }
            None => panic!("unparsable input: {}", s),
        }
    }
}

impl From<std::str::Lines<'_>> for Network {
    fn from(input: std::str::Lines) -> Self {
        let mut valves = HashMap::new();
        for line in input {
            let valve = Valve::from(line);
            valves.insert(valve.label.clone(), valve);
        }
        Network { valves }
    }
}

impl Network {
    fn execute_plan(&self, plan: &Plan) -> Result<u32, &str> {
        if plan.len() > 30 {
            return Err("plan is too long!");
        }
        let mut rate = 0;
        let mut total = 0;
        let mut actions = plan.iter();
        let mut here = self.valves.get("AA").expect("AA must exist");
        for t in 0..30 {
            total += rate;
            match actions.next() {
                Some(action) => match action {
                    Action::Open(valve) => {
                        if valve != &here.label {
                            return Err("can't open valve from another room");
                        }
                        if here.open {
                            return Err("Valve is already open!");
                        }
                        rate += here.rate;
                    },
                    Action::Move(valve) => {
                        if !here.tunnels.contains(valve) {
                            return Err("can't get there from here!");
                        }
                        here = self.valves.get(valve).expect("unknwon node");
                    },
                },
                None => (),
            }
        }
        Ok(total)
    }

    fn h(&self, id: &str) -> u32{ 
        // my rate if closed, or best discounted rate of closed neighbors 
        let mut h = 0;
        if let Some(valve) = self.valves.get(id) {
            if valve.open {
                h = valve.rate;
            }
            for neighbor in valve.tunnels.iter() {
                if let Some(neighbor_valve) = self.valves.get(neighbor) {
                    if neighbor_valve.open {
                        h = h.max(neighbor_valve.rate / 2);
                    }
                }
            }
        }
        h
    }

    fn all_open(&self) -> bool {
        self.valves.iter().all(|(_, v)| v.open)
    }

    fn optimal_plan(&self, start: &str) -> Option<Plan> {
        // https://en.wikipedia.org/wiki/A*_search_algorithm
        let longest = self.valves.len() as u32;
        let mut open = PriorityQueue::new();
        open.push(start, self.h(&start));

        let mut g_score = HashMap::new();
        g_score.insert(start.to_string(), 0u32);

        let mut from = HashMap::new();

        while !open.is_empty() {
            let (current, _) = open.pop().expect("while says it's not empty");
            let plan = unwind(current, &from);
            if self.all_open() || plan.len() == 30 {
                return Some(plan);
            }
            let valve = self.valves.get(current).expect("unknown valve");
            for next in valve.tunnels.iter() {
                if let Some(neighbor) = self.valves.get(next) {
                    let mut tentative_g_score = 
                        g_score.get(current).unwrap_or(&longest) +
                        if neighbor.open { 1 } else { 2 };
                    if tentative_g_score < *g_score.get(next).unwrap_or(&longest) {
                        from.insert(next.clone(), current.to_string());
                        g_score.insert(next.clone(), tentative_g_score);
                        open.push(&next, tentative_g_score + self.h(&next));
                    }
                }
            }
        }
        None
    }


}

fn unwind<'path>(current: &str, from: &HashMap<String, String>) -> Plan {
    let mut path = Vec::new();
    let mut p = current;
    while let Some(q) = from.get(p) {
        if p == *q {
            path.push(Action::Open(q.to_string()));
        } else { 
            path.push(Action::Move(q.to_string()));
        }
        p = q;
    }
    path.reverse();
    path
}

fn main() {
    let input = fs::read_to_string("input/016.txt").expect("file read error");
    let network: Network = Network::from(input.lines());
    println!("there are {} valves", network.valves.len());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

#[test]
fn test_regex() {
    let mut input = SAMPLE.lines();
    let cap = RE.captures(input.next().unwrap()).unwrap();
    assert_eq!(cap.get(1).unwrap().as_str(), "AA");
    assert_eq!(cap.get(2).unwrap().as_str().parse::<u32>().unwrap(), 0);
    assert_eq!(cap.get(3).unwrap().as_str(), "DD, II, BB");

    let cap = RE.captures(input.next().unwrap()).unwrap();
    assert_eq!(cap.get(1).unwrap().as_str(), "BB");
    assert_eq!(cap.get(2).unwrap().as_str().parse::<u32>().unwrap(), 13);
    assert_eq!(cap.get(3).unwrap().as_str(), "CC, AA");

    input.next().unwrap(); //CC
    input.next().unwrap(); //DD
    input.next().unwrap(); //EE
    input.next().unwrap(); //FF
    input.next().unwrap(); //GG
    let cap = RE.captures(input.next().unwrap()).unwrap();
    assert_eq!(cap.get(1).unwrap().as_str(), "HH");
    assert_eq!(cap.get(2).unwrap().as_str().parse::<u32>().unwrap(), 22);
    assert_eq!(cap.get(3).unwrap().as_str(), "GG");
}

#[test]
    fn test_parse() {
        let network: Network = Network::from(SAMPLE.lines());
        assert_eq!(network.valves.len(), 10);
        assert_eq!(network.valves.get("AA").unwrap().rate, 0);
        assert_eq!(network.valves.get("AA").unwrap().tunnels, Vec::from(["DD", "II", "BB"]));
        assert_eq!(network.valves.get("BB").unwrap().rate, 13);
        assert_eq!(network.valves.get("BB").unwrap().tunnels, Vec::from(["CC", "AA"]));
        assert_eq!(network.valves.get("JJ").unwrap().rate, 21);
        assert_eq!(network.valves.get("JJ").unwrap().tunnels, Vec::from(["II"]));
    }

    #[test]
    fn test_execute_plan() {
        let network: Network = Network::from(SAMPLE.lines());
        let plan = Vec::from([
            Move("DD".to_string()),
            Open("DD".to_string()),
            Move("CC".to_string()),
            Move("BB".to_string()),
            Open("BB".to_string()),
            Move("AA".to_string()),
            Move("II".to_string()),
            Move("JJ".to_string()),
            Open("JJ".to_string()),
            Move("II".to_string()),
            Move("AA".to_string()),
            Move("DD".to_string()),
            Move("EE".to_string()),
            Move("FF".to_string()),
            Move("GG".to_string()),
            Move("HH".to_string()),
            Open("HH".to_string()),
            Move("GG".to_string()),
            Move("FF".to_string()),
            Move("EE".to_string()),
            Open("EE".to_string()),
            Move("DD".to_string()),
            Move("CC".to_string()),
            Open("CC".to_string()),
        ]);
        assert_eq!(network.execute_plan(&plan), Ok(1651));
    }

    #[test]
    fn test_optimal_plan() {
        let network: Network = Network::from(SAMPLE.lines());
        assert!(network.optimal_plan("AA").is_some());
    }
}