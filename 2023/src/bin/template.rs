use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/008.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE1: &str = r#""#;

    #[test]
    fn test_parse() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    }
}
