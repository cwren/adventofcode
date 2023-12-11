use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    soil_to_fertilizer: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    fertilizer_to_water: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    water_to_light: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    light_to_temperature: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    temperature_to_humidity: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    humidity_to_location: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
}

impl Almanac {
    fn new() -> Almanac {
        Almanac {
            seeds: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }
}

impl Almanac {
    fn closest(self) -> u64 {
        self.seeds
            .iter()
            .map(|s| seed_to_location(*s, &self))
            .min()
            .expect("there's no minimum?")
    }
}

fn rmap(a: u64, d: u64, s: u64, l: u64) -> Option<u64> {
    if a >= s && a < (s + l) {
        Some(d + (a - s))
    } else {
        None
    }
}

fn load_map(j: usize, lines: &Vec<String>) -> (usize, Vec<Box<dyn Fn(u64) -> Option<u64>>>) {
    let mut map = Vec::new();
    let mut i = j;
    // destination start, source start, range length
    while i < lines.len() && !lines[i].is_empty() {
        let args: Vec<_> = lines[i].split(' ').collect();
        assert_eq!(args.len(), 3);
        let d = args[0].parse::<u64>().expect("bad map");
        let s = args[1].parse::<u64>().expect("bad map");
        let l = args[2].parse::<u64>().expect("bad map");
        map.push(Box::new(move |a| rmap(a, d, s, l)) as Box<dyn Fn(u64) -> Option<u64>>);
        i += 1;
    }
    map.push(Box::new(move |a| Some(a)));
    (i, map)
}

fn load_almanac(lines: Vec<String>)-> Almanac {
    let mut almanac = Almanac::new();
    let mut i = 0;
    let (header, numbers) = lines[i]
        .split_once(": ")
        .expect("missing seeds colon");
    assert_eq!(header, "seeds");
    almanac.seeds = numbers
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<u64>().expect("found a non-integer"))
        .collect::<Vec<u64>>();
    i += 2;

    assert_eq!(lines[i], "seed-to-soil map:");
    i += 1;
    (i, almanac.seed_to_soil) = load_map(i, &lines);
    i += 1;

    assert_eq!(lines[i], "soil-to-fertilizer map:");
    i += 1;
    (i, almanac.soil_to_fertilizer) = load_map(i, &lines);
    i += 1;
    
    assert_eq!(lines[i], "fertilizer-to-water map:");
    i += 1;
    (i, almanac.fertilizer_to_water) = load_map(i, &lines);
    i += 1;
    
    assert_eq!(lines[i], "water-to-light map:");
    i += 1;
    (i, almanac.water_to_light) = load_map(i, &lines);
    i += 1;
    
    assert_eq!(lines[i], "light-to-temperature map:");
    i += 1;
    (i, almanac.light_to_temperature) = load_map(i, &lines);
    i += 1;
    
    assert_eq!(lines[i], "temperature-to-humidity map:");
    i += 1;
    (i, almanac.temperature_to_humidity) = load_map(i, &lines);
    i += 1;
        
    assert_eq!(lines[i], "humidity-to-location map:");
    i += 1;
    (_, almanac.humidity_to_location) = load_map(i, &lines);

    almanac
}

fn map(a: u64, map: &Vec<Box<dyn Fn(u64) -> Option<u64>>>) -> u64 {
    for m in map {
        if let Some(b) = m(a) {
            return b
        }
    } 
    a
}

fn seed_to_location(seed: u64, almanac: &Almanac) -> u64 {
    let soil = map(seed, &almanac.seed_to_soil);
    let fertilizer = map(soil, &almanac.soil_to_fertilizer);
    let water = map(fertilizer, &almanac.fertilizer_to_water);
    let light = map(water, &almanac.water_to_light);
    let temperature = map(light, &almanac.light_to_temperature);
    let humidity = map(temperature, &almanac.temperature_to_humidity);
    map(humidity, &almanac.humidity_to_location)
}

fn main() {
    let f = File::open("input/005.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let almanac = load_almanac(lines);
    println!("nearest seed is at {}", almanac.closest());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_lambdas() {
        let offsets = [4, 2, 9, 1];
        let mut operators = Vec::new();
        for offset in offsets {
            operators.push(Box::new(move |x| x + offset));
        }
        assert_eq!((*operators[0])(1), 5);
        assert_eq!((*operators[1])(1), 3);
        assert_eq!((*operators[2])(1), 10);
        assert_eq!((*operators[3])(1), 2);
    }

    #[test]
    fn test_load() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = load_almanac(lines);
        assert_eq!(almanac.seeds.len(), 4);
        assert_eq!(almanac.seed_to_soil.len(), 3);
        assert_eq!(almanac.soil_to_fertilizer.len(), 4);
        assert_eq!(almanac.fertilizer_to_water.len(), 5);
        assert_eq!(almanac.water_to_light.len(), 3);
        assert_eq!(almanac.light_to_temperature.len(), 4);
        assert_eq!(almanac.temperature_to_humidity.len(), 3);
        assert_eq!(almanac.humidity_to_location.len(), 3);
    }

    #[test]
    fn test_mapper() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = load_almanac(lines);
        assert_eq!(map(79, &almanac.seed_to_soil), 81);
        assert_eq!(map(14, &almanac.seed_to_soil), 14);
        assert_eq!(map(55, &almanac.seed_to_soil), 57);
        assert_eq!(map(13, &almanac.seed_to_soil), 13);
    }

    #[test]
    fn test_seed_to_location() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = load_almanac(lines);
        assert_eq!(seed_to_location(79, &almanac), 82);
        assert_eq!(seed_to_location(14, &almanac), 43);
        assert_eq!(seed_to_location(55, &almanac), 86);
        assert_eq!(seed_to_location(13, &almanac), 35);
    }

    #[test]
    fn test_closest() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = load_almanac(lines);
        assert_eq!(almanac.closest(), 35);
    }
}
