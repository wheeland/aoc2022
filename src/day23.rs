use crate::vec2::{vec2, Vec2i32};
use std::collections::{HashMap, HashSet};

struct Field {
    elves: HashSet<Vec2i32>,
}

impl Field {
    fn parse(input: &str) -> Self {
        let mut elves = HashSet::new();
        for line in input.split("\n").enumerate() {
            for c in line.1.trim().chars().enumerate() {
                match c.1 {
                    '#' => {
                        elves.insert(vec2(c.0 as i32, line.0 as i32));
                    }
                    '.' => {}
                    _ => panic!("nay"),
                }
            }
        }
        Self { elves }
    }

    fn round(&mut self, num: i32) -> bool {
        let N = vec2(0, -1);
        let S = vec2(0, 1);
        let W = vec2(-1, 0);
        let E = vec2(1, 0);
        let dirs_main = [N, S, W, E];
        let dirs_secondary = [(W, E), (W, E), (N, S), (N, S)];
        let dirs_all = [N, S, W, E, N + W, N + E, S + W, S + E];

        let mut proposals = HashMap::new();
        for elf in &self.elves {
            proposals.insert(*elf, -1);

            let empty = dirs_all
                .iter()
                .all(|dir| !self.elves.contains(&(*elf + *dir)));
            if empty {
                continue;
            }

            for check in 0..4 {
                let dir = ((check + num) % 4) as usize;
                let check = [
                    *elf + dirs_main[dir],
                    *elf + dirs_main[dir] + dirs_secondary[dir].0,
                    *elf + dirs_main[dir] + dirs_secondary[dir].1,
                ];
                if !self.elves.contains(&check[0])
                    && !self.elves.contains(&check[1])
                    && !self.elves.contains(&check[2])
                {
                    *proposals.entry(check[0]).or_insert(0xFF) += 1;
                    assert!(proposals.contains_key(elf));
                    proposals.insert(*elf, dir as i32);
                    break;
                }
            }
        }

        let mut moved = false;
        let mut elves = HashSet::new();
        for elf in &self.elves {
            let proposal = *proposals.get(elf).unwrap();
            let mut new_pos = *elf;
            if proposal >= 0 {
                let check_pos = *elf + dirs_main[proposal as usize];
                if *proposals.get(&check_pos).unwrap() == 0x100 {
                    new_pos = check_pos;
                    moved = true;
                }
            }
            elves.insert(new_pos);
        }

        self.elves = elves;
        moved
    }

    fn print(&self, title: &str) {
        println!("{}", title);
        for y in 0..12 {
            let mut s = String::new();
            for x in 0..14 {
                if self.elves.contains(&vec2(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            println!("{}", s);
        }
        println!("");
    }

    fn empty_ground_tiles(&self) -> i32 {
        let mut min = vec2(i32::MAX, i32::MAX);
        let mut max = vec2(i32::MIN, i32::MIN);
        for elf in &self.elves {
            min = min.min(*elf);
            max = max.max(*elf);
        }
        let size = max - min + vec2(1, 1);
        size.x * size.y - self.elves.len() as i32
    }
}

pub fn solve() {
    let input = include_str!("inputs/23.txt");

    let mut field = Field::parse(input);
    for round in 0..10 {
        field.round(round);
    }
    let task1 = field.empty_ground_tiles();
    println!("[day 23] task 1: {}", task1);

    // let mut field = Field::parse(input);
    // let mut round = 0;
    // while field.round(round) {
    //     round += 1;
    // }
    // println!("[day 23] task 2: {}", round + 1);
}
