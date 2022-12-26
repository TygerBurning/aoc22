use std::{collections::HashMap, str::Split};

use regex::Regex;

fn calculate_size(lookup: &HashMap<String, (Vec<String>, u32)>, key: &String) -> u32 {
    let mut size = 0;
    let (children, local_size) = lookup.get(key).unwrap();
    for c in children {
        size += calculate_size(lookup, c)
    }
    size + local_size
}

fn parse_input(input: &str) -> Split<&str> {
    input.split("\n$ ")
}

fn solve(commands: Split<&str>) {
    let cd_re = Regex::new(r"cd (.*)").unwrap();
    let ls_re = Regex::new(r"ls\n(.*)").unwrap();
    let dir_re = Regex::new(r"dir (.*)").unwrap();
    let file_re = Regex::new(r"(\d*) (.*)").unwrap();

    let mut lookup = HashMap::new();
    // Current working directory - not part of the tree.
    let mut current_dir: Vec<String> = vec![];

    for command in commands {
        if cd_re.is_match(command) {
            let cap = cd_re.captures(command).unwrap();
            match &cap[1] {
                "/" => current_dir = vec![],
                ".." => {
                    current_dir.pop();
                }
                dir => {
                    current_dir.push(dir.to_string());
                }
            }
        } else if ls_re.is_match(command) {
            let mut fs = vec![];
            let mut local_size = 0;
            for f in command.lines() {
                if dir_re.is_match(f) {
                    let dir = dir_re.captures(f).unwrap()[1].to_string();
                    let mut entry = current_dir.clone();
                    entry.push(dir.to_string());
                    fs.push(format!("/{}", entry.join("/")));
                } else if file_re.is_match(f) {
                    let file = file_re.captures(f).unwrap();
                    let size = file[1].parse::<u32>().unwrap();
                    local_size += size;
                }
            }
            lookup.insert(format!("/{}", current_dir.join("/")), (fs, local_size));
        }
    }
    let mut part_a = 0;

    let mut directory_sizes = vec![];
    for k in lookup.keys() {
        let size = calculate_size(&lookup, k);
        if size <= 100000 {
            part_a += size;
        }
        directory_sizes.push((k, size));
    }
    println!("Part A is: {}", part_a);

    let total_size = 70000000;
    let required_space = 30000000;
    let used_size = calculate_size(&lookup, &"/".to_string());
    let max_size = total_size - required_space;

    let min_directory_to_delete = used_size - max_size;

    directory_sizes.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));
    println!(
        "Part B is: {:?}",
        directory_sizes
            .iter()
            .find(|(_, s)| s >= &min_directory_to_delete)
            .unwrap()
            .1
    );
}

pub fn day07() {
    let input = include_str!("../inputs/day07.txt");
    let commands = parse_input(input);
    solve(commands);
}

#[test]
fn sample_input_7() {
    let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    let commands = parse_input(input);
    solve(commands);
    assert!(false);
}
