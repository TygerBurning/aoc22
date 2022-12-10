use regex::Regex;

// Prints pixel - also handles newlines for us.
fn print_pixel(x: i32, cycle: i32) {
    if i32::abs(x - (cycle - 1) % 40) <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if cycle % 40 == 0 {
        println!();
    }
}

fn check_and_update_sum(x: i32, cycle: i32, sum: &mut i32) {
    if cycle % 40 == 20 {
        *sum += x * cycle;
    }
}

fn run_program(s: &str) -> i32 {
    let re = Regex::new(r"addx (.*)").unwrap();

    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    // Whenever the cycle is incremented, we must:
    // - CRT renders pixel
    // - Check for the sum
    for instruction in s.lines() {
        cycle += 1;
        print_pixel(x, cycle);
        check_and_update_sum(x, cycle, &mut sum);

        if instruction != "noop" {
            cycle += 1;
            print_pixel(x, cycle);
            check_and_update_sum(x, cycle, &mut sum);

            x += re.captures(instruction).unwrap()[1].parse::<i32>().unwrap();
        }
    }
    sum
}

pub fn day10() {
    let input = include_str!("../inputs/day10.txt");
    let sum = run_program(input);
    println!("Part A is: {}", sum);
}

#[test]
fn sample_input() {
    assert_eq!(
        13140,
        run_program(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
        )
    )
}
