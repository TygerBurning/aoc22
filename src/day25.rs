use std::fmt::Debug;

enum SnafuUnit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl SnafuUnit {
    fn to_decimal(&self) -> i64 {
        match self {
            SnafuUnit::Two => 2,
            SnafuUnit::One => 1,
            SnafuUnit::Zero => 0,
            SnafuUnit::Minus => -1,
            SnafuUnit::DoubleMinus => -2,
        }
    }
}

impl Debug for SnafuUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnafuUnit::Two => write!(f, "2"),
            SnafuUnit::One => write!(f, "1"),
            SnafuUnit::Zero => write!(f, "0"),
            SnafuUnit::Minus => write!(f, "-"),
            SnafuUnit::DoubleMinus => write!(f, "="),
        }
    }
}

struct Snafu {
    val: Vec<SnafuUnit>,
}

impl Snafu {
    fn to_decimal(&self) -> i64 {
        let mut base = 1;
        let mut sum = 0;
        for s in self.val.iter().rev() {
            sum += base * s.to_decimal();
            base *= 5;
        }
        sum
    }
}

impl Debug for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.val).finish()
    }
}

fn to_snafu(i: i64) -> Snafu {
    // This is naff but it's Christmas!
    let mut ord_index = 0;
    let mut count = 2;
    while i >= 5_i64.pow(ord_index + 1) - count {
        ord_index += 1;
        count = count + 5_i64.pow(ord_index) * 2;
    }

    let mut mod_i = i;
    for _i in 0..ord_index + 1 {
        mod_i += 2 * 5_i64.pow(_i)
    }

    let mut ss = vec![];
    for _i in (0..ord_index + 1).rev() {
        let x = mod_i / 5_i64.pow(_i);
        mod_i = mod_i - x * 5_i64.pow(_i);
        match x {
            0 => ss.push(SnafuUnit::DoubleMinus),
            1 => ss.push(SnafuUnit::Minus),
            2 => ss.push(SnafuUnit::Zero),
            3 => ss.push(SnafuUnit::One),
            4 => ss.push(SnafuUnit::Two),
            _ => panic!("Something has gone wrong..."),
        }
    }
    Snafu { val: ss }
}

fn parse_input(input: &str) -> Vec<Snafu> {
    input
        .lines()
        .map(|s| Snafu {
            val: s
                .chars()
                .map(|c| match c {
                    '2' => SnafuUnit::Two,
                    '1' => SnafuUnit::One,
                    '0' => SnafuUnit::Zero,
                    '-' => SnafuUnit::Minus,
                    '=' => SnafuUnit::DoubleMinus,
                    _ => panic!("Unexpected character"),
                })
                .collect(),
        })
        .collect()
}

pub fn day25() {
    let input = include_str!("../inputs/day25.txt");
    let snafus = parse_input(input);

    let snafu_sum = snafus.iter().map(|s| s.to_decimal()).sum::<i64>();
    println!("Part A is {:?}", to_snafu(snafu_sum));
}

#[test]
fn simple() {
    let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
    let snafus = parse_input(input);

    for s in &snafus {
        println!("{:?} translates to {}", s, s.to_decimal());
    }
    assert_eq!(1747, snafus[0].to_decimal());
}
