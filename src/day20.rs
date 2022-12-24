use crate::vec3::{vec3, Vec3i64};

struct Item {
    value: i64,
    prev: usize,
    next: usize,
}

fn plus_n(items: &[Item], from: usize, n: usize) -> usize {
    let n = n % items.len();
    let mut new_spot = from;
    for _ in 0..n {
        new_spot = items[new_spot].next;
    }
    new_spot
}

fn reset(items: &mut [Item]) {
    let count = items.len();
    for i in 0..items.len() {
        items[i].prev = (i + count - 1) % count;
        items[i].next = (i + 1) % count;
    }
}

fn mix(items: &mut [Item]) {
    let count = items.len();
    let count1 = count as i64 - 1;

    for i in 0..count {
        // TODO: do this one-by-one, without caching prev/next

        // print(&items);
        let prev = items[i].prev;
        let next = items[i].next;

        if items[i].value == 0 {
            continue;
        }

        let diff = (items[i].value % count1 + count1) % count1;
        let new_spot = plus_n(items, i, diff as usize);
        items[prev].next = next;
        items[next].prev = prev;

        let new_next = items[new_spot].next;
        items[new_next].prev = i;
        items[new_spot].next = i;
        items[i].prev = new_spot;
        items[i].next = new_next;
    }
    // print(&items);
}

fn result(items: &[Item]) -> (i64, Vec3i64) {
    let null = items.iter().position(|item| item.value == 0).unwrap();

    let plus1000 = plus_n(&items, null, 1000);
    let plus2000 = plus_n(&items, null, 2000);
    let plus3000 = plus_n(&items, null, 3000);
    let a = items[plus1000].value;
    let b = items[plus2000].value;
    let c = items[plus3000].value;
    (a + b + c, vec3(a, b, c))
}

fn print(items: &[Item]) {
    let mut curr = 0;
    let mut s = String::new();
    loop {
        s += &items[curr].value.to_string();
        s += ", ";
        curr = items[curr].next;
        if curr == 0 {
            break;
        }
    }
    println!("{}", s);
}

pub fn solve() {
    let input = String::from(include_str!("inputs/20.txt"));
    let mut items: Vec<Item> = input
        .split("\n")
        .filter_map(|line| {
            line.trim().parse().ok().map(|value| Item {
                value,
                prev: 0,
                next: 0,
            })
        })
        .collect();

    reset(&mut items);
    mix(&mut items);
    let task1 = result(&items);

    for item in &mut items {
        item.value *= 811589153;
    }
    reset(&mut items);
    for _ in 0..10 {
        mix(&mut items);
    }
    let task2 = result(&items);

    println!("[day 20] task 1: {:?}", task1);
    println!("[day 20] task 2: {:?}", task2);
}
