use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

type Lot = (u64, u64);
struct Mill {
    lots: Vec<Lot>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
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
            lots: almanac.seeds.chunks(2).map(|c| (c[0], c[1])).collect(),
            seed_to_soil: almanac.seed_to_soil.clone(),
            soil_to_fertilizer: almanac.soil_to_fertilizer.clone(),
            fertilizer_to_water: almanac.fertilizer_to_water.clone(),
            water_to_light: almanac.water_to_light.clone(),
            light_to_temperature: almanac.light_to_temperature.clone(),
            temperature_to_humidity: almanac.temperature_to_humidity.clone(),
            humidity_to_location: almanac.humidity_to_location.clone(),
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
    fn grind(input: &Lot, stones: &Vec<Range>) -> Vec<Lot> {
        let mut todo: Vec<Lot> = Vec::new();
        let mut done: Vec<Lot> = Vec::new();
        todo.push(*input);
        for stone in stones {
            let source_end = stone.s + stone.l;
            let mut hold: Vec<Lot> = Vec::new();
            for lot in todo {
                let lot_end = lot.0 + lot.1;
                if lot.0 < stone.s {
                    if lot_end < stone.s {
                        hold.push(lot);
                    } else if lot_end <= source_end {
                        hold.push((lot.0, stone.s - lot.0));
                        done.push((stone.d, lot_end - stone.s));
                    } else {
                        hold.push((lot.0, stone.s - lot.0));
                        hold.push((source_end, lot_end - source_end));
                        done.push((stone.d, stone.l));
                    }
                } else if lot.0 < source_end {
                    if lot_end <= source_end {
                        done.push((stone.d + lot.0 - stone.s, lot.1));
                    } else {
                        hold.push((source_end, lot_end - source_end));
                        done.push((stone.d + lot.0 - stone.s, source_end - lot.0));
                    }
                } else {
                    hold.push(lot);
                }
            }
            todo = hold;
        }
        done.extend(todo); // untouched lots pass through identically
        done.sort();
        done
    }

    fn process(input: &Vec<Lot>, stones: &Vec<Range>) -> Vec<Lot> {
        let mut output: Vec<Lot> = Vec::new();
        for lot in input {
            output.extend(Mill::grind(lot, stones));
        }
        output
    }

    fn closest(&self) -> u64 {
        let soil = Mill::process(&self.lots, &self.seed_to_soil);
        let fertilizer = Mill::process(&soil, &self.soil_to_fertilizer);
        let water = Mill::process(&fertilizer, &self.fertilizer_to_water);
        let light = Mill::process(&water, &self.water_to_light);
        let temperature = Mill::process(&light, &self.light_to_temperature);
        let humidity = Mill::process(&temperature, &self.temperature_to_humidity);
        let locations = Mill::process(&humidity, &self.humidity_to_location);
        let mut closest = u64::MAX;
        for location in locations {
            closest = closest.min(location.0);
        }
        closest
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
        assert_eq!(mill.lots.len(), 2);
        assert_eq!(mill.lots[0], (79, 14));
        assert_eq!(mill.lots[1], (55, 13));
    }

    #[test]
    fn test_grind_one_seed() {
        let output = Mill::grind(
            &(82, 1),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(84, 1)]);
    }

    #[test]
    fn test_grind_disjoint_before() {
        let output = Mill::grind(
            &(20, 20),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(20, 20)]);
    }

    #[test]
    fn test_grind_disjoint_after() {
        let output = Mill::grind(
            &(100, 110),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(100, 110)]);
    }

    #[test]
    fn test_grind_overlap_before() {
        let output = Mill::grind(
            &(40, 20),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(40, 10), (52, 10)]);
    }

    #[test]
    fn test_grind_overlap_after_first() {
        let output = Mill::grind(
            &(99, 10),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(51, 1), (100, 9)]);
    }

    #[test]
    fn test_grind_overlap_after_second() {
        let output = Mill::grind(
            &(99, 10),
            &vec![
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
                Range { d: 50, s: 98, l: 2 },
            ],
        );
        assert_eq!(output, vec![(51, 1), (100, 9)]);
    }

    #[test]
    fn test_grind_overlap_two_stones() {
        let output = Mill::grind(
            &(90, 10),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(50, 2), (92, 8)]);
    }

    #[test]
    fn test_grind_complete_overlap() {
        let output = Mill::grind(
            &(40, 90),
            &vec![
                Range { d: 50, s: 98, l: 2 },
                Range {
                    d: 52,
                    s: 50,
                    l: 48,
                },
            ],
        );
        assert_eq!(output, vec![(40, 10), (50, 2), (52, 48), (100, 30)]);
    }

    #[test]
    fn test_winning_path() {
        let lines = SAMPLE.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let almanac = Almanac::from(&lines);
        let mill = Mill::from(&almanac);
        assert_eq!(Mill::grind(&(82, 1), &mill.seed_to_soil), vec![(84, 1)]);
        assert_eq!(
            Mill::grind(&(84, 1), &mill.soil_to_fertilizer),
            vec![(84, 1)]
        );
        assert_eq!(
            Mill::grind(&(84, 1), &mill.fertilizer_to_water),
            vec![(84, 1)]
        );
        assert_eq!(Mill::grind(&(84, 1), &mill.water_to_light), vec![(77, 1)]);
        assert_eq!(
            Mill::grind(&(77, 1), &mill.light_to_temperature),
            vec![(45, 1)]
        );
        assert_eq!(
            Mill::grind(&(45, 1), &mill.temperature_to_humidity),
            vec![(46, 1)]
        );
        assert_eq!(
            Mill::grind(&(46, 1), &mill.humidity_to_location),
            vec![(46, 1)]
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
