use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> HashSet<Coord> {
    let mut hs = HashSet::new();
    for wall in input.lines() {
        for (point_a, point_b) in wall.split(" -> ").tuple_windows() {
            let mut a = point_a.split(",");
            let c_a = Coord {
                x: a.next().unwrap().parse::<usize>().unwrap(),
                y: a.next().unwrap().parse::<usize>().unwrap(),
            };
            let mut b = point_b.split(",");
            let c_b = Coord {
                x: b.next().unwrap().parse::<usize>().unwrap(),
                y: b.next().unwrap().parse::<usize>().unwrap(),
            };

            if c_a.x < c_b.x {
                for x in c_a.x..c_b.x {
                    hs.insert(Coord { x, y: c_a.y });
                }
            }
            if c_a.x > c_b.x {
                for x in c_b.x..c_a.x {
                    hs.insert(Coord { x, y: c_a.y });
                }
            }
            if c_a.y < c_b.y {
                for y in c_a.y..c_b.y {
                    hs.insert(Coord { x: c_a.x, y: y });
                }
            }
            if c_a.y > c_b.y {
                for y in c_b.y..c_a.y {
                    hs.insert(Coord { x: c_a.x, y: y });
                }
            }

            hs.insert(c_a);
            hs.insert(c_b);
        }
    }
    hs
}

// Returns whether it came to rest or not
fn drop_sand(coords: &mut HashSet<Coord>, max_y: usize) -> bool {
    let mut pos = Coord { x: 500, y: 0 };
    loop {
        if pos.y > max_y {
            println!("Returning full because bigger than {}", max_y);
            return false;
        }
        if !coords.contains(&Coord {
            x: pos.x,
            y: pos.y + 1,
        }) {
            pos.y += 1;
        } else if !coords.contains(&Coord {
            x: pos.x - 1,
            y: pos.y + 1,
        }) {
            pos.x -= 1;
            pos.y += 1;
        } else if !coords.contains(&Coord {
            x: pos.x + 1,
            y: pos.y + 1,
        }) {
            pos.x += 1;
            pos.y += 1;
        } else {
            // If we've not been able to move anywhere, we're full.
            if pos == (Coord { x: 500, y: 0 }) {
                println!("Returning because full");
                return false;
            }
            coords.insert(pos);
            return true;
        }
    }
}

pub fn day14() {
    let input = include_str!("../inputs/day14.txt");
    let mut coords = parse_input(input);
    let max_y = coords.iter().map(|c| c.y).max().unwrap();

    let mut count = 0;
    while drop_sand(&mut coords, max_y) {
        count += 1;
    }
    println!("Part A is: {}", count);

    let mut coords = parse_input(input);
    let min_x = coords.iter().map(|c| c.x).min().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    for x in min_x - max_y..max_x + max_y {
        coords.insert(Coord { x, y: max_y + 2 });
    }

    let mut count = 0;
    while drop_sand(&mut coords, max_y + 2) {
        count += 1;
    }
    println!("Part B is: {}", count + 1);
}

#[test]
fn sample_input() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    let mut coords = parse_input(input);
    let max_y = coords.iter().map(|c| c.y).max().unwrap();

    let mut count = 0;
    while drop_sand(&mut coords, max_y) {
        count += 1;
    }

    assert_eq!(count, 24);

    let mut coords = parse_input(input);
    let min_x = coords.iter().map(|c| c.x).min().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    for x in min_x - max_y..max_x + max_y {
        coords.insert(Coord { x, y: max_y + 2 });
    }

    let mut count = 0;
    while drop_sand(&mut coords, max_y + 2) {
        count += 1;
    }
    assert_eq!(count + 1, 93);
}
