use std::ops::Range;

fn contains_range(a: &Range<i32>, b: &Range<i32>) -> bool {
    (a.start <= b.start) && (a.end >= b.end)
}

fn overlaps(a: &Range<i32>, b: &Range<i32>) -> bool {
    (a.end > b.start) && (b.end > a.start)
}

fn parse_range(range: &str) -> Range<i32> {
    let mut parts = range.split("-");
    let a = parts.next().unwrap().parse::<i32>().unwrap();
    let b = parts.next().unwrap().parse::<i32>().unwrap();
    a..(b + 1)
}

fn parse_range_pair(line: &str) -> (Range<i32>, Range<i32>) {
    let mut parts = line.split(",");
    let a = parse_range(parts.next().unwrap());
    let b = parse_range(parts.next().unwrap());
    (a, b)
}

pub fn solve() {
    let lines = include_str!("inputs/4.txt").split('\n');
    let range_pairs: Vec<(Range<i32>, Range<i32>)> = lines
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(parse_range_pair(line))
            }
        })
        .collect();

    let task1 = range_pairs
        .iter()
        .filter(|(a, b)| contains_range(a, b) || contains_range(b, a))
        .count();
    let task2 = range_pairs.iter().filter(|(a, b)| overlaps(a, b)).count();

    println!("[day 04] task 1: {}", task1);
    println!("[day 04] task 2: {}", task2);
}
