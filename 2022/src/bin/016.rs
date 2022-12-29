use lazy_static::lazy_static;
use priority_queue::DoublePriorityQueue;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fmt, fs};

type Key = u64;

#[derive(Debug)]
struct Valve {
    key: Key,
    rate: usize,
    tunnels: HashMap<Key, usize>,
    open: bool,
}

#[derive(Debug)]
struct Network {
    valves: HashMap<Key, Valve>,
    routes: HashMap<(Key, Key), usize>,
}

#[derive(Clone, Copy)]
enum Action {
    Open(Key),
    Move(Key),
    Wait,
}
use Action::{Move, Open, Wait};

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Open(v) => write!(f, "Open({})", Valve::label(*v)),
            Move(v) => write!(f, "Move({})", Valve::label(*v)),
            Wait => write!(f, "Wait"),
        }
    }
}

#[derive(Debug, Clone)]
struct Plan {
    actions: VecDeque<Action>,
    score: usize,
}

impl Plan {
    fn empty(score: usize) -> Self {
        Plan {
            actions: VecDeque::new(),
            score: score,
        }
    }

    fn insert(&mut self, action: &Action) -> Self {
        self.actions.push_front(*action);
        self.clone()
    }

    fn execute(&self, net: &Network, start: &Key) -> usize {
        if self.actions.len() > 30 {
            panic!("plan is too long!");
        }
        let mut rate = 0;
        let mut total = 0;
        let nonvalve = Valve::none();
        let mut here = net.valves.get(start).unwrap_or(&nonvalve);
        let mut time = 0;
        for action in self.actions.iter() {
            total += rate;
            time += 1;
            match action {
                Open(valve) => {
                    if valve != &here.key {
                        // warp
                        println!("warp to {}", Valve::label(*valve));
                        let steps = net.routes.get(&(here.key, *valve)).unwrap();
                        total += rate * steps;
                        time += steps;
                        here = net.valves.get(valve).expect("unknwon node");
                    } else {
                        println!("open {}", Valve::label(*valve));
                    }
                    if here.open {
                        panic!("Valve {} is already open!", Valve::label(here.key));
                    }
                    rate += here.rate;
                }
                Move(to) => {
                    if !here.tunnels.contains_key(to) {
                        panic!(
                            "can't get to {} from {}!",
                            Valve::label(here.key),
                            Valve::label(here.key)
                        );
                    }
                    here = net.valves.get(to).expect("unknwon node");
                    println!("move {}", Valve::label(*to));
                }
                Wait => {
                    total += rate * (30 - time);
                }
            }
        }
        total
    }
}

impl Valve {
    fn none() -> Self {
        Valve {
            key: 0,
            rate: 0,
            tunnels: HashMap::new(),
            open: false,
        }
    }
    fn key(label: &str) -> Key {
        label
            .bytes()
            .filter(|b| *b < 128u8)
            .fold(0, |sum, b| (sum << 8) + (b as Key))
    }
    fn label(key: Key) -> String {
        unsafe {
            [
                char::from_u32_unchecked((key >> 8) as u32),
                char::from_u32_unchecked((key % 256) as u32),
            ]
            .iter()
            .collect()
        }
    }
}

lazy_static! {
    static ref RE: regex::Regex = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)$"
    )
    .unwrap();
}

impl From<&str> for Valve {
    fn from(s: &str) -> Self {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        match RE.captures(s) {
            Some(cap) => {
                let label = cap.get(1).expect("too few capture groups").as_str();
                let key = Valve::key(label);
                let rate = cap
                    .get(2)
                    .expect("too few capture groups")
                    .as_str()
                    .parse::<usize>()
                    .expect("not a capture groups");
                let tunnels = cap
                    .get(3)
                    .expect("too few capture groups")
                    .as_str()
                    .split(", ")
                    .map(Valve::key)
                    .map(|k| (k, 1))
                    .collect::<HashMap<Key, usize>>();
                let mut open = false;
                if rate == 0 {
                    open = true;
                }
                Valve {
                    key,
                    rate,
                    tunnels,
                    open,
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
        let routes = HashMap::new();
        let mut net = Network { valves, routes };
        net.find_routes();
        net
    }
}

impl Network {
    fn h(&self, f: Key, t: Key) -> usize {
        if f == t {
            return 0;
        }
        if self.valves.get(&f).unwrap().tunnels.contains_key(&t) {
            return 1;
        } else {
            return 2;
        }
    }

    fn find_routes(&mut self) {
        for (ka, _) in self.valves.iter() {
            for (kb, _) in self.valves.iter() {
                if ka <= kb {
                    if let Some(n) = self.shortest_path(*ka, *kb) {
			self.routes.insert((*ka, *kb), n);
			self.routes.insert((*kb, *ka), n);
		    } else {
			panic!("No path from {} to {}", Valve::label(*ka), Valve::label(*kb));
		    }
		}
            }
        }
    }

    fn shortest_path(&self, from: Key, to: Key) -> Option<usize> {
        // https://en.wikipedia.org/wiki/A*_search_algorithm
        let longest = self.valves.len() + 1;
        let mut open = DoublePriorityQueue::new();
        let current = from;
        open.push(current, self.h(current, to));

        let mut g_score = HashMap::new();
        g_score.insert(current, 0usize);

        let mut from = HashMap::new();
        while !open.is_empty() {
            let (current, _) = open.pop_min().expect("while says it's not empty");
            if current == to {
                // unwind
                let mut path = Vec::new();
                let mut p = current;
                while let Some(q) = from.get(&p) {
                    path.push(*q);
                    p = *q;
                }
                path.reverse();
                return Some(path.len());
            }
            for neighbor in self.valves.get(&current).unwrap().tunnels.keys() {
                let tentative_g_score = 1 + g_score.get(&current).unwrap_or(&longest);
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&longest) {
                    from.insert(*neighbor, current);
                    g_score.insert(*neighbor, tentative_g_score);
                    open.push(*neighbor, tentative_g_score + self.h(*neighbor, to));
                }
            }
        }
        None
    }

    fn optimal_plan(&self, start: &str) -> usize {
        let start = Valve::key(start);
        let mut closed = HashSet::new();
        for (k, v) in self.valves.iter() {
            if !v.open {
                closed.insert(*k);
            }
        }
        let plan = self.optimal_plan_worker(
            start, 0,  // pressure
            0,  // rate
            30, // depth
            &mut closed,
        );
        println!("best plan is: {:?}", plan.actions);
        println!("presumptive score is {}", plan.score);
        println!("actual score is {}", plan.execute(self, &Valve::key("AA")));
        plan.score
    }

    fn utility(&self, f: Key, t: Key, closed: &HashSet<Key>, depth: usize) -> f64 {
	let reward = self.valves.get(&t).expect("unknwon valve").rate as f64;
	let there = *self.routes.get(&(f, t)).expect("unknwon route");
	if there > depth {
	    return 0_f64;
	}
	let there = there as f64;
	let mut back = usize::MAX;
	for k in closed {
	    if t != *k {
		back = back.min(*self.routes.get(&(t, *k)).expect("unknwon route"));
	    }
	}
	let back = back as f64;
	reward / (there + back)
    }

    fn optimal_plan_worker(
        &self,
        start: Key,
        pressure: usize,
        rate: usize,
        depth: usize,
        closed: &mut HashSet<Key>,
    ) -> Plan {
	println!("{} @ {}", Valve::label(start), depth);
        if depth == 0 {
	    println!("end of time: {}", pressure);
            return Plan::empty(pressure);
        }
        let start = self.valves.get(&start).expect("unknown valve");
        let rate = rate + start.rate;
        closed.remove(&start.key);
        if closed.is_empty() {
            let mut wait = Plan::empty(pressure);
            wait.score = pressure + (depth * rate);
            wait.insert(&Wait);
            if start.rate > 0 {
                wait.insert(&Open(start.key));
            }
            return wait;
        }
	let mut best = (0, 0_f64);
        for next in closed.iter() {
            let guess = self.utility(start.key, *next, closed, depth);
            if guess > best.1 {
                best = (*next, guess);
            }
        }
        if let Some(steps) = self.routes.get(&(start.key, best.0)) {
	    let steps = steps + 1;
	    let pressure = pressure + (rate * steps);
	    println!("stepping {} to {}", steps, Valve::label(best.0));
	    let mut new_plan = self.optimal_plan_worker(best.0, pressure, rate, depth - steps, closed);
            if start.rate > 0 {
		new_plan.insert(&Open(start.key));
            }
            return new_plan;
	} else {
	    // nowhere left to go
            let mut wait = Plan::empty(pressure);
            wait.score = pressure + (depth * rate);
            wait.insert(&Wait);
            if start.rate > 0 {
                wait.insert(&Open(start.key));
            }
            return wait;
	}
    }
}

fn main() {
    let input = fs::read_to_string("input/016.txt").expect("file read error");
    let network: Network = Network::from(input.lines());
    println!("there are {} valves", network.valves.len());
    println!(
        "the optimal plan releases {} inches of pressure",
        network.optimal_plan("AA")
    );
    // 1745 is too low
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

    impl Valve {
        fn keys(labels: &[&str]) -> HashMap<Key, usize> {
            labels.iter().map(|s| (Valve::key(s), 1)).collect()
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

    impl Network {
        fn get(&self, label: &str) -> Option<&Valve> {
            self.valves.get(&Valve::key(label))
        }
    }

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
        assert_eq!(
            network.get("AA").unwrap().tunnels,
            Valve::keys(&["DD", "II", "BB"])
        );
        assert_eq!(network.get("BB").unwrap().rate, 13);
        assert_eq!(
            network.get("BB").unwrap().tunnels,
            Valve::keys(&["CC", "AA"])
        );
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
            Wait,
        ]);
        actions.reverse();
        let mut plan = Plan::empty(0);
        for action in actions {
            plan.insert(&action);
        }
        // make sure the returned value is consistent
        assert_eq!(plan.execute(&network, &Valve::key("AA")), 1651);
    }

    #[test]
    fn test_shortcut_plan() {
        let network: Network = Network::from(SAMPLE.lines());
        let mut actions = Vec::from([
            Action::o("DD"),
            Action::o("BB"),
            Action::o("JJ"),
            Action::o("HH"),
            Action::o("EE"),
            Action::o("CC"),
            Wait,
        ]);
        actions.reverse();
        let mut plan = Plan::empty(0);
        for action in actions {
            plan.insert(&action);
        }
        // make sure the returned value is consistent
        assert_eq!(plan.execute(&network, &Valve::key("AA")), 1651);
    }

    #[test]
    fn test_simplify() {
        let aa = Valve::key("AA");
        let bb = Valve::key("BB");
        let jj = Valve::key("JJ");
        let hh = Valve::key("HH");
        let network: Network = Network::from(SAMPLE.lines());
        assert_eq!(network.shortest_path(aa, aa).unwrap(), 0);
        assert_eq!(network.shortest_path(aa, bb).unwrap(), 1);
        assert_eq!(network.shortest_path(aa, jj).unwrap(), 2);
        assert_eq!(network.shortest_path(aa, hh).unwrap(), 5);
        assert_eq!(network.shortest_path(jj, hh).unwrap(), 7);

        assert_eq!(network.routes.get(&(aa, aa)).unwrap(), &0);
        assert_eq!(network.routes.get(&(aa, bb)).unwrap(), &1);
        assert_eq!(network.routes.get(&(aa, jj)).unwrap(), &2);
        assert_eq!(network.routes.get(&(aa, hh)).unwrap(), &5);
        assert_eq!(network.routes.get(&(hh, aa)).unwrap(), &5);
        assert_eq!(network.routes.get(&(jj, hh)).unwrap(), &7);
    }

    #[test]
    fn test_optimal_plan() {
        let network: Network = Network::from(SAMPLE.lines());
        println!("expecting DD, BB, JJ, HH, EE, CC, Wait");
        assert_eq!(network.optimal_plan("AA"), 1651);
    }
}
