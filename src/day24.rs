use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn n(self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn e(self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn s(self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn w(self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn get_neighbours(
        self,
        grid: &Grid,
        width: &usize,
        height: &usize,
        day: usize,
    ) -> BTreeSet<Coord> {
        let mut neighbours = BTreeSet::new();
        if !grid.contains_blizzard(&self, day) {
            neighbours.insert(self.clone());
        }
        if self.x > 1
            && self.y > 0
            && self.y < height - 1
            && !grid.contains_blizzard(&self.w(), day)
        {
            neighbours.insert(self.w());
        }
        if self.x < width - 2
            && self.y > 0
            && self.y < height - 1
            && !grid.contains_blizzard(&self.e(), day)
        {
            neighbours.insert(self.e());
        }
        if self.y > 1 && self.x > 0 && self.x < width - 1 && !grid.contains_blizzard(&self.n(), day)
        {
            neighbours.insert(self.n());
        }
        if self.y < height - 2
            && self.x > 0
            && self.x < width - 1
            && !grid.contains_blizzard(&self.s(), day)
        {
            neighbours.insert(self.s());
        }

        // Special case for if we're next to the exits
        if self.y == 1 && self.x == 1 {
            neighbours.insert(self.n());
        }
        if self.y == height - 2 && self.x == width - 2 {
            neighbours.insert(self.s());
        }
        neighbours
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Grid {
    blizzard_positions_up: BTreeSet<Coord>,
    blizzard_positions_down: BTreeSet<Coord>,
    blizzard_positions_left: BTreeSet<Coord>,
    blizzard_positions_right: BTreeSet<Coord>,
    height: usize,
    width: usize,
}

impl Grid {
    fn contains_blizzard(&self, c: &Coord, day: usize) -> bool {
        // There is never a blizzard where the player can stand.
        if c.y == 0 || c.y == self.height - 1 || c.x == 0 || c.x == self.width - 1 {
            return false;
        }

        // For there to be a blizzard in this position on day X, find
        // out where the blizzard needed to have originally started.
        // This is... fiddly.
        let original_coord_right = Coord {
            x: {
                (((c.x - 1) + (self.width - 2) - (day % (self.width - 2))) % (self.width - 2)) + 1
            },
            y: c.y,
        };
        let original_coord_left = Coord {
            x: { (((c.x - 1) + day) % (self.width - 2)) + 1 },
            y: c.y,
        };
        let original_coord_up = Coord {
            x: c.x,
            y: { (((c.y - 1) + day) % (self.height - 2)) + 1 },
        };
        let original_coord_down = Coord {
            x: c.x,
            y: {
                (((c.y - 1) + (self.height - 2) - (day % (self.height - 2))) % (self.height - 2))
                    + 1
            },
        };

        self.blizzard_positions_up.contains(&original_coord_up)
            || self.blizzard_positions_down.contains(&original_coord_down)
            || self
                .blizzard_positions_right
                .contains(&original_coord_right)
            || self.blizzard_positions_left.contains(&original_coord_left)
    }
}

fn bfs(grid: &mut Grid, start: &Coord, end: &Coord, initial_day: usize) -> usize {
    let mut visited = BTreeSet::new();
    let mut q = VecDeque::new();
    let cycle = (grid.height - 2) * (grid.width - 2);

    q.push_back((start.clone(), initial_day));

    while !q.is_empty() {
        let (current, path) = q.pop_front().unwrap();

        if &current == end {
            return path;
        }

        if visited.contains(&(current, path % cycle)) {
            continue;
        }
        visited.insert((current, path % cycle));

        for c in current.get_neighbours(&grid, &grid.width, &grid.height, path) {
            q.push_back((c, path + 1));
        }
    }
    0
}

fn parse_input(input: &str) -> Grid {
    println!("Parsing input");
    let mut blizzard_positions_up = BTreeSet::new();
    let mut blizzard_positions_down = BTreeSet::new();
    let mut blizzard_positions_left = BTreeSet::new();
    let mut blizzard_positions_right = BTreeSet::new();

    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            match c {
                '>' => {
                    blizzard_positions_right.insert(Coord { x, y });
                }
                '^' => {
                    blizzard_positions_up.insert(Coord { x, y });
                }
                'v' => {
                    blizzard_positions_down.insert(Coord { x, y });
                }
                '<' => {
                    blizzard_positions_left.insert(Coord { x, y });
                }
                '.' | '#' => {}
                _ => panic!("Didn't understand character: {}", c),
            }
        }
    }
    Grid {
        blizzard_positions_up,
        blizzard_positions_down,
        blizzard_positions_left,
        blizzard_positions_right,
        height: input.lines().count(),
        width: input.lines().next().unwrap().len(),
    }
}

pub fn day24() {
    let input = include_str!("../inputs/day24.txt");
    let mut grid = parse_input(input);

    let height = grid.height;
    let width = grid.width;

    let there = bfs(
        &mut grid,
        &Coord { x: 1, y: 0 },
        &Coord {
            x: width - 2,
            y: height - 1,
        },
        0,
    );
    let back = bfs(
        &mut grid,
        &Coord {
            x: width - 2,
            y: height - 1,
        },
        &Coord { x: 1, y: 0 },
        there - 1,
    );
    let there_again = bfs(
        &mut grid,
        &Coord { x: 1, y: 0 },
        &Coord {
            x: width - 2,
            y: height - 1,
        },
        back - 1,
    );
    println!("Part A is {:?}", there - 1);
    println!("Part B is {:?}", there_again - 1);
}

#[test]
fn sample_input_ajw3() {
    let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
    let mut grid = parse_input(input);

    let height = grid.height;
    let width = grid.width;

    let there = bfs(
        &mut grid,
        &Coord { x: 1, y: 0 },
        &Coord {
            x: width - 2,
            y: height - 1,
        },
        0,
    );
    assert_eq!(18, there - 1);

    let back = bfs(
        &mut grid,
        &Coord {
            x: width - 2,
            y: height - 1,
        },
        &Coord { x: 1, y: 0 },
        there - 1,
    );
    assert_eq!(23, back - there);

    let there_again = bfs(
        &mut grid,
        &Coord { x: 1, y: 0 },
        &Coord {
            x: width - 2,
            y: height - 1,
        },
        back - 1,
    );
    assert_eq!(13, there_again - back);

    assert_eq!(54, there_again - 1);
}
