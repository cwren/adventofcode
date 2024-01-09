use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Dataset {
    sensors: Vec<Sensor>,
}

struct Sensor {
    data: Vec<i32>,
}

impl From<&String> for Sensor {
    fn from(line: &String) -> Self {
        let data = line
            .split(" ")
            .map(|n| n.parse::<i32>().expect("malformed integer"))
            .collect();
        Sensor { data }
    }
}

impl From<Vec<String>> for Dataset {
    fn from(lines: Vec<String>) -> Self {
        let histories = lines.iter().map(Sensor::from).collect();
        Dataset { sensors: histories }
    }
}

impl Sensor {
    fn predict(&self) -> (i32, i32) {
        let mut stack = Vec::new();
        stack.push(self.data.clone());
        while stack
            .last()
            .expect("stack underrun")
            .iter()
            .any(|n| *n != 0)
        {
            let next = stack
                .last()
                .expect("stack underrun")
                .windows(2)
                .map(|n| n[1] - n[0])
                .collect();
            stack.push(next);
        }
        (
            stack
                .iter()
                .map(|v| v.last().expect("stacked vecs should not be zero length"))
                .sum(),
            stack
                .iter()
                .enumerate()
                .map(|(i, v)| if i % 2 == 0 { v[0] } else { -v[0] })
                .sum::<i32>(),
        )
    }
}

impl Dataset {
    fn analyze(&self) -> (i32, i32) {
        let predictions: Vec<(i32, i32)> = self.sensors.iter().map(Sensor::predict).collect();
        (
            predictions.iter().map(|x| x.0).sum(),
            predictions.iter().map(|x| x.1).sum(),
        )
    }
}

fn main() {
    let f = File::open("input/009.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let dataset = Dataset::from(lines);
    let analysis = dataset.analyze();
    println!("the result of prediciton is: {}", analysis.0);
    println!("the result of revision is: {}", analysis.1);
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE1: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_parse() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let dataset = Dataset::from(lines);
        assert_eq!(dataset.sensors.len(), 3);
        assert_eq!(dataset.sensors[0].data.len(), 6);
        assert_eq!(dataset.sensors[1].data.len(), 6);
        assert_eq!(dataset.sensors[2].data.len(), 6);
        assert_eq!(dataset.sensors[1].data[0], 1);
        assert_eq!(dataset.sensors[1].data[3], 10);
        assert_eq!(dataset.sensors[1].data[5], 21);
    }

    #[test]
    fn test_predict() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let dataset = Dataset::from(lines);
        assert_eq!(dataset.sensors[0].predict().0, 18);
        assert_eq!(dataset.sensors[1].predict().0, 28);
        assert_eq!(dataset.sensors[2].predict().0, 68);
    }

    #[test]
    fn test_revision() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let dataset = Dataset::from(lines);
        assert_eq!(dataset.sensors[0].predict().1, -3);
        assert_eq!(dataset.sensors[1].predict().1, 0);
        assert_eq!(dataset.sensors[2].predict().1, 5);
    }

    #[test]
    fn test_analyze() {
        let lines = SAMPLE1.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let dataset = Dataset::from(lines);
        assert_eq!(dataset.analyze(), (114, 2));
    }
}
