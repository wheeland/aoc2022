use std::collections::HashSet;

use crate::vec2::{vec2, Vec2i32};

struct Field {
    start: Vec2i32,
    end: Vec2i32,
    width: i32,
    height: i32,
    up: Vec<Vec<Vec2i32>>,
    down: Vec<Vec<Vec2i32>>,
    left: Vec<Vec<Vec2i32>>,
    right: Vec<Vec<Vec2i32>>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: Vec2i32,
    journey: i32,
}

impl Field {
    fn can_go(&self, pos: Vec2i32, step: i32) -> bool {
        if pos == self.start || pos == self.end {
            return true;
        }

        if pos.x < 0 || pos.y < 0 || pos.x >= self.width || pos.y >= self.height {
            return false;
        }

        // check that we didn't run into a blizzard
        let check = |blizz, ofs| {
            let blizz = blizz + ofs + vec2(self.width, self.height) * step;
            let blizz = blizz % vec2(self.width, self.height);
            blizz == pos
        };

        for blizz in &self.up[pos.x as usize] {
            if check(*blizz, vec2(0, -step)) {
                return false;
            }
        }
        for blizz in &self.down[pos.x as usize] {
            if check(*blizz, vec2(0, step)) {
                return false;
            }
        }
        for blizz in &self.left[pos.y as usize] {
            if check(*blizz, vec2(-step, 0)) {
                return false;
            }
        }
        for blizz in &self.right[pos.y as usize] {
            if check(*blizz, vec2(step, 0)) {
                return false;
            }
        }

        true
    }

    fn find_possible_paths(&self, curr_pos: State, curr_step: i32, result: &mut HashSet<State>) {
        let diffs = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];
        for diff in diffs {
            if self.can_go(curr_pos.pos + diff, curr_step + 1) {
                let mut new_state = curr_pos;
                new_state.pos += diff;
                if new_state.journey % 2 == 0 && new_state.pos == self.end {
                    new_state.journey += 1;
                }
                if new_state.journey % 2 == 1 && new_state.pos == self.start {
                    new_state.journey += 1;
                }
                result.insert(new_state);
            }
        }
    }
}

pub fn solve() {
    let input = include_str!("inputs/24.txt");
    let lines: Vec<&str> = input.split("\n").filter(|l| !l.trim().is_empty()).collect();
    let width = lines[0].trim().len() as i32 - 2;
    let height = lines.len() as i32 - 2;
    let start = vec2(lines[0].chars().position(|c| c == '.').unwrap() as i32, 0);
    let end = vec2(
        lines
            .last()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32,
        height + 1,
    );

    let mut up = Vec::new();
    let mut down = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for _ in 0..width {
        up.push(Vec::new());
        down.push(Vec::new());
    }
    for _ in 0..height {
        left.push(Vec::new());
        right.push(Vec::new());
    }
    for y in lines.iter().enumerate() {
        for x in y.1.chars().enumerate() {
            let xy = vec2(x.0 as i32 - 1, y.0 as i32 - 1);
            match x.1 {
                '<' => left[y.0 - 1].push(xy),
                '>' => right[y.0 - 1].push(xy),
                '^' => up[x.0 - 1].push(xy),
                'v' => down[x.0 - 1].push(xy),
                _ => {}
            };
        }
    }
    let start = start - vec2(1, 1);
    let end = end - vec2(1, 1);
    let field = Field {
        left,
        right,
        up,
        down,
        start,
        end,
        width,
        height,
    };
    let mut task1 = None;
    let mut task2 = None;

    let mut paths = HashSet::new();
    paths.insert(State {
        pos: start,
        journey: 0,
    });
    let mut step = 0;
    while task1.is_none() || task2.is_none() {
        if task1.is_none() && paths.iter().any(|state| state.journey == 1) {
            task1 = Some(step);
        }

        if task2.is_none() && paths.iter().any(|state| state.journey == 3) {
            task2 = Some(step);
        }

        let mut new_paths = HashSet::new();
        for path in &paths {
            field.find_possible_paths(*path, step, &mut new_paths);
        }
        paths = new_paths;
        step += 1;
    }
    println!("[day 24] task 1: {:?}", task1.unwrap());
    println!("[day 24] task 2: {:?}", task2.unwrap());
}
