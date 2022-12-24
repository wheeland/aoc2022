pub fn solve() {
    let data = include_str!("inputs/10.txt");
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.trim().is_empty()).collect();

    let mut values = vec![1];
    let mut x = 1;

    for line in lines {
        if line == "noop" {
            values.push(x);
        } else {
            let mut parts = line.split(' ');
            let cmd = parts.next().unwrap();
            assert!(cmd == "addx");
            let arg = parts.next().unwrap().parse::<i32>().unwrap();
            values.push(x);
            values.push(x);
            x += arg;
        }
    }

    let mut task1 = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        task1 += i as i32 * values[i];
    }

    println!("[day 10] task 1: {}", task1);

    for y in 0..6 {
        let mut line = String::new();
        for x in 0..40 {
            let v = values[y * 40 + x + 1];
            let visible = (v - x as i32).abs() < 2;
            line += if visible { "##" } else { ".." };
        }
        println!("[day 10] task 2: {}", line);
    }
}
