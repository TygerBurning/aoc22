use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn nw(self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    fn n(self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn en(self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    fn sw(self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn s(self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn se(self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    fn e(self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn w(self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn has_neighbours(self, s: &HashSet<Coord>) -> bool {
        s.contains(&self.s())
            || s.contains(&self.w())
            || s.contains(&self.sw())
            || s.contains(&self.nw())
            || s.contains(&self.n())
            || s.contains(&self.en())
            || s.contains(&self.e())
            || s.contains(&self.se())
    }
}

fn parse_input(input: &str) -> HashSet<Coord> {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Coord {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    elves
}

fn try_north(
    old_elves: &HashSet<Coord>,
    new_elves: &mut HashMap<Coord, Coord>,
    current: Coord,
) -> bool {
    if !old_elves.contains(&current.nw())
        && !old_elves.contains(&current.n())
        && !old_elves.contains(&current.en())
    {
        if !new_elves.contains_key(&current.n()) {
            new_elves.insert(current.n(), current);
        } else {
            // Collision!
            let old_val = new_elves.remove(&current.n()).unwrap();
            new_elves.insert(old_val, old_val);
            new_elves.insert(current, current);
        }
        true
    } else {
        false
    }
}

fn try_south(
    old_elves: &HashSet<Coord>,
    new_elves: &mut HashMap<Coord, Coord>,
    current: Coord,
) -> bool {
    if !old_elves.contains(&current.sw())
        && !old_elves.contains(&current.s())
        && !old_elves.contains(&current.se())
    {
        if !new_elves.contains_key(&current.s()) {
            new_elves.insert(current.s(), current);
        } else {
            // Collision!
            let old_val = new_elves.remove(&current.s()).unwrap();
            new_elves.insert(old_val, old_val);
            new_elves.insert(current, current);
        }
        true
    } else {
        false
    }
}

fn try_east(
    old_elves: &HashSet<Coord>,
    new_elves: &mut HashMap<Coord, Coord>,
    current: Coord,
) -> bool {
    if !old_elves.contains(&current.en())
        && !old_elves.contains(&current.e())
        && !old_elves.contains(&current.se())
    {
        if !new_elves.contains_key(&current.e()) {
            new_elves.insert(current.e(), current);
        } else {
            // Collision!
            let old_val = new_elves.remove(&current.e()).unwrap();
            new_elves.insert(old_val, old_val);
            new_elves.insert(current, current);
        }
        true
    } else {
        false
    }
}

fn try_west(
    old_elves: &HashSet<Coord>,
    new_elves: &mut HashMap<Coord, Coord>,
    current: Coord,
) -> bool {
    if !old_elves.contains(&current.nw())
        && !old_elves.contains(&current.w())
        && !old_elves.contains(&current.sw())
    {
        if !new_elves.contains_key(&current.w()) {
            new_elves.insert(current.w(), current);
        } else {
            // Collision!
            let old_val = new_elves.remove(&current.w()).unwrap();
            new_elves.insert(old_val, old_val);
            new_elves.insert(current, current);
        }
        true
    } else {
        false
    }
}

fn grid_size(elves: &HashSet<Coord>) -> (i32, i32, i32, i32) {
    let min_x = elves.iter().map(|c| c.x).min().unwrap();
    let max_x = elves.iter().map(|c| c.x).max().unwrap();
    let min_y = elves.iter().map(|c| c.y).min().unwrap();
    let max_y = elves.iter().map(|c| c.y).max().unwrap();
    (min_x, max_x, min_y, max_y)
}

// fn print_grid(elves: &HashSet<Coord>) {
//     let (min_x, max_x, min_y, max_y) = grid_size(elves);

//     for y in min_y..max_y + 1 {
//         for x in min_x..max_x + 1 {
//             if elves.contains(&Coord { x, y }) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
// }

fn solve(_elves: HashSet<Coord>) -> (i32, i32) {
    let mut elves = _elves.clone();
    let mut dirs = [try_north, try_south, try_west, try_east];
    let rounds_a = 10;

    let mut part_a = 0;
    let mut part_b = 0;

    for round in 1.. {
        let mut new_elves = HashMap::new();

        for coord in &elves {
            let mut moved = false;
            if !coord.has_neighbours(&elves) {
                // Do nothing
                new_elves.insert(coord.clone(), coord.clone());
            } else {
                for (i, d) in dirs.iter().enumerate() {
                    if d(&elves, &mut new_elves, coord.clone()) {
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    // Failed to move!
                    new_elves.insert(coord.clone(), coord.clone());
                }
            }
        }

        if new_elves.keys().cloned().collect::<HashSet<Coord>>() == elves {
            part_b = round;
            break;
        }

        elves = new_elves.keys().cloned().collect();
        dirs.rotate_left(1);

        println!("Done round: {}", round);

        if round == rounds_a {
            let (min_x, max_x, min_y, max_y) = grid_size(&elves);
            part_a = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
        }
    }

    (part_a, part_b)
}

pub fn day23() {
    let input = include_str!("../inputs/day23.txt");
    let elves = parse_input(input);
    let (a, b) = solve(elves);
    println!("Part A is: {}", a);
    println!("Part B is: {}", b);
}

#[test]
fn sample_input() {
    let input = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#;

    let elves = parse_input(input);
    let (a, b) = solve(elves);
    assert_eq!(a, 110);
    assert_eq!(b, 20);
}
