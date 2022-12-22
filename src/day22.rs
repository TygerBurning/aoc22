use std::collections::{hash_map::Keys, HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Dir {
    R,
    L,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Dir),
}

enum MoveResult {
    Ok,
    Stopped,
    Wrapped,
}

#[derive(Copy, Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

// Global position
#[derive(Debug)]
struct Pos {
    face_id: (usize, usize),
    coord: Coord,
}
// Must be a square grid.
struct Face {
    grid: Vec<Vec<char>>,
}

impl Face {
    fn move_left(&self, pos: Coord) -> MoveResult {
        if pos.x == 0 {
            MoveResult::Wrapped
        } else if self.grid[pos.x - 1][pos.y] == '#' {
            MoveResult::Stopped
        } else {
            MoveResult::Ok
        }
    }

    fn move_right(&self, pos: Coord) -> MoveResult {
        if pos.x == self.grid.len() - 1 {
            MoveResult::Wrapped
        } else if self.grid[pos.x + 1][pos.y] == '#' {
            MoveResult::Stopped
        } else {
            MoveResult::Ok
        }
    }

    fn move_up(&self, pos: Coord) -> MoveResult {
        if pos.y == 0 {
            MoveResult::Wrapped
        } else if self.grid[pos.x][pos.y - 1] == '#' {
            MoveResult::Stopped
        } else {
            MoveResult::Ok
        }
    }

    fn move_down(&self, pos: Coord) -> MoveResult {
        if pos.y == self.grid.len() - 1 {
            MoveResult::Wrapped
        } else if self.grid[pos.x][pos.y + 1] == '#' {
            MoveResult::Stopped
        } else {
            MoveResult::Ok
        }
    }
}

fn parse_input(input: &str) -> (HashMap<(usize, usize), Face>, Vec<Instruction>, usize) {
    let face_dimension = ((input
        .lines()
        .take_while(|&s| s != "")
        .map(|s| s.trim().len())
        .sum::<usize>()
        / 6) as f32)
        .sqrt() as usize;

    let c = input.lines().collect::<Vec<&str>>();

    // Find the space in which all the faces appear (they can't be further than 4 away to make a valid net)
    let mut net = HashMap::new();
    for x in 0..5 {
        for y in 0..5 {
            if y * 50 < c.len() {
                if x * 50 < c[y * 50].len() {
                    if c[y * 50].chars().nth(x * 50).unwrap() != ' ' {
                        println!("Cube at coords: {},{}", x, y);
                        let mut face = vec![];
                        // Construct column then rows, to make navigation easier later.
                        for i in 0..50 {
                            let mut col = vec![];
                            for j in 0..50 {
                                col.push(c[y * 50 + j].chars().nth(x * 50 + i).unwrap());
                            }
                            face.push(col);
                        }
                        net.insert((x, y), Face { grid: face });
                    }
                }
            }
        }
    }

    let mut i = vec![];
    let regex = Regex::new(r"([LR]|\d*)").unwrap();
    for cap in regex.captures_iter(&input.lines().last().unwrap()) {
        if &cap[1] == "L" {
            i.push(Instruction::Turn(Dir::L))
        } else if &cap[1] == "R" {
            i.push(Instruction::Turn(Dir::R))
        } else {
            i.push(Instruction::Move(cap[1].parse().unwrap()));
        }
    }

    (net, i, face_dimension)
}

// This is the interesting function.
// TODO - this is hardcoded - too hard to compute for all the various nets!?
fn find_next_pos(
    faces: &HashMap<(usize, usize), Face>,
    pos: Pos,
    dir: u8,
    dimension: usize,
) -> (Pos, u8) {
    let dimensions = dimension - 1;
    let new_pos_dir = match pos.face_id {
        (0, 2) => match dir {
            0 => (
                Pos {
                    face_id: (1, 2),
                    coord: Coord {
                        x: 0,
                        y: pos.coord.y,
                    },
                },
                0,
            ),
            1 => (
                Pos {
                    face_id: (0, 3),
                    coord: Coord {
                        x: pos.coord.x,
                        y: 0,
                    },
                },
                1,
            ),
            2 => (
                Pos {
                    face_id: (1, 0),
                    coord: Coord {
                        x: 0,
                        y: dimensions - pos.coord.y,
                    },
                },
                0,
            ),
            3 => (
                Pos {
                    face_id: (1, 1),
                    coord: Coord {
                        x: 0,
                        y: pos.coord.x,
                    },
                },
                0,
            ),
            _ => panic!("Didn't know about direction: {}", dir),
        },
        (0, 3) => match dir {
            0 => (
                Pos {
                    face_id: (1, 2),
                    coord: Coord {
                        x: pos.coord.y,
                        y: dimensions,
                    },
                },
                3,
            ),
            1 => (
                Pos {
                    face_id: (2, 0),
                    coord: Coord {
                        x: pos.coord.x,
                        y: 0,
                    },
                },
                1,
            ),
            2 => (
                Pos {
                    face_id: (1, 0),
                    coord: Coord {
                        x: pos.coord.y,
                        y: 0,
                    },
                },
                1,
            ),
            3 => (
                Pos {
                    face_id: (0, 2),
                    coord: Coord {
                        x: pos.coord.x,
                        y: dimensions,
                    },
                },
                3,
            ),
            _ => panic!("Didn't know about direction: {}", dir),
        },
        (1, 0) => match dir {
            0 => (
                Pos {
                    face_id: (2, 0),
                    coord: Coord {
                        x: 0,
                        y: pos.coord.y,
                    },
                },
                0,
            ),
            1 => (
                Pos {
                    face_id: (1, 1),
                    coord: Coord {
                        x: pos.coord.x,
                        y: 0,
                    },
                },
                1,
            ),
            2 => (
                Pos {
                    face_id: (0, 2),
                    coord: Coord {
                        x: 0,
                        y: dimensions - pos.coord.y,
                    },
                },
                0,
            ),
            3 => (
                Pos {
                    face_id: (0, 3),
                    coord: Coord {
                        x: 0,
                        y: pos.coord.x,
                    },
                },
                0,
            ),
            _ => panic!("Didn't know about direction: {}", dir),
        },
        (1, 1) => match dir {
            0 => (
                Pos {
                    face_id: (2, 0),
                    coord: Coord {
                        x: pos.coord.y,
                        y: dimensions,
                    },
                },
                3,
            ),
            1 => (
                Pos {
                    face_id: (1, 2),
                    coord: Coord {
                        x: pos.coord.x,
                        y: 0,
                    },
                },
                1,
            ),
            2 => (
                Pos {
                    face_id: (0, 2),
                    coord: Coord {
                        x: pos.coord.y,
                        y: 0,
                    },
                },
                1,
            ),
            3 => (
                Pos {
                    face_id: (1, 0),
                    coord: Coord {
                        x: pos.coord.x,
                        y: dimensions,
                    },
                },
                3,
            ),
            _ => panic!("Didn't know about direction: {}", dir),
        },
        (1, 2) => match dir {
            0 => (
                Pos {
                    face_id: (2, 0),
                    coord: Coord {
                        x: dimensions,
                        y: dimensions - pos.coord.y,
                    },
                },
                2,
            ),
            1 => (
                Pos {
                    face_id: (0, 3),
                    coord: Coord {
                        x: dimensions,
                        y: pos.coord.x,
                    },
                },
                2,
            ),
            2 => (
                Pos {
                    face_id: (0, 2),
                    coord: Coord {
                        x: dimensions,
                        y: pos.coord.y,
                    },
                },
                2,
            ),
            3 => (
                Pos {
                    face_id: (1, 1),
                    coord: Coord {
                        x: pos.coord.x,
                        y: dimensions,
                    },
                },
                3,
            ),
            _ => panic!("Didn't know about direction: {}", dir),
        },
        (2, 0) => match dir {
            0 => (
                Pos {
                    face_id: (1, 2),
                    coord: Coord {
                        x: dimensions,
                        y: dimensions - pos.coord.y,
                    },
                },
                2,
            ),
            1 => (
                Pos {
                    face_id: (1, 1),
                    coord: Coord {
                        x: dimensions,
                        y: pos.coord.x,
                    },
                },
                2,
            ),
            2 => (
                Pos {
                    face_id: (1, 0),
                    coord: Coord {
                        x: dimensions,
                        y: pos.coord.y,
                    },
                },
                2,
            ),
            3 => (
                Pos {
                    face_id: (0, 3),
                    coord: Coord {
                        x: pos.coord.x,
                        y: dimensions,
                    },
                },
                3,
            ),
            _ => panic!("Didn't know about direction: {}", dir),
        },
        _ => panic!("Didn't know about cube at {:?}", pos.face_id),
    };

    // If pos is valid
    // TODO - check we can move there!
    let next_face = faces.get(&new_pos_dir.0.face_id).unwrap();
    if next_face.grid[new_pos_dir.0.coord.x][new_pos_dir.0.coord.y] == '.' {
        new_pos_dir
    } else {
        (pos, dir)
    }
}

fn get_face(faces: &HashMap<(usize, usize), Face>, id: (usize, usize)) -> &Face {
    faces.get(&id).unwrap()
}

pub fn day22() {
    let input = include_str!("../inputs/day22.txt");
    let (faces, instructions, dimension) = parse_input(input);

    // TODO - hardcoded knowledge of where the first face is!
    let mut pos = Pos {
        face_id: (1, 0),
        coord: Coord { x: 0, y: 0 },
    };
    let mut dir = 0;

    for i in instructions {
        match i {
            Instruction::Move(d) => {
                for _ in 0..d {
                    match dir {
                        0 => match get_face(&faces, pos.face_id).move_right(pos.coord) {
                            MoveResult::Ok => {
                                pos = Pos {
                                    face_id: pos.face_id,
                                    coord: Coord {
                                        x: pos.coord.x + 1,
                                        y: pos.coord.y,
                                    },
                                };
                            }
                            MoveResult::Stopped => {}
                            MoveResult::Wrapped => {
                                (pos, dir) = find_next_pos(&faces, pos, dir, dimension);
                            }
                        },
                        1 => match get_face(&faces, pos.face_id).move_down(pos.coord) {
                            MoveResult::Ok => {
                                pos = Pos {
                                    face_id: pos.face_id,
                                    coord: Coord {
                                        x: pos.coord.x,
                                        y: pos.coord.y + 1,
                                    },
                                };
                            }
                            MoveResult::Stopped => {}
                            MoveResult::Wrapped => {
                                (pos, dir) = find_next_pos(&faces, pos, dir, dimension);
                            }
                        },
                        2 => match get_face(&faces, pos.face_id).move_left(pos.coord) {
                            MoveResult::Ok => {
                                pos = Pos {
                                    face_id: pos.face_id,
                                    coord: Coord {
                                        x: pos.coord.x - 1,
                                        y: pos.coord.y,
                                    },
                                };
                            }
                            MoveResult::Stopped => {}
                            MoveResult::Wrapped => {
                                (pos, dir) = find_next_pos(&faces, pos, dir, dimension);
                            }
                        },
                        3 => match get_face(&faces, pos.face_id).move_up(pos.coord) {
                            MoveResult::Ok => {
                                pos = Pos {
                                    face_id: pos.face_id,
                                    coord: Coord {
                                        x: pos.coord.x,
                                        y: pos.coord.y - 1,
                                    },
                                };
                            }
                            MoveResult::Stopped => {}
                            MoveResult::Wrapped => {
                                (pos, dir) = find_next_pos(&faces, pos, dir, dimension);
                            }
                        },
                        _ => panic!("Unsupported direction: {}", dir),
                    }
                }
            }
            Instruction::Turn(d) => match d {
                Dir::R => {
                    dir = (dir + 1) % 4;
                }
                Dir::L => {
                    dir = (dir + 3) % 4;
                }
            },
        }
    }
    let (a_x, a_y) = pos.face_id;
    let (real_x, real_y) = (a_x * 50 + pos.coord.x, a_y * 50 + pos.coord.y);
    println!(
        "Part B is {}",
        (real_y + 1) * 1000 + (real_x + 1) * 4 + dir as usize
    );
}
