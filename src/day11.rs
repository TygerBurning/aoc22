#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    total_items_inspected: u64,
    f: fn(u64) -> u64,
    next_monkey_index: fn(u64) -> usize,
    check: u32,
}

impl Monkey {
    // This function isn't in charge of getting the next item.
    fn find_next_monkey_for_item(
        self: &mut Monkey,
        item: u64,
        part_b: bool,
        modulus: u32,
    ) -> (usize, u64) {
        self.total_items_inspected += 1;

        let new = if part_b {
            (self.f)(item) % modulus as u64
        } else {
            (self.f)(item) / 3
        };
        ((self.next_monkey_index)(new), new)
    }

    fn add_item(self: &mut Monkey, item: u64) {
        self.items.push(item)
    }
}

// Pcha.
// fn parse_input(input: &str) -> Vec<Monkey> {
//     let re = Regex::new(
//         r#"Monkey \d+:
//   Starting items: (?P<starting_items>.*)
//   Operation: new = (?P<lhs>.*) (?P<op>.*) (?P<rhs>.*)
//   Test: (?P<test_op>.*) by (?P<test_val>\d*)
//     If true: throw to monkey (?P<t_m>\d+)
//     If false: throw to monkey (?P<f_m>\d+)"#,
//     )
//     .unwrap();
// }

fn hand_fed_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![98, 70, 75, 80, 84, 89, 55, 98],
            total_items_inspected: 0,
            f: (|elem| elem * 2),
            next_monkey_index: (|elem| if elem % 11 == 0 { 1 } else { 4 }),
            check: 11,
        },
        Monkey {
            items: vec![59],
            total_items_inspected: 0,
            f: (|elem| elem * elem),
            next_monkey_index: (|elem| if elem % 19 == 0 { 7 } else { 3 }),
            check: 19,
        },
        Monkey {
            items: vec![77, 95, 54, 65, 89],
            total_items_inspected: 0,
            f: (|elem| elem + 6),
            next_monkey_index: (|elem| if elem % 7 == 0 { 0 } else { 5 }),
            check: 7,
        },
        Monkey {
            items: vec![71, 64, 75],
            total_items_inspected: 0,
            f: (|elem| elem + 2),
            next_monkey_index: (|elem| if elem % 17 == 0 { 6 } else { 2 }),
            check: 17,
        },
        Monkey {
            items: vec![74, 55, 87, 98],
            total_items_inspected: 0,
            f: (|elem| elem * 11),
            next_monkey_index: (|elem| if elem % 3 == 0 { 1 } else { 7 }),
            check: 3,
        },
        Monkey {
            items: vec![90, 98, 85, 52, 91, 60],
            total_items_inspected: 0,
            f: (|elem| elem + 7),
            next_monkey_index: (|elem| if elem % 5 == 0 { 0 } else { 4 }),
            check: 5,
        },
        Monkey {
            items: vec![99, 51],
            total_items_inspected: 0,
            f: (|elem| elem + 1),
            next_monkey_index: (|elem| if elem % 13 == 0 { 5 } else { 2 }),
            check: 13,
        },
        Monkey {
            items: vec![98, 94, 59, 76, 51, 65, 75],
            total_items_inspected: 0,
            f: (|elem| elem + 5),
            next_monkey_index: (|elem| if elem % 2 == 0 { 3 } else { 6 }),
            check: 2,
        },
    ]
}

fn solve(monkeys: &mut Vec<Monkey>, rounds: u64, part_b: bool) -> u64 {
    let modulus = monkeys.iter().map(|m| m.check).product::<u32>();
    for _ in 1..=rounds {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                let (next_m, new_item) =
                    monkeys[i].find_next_monkey_for_item(item, part_b, modulus);
                monkeys[next_m].add_item(new_item);
            }
            monkeys[i].items.clear();
        }
    }

    let mut inspected = monkeys
        .iter()
        .map(|elem| elem.total_items_inspected)
        .collect::<Vec<u64>>();
    println!("Each monkey inspected: {:?}", inspected);
    inspected.sort();
    inspected.reverse();
    inspected.iter().take(2).product::<u64>()
}

pub fn day11() {
    let _input = include_str!("../inputs/day11.txt");
    let monkeys = hand_fed_input();

    println!("Part A is: {}", solve(&mut monkeys.clone(), 20, false));
    println!("Part B is: {:?}", solve(&mut monkeys.clone(), 10000, true));
}

#[test]
fn sample_input() {
    let monkeys = vec![
        Monkey {
            items: vec![79, 98],
            total_items_inspected: 0,
            f: (|elem| elem * 19),
            next_monkey_index: (|elem| if elem % 23 == 0 { 2 } else { 3 }),
            check: 23,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            total_items_inspected: 0,
            f: (|elem| elem + 6),
            next_monkey_index: (|elem| if elem % 19 == 0 { 2 } else { 0 }),
            check: 19,
        },
        Monkey {
            items: vec![79, 60, 97],
            total_items_inspected: 0,
            f: (|elem| elem * elem),
            next_monkey_index: (|elem| if elem % 13 == 0 { 1 } else { 3 }),
            check: 13,
        },
        Monkey {
            items: vec![74],
            total_items_inspected: 0,
            f: (|elem| elem + 3),
            next_monkey_index: (|elem| if elem % 17 == 0 { 0 } else { 1 }),
            check: 17,
        },
    ];

    assert_eq!(10605, solve(&mut monkeys.clone(), 20, false));
    assert_eq!(2713310158, solve(&mut monkeys.clone(), 10000, true));
}
