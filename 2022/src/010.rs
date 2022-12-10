use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Noop,
    Addx(i32),
}

type Program = Vec<Op>;

struct CPU<'cpu> {
    t: i32,
    x: i32,
    busy: usize,
    instructions: std::slice::Iter<'cpu, Op>,
    op: &'cpu Op,
    wrote: Option<i32>,
}

impl CPU<'_> {
    fn new(program: &Program) -> CPU {
        let mut cpu = CPU {
            t: 0,
            x: 1,
            busy: 0,
            instructions: program.iter(),
            op: &Op::Noop,
            wrote: None,
        };
        cpu.load();
        cpu
    }

    fn load(&mut self) -> bool {
        match self.instructions.next() {
            None => false,
            Some(op) => {
                self.op = op;
                match self.op {
                    Op::Noop => self.busy = 0,
                    Op::Addx(_) => self.busy = 1,
                }
                true
            }
        }
    }

    fn tick(&mut self)-> bool {
        self.t += 1;
        if let Some(new_x) = self.wrote {
            self.x = new_x;
            self.wrote = None;
        }
        match self.busy {
            0 => {
                match self.op {
                    Op::Noop => (),
                    Op::Addx(v) => self.wrote = Some(self.x + v),
                }
                self.load()
            }
            _ => {
                self.busy -= 1;
                true
            }
        }

    }

    fn signal(&mut self) -> i32 {
        let mut signal = 0;
        while self.tick() {
            if self.t == 20 || (self.t - 20) % 40 == 0 {
                signal += self.t * self.x;
            }
        }
        println!("{}", self.t);
        signal
    }

    fn render(&self, x: i32) -> String {
        if (x - self.x).abs() < 2 {
            String::from("#")
        } else {
            String::from(".")
        }
    }
    fn draw(&mut self) -> Option<String> {
        let mut trace = Vec::with_capacity(40);
        let mut more = false;
        loop {
            more = self.tick();
            if !more {
                break; 
            }
            let p = (self.t - 1) % 40;
            trace.push(self.render(p));
            if p == 39 {
                break;
            }
        }
        if trace.is_empty() {
            return None
        }
        while trace.len() < 40 {
            trace.push(self.render(trace.len().try_into().expect("vec larger than i32")));
        }
        Some(trace.join(""))
    }

}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        let mut parts = line.split(' ');
        match parts.next().expect("empty operator") {
            "noop" => Op::Noop,
            "addx" => {
                let operand = parts
                    .next()
                    .expect("empty operand")
                    .parse()
                    .expect("non-numeric operand");
                Op::Addx(operand)
            }
            _ => panic!("unrecognized operator"),
        }
    }
}

fn main() {
    let mut f = std::fs::File::open("input/010.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    let program: Program = input.lines().map(Op::from).collect();
    println!("there are {} cycles", program.len());

    let mut cpu = CPU::new(&program);
    println!("signal is {}" , cpu.signal());
    let mut cpu = CPU::new(&program);
    loop {
        match cpu.draw() {
            None => break,
            Some(row) => println!("{row}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"noop
addx 3
addx -5"#;
    
    const LONG_SAMPLE: &str = include_str!("../input/010-sample.txt");

    #[test]
    fn test_parse_moves() {
        let program: Program = SAMPLE.lines().map(Op::from).collect();
        assert_eq!(program.len(), 3);
        assert_eq!(program[0], Op::Noop);
        assert_eq!(program[1], Op::Addx(3));
        assert_eq!(program[2], Op::Addx(-5));
    }

    #[test]
    fn test_tick() {
        let program: Program = SAMPLE.lines().map(Op::from).collect();
        let mut cpu = CPU::new(&program);

        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 1);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.wrote, None);
        
        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 2);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.wrote, None);
        
        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 3);
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.wrote, Some(4));
        
        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 4);
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.wrote, None);
        
        assert_eq!(cpu.tick(), false);
        assert_eq!(cpu.t, 5);
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.wrote, Some(-1));

    }

    #[test]
    fn test_ticks() {
        let program: Program = LONG_SAMPLE.lines().map(Op::from).collect();
        let mut cpu = CPU::new(&program);

        while cpu.t < 20 { cpu.tick(); };
        assert_eq!(cpu.x, 21);
        assert_eq!(cpu.x * cpu.t, 420);

        while cpu.t < 60 { cpu.tick(); };
        assert_eq!(cpu.x, 19);

        while cpu.t < 100 { cpu.tick(); };
        assert_eq!(cpu.x, 18);

        while cpu.t < 140 { cpu.tick(); };
        assert_eq!(cpu.x, 21);

        while cpu.t < 180 { cpu.tick(); };
        assert_eq!(cpu.x, 16);

        while cpu.t < 220 { cpu.tick(); };
        assert_eq!(cpu.x, 18);
    }

    #[test]
    fn test_signal() {
        let program: Program = LONG_SAMPLE.lines().map(Op::from).collect();
        let mut cpu = CPU::new(&program);
        assert_eq!(cpu.signal(), 13140);
    }

    #[test]
    fn test_draw() {
        let program: Program = LONG_SAMPLE.lines().map(Op::from).collect();
        let mut cpu = CPU::new(&program);
        assert_eq!(cpu.draw(), Some("##..##..##..##..##..##..##..##..##..##..".to_string()));
        assert_eq!(cpu.draw(), Some("###...###...###...###...###...###...###.".to_string()));
        assert_eq!(cpu.draw(), Some("####....####....####....####....####....".to_string()));
        assert_eq!(cpu.draw(), Some("#####.....#####.....#####.....#####.....".to_string()));
        assert_eq!(cpu.draw(), Some("######......######......######......####".to_string()));
        assert_eq!(cpu.draw(), Some("#######.......#######.......#######.....".to_string()));
        assert_eq!(cpu.draw(), None);
    }
}
