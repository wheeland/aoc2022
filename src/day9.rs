use std::collections::HashMap;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("nay"),
        }
    }
}

pub fn solve() {
    let data = include_str!("inputs/9.txt");
    let commands: Vec<(Direction, i32)> = data
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(' ');
            let dir = Direction::from(parts.next().unwrap());
            let count = parts.next().unwrap().parse::<i32>().unwrap();
            (dir, count)
        })
        .collect();

    let mut knot = [(0i32, 0i32); 10];

    let mut visited1 = HashMap::from([((0, 0), true)]);
    let mut visited9 = HashMap::from([((0, 0), true)]);

    for command in commands {
        for _ in 0..command.1 {
            let diff = match command.0 {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
            };

            knot[0].0 += diff.0;
            knot[0].1 += diff.1;

            for i in 0..9 {
                let h = knot[i];
                let t = &mut knot[i + 1];
                let diff = (h.0 - t.0).abs().max((h.1 - t.1).abs());
                if diff > 1 {
                    if h.1 == t.1 {
                        t.0 += (h.0 - t.0) / 2;
                    } else if h.0 == t.0 {
                        t.1 += (h.1 - t.1) / 2;
                    } else {
                        assert!((h.0 - t.0) != 0);
                        assert!((h.1 - t.1) != 0);
                        t.0 += (h.0 - t.0).signum();
                        t.1 += (h.1 - t.1).signum();
                    }
                }
            }

            visited1.insert(knot[1], true);
            visited9.insert(knot[9], true);
        }
    }

    println!("[day 09] task 1: {}", visited1.len());
    println!("[day 09] task 2: {}", visited9.len());
}
