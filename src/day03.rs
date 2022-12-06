use itertools::Itertools;

fn get_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return c as u32 - 'a' as u32 + 1;
    } else if c.is_ascii_uppercase() {
        return c as u32 - 'A' as u32 + 26 + 1;
    }
    panic!("Couldn't find priority for {}", c)
}

fn find_overlap(a: &str, b: &str) -> u32 {
    let mut items: [bool; 52] = [false; 52];

    for c in a.chars() {
        items[get_priority(c) as usize - 1] = true;
    }

    for c in b.chars() {
        if items[get_priority(c) as usize - 1] {
            return get_priority(c);
        }
    }
    panic!("Couldn't find duplicate between {} and {}", a, b);
}

// Laziness... Should really just write a proper intersect function.
fn find_overlap_3(x: &str, y: &str, z: &str) -> u32 {
    let mut items: [u8; 52] = [0; 52];

    for c in x.chars().unique() {
        items[get_priority(c) as usize - 1] += 1;
    }

    for c in y.chars().unique() {
        items[get_priority(c) as usize - 1] += 1;
    }

    for c in z.chars() {
        if items[get_priority(c) as usize - 1] == 2 {
            return get_priority(c);
        }
    }
    panic!("Couldn't find duplicate between {}, {} and {}", x, y, z);
}

pub fn day03() {
    let rucksacks = include_str!("../inputs/day03.txt").lines();
    let priorities = rucksacks.clone().map(|rucksack| {
        find_overlap(
            &rucksack[..rucksack.len() / 2],
            &rucksack[rucksack.len() / 2..],
        )
    });

    let binding = rucksacks.collect::<Vec<_>>();
    let badges = binding
        .as_slice()
        .chunks(3)
        .map(|elem| find_overlap_3(elem[0], elem[1], elem[2]));

    println!("Part A answer is: {:?}", priorities.sum::<u32>());
    println!("Part B answer is: {:?}", badges.sum::<u32>());
}
