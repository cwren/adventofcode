struct Race {
    t: u64,
    d: u64,
}
fn win_race(race: &Race, t: u64) -> bool {
    let d = (race.t - t) * t;
    d > race.d
}

fn win_count(race: &Race) -> u64 {
    let mut lower = 0u64;
    let mut upper = race.t;
    while lower <= upper && !win_race(race, lower) {
        lower += 1u64;
    }
    while upper > lower && !win_race(race, upper) {
        upper -= 1u64;
    }
    upper - lower + 1
}

fn compete(events: &Vec<Race>) -> u64 {
    let mut power = 1;
    for event in events {
        power *= win_count(event);
    }
    power
}

fn main() {
    // Time:        61     70     90     66
    // Distance:   643   1184   1362   1041
    let shorts = vec![
        Race { t: 61, d: 643 },
        Race { t: 70, d: 1184 },
        Race { t: 90, d: 1362 },
        Race { t: 66, d: 1041 },
    ];
    let power = compete(&shorts);
    println!("product of winning combos is {power}");

    let long = Race {
        t: 61709066,
        d: 643118413621041u64,
    };
    let ways_to_win = win_count(&long);
    println!("number of winning strategies is {ways_to_win}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use lazy_static::lazy_static; // 1.3.0

    lazy_static! {
        // Time:      7  15   30
        // Distance:  9  40  200
        static ref SHORTS: Vec<Race> = vec![
            Race { t:  7, d:   9 },
            Race { t: 15, d:  40 },
            Race { t: 30, d: 200 },
        ];
        static ref LONG: Race = Race { t:  71530, d:   940200 };
    }

    #[test]
    fn test_win_race() {
        assert!(!win_race(&SHORTS[0], 1));
        assert!(win_race(&SHORTS[0], 2));
        assert!(win_race(&SHORTS[0], 5));
        assert!(!win_race(&SHORTS[0], 6));
    }

    #[test]
    fn test_win_count() {
        assert_eq!(win_count(&SHORTS[0]), 4);
        assert_eq!(win_count(&SHORTS[1]), 8);
        assert_eq!(win_count(&SHORTS[2]), 9);
    }

    #[test]
    fn test_compete() {
        let power = compete(&SHORTS);
        assert_eq!(power, 288);
    }

    #[test]
    fn test_compete_long() {
        let power = win_count(&LONG);
        assert_eq!(power, 71503);
    }
}
