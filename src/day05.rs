use regex::Regex;

pub fn day05() {
    let mut stacks = vec![
        vec!['L', 'N', 'W', 'T', 'D'],
        vec!['C', 'P', 'H'],
        vec!['W', 'P', 'H', 'N', 'D', 'G', 'M', 'J'],
        vec!['C', 'W', 'S', 'N', 'T', 'Q', 'L'],
        vec!['P', 'H', 'C', 'N'],
        vec!['T', 'H', 'N', 'D', 'M', 'W', 'Q', 'B'],
        vec!['M', 'B', 'R', 'J', 'G', 'S', 'L'],
        vec!['Z', 'N', 'W', 'G', 'V', 'B', 'R', 'T'],
        vec!['W', 'G', 'D', 'N', 'P', 'L'],
    ];

    let moves = include_str!("../inputs/day05.txt").lines();

    for m in moves {
        let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
        let cap = re.captures_iter(m).next().unwrap();
        let count = cap[1].parse::<u32>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        // Part 1
        // for _ in 0..count {
        //     let elem = stacks[from].pop().unwrap();
        //     stacks[to].push(elem);
        // }

        // Part 2
        let mut hold = vec![];
        for _ in 0..count {
            let elem = stacks[from].pop().unwrap();
            hold.push(elem);
        }
        for _ in 0..count {
            let elem = hold.pop().unwrap();
            stacks[to].push(elem);
        }
    }
    println!(
        "{:?}",
        stacks.iter().map(|v| v.last().unwrap()).collect::<Vec<_>>()
    );
}
