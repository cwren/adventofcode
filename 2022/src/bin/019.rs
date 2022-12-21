use lazy_static::lazy_static;
use regex::Regex;
use std::char::MAX;
use std::fs;
use std::collections::VecDeque;
use std::cmp::Ordering::{Less, Equal, Greater};

const MAX_DEPTH: usize = 10;

lazy_static! {
    static ref RE: regex::Regex =
        Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")
            .unwrap();
}
#[derive(Debug, Copy, Clone)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: [u32; 2], // ore & clay
    geode: [u32; 2],    // ore & obsidian
}

fn cap_to_u32(cap: Option<regex::Match>) -> u32 {
    cap.expect("too few numbers")
        .as_str()
        .parse::<u32>()
        .expect("not a number")
}

impl From<&str> for Blueprint {
    fn from(s: &str) -> Self {
        match RE.captures(s) {
            Some(cap) => {
                let id = cap_to_u32(cap.get(1));
                let ore = cap_to_u32(cap.get(2));
                let clay = cap_to_u32(cap.get(3));
                let obsidian = [cap_to_u32(cap.get(4)), cap_to_u32(cap.get(5))];
                let geode = [cap_to_u32(cap.get(6)), cap_to_u32(cap.get(7))];
                Blueprint {
                    id,
                    ore,
                    clay,
                    obsidian,
                    geode,
                }
            }
            None => panic!("unpaseable blueprint {s}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Ore,
    Clay,
    Obsidian,
    Geode
}
use Instruction::{Ore, Clay, Obsidian, Geode};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Plan {
    instructions: VecDeque<Instruction>,
}

impl From<&Vec<Instruction>> for Plan {
    fn from(input: &Vec<Instruction>) -> Self {
        Plan { instructions: VecDeque::from(input.clone()) }
    }
}

impl Plan {
    
    fn new() -> Self {
        Plan { instructions: VecDeque::new() }
    }

    fn push_back(&mut self, i: Instruction) {
        self.instructions.push_back(i)
    }

    fn push_front(&mut self, i: Instruction) {
        self.instructions.push_front(i)
    }

    fn pop_front(&mut self) -> Option<Instruction> {
        self.instructions.pop_front()
    }

    fn front(&mut self) -> Option<&Instruction> {
        self.instructions.front()
    }

    fn is_empty(&self) -> bool{
        self.instructions.is_empty()
    }

    fn optimize(blueprint: Blueprint) -> Plan {
        Plan::optimize_worker(blueprint, &State::new(), &Plan::new()).unwrap()
    }

    fn optimize_worker(blueprint: Blueprint, state: &State, plan: &Plan) -> (State, Plan) {
        if state.t == 24 {
            return (*state, *plan)
        }
        if !plan.is_empty() {
            let mut state = state.clone();
            let mut plan = plan.clone();
            state.tick(&blueprint, &mut plan);
            return Plan::optimize_worker(blueprint, &state, &plan); 
        }
        let mut best_plan = None;
        let mut best_state = None;
        for option in [Geode, Obsidian, Clay, Ore] {
            let mut plan = Plan::from(&vec![option]);
            let mut state = state.clone();
            state.tick(&blueprint, &mut plan);
            (state, plan) = Plan::optimize_worker(blueprint, &state, &plan) {
                plan = new_plan;
            }
            if best_state.is_none() || state > best_state.unwrap() {
                plan.push_front(option);
                best_state = Some(state);
                best_plan= Some(plan);
            }
        }
        best_plan
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    t: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.geode.cmp(&other.geode) {
            Equal => {}
            ord => return ord,
        }
        match self.obsidian.cmp(&other.obsidian) {
            Equal => {}
            ord => return ord,
        }
        match self.clay.cmp(&other.clay) {
            Equal => {}
            ord => return ord,
        }
        match self.ore.cmp(&other.ore) {
            Equal => {}
            ord => return ord,
        }
        Equal
    }
}

impl State {
    fn new() -> Self {
        State {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            t: 0,
        }
    }

    fn tick(&mut self, blueprint: &Blueprint, plan: &mut Plan) {
        
        // build
        let mut built = None;
        match plan.front() {
            Some(instr) => match instr {
                Ore => if self.ore >= blueprint.ore {
                    // println!("Spend {} ore to start building an ore-collecting robot.", blueprint.ore);
                    self.ore -= blueprint.ore;
                    built = plan.pop_front();
                },
                Clay => if self.ore >= blueprint.clay {
                    // println!("Spend {} ore to start building a clay-collecting robot.", blueprint.clay);
                    self.ore -= blueprint.clay;
                    built = plan.pop_front();
                },
                Obsidian => if self.ore >= blueprint.obsidian[0] && self.clay >= blueprint.obsidian[1] {
                    // println!("Spend {} ore and {} clay to start building an obsidian-collecting robot.", blueprint.obsidian[0], blueprint.obsidian[1]);
                    self.ore -= blueprint.obsidian[0];
                    self.clay -= blueprint.obsidian[1];
                    built = plan.pop_front();
                },
                Geode => if self.ore >= blueprint.geode[0] && self.obsidian >= blueprint.geode[1] {
                    // println!("Spend {} ore and {} obsidian to start building an geode-collecting robot.", blueprint.geode[0], blueprint.geode[1]);
                    self.ore -= blueprint.geode[0];
                    self.obsidian -= blueprint.geode[1];
                    built = plan.pop_front();
                },
            }
            None => (),  // we have run off the end of the plan
        }

        // collect
        self.ore += self.ore_robot;
        // println!("{} ore-collecting robot collects {} ore; you now have {} ore.", self.ore_robot, self.ore_robot, self.ore);
        self.clay += self.clay_robot;
        // if self.clay_robot > 0 { println!("{} clay-collecting robot collects {} clay; you now have {} clay.", self.clay_robot, self.clay_robot, self.clay); }
        self.obsidian += self.obsidian_robot;
        // if self.obsidian_robot > 0 { println!("{} obsidian-collecting robot collects {} obsidian; you now have {} obsidian.", self.obsidian_robot, self.obsidian_robot, self.obsidian); }
        self.geode += self.geode_robot;
        // if self.geode_robot > 0 { println!("{} geode-collecting robot collects {} geode; you now have {} geode.", self.geode_robot, self.geode_robot, self.geode); }

        // build
        match built {
            Some(instr) => match instr {
                Ore => {
                    self.ore_robot += 1;
                    // println!("The new ore-collecting robot is ready; you now have {} of them..", self.ore_robot);
                },
                Clay => {
                    self.clay_robot += 1;
                    // println!("The new clay-collecting robot is ready; you now have {} of them..", self.clay_robot);
                },
                Obsidian => {
                    self.obsidian_robot += 1;
                    // println!("The new obsidian-collecting robot is ready; you now have {} of them..", self.obsidian_robot);
                },
                Geode => {
                    self.geode_robot += 1;
                    // println!("The new geode-collecting robot is ready; you now have {} of them..", self.geode_robot);
                },
            }
            None => (),  // nothing being built
        }
        
        // time advances
        self.t += 1;
        // println!();
    }

    fn run(&mut self, blueprint: &Blueprint, plan: &mut Plan) {
        while self.t < 24 {
            self.tick(blueprint, plan);
        }
    }
}

fn main() {
    let input: &str = &fs::read_to_string("input/019.txt").expect("file read error");
    println!("there are {} blueprints", input.lines().count());
}

#[cfg(test)]
mod tests {
    use std::os::macos::raw::stat;

    use crate::*;
    const SAMPLE: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    fn test_parse_cube() {
        let blueprints: Vec<Blueprint> = SAMPLE.lines().map(Blueprint::from).collect();
        assert_eq!(blueprints.len(), 2);
        assert_eq!(blueprints[0].id, 1);
        assert_eq!(blueprints[0].ore, 4);
        assert_eq!(blueprints[0].clay, 2);
        assert_eq!(blueprints[0].obsidian, [3, 14]);
        assert_eq!(blueprints[0].geode, [2, 7]);
        assert_eq!(blueprints.len(), 2);

        assert_eq!(blueprints[1].id, 2);
        assert_eq!(blueprints[1].ore, 2);
        assert_eq!(blueprints[1].clay, 3);
        assert_eq!(blueprints[1].obsidian, [3, 8]);
        assert_eq!(blueprints[1].geode, [3, 12]);
    }

    #[test]
    fn test_tick() {
        let blueprints: Vec<Blueprint> = SAMPLE.lines().map(Blueprint::from).collect();
        let mut state = State::new();
        let mut plan = Plan::from(&vec![Clay, Clay, Clay, Obsidian, Clay, Obsidian, Geode, Geode]);
        while state.t < 24 {
            // println!("== Minute {} ==", state.t + 1);
            state.tick(&blueprints[0], &mut plan);
        }
        assert_eq!(state.geode, 9);
    }

    #[test]
    fn test_State_partial_order() {
        let a = State { ore_robot: 1, clay_robot: 4, obsidian_robot: 2, geode_robot: 2, ore: 6, clay: 41, obsidian: 8, geode: 9, t: 24 };
        let b = State { ore_robot: 8, clay_robot: 1, obsidian_robot: 0, geode_robot: 0, ore: 89, clay: 9, obsidian: 0, geode: 0, t: 24 };
        assert_eq!(a.cmp(&b), Greater);
        assert!(a > b);
        assert_eq!(a > b, b < a);
    }

    #[test]
    fn test_validate_planner() {
        let blueprints: Vec<Blueprint> = SAMPLE.lines().map(Blueprint::from).collect();
        let plan = Plan::optimize(blueprints[0]);
        assert_eq!(plan, Plan::from(&vec![Clay, Clay, Clay, Obsidian, Clay, Obsidian, Geode, Geode]));
    }

    #[test]
    fn test_planner() {
        assert!(false);
        let blueprints: Vec<Blueprint> = SAMPLE.lines().map(Blueprint::from).collect();
        let mut state = State::new();
        let mut plan = Plan::optimize(blueprints[1]);
        while state.t < 24 {
            state.tick(&blueprints[1], &mut plan);
        }
        assert_eq!(state.geode, 12);
    }

}
