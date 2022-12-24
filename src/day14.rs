use crate::{array2d::Array2D, vec2::Vec2i32};

pub fn solve() {
    let data = include_str!("inputs/14.txt");

    let lines: Vec<Vec<Vec2i32>> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(" -> ")
                .map(|part| {
                    let mut parts = part.split(',');
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.next().unwrap().parse().unwrap();
                    Vec2i32::new(x, y)
                })
                .collect()
        })
        .collect();

    let min_y = lines
        .iter()
        .flatten()
        .min_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y;
    let max_y = lines
        .iter()
        .flatten()
        .min_by(|a, b| b.y.cmp(&a.y))
        .unwrap()
        .y;
    let min_x = lines
        .iter()
        .flatten()
        .min_by(|a, b| a.x.cmp(&b.x))
        .unwrap()
        .x;
    let max_x = lines
        .iter()
        .flatten()
        .min_by(|a, b| b.x.cmp(&a.x))
        .unwrap()
        .x;

    let floor = max_y + 2;
    let min_x = min_x - floor;
    let max_x = max_x + floor;
    let min_y = min_y.min(0);
    let min = Vec2i32::new(min_x, min_y);

    let mut grid = Array2D::new((max_x - min_x + 3) as usize, (max_y - min_y + 5) as usize);
    grid.fill(false);

    for line in &lines {
        for i in 1..line.len() {
            let from = line[i - 1];
            let to = line[i];
            let dir = (to - from).signum();
            let mut v = from;
            loop {
                grid.set(v - min, true);
                if v == to {
                    break;
                }
                v += dir;
            }
        }
    }

    for x in min_x..max_x {
        grid.set(Vec2i32::new(x, floor) - min, true);
    }

    let mut curr = 0;
    let mut task1 = None;
    let mut task2 = None;
    while task1.is_none() || task2.is_none() {
        let mut sand = Vec2i32::new(500, 0);

        while sand.y < floor {
            let mut moved = false;
            for ofs in [(0, 1), (-1, 1), (1, 1)] {
                if !grid.at(sand - min + ofs) {
                    sand += ofs;
                    moved = true;
                    break;
                }
            }

            if !moved {
                grid.set(sand - min, true);
                break;
            }
        }

        if sand.y >= max_y && task1.is_none() {
            task1 = Some(curr);
        }

        curr += 1;

        if *grid.at(Vec2i32::new(500, 0) - min) {
            task2 = Some(curr);
            break;
        }
    }

    println!("[day 14] task 1: {}", task1.unwrap());
    println!("[day 14] task 2: {}", task2.unwrap());
}
