use gcollections::ops::*;
use interval::interval_set::ToIntervalSet;
use interval::IntervalSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::*;

#[derive(Clone)]
struct Range {
    d: u64,
    s: u64,
    l: u64,
}
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

struct Farmer {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    soil_to_fertilizer: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    fertilizer_to_water: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    water_to_light: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    light_to_temperature: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    temperature_to_humidity: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
    humidity_to_location: Vec<Box<dyn Fn(u64) -> Option<u64>>>,
}

type Lot = IntervalSet<u64>;
struct Stone {
    destination: u64,
    source: IntervalSet<u64>,
}

struct Mill {
    lot: Lot,
    seed_to_soil: Vec<Stone>,
    soil_to_fertilizer: Vec<Stone>,
    fertilizer_to_water: Vec<Stone>,
    water_to_light: Vec<Stone>,
    light_to_temperature: Vec<Stone>,
    temperature_to_humidity: Vec<Stone>,
    humidity_to_location: Vec<Stone>,
}

impl Almanac {
    fn load_map(j: usize, lines: &Vec<String>) -> (usize, Vec<Range>) {
        let mut map = Vec::new();
        let mut i = j;
        // destination start, source start, range length
        while i < lines.len() && !lines[i].is_empty() {
            let args: Vec<_> = lines[i].split(' ').collect();
            assert_eq!(args.len(), 3);
            let d = args[0].parse::<u64>().expect("bad map");
            let s = args[1].parse::<u64>().expect("bad map");
            let l = args[2].parse::<u64>().expect("bad map");
            map.push(Range { d, s, l });
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
        let (i, seed_to_soil) = Almanac::load_map(3, lines);

        assert_eq!(lines[i], "soil-to-fertilizer map:");
        let (i, soil_to_fertilizer) = Almanac::load_map(i + 1, lines);

        assert_eq!(lines[i], "fertilizer-to-water map:");
        let (i, fertilizer_to_water) = Almanac::load_map(i + 1, lines);

        assert_eq!(lines[i], "water-to-light map:");
        let (i, water_to_light) = Almanac::load_map(i + 1, lines);

        assert_eq!(lines[i], "light-to-temperature map:");
        let (i, light_to_temperature) = Almanac::load_map(i + 1, lines);

        assert_eq!(lines[i], "temperature-to-humidity map:");
        let (i, temperature_to_humidity) = Almanac::load_map(i + 1, lines);

        assert_eq!(lines[i], "humidity-to-location map:");
        let (_, humidity_to_location) = Almanac::load_map(i + 1, lines);

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

impl From<&Almanac> for Farmer {
    fn from(almanac: &Almanac) -> Self {
        Farmer {
            seeds: almanac.seeds.clone(),
            seed_to_soil: Farmer::compile(&almanac.seed_to_soil),
            soil_to_fertilizer: Farmer::compile(&almanac.soil_to_fertilizer),
            fertilizer_to_water: Farmer::compile(&almanac.fertilizer_to_water),
            water_to_light: Farmer::compile(&almanac.water_to_light),
            light_to_temperature: Farmer::compile(&almanac.light_to_temperature),
            temperature_to_humidity: Farmer::compile(&almanac.temperature_to_humidity),
            humidity_to_location: Farmer::compile(&almanac.humidity_to_location),
        }
    }
}

impl From<&Almanac> for Mill {
    fn from(almanac: &Almanac) -> Self {
        Mill {
            lot: almanac
                .seeds
                .chunks(2)
                .map(|c| (c[0], c[0] + c[1] - 1))
                .collect::<Vec<(u64, u64)>>()
                .to_interval_set(),
            seed_to_soil: Mill::compile(&almanac.seed_to_soil),
            soil_to_fertilizer: Mill::compile(&almanac.soil_to_fertilizer),
            fertilizer_to_water: Mill::compile(&almanac.fertilizer_to_water),
            water_to_light: Mill::compile(&almanac.water_to_light),
            light_to_temperature: Mill::compile(&almanac.light_to_temperature),
            temperature_to_humidity: Mill::compile(&almanac.temperature_to_humidity),
            humidity_to_location: Mill::compile(&almanac.humidity_to_location),
        }
    }
}

impl Farmer {
    fn closest(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| self.seed_to_location(*s))
            .min()
            .expect("there's no minimum?")
    }

    fn compile(ranges: &Vec<Range>) -> Vec<Box<dyn Fn(u64) -> Option<u64>>> {
        let mut processor = Vec::new();
        for range in ranges {
            let d = range.d;
            let s = range.s;
            let l = range.l;
            processor.push(
                Box::new(move |a| Farmer::rmap(a, d, s, l)) as Box<dyn Fn(u64) -> Option<u64>>
            );
        }
        processor.push(Box::new(Some));
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
        let soil = Farmer::map(seed, &self.seed_to_soil);
        let fertilizer = Farmer::map(soil, &self.soil_to_fertilizer);
        let water = Farmer::map(fertilizer, &self.fertilizer_to_water);
        let light = Farmer::map(water, &self.water_to_light);
        let temperature = Farmer::map(light, &self.light_to_temperature);
        let humidity = Farmer::map(temperature, &self.temperature_to_humidity);
        Farmer::map(humidity, &self.humidity_to_location)
    }
}

impl Mill {
    fn compile(ranges: &Vec<Range>) -> Vec<Stone> {
        let mut pile = Vec::new();
        for range in ranges {
            pile.push(Stone {
                destination: range.d,
                source: vec![(range.s, range.s + range.l - 1)].to_interval_set(),
            });
        }
        pile
    }

    fn grind(input: &Lot, stones: &Vec<Stone>) -> Lot {
        let mut todo: Lot = input.clone();
        let mut done: Lot = IntervalSet::empty();
        for stone in stones {
            let hit = todo.intersection(&stone.source) + stone.destination - stone.source.lower();
            done = done.union(&hit);
            todo = todo.difference(&stone.source);
        }
        done = done.union(&todo); // untouched lots pass through identically
        done
    }

    fn closest(&self) -> u64 {
        let soil = Mill::grind(&self.lot, &self.seed_to_soil);
        let fertilizer = Mill::grind(&soil, &self.soil_to_fertilizer);
        let water = Mill::grind(&fertilizer, &self.fertilizer_to_water);
        let light = Mill::grind(&water, &self.water_to_light);
        let temperature = Mill::grind(&light, &self.light_to_temperature);
        let humidity = Mill::grind(&temperature, &self.temperature_to_humidity);
        let location = Mill::grind(&humidity, &self.humidity_to_location);
        location.lower()
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
    let processor = Farmer::from(&almanac);
    println!("nearest seed is at {}", processor.closest());

    let mill = Mill::from(&almanac);
    println!("nearest lot is at {}", mill.closest());
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
        let processor = Farmer::from(&almanac);
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
        let processor = Farmer::from(&almanac);
        assert_eq!(Farmer::map(79, &processor.seed_to_soil), 81);
        assert_eq!(Farmer::map(14, &processor.seed_to_soil), 14);
        assert_eq!(Farmer::map(55, &processor.seed_to_soil), 57);
        assert_eq!(Farmer::map(13, &processor.seed_to_soil), 13);
    }

    #[test]
    fn test_seed_to_location() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let processor = Farmer::from(&almanac);
        assert_eq!(processor.seed_to_location(79), 82);
        assert_eq!(processor.seed_to_location(14), 43);
        assert_eq!(processor.seed_to_location(55), 86);
        assert_eq!(processor.seed_to_location(13), 35);
    }

    #[test]
    fn test_closest() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let processor = Farmer::from(&almanac);
        assert_eq!(processor.closest(), 35);
    }

    #[test]
    fn test_lots() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let mill = Mill::from(&almanac);
        //  79 14 55 13
        assert_eq!(mill.lot, vec![(55, 67), (79, 92)].to_interval_set());
    }

    #[test]
    fn test_grind_one_seed() {
        let output = Mill::grind(
            &vec![(82, 82)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 100)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 98)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(84, 84)].to_interval_set());
    }

    #[test]
    fn test_grind_disjoint_before() {
        let output = Mill::grind(
            &vec![(20, 40)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 100)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 98)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(20, 40)].to_interval_set());
    }

    #[test]
    fn test_grind_disjoint_after() {
        let output = Mill::grind(
            &vec![(100, 110)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 99)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 97)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(100, 110)].to_interval_set());
    }

    #[test]
    fn test_grind_overlap_before() {
        let output = Mill::grind(
            &vec![(40, 59)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 99)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 97)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(40, 49), (52, 61)].to_interval_set());
    }

    #[test]
    fn test_grind_overlap_after_first() {
        let output = Mill::grind(
            &vec![(99, 109)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 99)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 97)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(51, 51), (100, 109)].to_interval_set());
    }

    #[test]
    fn test_grind_overlap_after_second() {
        let output = Mill::grind(
            &vec![(99, 109)].to_interval_set(),
            &vec![
                Stone {
                    destination: 52,
                    source: vec![(50, 97)].to_interval_set(),
                },
                Stone {
                    destination: 50,
                    source: vec![(98, 99)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(51, 51), (100, 109)].to_interval_set());
    }

    #[test]
    fn test_grind_overlap_two_stones() {
        let output = Mill::grind(
            &vec![(90, 100)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 99)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 97)].to_interval_set(),
                },
            ],
        );
        assert_eq!(output, vec![(50, 51), (92, 100)].to_interval_set());
    }

    #[test]
    fn test_grind_complete_overlap() {
        let output = Mill::grind(
            &vec![(40, 130)].to_interval_set(),
            &vec![
                Stone {
                    destination: 50,
                    source: vec![(98, 99)].to_interval_set(),
                },
                Stone {
                    destination: 52,
                    source: vec![(50, 97)].to_interval_set(),
                },
            ],
        );
        assert_eq!(
            output,
            vec![(40, 49), (50, 51), (52, 99), (100, 130)].to_interval_set()
        );
    }

    #[test]
    fn test_winning_path() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let mill = Mill::from(&almanac);
        assert_eq!(
            Mill::grind(&vec![(82, 82)].to_interval_set(), &mill.seed_to_soil).lower(),
            84
        );
        assert_eq!(
            Mill::grind(&vec![(84, 84)].to_interval_set(), &mill.soil_to_fertilizer).lower(),
            84
        );
        assert_eq!(
            Mill::grind(&vec![(84, 84)].to_interval_set(), &mill.fertilizer_to_water).lower(),
            84
        );
        assert_eq!(
            Mill::grind(&vec![(84, 84)].to_interval_set(), &mill.water_to_light).lower(),
            77
        );
        assert_eq!(
            Mill::grind(
                &vec![(77, 77)].to_interval_set(),
                &mill.light_to_temperature
            )
            .lower(),
            45
        );
        assert_eq!(
            Mill::grind(
                &vec![(45, 45)].to_interval_set(),
                &mill.temperature_to_humidity
            )
            .lower(),
            46
        );
        assert_eq!(
            Mill::grind(
                &vec![(46, 46)].to_interval_set(),
                &mill.humidity_to_location
            )
            .lower(),
            46
        );
    }

    #[test]
    fn test_first_harvest() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let mill = Mill::from(&almanac);
        assert_eq!(mill.closest(), 46);
    }
}
