use itertools::izip;

fn count_increases(readings: &[usize]) -> usize {
    readings
        .iter()
        .zip(readings[1..].iter())
        .filter(|(prev, next)| prev < next)
        .count()
}

pub fn day01() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
    let readings: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    println!("Part A answer is: {}", count_increases(&readings));

    let x: Vec<usize> = izip!(
        &readings,
        &readings[1..],
        &readings[2..]
    )
    .map(|(x, y, z)| x + y + z)
    .collect();

    println!("Part B answer is: {}", count_increases(&x))
}
