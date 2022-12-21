use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_input(input: &str) -> HashMap<Coord, Coord> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut hm = HashMap::new();
    for elem in input.lines() {
        let cap = re.captures(elem).unwrap();
        hm.insert(
            Coord {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            },
            Coord {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            },
        );
    }
    hm
}

// Finds the number of impossible beacons at a given y co-ordinate.
fn find_impossible_beacons(
    hm: &HashMap<Coord, Coord>,
    target_y: i32,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
) -> i32 {
    let mut v = vec![];
    for (s, b) in hm {
        let distance_to_y = (s.y - target_y).abs();
        let diff = s.distance(b) - distance_to_y;
        if diff >= 0 {
            let lb = std::cmp::max(lower_bound.unwrap_or(i32::MIN), s.x - diff);
            let ub = std::cmp::min(upper_bound.unwrap_or(i32::MAX), s.x + diff);
            v.push((lb, true));
            v.push((ub, false));
        }
    }

    v.sort();
    // Start with the first range.
    let mut sum = 0;
    let mut in_ranges: i32 = 0;
    let mut previously_added = v[0].0;
    let mut previous_bound = v[0].0;

    for (b, entering_range) in v {
        if in_ranges > 0 {
            sum += b - previous_bound;

            // Don't ask.
            if previously_added < previous_bound {
                sum += 1;
            }
            previously_added = b;
        }
        previous_bound = b;
        if entering_range {
            in_ranges += 1;
        } else {
            in_ranges -= 1;
        }
    }
    assert!(in_ranges == 0);
    sum
}

// Could there be a beacon at these coordinates?
fn possible_beacon(hm: &HashMap<Coord, Coord>, c: Coord) -> bool {
    for (s, b) in hm {
        if s.distance(&c) <= s.distance(b) {
            return false;
        }
    }
    return true;
}

pub fn day15() {
    let input = include_str!("../inputs/day15.txt");
    let hm = parse_input(input);

    let target_y = 2000000;

    println!(
        "Part A is: {}",
        find_impossible_beacons(&hm, target_y, None, None)
    );

    for y in 0..4000000 {
        let impossible = find_impossible_beacons(&hm, y, Some(0), Some(4000000));
        if impossible != 4000000 {
            for x in 0..4000000 {
                if possible_beacon(&hm, Coord { x, y }) {
                    println!("Part B is: {}", x as i64 * 4000000 + y as i64);
                    return;
                }
            }
        }
    }
    //     println!("Part B is: {}", b);
}

#[test]
fn sample_input() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    let hm = parse_input(input);

    for target_y in 0..20 {
        let impossible = find_impossible_beacons(&hm, target_y, Some(0), Some(20));
        if impossible != 10 {
            println!("y-coord is: {} ({})", target_y, impossible);
        }
    }
    assert!(false);
}
