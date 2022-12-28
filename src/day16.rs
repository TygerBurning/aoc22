use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

#[derive(Debug, Clone)]
struct Room {
    flow: u32,
    neighbours: Vec<String>,
}

fn parse_input(input: &str) -> HashMap<String, Room> {
    let re =
        Regex::new(r"Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let mut hm = HashMap::new();

    for room_s in input.lines() {
        let caps = re.captures(room_s).unwrap();
        hm.insert(
            caps[1].to_string(),
            Room {
                flow: caps[2].parse().unwrap(),
                neighbours: caps[3]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            },
        );
    }
    hm
}

fn have_been_here_before(
    visited: &HashMap<(String, String), Vec<(u8, u32, u32)>>,
    current_room_a: String,
    current_room_b: String,
    time_remaining: u8,
    pressure_released: u32,
    flow_rate: u32,
) -> bool {
    match visited.get(&(current_room_a, current_room_b)) {
        Some(previous_times) => {
            // Don't explore this route if we've been here before with more time, flow and pressure
            return previous_times
                .iter()
                .find(|&(t, p, f)| {
                    *t >= time_remaining && *p >= pressure_released && *f >= flow_rate
                })
                .is_some();
        }
        None => return false,
    }
}

fn bfs(rooms: &HashMap<String, Room>, start: &str, time_remaining: u8) -> u32 {
    let mut visited: HashMap<(String, String), Vec<(u8, u32, u32)>> = HashMap::new();
    let mut q = VecDeque::new();

    q.push_back((
        start,
        start,
        time_remaining,
        0,
        0,
        HashSet::new(),
        vec![start.to_string()],
    ));

    let mut max = 0;

    while !q.is_empty() {
        let (
            current_a,
            current_b,
            time_remaining,
            pressure_released,
            flow_rate,
            rooms_opened,
            route,
        ) = q.pop_front().unwrap();

        println!("Exploring with {time_remaining} left");

        if time_remaining == 0 {
            if pressure_released > max {
                max = pressure_released;
            }
            continue;
        }

        if have_been_here_before(
            &visited,
            current_a.to_string(),
            current_b.to_string(),
            time_remaining,
            pressure_released,
            flow_rate,
        ) || have_been_here_before(
            &visited,
            current_b.to_string(),
            current_a.to_string(),
            time_remaining,
            pressure_released,
            flow_rate,
        ) {
            // println!("I've been here before!");
            if pressure_released > max {
                max = pressure_released;
            }
            continue;
        }
        match visited.get_mut(&(current_a.to_string(), current_b.to_string())) {
            Some(previous_times) => {
                previous_times.push((time_remaining, pressure_released, flow_rate))
            }
            None => {
                _ = visited.insert(
                    (current_a.to_string(), current_b.to_string()),
                    vec![(time_remaining, pressure_released, flow_rate)],
                )
            }
        }

        // We both try to open
        if !rooms_opened.contains(current_a)
            && rooms.get(current_a).unwrap().flow > 0
            && current_a != current_b
            && !rooms_opened.contains(current_b)
            && rooms.get(current_b).unwrap().flow > 0
        {
            let mut candidate_rooms = rooms_opened.clone();
            candidate_rooms.insert(current_a);
            candidate_rooms.insert(current_b);

            let mut candidate_route = route.clone();
            candidate_route.push(current_a.to_string() + ": opened");

            q.push_back((
                current_a,
                current_b,
                time_remaining - 1,
                pressure_released + flow_rate,
                flow_rate + rooms.get(current_a).unwrap().flow + rooms.get(current_b).unwrap().flow,
                candidate_rooms,
                candidate_route,
            ))
        }

        // A opens, B moves
        if !rooms_opened.contains(current_a) && rooms.get(current_a).unwrap().flow > 0 {
            let mut candidate_rooms = rooms_opened.clone();
            candidate_rooms.insert(current_a);

            let mut candidate_route = route.clone();
            candidate_route.push(current_a.to_string() + ": opened");

            for c in &rooms.get(current_b).unwrap().neighbours {
                q.push_back((
                    current_a,
                    c,
                    time_remaining - 1,
                    pressure_released + flow_rate,
                    flow_rate + rooms.get(current_a).unwrap().flow,
                    candidate_rooms.clone(),
                    candidate_route.clone(),
                ))
            }
        }

        // A moves, B opens
        if !rooms_opened.contains(current_b) && rooms.get(current_b).unwrap().flow > 0 {
            let mut candidate_rooms = rooms_opened.clone();
            candidate_rooms.insert(current_b);

            let mut candidate_route = route.clone();
            candidate_route.push(current_a.to_string() + ": opened");

            for c in &rooms.get(current_a).unwrap().neighbours {
                q.push_back((
                    c,
                    current_b,
                    time_remaining - 1,
                    pressure_released + flow_rate,
                    flow_rate + rooms.get(current_b).unwrap().flow,
                    candidate_rooms.clone(),
                    candidate_route.clone(),
                ))
            }
        }

        // We both move.
        for c1 in &rooms.get(current_a).unwrap().neighbours {
            for c2 in &rooms.get(current_b).unwrap().neighbours {
                let mut candidate_route = route.clone();
                candidate_route.push(c1.to_string() + ": moved");
                q.push_back((
                    c1,
                    c2,
                    time_remaining - 1,
                    pressure_released + flow_rate,
                    flow_rate,
                    rooms_opened.clone(),
                    candidate_route,
                ))
            }
        }
    }
    max
}

pub fn day16() {
    let input = include_str!("../inputs/day16.txt");
    let rooms = parse_input(input);

    let max = bfs(&rooms, &"AA", 26);
    println!("Part B is: {}", max);
}

#[test]
fn sample_input_16() {
    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    let rooms = parse_input(input);
    let max = bfs(&rooms, &"AA", 30);
    assert_eq!(1651, max);
}
