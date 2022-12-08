use std::{collections::HashSet, hash::Hash};

use regex::Regex;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn neighbours(self: &Coords, other: &Coords) -> bool {
        i32::abs(self.x - other.x) <= 1 && i32::abs(self.y - other.y) <= 1
    }

    fn move_c(self: &mut Coords, c: char) {
        match c {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => {}
        }
    }
}

fn move_tail(head_pos: &Coords, tail_pos: &Coords) -> Coords {
   if tail_pos.neighbours(head_pos) {
        // No need to move the tail
        return tail_pos.clone();
   }

    let mut new_tail = tail_pos.clone();
    if head_pos.x != tail_pos.x {
        if head_pos.x < tail_pos.x {
            new_tail.move_c('L');
        } else {
            new_tail.move_c('R');
        }
    }
    if head_pos.y != tail_pos.y {
        if head_pos.y < tail_pos.y {
            new_tail.move_c('D');
        } else {
            new_tail.move_c('U');
        }
    }
    new_tail
}

fn solve_with_knots(input: &str, knot_count: u8) -> HashSet<Coords> {
    let mut tail_visits: HashSet<Coords> = HashSet::new();
    let mut knot_positions = vec![Coords {x: 0, y: 0}; knot_count as usize];
    tail_visits.insert(knot_positions[0].clone());

    let re = Regex::new(r"(.) (\d*)").unwrap();
    for caps in re.captures_iter(input) {
        let direction = caps[1].chars().next().unwrap();
        let count = caps[2].parse::<u8>().unwrap();

        for _ in 0..count {
            knot_positions[0].move_c(direction);

            // Get each tail knot to follow the one ahead of it
            for step in 1..knot_positions.len() {
                let lead_pos = &knot_positions[step - 1];
                let follow_pos = &knot_positions[step];

                // Is this idiomatic Rust? It doesn't feel it.
                let new_follower = move_tail(&lead_pos, follow_pos);
                knot_positions[step] = new_follower;
            }

            tail_visits.insert(knot_positions[knot_positions.len() - 1].clone());
        }
    }
    tail_visits
}

pub fn day09() {
    let input = include_str!("../inputs/day09.txt");

    let tail_visits = solve_with_knots(input, 2);
    println!("Part A is: {}", tail_visits.len());

    let tail_visits = solve_with_knots(input, 10);
    println!("Part B is: {}", tail_visits.len());
}
