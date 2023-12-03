use lazy_static::lazy_static; // 1.3.0
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/001.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let numbers = parse_numbers(lines.clone());
    println!("numerical sum is {:?}", numbers.iter().sum::<u32>());
    let words = parse_words(lines.clone());
    println!("word sum is {:?}", words.iter().sum::<u32>());
}

const CALIB_2_RE: &str = r"^[^\d]*(\d).*(\d)[^\d]*$";
const CALIB_1_RE: &str = r"^[^\d]*(\d)[^\d]*$";

fn parse_number(input: &str) -> u32 {
    let calib_1_re: regex::Regex = Regex::new(CALIB_1_RE).unwrap();
    let calib_2_re: regex::Regex = Regex::new(CALIB_2_RE).unwrap();
    match calib_2_re.captures(input) {
        Some(caps) => {
            let tens = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let ones = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            return 10 * tens + ones;
        }
        None => {
            let cap = calib_1_re.captures(input).unwrap();
            let digit = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            return 10 * digit + digit;
        }
    }
}

fn parse_numbers(lines: Vec<String>) -> Vec<u32> {
    let mut numbers = Vec::new();
    for line in lines {
        if !line.is_empty() {
            numbers.push(parse_number(&line));
        }
    }
    numbers
}

lazy_static! {
    static ref WORDS_RE: regex::Regex =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine){1}").unwrap();
}

fn parse_word(input: &str) -> String {
    let mut translated = input.to_string();
    while let Some(mat) = WORDS_RE.find(&translated) {
        match mat.as_str() {
            "one" => translated = translated.replacen("one", "1", 1),
            "two" => translated = translated.replacen("two", "2", 1),
            "three" => translated = translated.replacen("three", "3", 1),
            "four" => translated = translated.replacen("four", "4", 1),
            "five" => translated = translated.replacen("five", "5", 1),
            "six" => translated = translated.replacen("six", "6", 1),
            "seven" => translated = translated.replacen("seven", "7", 1),
            "eight" => translated = translated.replacen("eight", "8", 1),
            "nine" => translated = translated.replacen("nine", "9", 1),
            &_ => {}
        }
    }
    translated.to_string()
}

fn parse_words(lines: Vec<String>) -> Vec<u32> {
    let mut numbers = Vec::new();
    for line in lines {
        if !line.is_empty() {
            numbers.push(parse_number(&parse_word(&line)));
        }
    }
    numbers
}

#[cfg(test)]
mod tests {
    use crate::parse_number;
    use crate::parse_word;
    #[test]
    fn test_numbers() {
        assert_eq!(parse_number("1abc2"), 12);
        assert_eq!(parse_number("pqr3stu8vwx"), 38);
        assert_eq!(parse_number("a1b2c3d4e5f"), 15);
        assert_eq!(parse_number("treb7uchet"), 77);
        assert_eq!(parse_number("823"), 83);
    }
    #[test]
    fn test_words() {
        assert_eq!(parse_word("two1nine"), "219"); // 29);
        assert_eq!(parse_word("eightwothree"), "8wo3"); // 83);
        assert_eq!(parse_word("abcone2threexyz"), "abc123xyz"); // 13);
        assert_eq!(parse_word("xtwone3four"), "x2ne34"); // 24);
        assert_eq!(parse_word("4nineeightseven2"), "49872"); // 42);
        assert_eq!(parse_word("zoneight234"), "z1ight234"); // 14);
        assert_eq!(parse_word("7pqrstsixteen"), "7pqrst6teen"); // 76);
    }
}
