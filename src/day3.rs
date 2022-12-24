fn priority(c: char) -> i32 {
    let n = c as i32;
    if n >= 'a' as i32 && n <= 'z' as i32 {
        n - 'a' as i32 + 1
    } else if n >= 'A' as i32 && n <= 'Z' as i32 {
        n - 'A' as i32 + 27
    } else {
        panic!("nay");
    }
}

pub fn solve() {
    let lines: Vec<&str> = include_str!("inputs/3.txt").split('\n').collect();

    let mut task1 = 0;
    let mut task2 = 0;

    for line in &lines {
        let sz = line.len() / 2;
        let comp1 = &line[0..sz];
        let comp2 = &line[sz..2 * sz];
        for c in comp1.chars() {
            if comp2.contains(c) {
                task1 += priority(c);
                break;
            }
        }
    }

    for i in 0..lines.len() / 3 {
        let a = lines[3 * i];
        let b = lines[3 * i + 1];
        let c = lines[3 * i + 2];
        for i in 0..52 {
            let ch = if i < 26 {
                (i + 'a' as u8) as char
            } else {
                (i + 'A' as u8 - 26) as char
            };
            if a.contains(ch) && b.contains(ch) && c.contains(ch) {
                task2 += (i + 1) as i32;
            }
        }
    }

    println!("[day 03] task 1: {}", task1);
    println!("[day 03] task 2: {}", task2);
}
