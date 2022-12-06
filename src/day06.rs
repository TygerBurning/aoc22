use itertools::Itertools;

fn unique_chars(stream: Vec<char>, count: usize) -> usize {
    stream
        .windows(count)
        .position(|cs| cs.iter().unique().count() == cs.len())
        .unwrap()
        + count
}

pub fn day06() {
    let stream: Vec<char> = include_str!("../inputs/day06.txt").chars().collect();

    println!("Part A answer is: {:?}", unique_chars(stream.clone(), 4));
    println!("Part B answer is: {:?}", unique_chars(stream, 14));
}
