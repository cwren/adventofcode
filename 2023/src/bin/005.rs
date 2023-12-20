use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

struct Processor {
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
    fn load_map(j: usize, lines: &Vec<String>) -> (usize, Vec<(u64, u64, u64)>) {
        let mut map = Vec::new();
        let mut i = j;
        // destination start, source start, range length
        while i < lines.len() && !lines[i].is_empty() {
            let args: Vec<_> = lines[i].split(' ').collect();
            assert_eq!(args.len(), 3);
            let d = args[0].parse::<u64>().expect("bad map");
            let s = args[1].parse::<u64>().expect("bad map");
            let l = args[2].parse::<u64>().expect("bad map");
            map.push((d, s, l));
            i += 1;
        }
        (i + 1, map)
    }
}

impl From<&Vec<String>> for Almanac {
    fn from(lines: &Vec<String>) -> Self {
        let (header, numbers) = lines[0].split_once(": ").expect("missing seeds colon");
        assert_eq!(header, "seeds");
        let seeds = numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<u64>().expect("found a non-integer"))
            .collect::<Vec<u64>>();

        assert_eq!(lines[2], "seed-to-soil map:");
        let (i, seed_to_soil) = Almanac::load_map(3, &lines);

        assert_eq!(lines[i], "soil-to-fertilizer map:");
        let (i, soil_to_fertilizer) = Almanac::load_map(i + 1, &lines);

        assert_eq!(lines[i], "fertilizer-to-water map:");
        let (i, fertilizer_to_water) = Almanac::load_map(i + 1, &lines);

        assert_eq!(lines[i], "water-to-light map:");
        let (i, water_to_light) = Almanac::load_map(i + 1, &lines);

        assert_eq!(lines[i], "light-to-temperature map:");
        let (i, light_to_temperature) = Almanac::load_map(i + 1, &lines);

        assert_eq!(lines[i], "temperature-to-humidity map:");
        let (i, temperature_to_humidity) = Almanac::load_map(i + 1, &lines);

        assert_eq!(lines[i], "humidity-to-location map:");
        let (_, humidity_to_location) = Almanac::load_map(i + 1, &lines);

        Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

impl From<&Almanac> for Processor {
    fn from(almanac: &Almanac) -> Self {
        Processor {
            seeds: almanac.seeds.clone(),
            seed_to_soil: Processor::compile(&almanac.seed_to_soil),
            soil_to_fertilizer: Processor::compile(&almanac.soil_to_fertilizer),
            fertilizer_to_water: Processor::compile(&almanac.fertilizer_to_water),
            water_to_light: Processor::compile(&almanac.water_to_light),
            light_to_temperature: Processor::compile(&almanac.light_to_temperature),
            temperature_to_humidity: Processor::compile(&almanac.temperature_to_humidity),
            humidity_to_location: Processor::compile(&almanac.humidity_to_location),
        }
    }
}
impl Processor {
    fn closest(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| self.seed_to_location(*s))
            .min()
            .expect("there's no minimum?")
    }

    fn compile(map: &Vec<(u64, u64, u64)>) -> Vec<Box<dyn Fn(u64) -> Option<u64>>> {
        let mut processor = Vec::new();
        // destination start, source start, range length
        for tup in map {
            let d = tup.0;
            let s = tup.1;
            let l = tup.2;
            processor
                .push(Box::new(move |a| Processor::rmap(a, d, s, l))
                    as Box<dyn Fn(u64) -> Option<u64>>);
        }
        processor.push(Box::new(move |a| Some(a)));
        processor
    }

    fn map(a: u64, map: &Vec<Box<dyn Fn(u64) -> Option<u64>>>) -> u64 {
        for m in map {
            if let Some(b) = m(a) {
                return b;
            }
        }
        a
    }

    fn rmap(a: u64, d: u64, s: u64, l: u64) -> Option<u64> {
        if a >= s && a < (s + l) {
            Some(d + (a - s))
        } else {
            None
        }
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = Processor::map(seed, &self.seed_to_soil);
        let fertilizer = Processor::map(soil, &self.soil_to_fertilizer);
        let water = Processor::map(fertilizer, &self.fertilizer_to_water);
        let light = Processor::map(water, &self.water_to_light);
        let temperature = Processor::map(light, &self.light_to_temperature);
        let humidity = Processor::map(temperature, &self.temperature_to_humidity);
        Processor::map(humidity, &self.humidity_to_location)
    }
}

fn main() {
    let f = File::open("input/005.txt").expect("File Error");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();
    let almanac = Almanac::from(&lines);
    let processor = Processor::from(&almanac);
    println!("nearest seed is at {}", processor.closest());
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
        let almanac = Almanac::from(&lines);
        assert_eq!(almanac.seeds.len(), 4);
        assert_eq!(almanac.seed_to_soil.len(), 2);
        assert_eq!(almanac.soil_to_fertilizer.len(), 3);
        assert_eq!(almanac.fertilizer_to_water.len(), 4);
        assert_eq!(almanac.water_to_light.len(), 2);
        assert_eq!(almanac.light_to_temperature.len(), 3);
        assert_eq!(almanac.temperature_to_humidity.len(), 2);
        assert_eq!(almanac.humidity_to_location.len(), 2);
    }

    #[test]
    fn test_from() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let processor = Processor::from(&almanac);
        assert_eq!(processor.seeds.len(), 4);
        assert_eq!(processor.seed_to_soil.len(), 3);
        assert_eq!(processor.soil_to_fertilizer.len(), 4);
        assert_eq!(processor.fertilizer_to_water.len(), 5);
        assert_eq!(processor.water_to_light.len(), 3);
        assert_eq!(processor.light_to_temperature.len(), 4);
        assert_eq!(processor.temperature_to_humidity.len(), 3);
        assert_eq!(processor.humidity_to_location.len(), 3);
    }

    #[test]
    fn test_mapper() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let processor = Processor::from(&almanac);
        assert_eq!(Processor::map(79, &processor.seed_to_soil), 81);
        assert_eq!(Processor::map(14, &processor.seed_to_soil), 14);
        assert_eq!(Processor::map(55, &processor.seed_to_soil), 57);
        assert_eq!(Processor::map(13, &processor.seed_to_soil), 13);
    }

    #[test]
    fn test_seed_to_location() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let processor = Processor::from(&almanac);
        assert_eq!(processor.seed_to_location(79), 82);
        assert_eq!(processor.seed_to_location(14), 43);
        assert_eq!(processor.seed_to_location(55), 86);
        assert_eq!(processor.seed_to_location(13), 35);
    }

    #[test]
    fn test_closest() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let processor = Processor::from(&almanac);
        assert_eq!(processor.closest(), 35);
    }
}
