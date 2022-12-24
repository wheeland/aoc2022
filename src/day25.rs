use core::panic;

fn from_snafu(str: &str) -> i64 {
    let mut sum = 0;
    let mut base = 1;
    for c in str.trim().chars().rev() {
        let digit = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("nay"),
        };
        sum += digit * base;
        base *= 5;
    }
    sum
}

fn to_snafu(mut n: i64) -> String {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 5);
        n /= 5;
    }
    digits.push(0);

    for i in 0..digits.len() {
        while digits[i] > 2 {
            digits[i] -= 5;
            digits[i + 1] += 1;
        }
    }

    while *digits.last().unwrap() == 0 {
        digits.pop();
    }

    digits
        .iter()
        .rev()
        .map(|n| match *n {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => panic!("nay"),
        })
        .collect()
}

pub fn solve() {
    let input = include_str!("inputs/25.txt");
    let numbers: Vec<i64> = input
        .split("\n")
        .filter_map(|l| {
            if l.trim().is_empty() {
                None
            } else {
                Some(from_snafu(l))
            }
        })
        .collect();

    let task1 = to_snafu(numbers.iter().sum());
    println!("[day 25] task 1: {}", task1);
}
