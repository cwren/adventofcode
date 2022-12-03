use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
struct Elf {
    snacks: Vec<u32>
}

fn new_elf() -> Elf {
    Elf {
        snacks: Vec::new()
    }
}

impl Elf {
    fn total_snacks(&self) -> u32 {
        self.snacks.iter().sum()
    }
}

fn main() {
    let f = File::open("input/001.txt")
        .expect("File Error");
    let reader = BufReader::new(f);
    let mut elves = Vec::new();
    let mut elf = new_elf();
    for line in reader.lines() {
        if let Ok(snack) = line {
            if snack.len() == 0 {
                elves.push(elf);
                elf = new_elf();
            } else {
                let calories: u32 = snack.trim().parse()
                    .expect("Not a Number");
                elf.snacks.push(calories);
            }
        }
    }
    if elf.snacks.len() > 0 {
        elves.push(elf);
    }
    let mut inventory: Vec<u32> = elves.iter().map(|elf| { elf.total_snacks() }).collect();
    println!("max is {:?}", inventory.iter().max());

    inventory.sort();
    inventory.reverse();
    println!("top 3 is {:?}", inventory[0..3].iter().sum::<u32>());
}   
