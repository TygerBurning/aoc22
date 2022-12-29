#[derive(Debug)]
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
    heights: [[bool; 5000]; 7],
    highest_point: usize,
}

impl Grid {
    fn would_collide(&self, shape: &Shape, dir: &Dir) -> bool {
        for c in shape.coordinates() {
            match dir {
                Dir::Left => {
                    if c.x == 0 || self.heights[c.x - 1][c.y % 5000] {
                        return true;
                    }
                }
                Dir::Right => {
                    if c.x == 6 || self.heights[c.x + 1][c.y % 5000] {
                        return true;
                    }
                }
                Dir::Down => {
                    if c.y == 0 || self.heights[c.x][(c.y - 1) % 5000] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn add_shape(&mut self, shape: &Shape) {
        for c in shape.coordinates() {
            self.heights[c.x][c.y % 5000] = true;
            if c.y > self.highest_point {
                self.highest_point = c.y;
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
        Shape::Dash(_) | Shape::Plus(_) | Shape::Corner(_) | Shape::Line(_) | Shape::Square(_) => {
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

    let mut grid = Grid {
        heights: [[false; 5000]; 7],
        highest_point: 0,
    };
    for i in 0..7 {
        grid.heights[i][0] = true;
    }

    let shapes = [
        Shape::Dash,
        Shape::Plus,
        Shape::Corner,
        Shape::Line,
        Shape::Square,
        ];

    // After many shapes have been dropped, we happen to create a flat floor.
    // I think due to some magic, it's likely this happens repeatedly - coinciding
    // with a matching jet index. Thus we have 3 heights:
    // - The first height - running up to the point we re-create a floor
    // - The middle height - calculated by multiplying the cycle height by the amount of cycles
    // - The final height - calculated by just running the old program.

    // By observation and printlns.
    let first_height = 2318;

    // By observation and printls
    let wrap_info_shape_start = 1485 + 1;
    let wrap_info_shape_cycle = 1700;
    let wrap_info_height_cycle = 2642;

    let huge = 1000000000000;
    let num_of_cycles = (huge - wrap_info_shape_start) / wrap_info_shape_cycle;
    let middle_height = wrap_info_height_cycle * num_of_cycles;


    let mut jet_index = 8772;
    let starting_shape = wrap_info_shape_start + num_of_cycles * wrap_info_shape_cycle;

    for i in starting_shape..huge {
        let mut shape = shapes[i % shapes.len()](Coord {
            x: 2,
            y: grid.highest_point + 4,
        });
        drop_shape(&mut shape, &jets, &mut jet_index, &mut grid);
    }
    println!("Part B is: {}", first_height + middle_height + grid.highest_point);
}