use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn blizzard_move(
    blizzards: &HashMap<Coord, Vec<Direction>>,
    max_x: usize,
    max_y: usize,
) -> HashMap<Coord, Vec<Direction>> {
    let mut new_blizzards: HashMap<Coord, Vec<Direction>> = HashMap::new();
    for (location, direction_vec) in blizzards.iter() {
        for direction in direction_vec {
            let mut new_coord = *location;
            if *direction == Direction::Right {
                if new_coord.x == max_x {
                    new_coord.x = 1;
                } else {
                    new_coord.x += 1;
                }
            } else if *direction == Direction::Left {
                if new_coord.x == 1 {
                    new_coord.x = max_x;
                } else {
                    new_coord.x -= 1;
                }
            } else if *direction == Direction::Down {
                if new_coord.y == max_y {
                    new_coord.y = 1;
                } else {
                    new_coord.y += 1;
                }
            } else if *direction == Direction::Up {
                if new_coord.y == 1 {
                    new_coord.y = max_y;
                } else {
                    new_coord.y -= 1;
                }
            }
            if let std::collections::hash_map::Entry::Vacant(e) = new_blizzards.entry(new_coord) {
                e.insert(vec![*direction]);
            } else {
                new_blizzards.get_mut(&new_coord).unwrap().push(*direction);
            }
        }
    }
    new_blizzards
}

fn go(
    blizzard_positions: &Vec<HashMap<Coord, Vec<Direction>>>,
    current_position: &Coord,
    goals: &[Coord],
    goal_index: usize,
    current_cost: usize,
    complete_paths: &mut HashSet<usize>,
    states: &mut HashSet<String>,
) {
    if current_position == goals.get(goal_index).unwrap() {
        if goal_index == goals.len() - 1 {
            complete_paths.insert(current_cost);
            return;
        } else {
            go(
                blizzard_positions,
                current_position,
                goals,
                goal_index + 1,
                current_cost,
                complete_paths,
                states,
            );
            return;
        }
    } else if blizzard_positions
        .get(current_cost)
        .unwrap()
        .contains_key(current_position)
        || complete_paths.iter().min().unwrap()
            < &((goals.len() - 1 - goal_index) * (goals[0].x - 1 + goals[0].y)
                + (goals.get(goal_index).unwrap().x as i32 - current_position.x as i32)
                    .unsigned_abs() as usize
                + (goals.get(goal_index).unwrap().y as i32 - current_position.y as i32)
                    .unsigned_abs() as usize
                + current_cost)
    {
        return;
    }
    let state = format!(
        "{}.{}.{}.{}",
        goal_index, current_position.x, current_position.y, current_cost
    );
    if states.contains(&state) {
        return;
    } else {
        states.insert(state);
    }
    if (current_position.y < goals[0].y - 1)
        || (current_position.x == goals[0].x && current_position.y != goals[0].y)
    {
        let mut new_position_down: Coord = *current_position;
        new_position_down.y += 1;
        go(
            blizzard_positions,
            &new_position_down,
            goals,
            goal_index,
            current_cost + 1,
            complete_paths,
            states,
        );
    }
    if current_position.x < goals[0].x && current_position.y != 0 {
        let mut new_position_right: Coord = *current_position;
        new_position_right.x += 1;
        go(
            blizzard_positions,
            &new_position_right,
            goals,
            goal_index,
            current_cost + 1,
            complete_paths,
            states,
        );
    }
    if current_position.x > 1 && current_position.y != goals[0].y {
        let mut new_position_left: Coord = *current_position;
        new_position_left.x -= 1;
        go(
            blizzard_positions,
            &new_position_left,
            goals,
            goal_index,
            current_cost + 1,
            complete_paths,
            states,
        );
    }
    if current_position.y > 1 || (current_position.x == 1 && current_position.y == 1) {
        let mut new_position_up: Coord = *current_position;
        new_position_up.y -= 1;
        go(
            blizzard_positions,
            &new_position_up,
            goals,
            goal_index,
            current_cost + 1,
            complete_paths,
            states,
        );
    }
    go(
        blizzard_positions,
        current_position,
        goals,
        goal_index,
        current_cost + 1,
        complete_paths,
        states,
    );
}

fn shortest_route(
    blizzard_positions: &Vec<HashMap<Coord, Vec<Direction>>>,
    max_path: usize,
    goals: &[Coord],
    start_position: &Coord,
) -> usize {
    let mut complete_paths = HashSet::new();
    complete_paths.insert(max_path * goals.len());
    let mut states: HashSet<String> = HashSet::new();
    go(
        blizzard_positions,
        start_position,
        goals,
        0,
        0,
        &mut complete_paths,
        &mut states,
    );
    *complete_paths.iter().min().unwrap()
}

pub(crate) fn day24() {
    let f: File = File::open("inputs/day24.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let start_position = Coord { x: 1, y: 0 };
    let goal = Coord {
        x: input_data.first().unwrap().len() - 2,
        y: input_data.len() - 1,
    };
    let part1_goals = [goal];
    let part2_goals = [goal, start_position, goal];
    let max_path = 4 * (goal.x - 1 + goal.y);
    let mut blizzards: HashMap<Coord, Vec<Direction>> = HashMap::new();
    for (ii, line) in input_data.iter().enumerate() {
        for (jj, c) in line.chars().enumerate() {
            match c {
                'v' => blizzards.insert(Coord { x: jj, y: ii }, vec![Direction::Down]),
                '>' => blizzards.insert(Coord { x: jj, y: ii }, vec![Direction::Right]),
                '<' => blizzards.insert(Coord { x: jj, y: ii }, vec![Direction::Left]),
                '^' => blizzards.insert(Coord { x: jj, y: ii }, vec![Direction::Up]),
                _ => None,
            };
        }
    }
    let mut blizzard_positions: Vec<HashMap<Coord, Vec<Direction>>> = vec![blizzards.clone()];
    for _ in 0..max_path * part2_goals.len() {
        let new_blizzards: HashMap<Coord, Vec<Direction>> =
            blizzard_move(&blizzards, goal.x, goal.y - 1);
        blizzard_positions.push(new_blizzards.clone());
        blizzards = new_blizzards;
    }

    println!(
        "Shortest part to exit has length {}",
        shortest_route(&blizzard_positions, max_path, &part1_goals, &start_position)
    );
    println!(
        "Shortest path to exit, back to the start, and back to the exit again has length {}",
        shortest_route(&blizzard_positions, max_path, &part2_goals, &start_position)
    );
}
