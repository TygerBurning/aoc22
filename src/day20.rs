#[derive(Debug, Copy, Clone)]
struct N {
    original_pos: usize,
    wrap_val: usize,
    real_val: i64,
}

fn modulo(a: i64, b: usize) -> usize {
    (((a % b as i64) + b as i64) % b as i64) as usize
}

fn parse_input(s: &str) -> Vec<N> {
    let len = s.lines().count();

    // PART_B COMMENT HERE
    // let multiplier = 1;
    let multiplier = 811589153;

    s.lines()
        .enumerate()
        .map(|(i, s)| N {
            original_pos: i,
            wrap_val: modulo(s.parse::<i64>().unwrap() * multiplier, len - 1),
            real_val: s.parse::<i64>().unwrap() * multiplier,
        })
        .collect()
}

fn calculate_answer(nums: &Vec<N>) -> i64 {
    let zero_index = nums.iter().position(|&elem| elem.real_val == 0).unwrap() as usize;
    let a = nums[(1000 + zero_index) % nums.len()];
    let b = nums[(2000 + zero_index) % nums.len()];
    let c = nums[(zero_index + 3000) % nums.len()];
    a.real_val + b.real_val + c.real_val
}

fn solve(nums: &mut Vec<N>) -> (i64, i64) {
    let mut part_a = 0;
    for round in 0..10 {
    for old_index in 0..nums.len() {
        let current_index = nums
            .iter()
            .position(|n| n.original_pos == old_index)
            .unwrap();

        let new_index = modulo(
            current_index as i64
                + nums
                    .iter()
                    .find(|n| n.original_pos == old_index)
                    .unwrap()
                    .wrap_val as i64,
            nums.len() - 1,
        );

        if current_index < new_index + 1 {
            nums[current_index..new_index + 1].rotate_left(1);
        } else {
            nums[new_index..current_index + 1].rotate_right(1);
        }
    }
        if round == 0 {
            part_a = calculate_answer(nums)
        }
    }

    (part_a, calculate_answer(nums))
}

pub fn day20() {
    let input = include_str!("../inputs/day20.txt");
    let mut nums: Vec<N> = parse_input(input);
    let (a, b) = solve(&mut nums);
    println!("Part A is: {}", a);
    println!("Part B is: {}", b);
}

#[test]
fn sample_input() {
    let input = r#"1
2
-3
3
-2
0
4"#;
    let mut nums: Vec<N> = parse_input(input);
    let (a, b) = solve(&mut nums);
    assert_eq!(a, 3);
    assert_eq!(b, 1623178306);
}
