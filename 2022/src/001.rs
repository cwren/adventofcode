use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::borrow::Borrow;

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
    let lines = reader.lines().flatten();
    let elves = parse_elves(lines);
    let mut inventory: Vec<u32> = elves.iter().map(|elf| { elf.total_snacks() }).collect();
    println!("max is {:?}", inventory.iter().max());

    inventory.sort();
    inventory.reverse();
    println!("top 3 is {:?}", inventory[0..3].iter().sum::<u32>());
}

fn parse_elves<I>(lines: I) -> Vec<Elf> 
where
    I: IntoIterator,
    I::Item: Borrow<str>,{
    let mut elves = Vec::new();
    let mut elf = new_elf();
    for snack in lines {
        if snack.borrow().is_empty() {
            elves.push(elf);
            elf = new_elf();
        } else {
            let calories: u32 = snack.borrow().trim().parse()
                .expect("Not a Number");
            elf.snacks.push(calories);
        }
    }
    if !elf.snacks.is_empty() {
        elves.push(elf);
    }
    elves
}   

#[cfg(test)]
mod tests {
    use crate::parse_elves;
    #[test]
    fn test_parse() {
        let elves = parse_elves(Vec::from([
            "10",
            "11",
            "",
            "20",
            "21",
            "22",
            "23",
            "24",
            "25",
            "",
            "30",
            "31",
            "32",
            "33",
            "",
            "44"]).into_iter()
        );
        assert_eq!(elves.len(), 4);
        assert_eq!(elves[0].snacks.len(), 2);
        assert_eq!(elves[1].snacks.iter().max(), Some(&25));
        assert_eq!(elves[2].snacks.iter().sum::<u32>(), 30 + 31 + 32 + 33);
        assert_eq!(elves[3].snacks.len(), 1);
    }
}
