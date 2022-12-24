use crate::array2d::Array2D;

pub fn solve() {
    let data = include_str!("inputs/12.txt");
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.trim().is_empty()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut heights = Array2D::new(width, height);
    let mut path = Array2D::new(width, height);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for j in 0..height {
        for i in 0..width {
            let mut ch = lines[j].chars().nth(i).unwrap();
            if ch == 'S' {
                start = (i, j);
                ch = 'a';
            }
            if ch == 'E' {
                end = (i, j);
                ch = 'z';
            }
            heights.set((i, j), ch as usize - 'a' as usize);
            path.set((i, j), usize::MAX);
        }
    }

    path.set(end, 0);

    let mut curr = 0;
    let mut curr_path = vec![end];

    loop {
        let mut next_path = Vec::new();
        curr += 1;

        let mut try_go = |pos, from_height, next: &mut Vec<(usize, usize)>| {
            if *path.at(pos) == usize::MAX && *heights.at(pos) + 1 >= from_height {
                path.set(pos, curr);
                if !next.contains(&pos) {
                    next.push(pos);
                }
            }
        };

        for pos in &curr_path {
            let h = *heights.at(*pos);
            if pos.0 > 0 {
                try_go((pos.0 - 1, pos.1), h, &mut next_path);
            }
            if pos.0 < width - 1 {
                try_go((pos.0 + 1, pos.1), h, &mut next_path);
            }
            if pos.1 > 0 {
                try_go((pos.0, pos.1 - 1), h, &mut next_path);
            }
            if pos.1 < height - 1 {
                try_go((pos.0, pos.1 + 1), h, &mut next_path);
            }
        }

        if next_path.is_empty() {
            break;
        }
        curr_path = next_path;
    }

    let mut task2 = usize::MAX;
    for y in 0..height {
        for x in 0..width {
            if *heights.at((x, y)) == 0 {
                task2 = task2.min(*path.at((x, y)));
            }
        }
    }

    println!("[day 12] task 1: {}", path.at(start));
    println!("[day 12] task 2: {}", task2);
}
