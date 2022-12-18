use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::{fs, fmt};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

type Key = u64;
type MoveLabel = u64;

#[derive(Debug)]
struct Valve {
    key: Key,
    rate: u32,
    tunnels: Vec<Key>,
    open: bool,
}

#[derive(Debug)]
struct Network {
    valves: HashMap<Key, Valve>,
}

#[derive(Clone, Copy)]
enum Action {
     Open(Key),
     Move(Key),
     Wait
}
use Action::{Open, Move, Wait};

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Open(v) => write!(f, "Open({})", Valve::label(*v)),
            Move(v) => write!(f, "Move({})", Valve::label(*v)),
            Wait => write!(f, "Wait"),
        }
    }
}
impl Action {
    fn m(t: &str) -> Action {
        Move(Valve::key(t))
    }

    fn o(v: &str) -> Action {
        Open(Valve::key(v))
    }
}

#[derive(Debug, Clone)]
struct Plan {
    actions: VecDeque<Action>,
    score: u32,
}

impl Plan {
    fn empty(score: u32) -> Self{
        Plan { actions: VecDeque::new(), score: score }
    }

    fn insert(&mut self, net: &Network, action: &Action) -> Self {
        self.actions.push_front(*action);
        self.clone()
    }

    fn execute(&self, net: &Network, start: &Key) -> u32 {
        if self.actions.len() > 30 {
            panic!("plan is too long!");
        }
        let mut rate = 0;
        let mut total = 0;
        let nonvalve = Valve::none();
        let mut here = net.valves.get(start).unwrap_or(&nonvalve);
        for action in self.actions.iter() {
            total += rate;
            match action {
                Open(valve) => {
                    if valve != &here.key {
                        panic!("can't open {} from room {}", Valve::label(*valve), Valve::label(here.key));
                    }
                    if here.open {
                        panic!("Valve {} is already open!", Valve::label(here.key));
                    }
                    rate += here.rate;
                },
                Move(to) => {
                    if !here.tunnels.contains(to) {
                        panic!("can't get to {} from {}!", Valve::label(here.key), Valve::label(here.key));
                    }
                    here = net.valves.get(to).expect("unknwon node");
                },
                Wait => (),
            }
        }
        total
    }

}

impl Valve {
    fn none () -> Self {
        Valve { key:0, rate:0, tunnels: Vec::new(), open: false}
    }
    fn key(label: &str) -> Key {
        label.bytes().filter(|b| *b < 128u8).fold(0, |sum, b| (sum << 8) + (b as Key))
    }
    fn label(key: Key) -> String {
        unsafe {
            [char::from_u32_unchecked((key >> 8) as u32), char::from_u32_unchecked((key % 256) as u32)].iter().collect()
        }
    }
    fn keys(labels: &[&str]) -> Vec<Key> {
        labels.iter().map(|s| Valve::key(s)).collect()
    }
}

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
                let key = Valve::key(cap.get(1)
                    .expect("too few capture groups")
                    .as_str());
                let rate = cap.get(2)
                    .expect("too few capture groups")
                    .as_str()
                    .parse::<u32>()
                    .expect("not a capture groups");
                let tunnels = cap.get(3)
                    .expect("too few capture groups")
                    .as_str()
                    .split(", ")
                    .map(Valve::key)
                    .collect();
                Valve {
                    key,
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
            valves.insert(valve.key, valve);
        }
        Network { valves }
    }
}

impl Network {
    fn get(&self, label: &str) -> Option<&Valve> {
        self.valves.get(&Valve::key(label))
    }

    fn h(&self, id: &Key) -> u32{ 
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

    fn move_label(depth: usize, action: &Action) -> MoveLabel {
        ((depth as u64) << (32 as MoveLabel)) + 
            match action {
                Open(key) => *key as MoveLabel + 1 << 16,
                Move(key) => *key as MoveLabel,
                Wait => 0
            }
    }

    fn optimal_plan(&self, start: &str) -> Plan {
        let start = Valve::key(start);
        let (_, plan) = self.optimal_plan_worker(start, 0, 0, 10, &HashSet::new(), &mut HashMap::new());
        println!("{:?}", plan);
        plan
    }

    fn optimal_plan_worker(&self, start: Key, pressure: u32, rate: u32, depth: usize, open: &HashSet<Key>, memo: &mut HashMap<MoveLabel, Plan>)
    -> (HashMap<MoveLabel, Plan>, Plan) {
        let prefix = std::iter::repeat(" ").take(30 - depth).collect::<String>();
        if depth == 0 {
            return (memo.clone(), Plan::empty(pressure));
        }
        let pressure = pressure + rate;
        let start = self.valves.get(&start).expect("unknwon valve");
        let mut options = Vec::new();
        for neighbor in start.tunnels.iter() {
            options.push(Move(*neighbor));
        }
        if !open.contains(&start.key) {
            options.push(Open(start.key));
        }
        let mut best_plan = Plan::empty(0);
        for action in options {
            println!("{prefix}consider {:?}", action);
            let movelabel = Self::move_label(depth, &action);
            if let Some(remembered_plan) = memo.get(&movelabel) {
                if remembered_plan.score > best_plan.score {
                    best_plan = remembered_plan.clone();
                }
            } else {
                let mut new_plan = Plan::empty(0);
                match action {
                    Open(v) => {
                        let mut open = open.clone();
                        open.insert(v);  
                        let newrate = rate + self.valves.get(&v).expect("unknown valve").rate;
                        (*memo, new_plan) = self.optimal_plan_worker(v, pressure, newrate, depth - 1, &open, memo);

                    }
                    Move(t) => {
                        (*memo, new_plan) = self.optimal_plan_worker(t, pressure, rate, depth - 1, &open, memo);
                    }
                    Wait => panic!("contructed plans will never have a wait action"),
                }                        
                new_plan.insert(self, &action);
                if new_plan.score > best_plan.score {
                    memo.insert(movelabel, new_plan.clone());
                    best_plan = new_plan;
                }
            }
        }
        (memo.clone(), best_plan.clone())
    }
}

fn main() {
    let input = fs::read_to_string("input/016.txt").expect("file read error");
    let network: Network = Network::from(input.lines());
    println!("there are {} valves", network.valves.len());
    println!("the optimal plan releases {} inches of pressure", network.optimal_plan("AA").score);
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
        assert_eq!(network.get("AA").unwrap().rate, 0);
        assert_eq!(network.get("AA").unwrap().tunnels, Valve::keys(&["DD", "II", "BB"]));
        assert_eq!(network.get("BB").unwrap().rate, 13);
        assert_eq!(network.get("BB").unwrap().tunnels, Valve::keys(&["CC", "AA"]));
        assert_eq!(network.get("JJ").unwrap().rate, 21);
        assert_eq!(network.get("JJ").unwrap().tunnels, Valve::keys(&["II"]));
    }

#[test]
    fn test_valve_key() {
        assert_eq!(Valve::key("AA"), 16705);
        assert_eq!(Valve::label(Valve::key("AA")), "AA");
        assert_eq!(Valve::label(Valve::key("IQ")), "IQ");
    }

#[test]
    fn test_execute_plan() {
        let network: Network = Network::from(SAMPLE.lines());
        let mut actions = Vec::from([
                Action::m("DD"),
                Action::o("DD"),
                Action::m("CC"),
                Action::m("BB"),
                Action::o("BB"),
                Action::m("AA"),
                Action::m("II"),
                Action::m("JJ"),
                Action::o("JJ"),
                Action::m("II"),
                Action::m("AA"),
                Action::m("DD"),
                Action::m("EE"),
                Action::m("FF"),
                Action::m("GG"),
                Action::m("HH"),
                Action::o("HH"),
                Action::m("GG"),
                Action::m("FF"),
                Action::m("EE"),
                Action::o("EE"),
                Action::m("DD"),
                Action::m("CC"),
                Action::o("CC"),
            ]);
        while actions.len() < 30 {
            actions.push(Wait);
        }
        actions.reverse();
        let mut plan = Plan::empty(0);
        for action in actions {
            plan.insert(&network, &action);
        };
        // make sure the returned value is consistent
        assert_eq!(plan.execute(&network, &Valve::key("AA")), 1651);
    }

    #[test]
    fn test_move_label() {
        assert_ne!(Network::move_label(0, &Action::m("CC")), Network::move_label(1, &Action::m("CC")));
        assert_ne!(Network::move_label(1, &Action::m("CC")), Network::move_label(1, &Action::o("CC")));
        assert_ne!(Network::move_label(1, &Action::m("AA")), Network::move_label(1, &Action::o("CC")));
        assert_ne!(Network::move_label(3, &Action::m("CC")), Network::move_label(1, &Action::m("CC")));
        assert_ne!(Network::move_label(4, &Action::m("CC")), Network::move_label(1, &Action::m("DD")));
        assert_ne!(Network::move_label(4, &Action::m("CC")), Network::move_label(1, &Action::m("DC")));
        assert_ne!(Network::move_label(4, &Action::m("CC")), Network::move_label(1, &Action::m("CD")));
        assert_eq!(Network::move_label(30, &Action::m("CC")), Network::move_label(30, &Action::m("CC")));
        assert_eq!(Network::move_label(15, &Action::o("AA")), Network::move_label(15, &Action::o("AA")));
    }

    #[test]
    fn test_optimal_plan() {
        // assert!(false);
        let network: Network = Network::from(SAMPLE.lines());
        let plan = network.optimal_plan("AA");
        println!("found a plan: {:?}", plan);
        assert_eq!(plan.score, 1651);
    }

}