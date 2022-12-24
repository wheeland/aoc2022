pub fn solve() {
    let mut elves = Vec::new();

    let input = String::from(include_str!("inputs/1.txt"));
    for line in input.split('\n') {
        if line.is_empty() {
            elves.push(Vec::new());
        } else if let Some(num) = line.parse::<i32>().ok() {
            if let Some(elf) = elves.last_mut() {
                elf.push(num);
            }
        }
    }

    let mut calories: Vec<i32> = elves.iter().map(|food| food.iter().sum()).collect();
    calories.sort();

    let top1 = calories.iter().max().unwrap();
    let top3: i32 = calories.iter().rev().take(3).sum();

    println!("[day 01] task 1: {}", top1);
    println!("[day 01] task 2: {}", top3);
}
