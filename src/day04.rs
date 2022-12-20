use regex::Regex;

fn parse(elem: &str) -> (u32, u32, u32, u32) {
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    let cap = re.captures_iter(elem).next().unwrap();
    (
        cap[1].parse::<u32>().unwrap(),
        cap[2].parse::<u32>().unwrap(),
        cap[3].parse::<u32>().unwrap(),
        cap[4].parse::<u32>().unwrap(),
    )
}

fn parse_and_subset(elem: &str) -> bool {
    let (a_min, a_max, b_min, b_max) = parse(elem);
    (a_min >= b_min && a_max <= b_max) || (a_min <= b_min && a_max >= b_max)
}

fn parse_and_overlap(elem: &str) -> bool {
    let (a_min, a_max, b_min, b_max) = parse(elem);
    (a_max >= b_min && a_min <= b_max) || (b_max >= a_min && b_min <= a_max)
}

pub fn day04() {
    let section_pairs = include_str!("../inputs/day04.txt").lines();

    let subset = section_pairs.clone().map(parse_and_subset);
    let overlap = section_pairs.map(parse_and_overlap);

    println!("Part A answer is: {:?}", subset.filter(|p| *p).count());
    println!("Part B answer is: {:?}", overlap.filter(|p| *p).count());
}
