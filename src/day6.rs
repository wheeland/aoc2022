use crate::timer::Timer;

fn adjust(counts: &mut [i32; 26], unique: &mut i32, index: usize, add: bool) {
    counts[index] += if add { 1 } else { -1 };
    if add && counts[index] == 1 {
        *unique += 1;
    }
    if !add && counts[index] == 0 {
        *unique -= 1;
    }
}

pub fn solve() {
    let data = include_str!("inputs/6.txt").trim();

    let char_index: Vec<usize> = data.chars().map(|c| c as usize - 'a' as usize).collect();

    let mut counts1 = [0; 26];
    let mut counts2 = [0; 26];
    let mut unique1 = 0;
    let mut unique2 = 0;
    let mut task1 = None;
    let mut task2 = None;
    for i in 0..data.len() {
        if i >= 4 {
            adjust(&mut counts1, &mut unique1, char_index[i - 4], false);
        }
        if i >= 14 {
            adjust(&mut counts2, &mut unique2, char_index[i - 14], false);
        }

        adjust(&mut counts1, &mut unique1, char_index[i], true);
        adjust(&mut counts2, &mut unique2, char_index[i], true);

        if unique1 == 4 && task1.is_none() {
            task1 = Some(i + 1);
        }
        if unique2 == 14 && task2.is_none() {
            task2 = Some(i + 1);
        }
    }
    println!("[day 06] task 1: {}", task1.unwrap());
    println!("[day 06] task 2: {}", task2.unwrap());
}
