use itertools::izip;

pub fn day06() {
    let stream: Vec<char> = include_str!("../inputs/day06.txt").chars().collect();

    let (_, _, _, _, i) = izip!(&stream, &stream[1..], &stream[2..], &stream[3..], 1..)
        .find(|(&a, &b, &c, &d, _)| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap();

    // +3 because they want the index of the *last* char, not the first char.
    println!("Part A answer is: {:?}", i + 3);
}
