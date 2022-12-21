use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Hash)]
struct Coord {
    x: usize,
    y: usize,
    height: u32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Coord {
    fn get_neighbours(&self, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        if self.x > 0 {
            v.push((self.x - 1, self.y));
        }
        if self.x < max_x {
            v.push((self.x + 1, self.y));
        }
        if self.y > 0 {
            v.push((self.x, self.y - 1));
        }
        if self.y < max_y {
            v.push((self.x, self.y + 1));
        }
        v
    }

    fn can_reach_a(&self, other: &Coord) -> bool {
        self.height + 1 >= other.height
    }
    fn can_reach_b(&self, other: &Coord) -> bool {
        self.height - 1 <= other.height
    }
}

fn bfs(grid: &Vec<Vec<Coord>>, start: &Coord, end: &Coord, part_a: bool) -> Vec<Coord> {
    let mut visited = HashSet::new();

    let mut q: std::collections::VecDeque<(Coord, Vec<Coord>)> = VecDeque::new();
    q.push_back((start.clone(), vec![start.clone()]));

    while !q.is_empty() {
        let (current, path) = q.pop_front().unwrap();
        if part_a && &current == end {
            return path;
        }
        if !part_a && current.height == end.height {
            return path;
        }

        for (x, y) in current.get_neighbours(grid.len() - 1, grid[0].len() - 1) {
            let other = grid[x][y].clone();
            let can_reach = if part_a {
                current.can_reach_a(&other)
            } else {
                current.can_reach_b(&other)
            };
            if can_reach && !visited.contains(&other) && !q.iter().map(|(c, _)| c).contains(&other)
            {
                let mut p = path.clone();
                p.push(other.clone());
                q.push_back((other, p));
            }
        }

        visited.insert(current);
    }
    vec![]
}

fn parse_input(input: &str) -> (Vec<Vec<Coord>>, Coord, Coord) {
    let mut start = None;
    let mut end = None;

    let mut grid = vec![];
    for (x, r) in input.lines().enumerate() {
        let mut row = vec![];
        for (y, c) in r.chars().enumerate() {
            if c == 'S' {
                start = Some(Coord {
                    x: x,
                    y: y,
                    height: 'a' as u32,
                });
                row.push(Coord {
                    x: x,
                    y: y,
                    height: 'a' as u32,
                });
            } else if c == 'E' {
                end = Some(Coord {
                    x: x,
                    y: y,
                    height: 'z' as u32,
                });
                row.push(Coord {
                    x: x,
                    y: y,
                    height: 'z' as u32,
                })
            } else {
                row.push(Coord {
                    x: x,
                    y: y,
                    height: c as u32,
                })
            }
        }
        grid.push(row);
    }

    (grid, start.unwrap(), end.unwrap())
}

pub fn day12() {
    let input = include_str!("../inputs/day12.txt");

    let (grid, start, end) = parse_input(input);

    let path = bfs(&grid, &start, &end, true);
    println!("Part A is: {:?}", path.len() - 1);

    let path = bfs(&grid, &end, &start, false);
    println!("Part B is: {:?}", path.len() - 1);
}

#[test]
fn sample_input() {
    let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    let (grid, start, end) = parse_input(input);
    let path = bfs(&grid, &start, &end, true);
    println!("Path is: {:?}", path);
    assert_eq!(path.len() - 1, 31)
}

#[test]
fn simple_input() {
    let input = r#"SabcdefE"#;

    let (grid, start, end) = parse_input(input);
    let path = bfs(&grid, &start, &end, true);
    println!("{:?}", path);
    assert_eq!(path.len() - 1, 7)
}
