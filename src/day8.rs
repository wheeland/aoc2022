use crate::array2d::Array2D;

pub fn solve() {
    let data = include_str!("inputs/8.txt");
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.trim().is_empty()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut heights = Array2D::new(width, height);
    let mut visible = Array2D::new(width, height);
    for j in 0..height {
        for i in 0..width {
            heights.set((i, j), lines[j][i..i + 1].parse::<i32>().unwrap());
            visible.set((i, j), false);
        }
    }

    let mut check = |x, y, min_height: &mut i32| {
        let h = *heights.at((x, y));
        if h > *min_height {
            visible.set((x, y), true);
            *min_height = h;
        }
    };

    for x in 0..width {
        let mut h = -1;
        for y in 0..height {
            check(x, y, &mut h)
        }
        let mut h = -1;
        for y in (0..height).rev() {
            check(x, y, &mut h)
        }
    }

    for y in 0..height {
        let mut h = -1;
        for x in 0..width {
            check(x, y, &mut h)
        }
        let mut h = -1;
        for x in (0..width).rev() {
            check(x, y, &mut h)
        }
    }

    let task1 = visible.iter().filter(|v| **v).count();
    let mut task2 = 0;

    for y0 in 0..height {
        for x0 in 0..width {
            let h = heights.at((x0, y0));
            let mut l = x0 - 1.min(x0);
            let mut r = (x0 + 1).min(width - 1);
            let mut u = y0 - 1.min(y0);
            let mut d = (y0 + 1).min(height - 1);
            while l > 0 && heights.at((l, y0)) < h {
                l -= 1;
            }
            while r < width - 1 && heights.at((r, y0)) < h {
                r += 1;
            }
            while u > 0 && heights.at((x0, u)) < h {
                u -= 1;
            }
            while d < height - 1 && heights.at((x0, d)) < h {
                d += 1;
            }
            let l = x0 - l;
            let r = r - x0;
            let u = y0 - u;
            let d = d - y0;
            task2 = task2.max(l * r * u * d);
        }
    }

    println!("[day 08] task 1: {}", task1);
    println!("[day 08] task 1: {}", task2);
}
