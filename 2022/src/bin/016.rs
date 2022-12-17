use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::{fs, fmt};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

type Label = u64;
type MoveLabel = u64;

#[derive(Debug)]
struct Valve {
    key: Label,
    rate: u32,
    tunnels: Vec<Label>,
    open: bool,
}

#[derive(Debug)]
struct Network {
    valves: HashMap<Label, Valve>,
}

#[derive(Clone)]
enum Action {
     Open(Label),
     Move(Label),
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
fn make_action(m: &str, t: &str) -> Action {
    let t = Valve::key(t);
    match m {
        "Move" => Move(t),
        "Open" => Open(t),
        _ => panic!("unknown action")
    }
}

#[derive(Debug, Clone)]
struct Plan {
    actions: VecDeque<Action>,
    score: u32,
}

impl Valve {
    fn key(label: &str) -> Label {
        label.bytes().filter(|b| *b < 128u8).fold(0, |sum, b| (sum << 8) + (b as Label))
    }
    fn label(key: Label) -> String {
        unsafe {
            [char::from_u32_unchecked((key >> 8) as u32), char::from_u32_unchecked((key % 256) as u32)].iter().collect()
        }
    }
    fn keys(labels: &[&str]) -> Vec<Label> {
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

    fn execute_plan(&self, plan: &Plan) -> Result<u32, &str> {
        if plan.actions.len() > 30 {
            return Err("plan is too long!");
        }
        let mut rate = 0;
        let mut total = 0;
        let mut here = self.valves.get(&Valve::key("AA")).expect("AA must exist");
        for action in plan.actions.iter() {
            total += rate;
            match action {
                Open(valve) => {
                    if valve != &here.key {
                        return Err("can't open valve from another room");
                    }
                    if here.open {
                        return Err("Valve is already open!");
                    }
                    rate += here.rate;
                },
                Move(valve) => {
                    if !here.tunnels.contains(valve) {
                        return Err("can't get there from here!");
                    }
                    here = self.valves.get(valve).expect("unknwon node");
                },
                Wait => (),
            }
        }
        Ok(total)
    }

    fn h(&self, id: &Label) -> u32{ 
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

    fn move_label(depth: &usize, action: &Action) -> MoveLabel {
        ((*depth as u64) << (32 as MoveLabel)) + 
            match action {
                Open(key) => *key as MoveLabel + 1 << 16,
                Move(key) => *key as MoveLabel,
                Wait => 0
            }
    }

    fn optimal_plan(&self, start: &str) -> Plan {
        let start = Valve::key(start);
        let (_, plan) = self.optimal_plan_worker(start, 0, 0, 30, &HashSet::new(), &mut HashMap::new());
        println!("{:?}", plan);
        plan
    }

    fn optimal_plan_worker(&self, start: Label, pressure: u32, rate: u32, depth: usize, open: &HashSet<Label>, memo: &mut HashMap<MoveLabel, Plan>)
    -> (HashMap<MoveLabel, Plan>, Plan) {
        let prefix = std::iter::repeat(" ").take(30 - depth).collect::<String>();
        println!("{prefix} at {depth}");
        if depth == 0 {
            return (memo.clone(), Plan { actions: VecDeque::new(), score: pressure});
        }
        let pressure = pressure + rate;
        let start = self.valves.get(&start).expect("unknwon valve");
        let mut best_plan = Plan { actions: VecDeque::new(), score: 0 }; 
        if !open.contains(&start.key) {
            let action = Open(start.key);
            println!("{prefix}consider {:?}", action);
            let movelabel = Self::move_label(&depth, &action);
            if let Some(remembered_plan) = memo.get(&movelabel) {
                if remembered_plan.score > best_plan.score {
                    best_plan = remembered_plan.clone();
                }
            } else {
                let mut open = open.clone();
                open.insert(start.key);
                let mut new_plan = Plan { actions: VecDeque::new(), score: 0 };
                (*memo, new_plan) = self.optimal_plan_worker(start.key, pressure, rate + start.rate, depth - 1, &open, memo);
                if new_plan.score > best_plan.score {
                    new_plan.actions.push_front(action);
                    memo.insert(movelabel, new_plan.clone());
                    best_plan = new_plan;
                }
            }
        }
        for id in start.tunnels.iter() {
            let action = Move(*id);
            println!("{prefix}consider {:?}", action);
            let movelabel = Self::move_label(&depth, &action);
            if let Some(remembered_plan) = memo.get(&movelabel) {
                if remembered_plan.score > best_plan.score {
                    best_plan = remembered_plan.clone();
                }
            } else {
                let mut new_plan = Plan { actions: VecDeque::new(), score: 0 };
                (*memo, new_plan) = self.optimal_plan_worker(*id, pressure, rate, depth - 1, &open, memo);
                if new_plan.score > best_plan.score {
                    new_plan.actions.push_front(action);
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
        let mut plan = Plan {
            actions: VecDeque::from([
                make_action("Move", "DD"),
                make_action("Open", "DD"),
                make_action("Move", "CC"),
                make_action("Move", "BB"),
                make_action("Open", "BB"),
                make_action("Move", "AA"),
                make_action("Move", "II"),
                make_action("Move", "JJ"),
                make_action("Open", "JJ"),
                make_action("Move", "II"),
                make_action("Move", "AA"),
                make_action("Move", "DD"),
                make_action("Move", "EE"),
                make_action("Move", "FF"),
                make_action("Move", "GG"),
                make_action("Move", "HH"),
                make_action("Open", "HH"),
                make_action("Move", "GG"),
                make_action("Move", "FF"),
                make_action("Move", "EE"),
                make_action("Open", "EE"),
                make_action("Move", "DD"),
                make_action("Move", "CC"),
                make_action("Open", "CC"),
            ]),
        score: 0 };
        while plan.actions.len() < 30 {
            plan.actions.push_back(Wait);
        }
        assert_eq!(network.execute_plan(&plan), Ok(1651));
    }

    #[test]
    fn test_optimal_plan() {
        let network: Network = Network::from(SAMPLE.lines());
        let plan = network.optimal_plan("AA");
        assert_eq!(network.execute_plan(&plan), Ok(plan.score));
        assert_eq!(plan.score, 1651);
    }

}