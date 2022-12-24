use std::collections::HashMap;

enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("nay"),
        }
    }
}

enum Monkey {
    Human,
    Number(i64),
    Operation(String, Operator, String),
}

impl Monkey {
    fn from(s: &str) -> Self {
        let s = s.trim();
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() == 1 {
            Self::Number(parts[0].parse().unwrap())
        } else {
            assert!(parts.len() == 3);
            Self::Operation(
                parts[0].to_string(),
                Operator::from(parts[1]),
                parts[2].to_string(),
            )
        }
    }
}

fn get(monkeys: &HashMap<String, Monkey>, name: &str) -> Option<i64> {
    match monkeys.get(name).unwrap() {
        Monkey::Human => None,
        Monkey::Number(num) => Some(*num),
        Monkey::Operation(name1, op, name2) => {
            let a = get(monkeys, name1);
            let b = get(monkeys, name2);
            a.zip(b).map(|(a, b)| match op {
                Operator::Plus => a + b,
                Operator::Minus => a - b,
                Operator::Mul => a * b,
                Operator::Div => a / b,
            })
        }
    }
}

fn get_human_number(monkeys: &HashMap<String, Monkey>, node: &str, expected: i64) -> i64 {
    let monkey = monkeys.get(node).unwrap();
    if let Monkey::Human = monkey {
        return expected;
    }

    let (left, op, right) = match monkey {
        Monkey::Operation(left, op, right) => (left, op, right),
        _ => panic!("nay"),
    };

    let left_value = get(&monkeys, left);
    let right_value = get(&monkeys, right);
    match op {
        Operator::Plus => {
            if left_value.is_some() {
                get_human_number(monkeys, right, expected - left_value.unwrap())
            } else {
                get_human_number(monkeys, left, expected - right_value.unwrap())
            }
        }
        Operator::Mul => {
            if left_value.is_some() {
                get_human_number(monkeys, right, expected / left_value.unwrap())
            } else {
                get_human_number(monkeys, left, expected / right_value.unwrap())
            }
        }
        Operator::Minus => {
            if left_value.is_some() {
                get_human_number(monkeys, right, left_value.unwrap() - expected)
            } else {
                get_human_number(monkeys, left, right_value.unwrap() + expected)
            }
        }
        Operator::Div => {
            if left_value.is_some() {
                get_human_number(monkeys, right, left_value.unwrap() / expected)
            } else {
                get_human_number(monkeys, left, right_value.unwrap() * expected)
            }
        }
    }
}

pub fn solve() {
    let input = include_str!("inputs/21.txt");

    let mut monkeys = HashMap::new();
    for line in input.split("\n") {
        let line = line.trim();
        if !line.is_empty() {
            let name = line[0..4].to_string();
            let monkey = Monkey::from(&line[6..]);
            monkeys.insert(name, monkey);
        }
    }

    let task1 = get(&monkeys, "root");
    println!("[day 21] task 1: {}", task1.unwrap());

    monkeys.insert(String::from("humn"), Monkey::Human);
    let (left, right) = if let Monkey::Operation(left, op, right) = monkeys.get("root").unwrap() {
        (left, right)
    } else {
        panic!();
    };
    let left_value = get(&monkeys, left);
    let right_value = get(&monkeys, right);
    let human = if left_value.is_some() {
        get_human_number(&monkeys, right, left_value.unwrap())
    } else {
        get_human_number(&monkeys, left, right_value.unwrap())
    };
    println!("[day 21] task 2: {}", human);
}
