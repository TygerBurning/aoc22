use std::fmt::Debug;
use std::vec;

#[derive(Eq, Clone)]
enum List {
    Val(u32),
    Ls(Vec<List>),
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match self {
            List::Val(v1) => match other {
                List::Val(v2) => v1 == v2,
                List::Ls(vec2) => {
                    println!(" {:?} with {}", vec2, v1);
                    vec2.eq(&vec![List::Val(v1.clone())])
                }
            },
            List::Ls(vec1) => match other {
                List::Val(v2) => vec1.eq(&vec![List::Val(v2.clone())]),
                List::Ls(vec2) => vec1.eq(vec2),
            },
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            List::Val(v1) => match other {
                List::Val(v2) => Some(v1.cmp(v2)),
                List::Ls(_) => List::Ls(vec![List::Val(v1.clone())]).partial_cmp(&other),
            },
            List::Ls(vec1) => match other {
                List::Val(v2) => self.partial_cmp(&List::Ls(vec![List::Val(v2.clone())])),
                List::Ls(vec2) => {
                    for (a, b) in vec1.iter().zip(vec2) {
                        if a > b {
                            return Some(std::cmp::Ordering::Greater);
                        }
                        if a < b {
                            return Some(std::cmp::Ordering::Less);
                        }
                    }
                    if vec1.len() > vec2.len() {
                        return Some(std::cmp::Ordering::Greater);
                    }
                    if vec1.len() < vec2.len() {
                        return Some(std::cmp::Ordering::Less);
                    }
                    return Some(std::cmp::Ordering::Equal);
                }
            },
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(arg0) => write!(f, "{}", arg0),
            Self::Ls(arg0) => f.debug_list().entries(arg0).finish(),
        }
    }
}

impl List {
    fn new() -> Self {
        List::Ls(vec![])
    }

    fn add_at_depth(&mut self, val: u32, depth: i32) {
        match self {
            List::Val(_) => todo!(),
            List::Ls(v) => {
                if depth == 1 {
                    v.push(List::Val(val));
                } else {
                    v.last_mut().unwrap().add_at_depth(val, depth - 1)
                }
            }
        }
    }

    fn create_sublist_at_depth(&mut self, depth: i32) {
        match self {
            List::Val(_) => todo!(),
            List::Ls(v) => {
                if depth == 1 {
                    v.push(List::Ls(vec![]))
                } else {
                    v.last_mut().unwrap().create_sublist_at_depth(depth - 1)
                }
            }
        }
    }
}

fn parse_input(input: &str) -> List {
    let mut root = List::new();
    let mut depth = 1;
    let mut digit = None;
    for elem in input.chars().skip(1) {
        match elem {
            '[' => {
                root.create_sublist_at_depth(depth);
                depth += 1;
            }
            ']' => {
                if digit.is_some() {
                    root.add_at_depth(digit.unwrap(), depth);
                }
                digit = None;
                depth -= 1;
            }
            ',' => {
                if digit.is_some() {
                    root.add_at_depth(digit.unwrap(), depth);
                    digit = None;
                }
            }
            x => match digit {
                None => digit = Some(x.to_digit(10).unwrap()),
                Some(d) => digit = Some(d * 10 + x.to_digit(10).unwrap()),
            },
        }
    }
    root
}

fn part_a(input: &str) -> u32 {
    let mut sum = 0;
    for (index, lss) in input.split("\n\n").enumerate() {
        let a = lss.split("\n").collect::<Vec<&str>>()[0];
        let b = lss.split("\n").collect::<Vec<&str>>()[1];

        if parse_input(a) < parse_input(b) {
            sum += index as u32 + 1;
        }
    }
    sum
}

pub fn day13() {
    let input = include_str!("../inputs/day13.txt");

    let sum = part_a(input.clone());
    println!("Part A is {}", sum);

    let mut all_messages = input
        .split("\n")
        .filter(|&elem| elem != "")
        .map(|elem| parse_input(elem))
        .collect::<Vec<List>>();
    let divider_2 = parse_input("[[2]]");
    all_messages.push(divider_2.clone());
    let divider_6 = parse_input("[[6]]");
    all_messages.push(divider_6.clone());
    all_messages.sort();

    println!(
        "Part B is {}",
        (all_messages.binary_search(&divider_2).unwrap() + 1)
            * (all_messages.binary_search(&divider_6).unwrap() + 1)
    );
}

#[test]
fn simple_input() {
    // Val vs Val
    assert!(List::Val(1) < List::Val(2));

    // Ls vs Val
    assert!(List::Ls(vec![List::Val(1)]) < List::Val(2));
    assert!(!(List::Ls(vec![List::Val(1), List::Val(0)]) < List::Val(1)));

    // Val vs Ls
    assert!(List::Val(1) < List::Ls(vec![List::Val(2)]));
    assert!(List::Val(1) < List::Ls(vec![List::Val(1), List::Val(2)]));

    // Ls vs Ls (same length)
    assert!(List::Ls(vec![List::Val(1)]) < List::Ls(vec![List::Val(2)]));

    // Ls vs Ls (shorter length)
    assert!(List::Ls(vec![List::Val(1)]) < List::Ls(vec![List::Val(1), List::Val(0)]));
    assert!(!(List::Ls(vec![List::Val(1), List::Val(0)]) < List::Ls(vec![List::Val(1)])));

    // Sample 2
    assert!(
        List::Ls(vec![
            List::Ls(vec![List::Val(1)]),
            List::Ls(vec![List::Val(2), List::Val(3), List::Val(4)])
        ]) < List::Ls(vec![List::Ls(vec![List::Val(1)]), List::Val(4)])
    );
    // Bit more complex
    assert!(
        List::Ls(vec![
            List::Val(0),
            List::Ls(vec![List::Val(1), List::Val(2), List::Val(3)]),
            List::Val(5)
        ]) < List::Ls(vec![
            List::Val(0),
            List::Ls(vec![List::Val(1), List::Val(2), List::Val(3)]),
            List::Val(6)
        ])
    );
}

#[test]
fn sample_input() {
    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    let sum = part_a(input);
    assert_eq!(sum, 13);
}

#[test]
fn real_data() {
    let input = r#"[[[[2,0,1],6,[1],7]],[7,10,2],[],[[]],[2,1,[[],9],[[6,9,9,6,0],[],[10,2]]]]
[[4,[],6,5],[[],[[],[5,2]],[1,4],0,[[5],[9,9,4,1,5]]],[[],10,10,[8],[1,[8],2,9,2]],[7,3,[[0,8,5,2],[4,2,10],0]],[1,[[1,9,5],3,10,[10,8]],9,[[]]]]"#;

    let sum = part_a(input);
    assert_eq!(sum, 1);
}
