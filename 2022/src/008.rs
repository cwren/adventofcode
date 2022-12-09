use std::io::Read;

fn parse_trees(input: &str) -> Vec<Vec<i8>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let n = line.len();
        let items = line.split("");
        let heights: Vec<i8> = items
            .skip(1)
            .take(n)
            .map(|s| s.parse::<i8>().expect("found a non-integer"))
            .collect();
        matrix.push(heights);
    }
    matrix
}

fn visible(orchard: &Vec<Vec<i8>>) -> usize {
    let h = orchard.len();
    let w = orchard[0].len();

    let mut sights = Vec::new();
    for _ in orchard {
        sights.push(vec![(false, false); w]);
    }

    let mut max = vec![None; h];
    for j in 0..w {
        for i in 0..h {
            if max[i].unwrap_or(-1i8) < orchard[i][j] {
                sights[i][j].0 = true;
            }
            max[i] = Some(max[i].unwrap_or(-1).max(orchard[i][j]));
        }
    }

    let mut max = vec![None; h];
    for j in 0..w {
        let j = w - j - 1;
        for i in 0..h {
            if max[i].unwrap_or(-1i8) < orchard[i][j] {
                sights[i][j].0 = true;
            }
            max[i] = Some(max[i].unwrap_or(-1).max(orchard[i][j]));
        }
    }

    let mut max = vec![None; w];
    for i in 0..h {
        for j in 0..w {
            if max[j].unwrap_or(-1) < orchard[i][j] {
                sights[i][j].1 = true;
            }
            max[j] = Some(max[j].unwrap_or(-1).max(orchard[i][j]));
        }
    }

    let mut max = vec![None; w];
    for i in 0..h {
        let i = h - i - 1;
        for j in 0..w {
            if max[j].unwrap_or(-1i8) < orchard[i][j] {
                sights[i][j].1 = true;
            }
            max[j] = Some(max[j].unwrap_or(-1).max(orchard[i][j]));
        }
    }

    let mut total = 0;
    for i in 0..h {
        for j in 0..w {
            if sights[i][j].0 || sights[i][j].1 {
                total += 1;
            }
        }
    }
    total
}

fn score_location(orchard: &Vec<Vec<i8>>, y: usize, x:usize) -> u32 {
    let h = orchard.len();
    let w = orchard[0].len();
    let mut scores: Vec<u32> = Vec::new();

    let tree = orchard[y][x];
    let i = y;
    let mut score = 0;
    let mut j = x + 1;
    while j < w {
        score += 1;
        if tree <= orchard[i][j] {
            break;
        }
        j += 1;
    }
    scores.push(score);
    let mut score = 0;
    if x > 0 {
        let mut j = x - 1;
        loop {
            score += 1;
            if tree <= orchard[i][j] {
                break;
            }
            if j == 0 {
                break
            }
            j -= 1;
        }
    }
    scores.push(score);

    let j = x;
    let mut score = 0;
    let mut i = y + 1;
    while i < h {
        score += 1;
        if tree <= orchard[i][j] {
            break;
        }
        i += 1;
    }
    scores.push(score);
    let mut score = 0;
    if y > 0 {
        let mut i = y - 1;
        loop {
            score += 1;
            if tree <= orchard[i][j] {
                break;
            }
            if i == 0 {
                break
            }
            i -= 1;
        }
    }
    scores.push(score);

    scores.iter().product()
}


fn best_view(orchard: &Vec<Vec<i8>>) -> u32 {
    let mut max = 0;
    for (i, row) in orchard.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let score =score_location(orchard, i, j);
            max = max.max(score);
        }
    }
    max
}

fn main() {
    let mut f = std::fs::File::open("input/008.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");

    let orchard = parse_trees(&input);
    println!("there are {} visible trees", visible(&orchard));
    println!("the best tree scored {}", best_view(&orchard));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_parse_log() {
        let orchard = parse_trees(SAMPLE);
        assert_eq!(orchard[0][0], 3);
        assert_eq!(orchard[3][2], 5);
    }

    #[test]
    fn test_visibile() {
        let visibile = visible(&parse_trees(SAMPLE));
        assert_eq!(visibile, 21);
    }

    #[test]
    fn test_score_location() {
        let orchard = parse_trees(SAMPLE);
        assert_eq!(score_location(&orchard, 1, 2), 4);
        assert_eq!(score_location(&orchard, 3, 2), 8);
    }

    #[test]
    fn test_best_location() {
        let orchard = parse_trees(SAMPLE);
        assert_eq!(best_view(&orchard), 8);
    }
}
