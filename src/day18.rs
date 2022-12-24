use crate::{
    array3d::Array3D,
    vec3::{vec3, Vec3i32},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Air,
    Rock,
    Steam,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Air
    }
}

fn flood_fill_steam(grid: &mut Array3D<Cell>, pos: Vec3i32) {
    let mut cells = vec![pos];
    let mut idx = 0;

    while idx < cells.len() {
        let pos = cells[idx];
        let cell = *grid.at(pos);
        idx += 1;

        if cell != Cell::Air {
            continue;
        }
        grid.set(pos, Cell::Steam);

        let x0 = (pos.x - 1).max(0);
        let y0 = (pos.y - 1).max(0);
        let z0 = (pos.z - 1).max(0);
        let x1 = (pos.x + 2).min(grid.width() as i32);
        let y1 = (pos.y + 2).min(grid.height() as i32);
        let z1 = (pos.z + 2).min(grid.depth() as i32);

        for x in x0..x1 {
            if x != pos.x && *grid.at((x, pos.y, pos.z)) == Cell::Air {
                cells.push(vec3(x, pos.y, pos.z));
            }
        }
        for y in y0..y1 {
            if y != pos.y && *grid.at((pos.x, y, pos.z)) == Cell::Air {
                cells.push(vec3(pos.x, y, pos.z));
            }
        }
        for z in z0..z1 {
            if z != pos.z && *grid.at((pos.x, pos.y, z)) == Cell::Air {
                cells.push(vec3(pos.x, pos.y, z));
            }
        }
    }
}

pub fn solve() {
    let data = include_str!("inputs/18.txt");
    let cubes: Vec<Vec3i32> = data
        .split("\n")
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                let z = parts.next().unwrap().parse().unwrap();
                Some(vec3(x, y, z))
            }
        })
        .collect();

    let mut min = vec3(i32::MAX, i32::MAX, i32::MAX);
    let mut max = vec3(i32::MIN, i32::MIN, i32::MIN);
    for cube in &cubes {
        min.x = min.x.min(cube.x);
        min.y = min.y.min(cube.y);
        min.z = min.z.min(cube.z);
        max.x = max.x.max(cube.x);
        max.y = max.y.max(cube.y);
        max.z = max.z.max(cube.z);
    }
    min -= (1, 1, 1);
    max += (2, 2, 2);

    let sz = max - min;
    let mut grid = Array3D::new(sz.x as usize, sz.y as usize, sz.z as usize);

    for cube in &cubes {
        grid.set(*cube - min, Cell::Rock);
    }

    let task1 = cubes
        .iter()
        .map(|cube| {
            let mut sides = 0;
            sides += if *grid.at(*cube + (-1, 0, 0) - min) == Cell::Rock {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (1, 0, 0) - min) == Cell::Rock {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, -1, 0) - min) == Cell::Rock {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, 1, 0) - min) == Cell::Rock {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, 0, -1) - min) == Cell::Rock {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, 0, 1) - min) == Cell::Rock {
                0
            } else {
                1
            };
            sides
        })
        .sum::<i32>();
    println!("[day 18] task 1: {}", task1);

    flood_fill_steam(&mut grid, vec3(0, 0, 0));

    let task2 = cubes
        .iter()
        .map(|cube| {
            let mut sides = 0;
            sides += if *grid.at(*cube + (-1, 0, 0) - min) != Cell::Steam {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (1, 0, 0) - min) != Cell::Steam {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, -1, 0) - min) != Cell::Steam {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, 1, 0) - min) != Cell::Steam {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, 0, -1) - min) != Cell::Steam {
                0
            } else {
                1
            };
            sides += if *grid.at(*cube + (0, 0, 1) - min) != Cell::Steam {
                0
            } else {
                1
            };
            sides
        })
        .sum::<i32>();
    println!("[day 18] task 2: {}", task2);
}
