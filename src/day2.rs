fn get_score(a: i32, b: i32) -> i32 {
    let diff = b - a;
    let score = match diff {
        -2 => 6,
        -1 => 0,
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("nay"),
    };
    score + (b + 1)
}

fn get_score_1(a: char, b: char) -> i32 {
    let a = a as i32 - 'A' as i32;
    let b = b as i32 - 'X' as i32;
    get_score(a, b)
}

fn get_score_2(a: char, b: char) -> i32 {
    let a = a as i32 - 'A' as i32;
    let outcome = b as i32 - 'X' as i32;
    let b = match outcome {
        0 => (a + 2) % 3,
        1 => a,
        2 => (a + 1) % 3,
        _ => panic!("nay"),
    };
    get_score(a, b)
}

pub fn solve() {
    let lines = include_str!("inputs/2.txt").split('\n');

    let mut score1 = 0;
    let mut score2 = 0;
    for line in lines {
        if !line.is_empty() {
            let mut parts = line.split(' ');
            let a = parts.next().unwrap().chars().next().unwrap();
            let b = parts.next().unwrap().chars().next().unwrap();
            score1 += get_score_1(a, b);
            score2 += get_score_2(a, b);
        }
    }

    println!("[day 02] task 1: {}", score1);
    println!("[day 02] task 2: {}", score2);
}
