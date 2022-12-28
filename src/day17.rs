enum Shape {
    Dash(Coord),
    Plus(Coord),
    Corner(Coord),
    Line(Coord),
    Square(Coord),
}

impl Shape {
    fn coordinates(&self) -> Vec<Coord> {
        match self {
            Shape::Dash(pos) => vec![pos.clone(), pos.e(), pos.e().e(), pos.e().e().e()],
            Shape::Plus(pos) => vec![
                pos.n(),
                pos.e(),
                pos.e().n(),
                pos.e().n().n(),
                pos.e().n().e(),
            ],
            Shape::Corner(pos) => vec![
                pos.clone(),
                pos.e(),
                pos.e().e(),
                pos.e().e().n(),
                pos.e().e().n().n(),
            ],
            Shape::Line(pos) => vec![pos.clone(), pos.n(), pos.n().n(), pos.n().n().n()],
            Shape::Square(pos) => vec![pos.clone(), pos.n(), pos.e(), pos.n().e()],
        }
    }

    fn mov(&mut self, grid: &Grid, dir: &Dir) -> bool {
        if grid.would_collide(&self, dir) {
            return false;
        }

        match self {
            Shape::Dash(c)
            | Shape::Plus(c)
            | Shape::Corner(c)
            | Shape::Line(c)
            | Shape::Square(c) => match dir {
                Dir::Left => c.x -= 1,
                Dir::Right => c.x += 1,
                Dir::Down => c.y -= 1,
            },
        }
        true
    }
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

// Unusually, we'll use the bottom left as our origin.
impl Coord {
    fn n(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn e(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Left,
    Right,
    Down,
}

struct Grid {
    heights: [[bool; 10000]; 7],
    highest_point: usize,
}

impl Grid {
    fn would_collide(&self, shape: &Shape, dir: &Dir) -> bool {
        for c in shape.coordinates() {
            match dir {
                Dir::Left => {
                    if c.x == 0 || self.heights[c.x - 1][c.y] {
                        return true;
                    }
                }
                Dir::Right => {
                    if c.x == 6 || self.heights[c.x + 1][c.y] {
                        return true;
                    }
                }
                Dir::Down => {
                    if c.y == 0 || self.heights[c.x][c.y - 1] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn add_shape(&mut self, shape: &Shape) {
        for c in shape.coordinates() {
            self.heights[c.x][c.y] = true;
            if c.y + 1 > self.highest_point {
                self.highest_point = c.y + 1;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Dir> {
    input
        .chars()
        .map(|c| match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            _ => panic!("Unsupported char!"),
        })
        .collect()
}

fn drop_shape(shape: &mut Shape, jets: &Vec<Dir>, jet_index: &mut usize, grid: &mut Grid) {
    match shape {
        Shape::Dash(c) | Shape::Plus(c) | Shape::Corner(c) | Shape::Line(c) | Shape::Square(c) => {
                loop {
                let dir = jets[*jet_index % jets.len()];
                *jet_index += 1;
                let _ = shape.mov(grid, &dir);
                if !shape.mov(grid, &Dir::Down) {
                    grid.add_shape(shape);
                    break;
                }
            }
        }
    }
}

pub fn day17() {
    let input = include_str!("../inputs/day17.txt");

    let jets = parse_input(input);
    let mut jet_index = 0;

    let mut grid = Grid {
        heights: [[false; 10000]; 7],
        highest_point: 0,
    };

    let shapes = [
        Shape::Dash,
        Shape::Plus,
        Shape::Corner,
        Shape::Line,
        Shape::Square,
    ];

    for i in 0..2022 {
        let mut shape = shapes[i % shapes.len()](Coord {
            x: 2,
            y: grid.highest_point + 3,
        });
        drop_shape(&mut shape, &jets, &mut jet_index, &mut grid);
    }

    println!("Part A is: {}", grid.highest_point);
}

#[test]
fn sample_input_17() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let jets = parse_input(input);
    let mut jet_index = 0;
    let mut grid = Grid {
        heights: [[false; 10000]; 7],
        highest_point: 0,
    };

    let shapes = [
        Shape::Dash,
        Shape::Plus,
        Shape::Corner,
        Shape::Line,
        Shape::Square,
    ];

    for i in 0..2022 {
        let mut shape = shapes[i % shapes.len()](Coord {
            x: 2,
            y: grid.highest_point + 3,
        });
        drop_shape(&mut shape, &jets, &mut jet_index, &mut grid);
    }

    println!("Part A is: {}", grid.highest_point);
    assert_eq!(grid.highest_point , 3068);
}
