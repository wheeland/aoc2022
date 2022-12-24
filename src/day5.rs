use text_io::scan;

const STACKS: usize = 9;

pub fn solve() {
    let lines: Vec<&str> = include_str!("inputs/5.txt").split('\n').collect();

    let mut stacks1: [Vec<char>; STACKS] = Default::default();
    let mut stacks2: [Vec<char>; STACKS] = Default::default();
    let mut init_done = false;

    for line in lines {
        if line.is_empty() {
            init_done = true;
        } else {
            if !init_done {
                for x in 0..STACKS {
                    let ch = line.chars().nth(1 + 4 * x).unwrap();
                    if ch.is_uppercase() {
                        stacks1[x].insert(0, ch);
                        stacks2[x].insert(0, ch);
                    }
                }
            } else {
                let num: usize;
                let from: usize;
                let to: usize;
                scan!(line.bytes() => "move {} from {} to {}", num, from, to);
                let from = from - 1;
                let to = to - 1;

                for _ in 0..num {
                    let v = stacks1[from].pop().unwrap();
                    stacks1[to].push(v);
                }

                let start = stacks2[from].len() - num;
                let items: Vec<char> = stacks2[from].drain(start..).collect();
                stacks2[to].extend(&items);
            }
        }
    }

    let mut task1 = String::new();
    let mut task2 = String::new();
    for i in 0..STACKS {
        task1.push(*stacks1[i].last().unwrap());
        task2.push(*stacks2[i].last().unwrap());
    }
    println!("[day 05] task 1: {}", task1);
    println!("[day 05] task 2: {}", task2);
}
