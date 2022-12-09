#[derive(Debug, Clone)]
struct Tree {
    i: usize,
    j: usize,
    height: u8,
    nearest_heights_left: [Option<usize>; 10],
    nearest_heights_top: [Option<usize>; 10],
    nearest_heights_right: [Option<usize>; 10],
    nearest_heights_bottom: [Option<usize>; 10],
}

fn how_visible(tree_grid: Vec<Vec<Tree>>) -> i32 {
    let mut sum = 0;
    for row in tree_grid.iter() {
        for tree in row.iter() {
            // Check that in each direction that all trees our size or bigger haven't
            // been seen (e.g. only check the subset of the array.)
            if tree.nearest_heights_right[(tree.height as usize)..]
                .iter()
                .all(|elem| elem.is_none())
                || tree.nearest_heights_bottom[(tree.height as usize)..]
                    .iter()
                    .all(|elem| elem.is_none())
                || tree.nearest_heights_left[(tree.height as usize)..]
                    .iter()
                    .all(|elem| elem.is_none())
                || tree.nearest_heights_top[(tree.height as usize)..]
                    .iter()
                    .all(|elem| elem.is_none())
            {
                sum += 1;
            }
        }
    }

    sum
}

fn how_scenic_am_i(tree: &Tree, max_height: usize, max_width: usize) -> u32 {
    let up_max = &tree.nearest_heights_top[(tree.height as usize)..]
        .iter()
        .map(|e| e.unwrap_or(0))
        .max()
        .unwrap();

    let left_max = &tree.nearest_heights_left[(tree.height as usize)..]
        .iter()
        .map(|e| e.unwrap_or(0))
        .max()
        .unwrap();

    let bottom_min = &tree.nearest_heights_bottom[(tree.height as usize)..]
        .iter()
        .map(|e| e.unwrap_or(max_height))
        .min()
        .unwrap();

    let right_min = &tree.nearest_heights_right[(tree.height as usize)..]
        .iter()
        .map(|e| e.unwrap_or(max_width))
        .min()
        .unwrap();

    ((tree.i - up_max) * (tree.j - left_max) * (bottom_min - tree.i) * (right_min - tree.j)) as u32
}

fn how_scenic(tree_grid: Vec<Vec<Tree>>) -> Vec<Vec<u32>> {
    let score_grid = tree_grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|tree| how_scenic_am_i(tree, tree_grid.len() - 1, tree_grid[0].len() - 1))
                .collect()
        })
        .collect();
    score_grid
}

fn build(input: &str) -> Vec<Vec<Tree>> {
    let mut tree_grid: Vec<Vec<Tree>> = vec![];
    for (i, line) in input.lines().enumerate() {
        tree_grid.push(
            line.chars()
                .enumerate()
                .map(|(j, t)| Tree {
                    i: i,
                    j: j,
                    height: t.to_digit(10).unwrap() as u8,
                    nearest_heights_left: [None; 10],
                    nearest_heights_top: [None; 10],
                    nearest_heights_right: [None; 10],
                    nearest_heights_bottom: [None; 10],
                })
                .collect(),
        );
    }

    // Build top and left.
    for i in 0..tree_grid.len() {
        for j in 0..tree_grid[0].len() {
            if i > 0 {
                let above = tree_grid.get(i - 1).unwrap().get(j).unwrap();
                let mut top_trees = above.nearest_heights_top.clone();
                top_trees[above.height as usize] = Some(i - 1);

                tree_grid
                    .get_mut(i)
                    .unwrap()
                    .get_mut(j)
                    .unwrap()
                    .nearest_heights_top = top_trees;
            }
            if j > 0 {
                let left = tree_grid.get(i).unwrap().get(j - 1).unwrap();
                let mut left_trees = left.nearest_heights_left.clone();
                left_trees[left.height as usize] = Some(j - 1);

                tree_grid
                    .get_mut(i)
                    .unwrap()
                    .get_mut(j)
                    .unwrap()
                    .nearest_heights_left = left_trees;
            }
        }
    }

    // Built right and bottom by working backwards.
    for i in (0..tree_grid.len()).rev() {
        for j in (0..tree_grid[0].len()).rev() {
            if i < tree_grid.len() - 1 {
                let bottom = tree_grid.get(i + 1).unwrap().get(j).unwrap();
                let mut bottom_trees = bottom.nearest_heights_bottom.clone();
                bottom_trees[bottom.height as usize] = Some(i + 1);

                tree_grid
                    .get_mut(i)
                    .unwrap()
                    .get_mut(j)
                    .unwrap()
                    .nearest_heights_bottom = bottom_trees;
            }

            if j < tree_grid[0].len() - 1 {
                let right = tree_grid.get(i).unwrap().get(j + 1).unwrap();
                let mut right_trees = right.nearest_heights_right.clone();
                right_trees[right.height as usize] = Some(j + 1);

                tree_grid
                    .get_mut(i)
                    .unwrap()
                    .get_mut(j)
                    .unwrap()
                    .nearest_heights_right = right_trees;
            }
        }
    }
    tree_grid
}

pub fn day08() {
    let input = include_str!("../inputs/day08.txt");

    let grid = build(input);

    println!("Part A is {}", how_visible(grid.clone()));
    println!(
        "Part B is {}",
        how_scenic(grid).iter().flatten().max().unwrap()
    );
}

#[test]
fn sample_input() {
    assert_eq!(
        21,
        how_visible(build(
            "30373
25512
65332
33549
35390",
        ))
    )
}

#[test]
fn sample_input_b() {
    assert_eq!(
        &8,
        how_scenic(build(
            "30373
25512
65332
33549
35390",
        ))
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
    )
}

#[test]
fn edged_forest() {
    assert_eq!(
        20,
        how_visible(build(
            "1111111111
1111111111"
        ))
    )
}

#[test]
fn subset_forest() {
    assert_eq!(338, how_visible(build(
"404310113342042430523206553054613026452223317713352040314520532034136044035422242052404520323121423
120022131035104020413264056552251024207112052153646716635163015540126546334604634352130343204203212
114444043404350333465355412140536532335724647116565610572546474626506052303052063065052530351012030
212240414554223531015555065165021520410363021033163304514155663733352116600665403462501544352500033
324144130111224105125305614015103771206061460425505011635353313773273343325122626415203015344311232"
        )))
}

#[test]
fn real_input() {
    assert_eq!(
        1538,
        how_visible(build(include_str!("../inputs/day08.txt")))
    );
    assert_eq!(
        &496125,
        how_scenic(build(include_str!("../inputs/day08.txt")))
            .iter()
            .flatten()
            .max()
            .unwrap()
    );
}
