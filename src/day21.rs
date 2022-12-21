use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Monkey {
    Val(i64),
    Op(String, char, String),
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let re_val = Regex::new(r"(.*): (\d+)").unwrap();
    let re_op = Regex::new(r"(.*): (.+) (.) (.+)").unwrap();

    let mut hm = HashMap::new();
    for s in input.lines() {
        if re_val.is_match(s) {
            let x = re_val.captures(s).unwrap();
            hm.insert(x[1].to_string(), Monkey::Val(x[2].parse().unwrap()));
        } else if re_op.is_match(s) {
            let x = re_op.captures(s).unwrap();
            hm.insert(
                x[1].to_string(),
                Monkey::Op(
                    x[2].to_string(),
                    x[3].chars().next().unwrap(),
                    x[4].to_string(),
                ),
            );
        }
    }
    hm
}

fn contains(hm: &HashMap<String, Monkey>, root: &str, search: &str) -> bool {
    let m = hm.get(root).unwrap();
    match m {
        Monkey::Val(_) => root == search,
        Monkey::Op(l, _, r) => contains(hm, l, search) || contains(hm, r, search),
    }
}

fn calculate_value(hm: &HashMap<String, Monkey>, name: &str) -> i64 {
    let m = hm.get(name).unwrap();
    match m {
        Monkey::Val(x) => *x,
        Monkey::Op(l, op, r) => {
            let l_val = calculate_value(hm, l);
            let r_val = calculate_value(hm, r);
            let output = match op {
                '+' => l_val + r_val,
                '-' => l_val - r_val,
                '*' => l_val * r_val,
                '/' => l_val / r_val,
                _ => panic!("Non-matching operator: {}", op),
            };
            output
        }
    }
}

// Provide the output value of this Monkey node. The provided value should be calculated
fn find_value_for(hm: &HashMap<String, Monkey>, root: &str, output: i64, search: &str) -> i64 {
    if root == search {
        return output;
    }

    let m = hm.get(root).unwrap();
    match m {
        Monkey::Val(x) => {
            assert_eq!(root, search);
            *x
        }
        Monkey::Op(l, op, r) => {
            if contains(hm, l, search) {
                let r_val = calculate_value(hm, r);
                let output = match op {
                    '+' => find_value_for(hm, l, output - r_val, search),
                    '-' => find_value_for(hm, l, output + r_val, search),
                    '*' => find_value_for(hm, l, output / r_val, search),
                    '/' => find_value_for(hm, l, output * r_val, search),
                    _ => panic!("Non-matching operator: {}", op),
                };
                output
            } else if contains(hm, r, search) {
                let l_val = calculate_value(hm, l);
                let output = match op {
                    '+' => find_value_for(hm, r, output - l_val, search),
                    '-' => find_value_for(hm, r, l_val - output, search),
                    '*' => find_value_for(hm, r, output / l_val, search),
                    '/' => find_value_for(hm, r, l_val / output, search),
                    _ => panic!("Non-matching operator: {}", op),
                };
                output
            } else {
                panic!("humn is in neither fork!");
            }
        }
    }
}

fn solve_part_b(monkeys: &HashMap<String, Monkey>) -> i64 {
    let (l, r) = match monkeys.get("root").unwrap() {
        Monkey::Val(_) => panic!("root is a val!"),
        Monkey::Op(l, _, r) => (l, r),
    };

    let humn;
    if contains(&monkeys, l, "humn") {
        let r_val = calculate_value(&monkeys, r);
        humn = find_value_for(&monkeys, l, r_val, "humn");
    } else if contains(&monkeys, r, "humn") {
        let l_val = calculate_value(&monkeys, l);
        humn = find_value_for(&monkeys, r, l_val, "humn");
    } else {
        panic!("I'm in neither fork!");
    }

    humn
}

pub fn day21() {
    let input = include_str!("../inputs/day21.txt");

    let monkeys = parse_input(input);
    let val = calculate_value(&monkeys, "root");
    println!("Part A is: {}", val);

    println!("Part B is: {}", solve_part_b(&monkeys));
}

#[test]
fn sample_input() {
    let input = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    let monkeys = parse_input(input);
    let val = calculate_value(&monkeys, "root");
    assert_eq!(val, 152);

    assert_eq!(solve_part_b(&monkeys), 301);
}
