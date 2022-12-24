#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Plus,
    Mul,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op_type: Operation,
    op_args: (i64, i64),
    div: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn parse(lines: &[&str]) -> Self {
        assert!(lines.len() == 6);
        let items = lines[1][18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut ops = lines[2][19..].split(" ");
        let arg1 = ops.next().unwrap();
        let op = ops.next().unwrap();
        let arg2 = ops.next().unwrap();
        let arg1 = if arg1 == "old" {
            -1
        } else {
            arg1.parse().unwrap()
        };
        let arg2 = if arg2 == "old" {
            -1
        } else {
            arg2.parse().unwrap()
        };
        let op = match op {
            "*" => Operation::Mul,
            "+" => Operation::Plus,
            _ => panic!("nay"),
        };
        let div = lines[3][21..].parse().unwrap();
        let if_true = lines[4][29..].parse().unwrap();
        let if_false = lines[5][30..].parse().unwrap();
        Self {
            items,
            op_type: op,
            op_args: (arg1, arg2),
            div,
            if_true,
            if_false,
        }
    }
}

pub fn solve() {
    let data = include_str!("inputs/11.txt").trim();
    let lines: Vec<&str> = data.split('\n').collect();

    let mut monkeys = Vec::new();
    let mut all_div = 1;
    for i in 0..((lines.len() + 1) / 7) {
        monkeys.push(Monkey::parse(&lines[7 * i..7 * i + 6]));
        if all_div % monkeys[i].div != 0 {
            all_div *= monkeys[i].div;
        }
    }

    let mut monkeys = vec![monkeys.clone(), monkeys.clone()];

    let mut tasks = [0; 2];
    for task in 0..2 {
        let mut inspectations = vec![0; monkeys[task].len()];
        let rounds = [20, 10000][task];

        for _ in 0..rounds {
            for i in 0..monkeys[task].len() {
                let if_true = monkeys[task][i].if_true;
                let if_false = monkeys[task][i].if_false;
                let op_args = monkeys[task][i].op_args;
                let op_type = monkeys[task][i].op_type;
                let div = monkeys[task][i].div;

                inspectations[i] += monkeys[task][i].items.len();

                // assign new priorities
                for item in &mut monkeys[task][i].items {
                    let a1 = if op_args.0 >= 0 { op_args.0 } else { *item };
                    let a2 = if op_args.1 >= 0 { op_args.1 } else { *item };
                    *item = match op_type {
                        Operation::Mul => a1 * a2,
                        Operation::Plus => a1 + a2,
                    };
                    if task == 0 {
                        *item /= 3;
                    }
                    *item = *item % all_div;
                }

                let items = monkeys[task][i].items.split_off(0);
                for item in &items {
                    if item % div == 0 {
                        monkeys[task][if_true].items.push(*item);
                    } else {
                        monkeys[task][if_false].items.push(*item);
                    }
                }
            }
        }

        inspectations.sort();
        inspectations.reverse();
        let _ = inspectations.split_off(2);
        tasks[task] = inspectations[0] * inspectations[1];
    }

    println!("[day 11] task 1: {}", tasks[0]);
    println!("[day 11] task 2: {}", tasks[1]);
}
