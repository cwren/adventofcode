use std::io::Read;

#[derive(Debug, PartialEq)]
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
}

impl CPU<'_> {
    fn new(program: &Program) -> CPU {
        let mut cpu = CPU {t: 0, x: 1, busy: 0, instructions: program.iter(), op: &Op::Noop};
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
                    Op::Addx(_) => self.busy = 1
                }
                true
            }
        }
    }

    fn tick(&mut self)-> bool {
        self.t += 1;
        match self.busy {
            0 => {
                match self.op {
                    Op::Noop => (),
                    Op::Addx(v) => {
                        self.x += v;
                    }
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
            if self.t == 19 || (self.t + 1 - 20) % 40 == 0 {
                signal += (self.t + 1) * self.x;
            }
        }
        println!("{}", self.t);
        signal
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
        
        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 2);
        assert_eq!(cpu.x, 1);
        
        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 3);
        assert_eq!(cpu.x, 4);
        
        assert_eq!(cpu.tick(), true);
        assert_eq!(cpu.t, 4);
        assert_eq!(cpu.x, 4);
        
        assert_eq!(cpu.tick(), false);
        assert_eq!(cpu.t, 5);
        assert_eq!(cpu.x, -1);

    }

    #[test]
    fn test_ticks() {
        let program: Program = LONG_SAMPLE.lines().map(Op::from).collect();
        let mut cpu = CPU::new(&program);

        while cpu.t < 19 { cpu.tick(); };
        assert_eq!(cpu.x, 21);

        while cpu.t < 59 { cpu.tick(); };
        assert_eq!(cpu.x, 19);

        while cpu.t < 99 { cpu.tick(); };
        assert_eq!(cpu.x, 18);

        while cpu.t < 139 { cpu.tick(); };
        assert_eq!(cpu.x, 21);

        while cpu.t < 179 { cpu.tick(); };
        assert_eq!(cpu.x, 16);

        while cpu.t < 219 { cpu.tick(); };
        assert_eq!(cpu.x, 18);
    }

    #[test]
    fn test_signal() {
        let program: Program = LONG_SAMPLE.lines().map(Op::from).collect();
        let mut cpu = CPU::new(&program);
        assert_eq!(cpu.signal(), 13140);
    }
}
