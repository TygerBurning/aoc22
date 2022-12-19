#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Cube {
    solid: bool,
    visited: bool,
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|elem| Coord {
            x: elem.split(",").collect::<Vec<&str>>()[0].parse().unwrap(),
            y: elem.split(",").collect::<Vec<&str>>()[1].parse().unwrap(),
            z: elem.split(",").collect::<Vec<&str>>()[2].parse().unwrap(),
        })
        .collect()
}

fn dfs(grid: &mut Vec<Vec<Vec<Cube>>>, node: Coord) {
    if grid[node.x][node.y][node.z].visited || grid[node.x][node.y][node.z].solid {
        return;
    }
    grid[node.x][node.y][node.z].visited = true;

    // x
    if node.x > 0 {
        dfs(
            grid,
            Coord {
                x: node.x - 1,
                y: node.y,
                z: node.z,
            },
        );
    }
    if node.x < grid.len() - 1 {
        dfs(
            grid,
            Coord {
                x: node.x + 1,
                y: node.y,
                z: node.z,
            },
        );
    }
    // y
    if node.y > 0 {
        dfs(
            grid,
            Coord {
                x: node.x,
                y: node.y - 1,
                z: node.z,
            },
        );
    }
    if node.y < grid[0].len() - 1 {
        dfs(
            grid,
            Coord {
                x: node.x,
                y: node.y + 1,
                z: node.z,
            },
        );
    }
    // z
    if node.z > 0 {
        dfs(
            grid,
            Coord {
                x: node.x,
                y: node.y,
                z: node.z - 1,
            },
        );
    }
    if node.z < grid[0][0].len() - 1 {
        dfs(
            grid,
            Coord {
                x: node.x,
                y: node.y,
                z: node.z + 1,
            },
        );
    }
}

// Note - previously_solid implies previously_internal
fn has_visible_face(
    c: &Cube,
    previously_solid: &mut bool,
    previously_internal: &mut bool,
    part_a: &mut u32,
    part_b: &mut u32,
) {
    if *previously_solid {
        // No face will be viewable from this side - solid or not!
    } else if *previously_internal {
        if c.solid {
            *part_a += 1;
        }
    } else {
        if c.solid {
            *part_a += 1;
            *part_b += 1;
        }
    }
    if *previously_solid {
        assert!(*previously_internal);
    }
    *previously_solid = c.solid;
    *previously_internal = !c.visited;
}

fn solve(coords: Vec<Coord>) -> (u32, u32) {
    let max_x = coords.iter().map(|c| c.x).max().unwrap() + 1;
    let max_y = coords.iter().map(|c| c.y).max().unwrap() + 1;
    let max_z = coords.iter().map(|c| c.z).max().unwrap() + 1;

    // x,y,z - make it slightly larger than it needs to be, to help DFS achieve
    let mut grid: Vec<Vec<Vec<Cube>>> = vec![
        vec![
            vec![
                Cube {
                    solid: false,
                    visited: false
                };
                max_z + 1
            ];
            max_y + 1
        ];
        max_x + 1
    ];

    // Fill in the grid
    for c in coords {
        grid[c.x][c.y][c.z].solid = true;
    }

    // Discover all the external squares - assume they're all connected to the edge
    dfs(
        &mut grid,
        Coord {
            x: max_x - 1,
            y: max_y - 1,
            z: max_z - 1,
        },
    );

    let mut part_a = 0;
    let mut part_b = 0;

    // Iterate over z
    for x in 0..max_x {
        for y in 0..max_y {
            let mut previously_solid = false;
            let mut previously_internal = false;
            for z in 0..max_z {
                has_visible_face(
                    &grid[x][y][z],
                    &mut previously_solid,
                    &mut previously_internal,
                    &mut part_a,
                    &mut part_b,
                )
            }

            let mut previously_solid = false;
            let mut previously_internal = false;
            for z in (0..max_z).rev() {
                has_visible_face(
                    &grid[x][y][z],
                    &mut previously_solid,
                    &mut previously_internal,
                    &mut part_a,
                    &mut part_b,
                )
            }
        }
    }

    // Iterate over x
    for y in 0..max_y {
        for z in 0..max_z {
            let mut previously_solid = false;
            let mut previously_internal = false;
            for x in 0..max_x {
                has_visible_face(
                    &grid[x][y][z],
                    &mut previously_solid,
                    &mut previously_internal,
                    &mut part_a,
                    &mut part_b,
                )
            }

            let mut previously_solid = false;
            let mut previously_internal = false;
            for x in (0..max_x).rev() {
                has_visible_face(
                    &grid[x][y][z],
                    &mut previously_solid,
                    &mut previously_internal,
                    &mut part_a,
                    &mut part_b,
                )
            }
        }
    }

    // Iterate over y
    for z in 0..max_z {
        for x in 0..max_x {
            let mut previously_solid = false;
            let mut previously_internal = false;
            for y in 0..max_y {
                has_visible_face(
                    &grid[x][y][z],
                    &mut previously_solid,
                    &mut previously_internal,
                    &mut part_a,
                    &mut part_b,
                )
            }

            let mut previously_solid = false;
            let mut previously_internal = false;
            for y in (0..max_y).rev() {
                has_visible_face(
                    &grid[x][y][z],
                    &mut previously_solid,
                    &mut previously_internal,
                    &mut part_a,
                    &mut part_b,
                )
            }
        }
    }

    println!("Part A is: {}", part_a);
    println!("Part B is: {}", part_b);
    (part_a, part_b)
}

pub fn day18() {
    let input = include_str!("../inputs/day18.txt");
    let coords = parse_input(input);

    _ = solve(coords)
}

#[test]
fn sample_input() {
    let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    let cs = parse_input(input);
    let (part_a, part_b) = solve(cs);
    assert_eq!(part_a, 64);
    assert_eq!(part_b, 58);
}

#[test]
fn c_shape() {
    let input = r#"1,1,0
2,1,0
1,2,0
1,3,0
2,3,0
3,3,0"#;

    let cs = parse_input(input);
    let (part_a, part_b) = solve(cs);
    assert_eq!(part_a, 26);
    assert_eq!(part_b, 26);
}
