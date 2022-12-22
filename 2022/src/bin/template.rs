use std::fs;


fn main() {
    let input: &str = &fs::read_to_string("input/019.txt").expect("file read error");
    println!("there are {} lines", input.lines().count());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn test_parse_input() {
        assert_eq!(SAMPLE.lines().count(), 7);
    }

}
