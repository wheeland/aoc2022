use std::{collections::HashMap, hash::Hash};

struct InputValve {
    id: String,
    rate: i32,
    out: Vec<String>,
}

fn get_connections(
    valves: &HashMap<String, InputValve>,
    curr: &str,
    curr_path: i32,
    connections: &mut HashMap<String, i32>,
) {
    if let Some(existing) = connections.get(curr) {
        if *existing <= curr_path {
            return;
        }
    }
    connections.insert(curr.to_string(), curr_path);

    let valve = valves.get(curr).unwrap();
    for out in &valve.out {
        get_connections(valves, out, curr_path + 1, connections);
    }
}

struct MergedValve {
    id: String,
    rate: i32,
    out: Vec<(i32, usize)>,
}

fn find_best_path(
    valves: &[MergedValve],
    curr_valve: usize,
    visited_mask: usize,
    time_left: i32,
    curr_released: i32,
) -> i32 {
    let mut max_released = curr_released;

    for out in &valves[curr_valve].out {
        if time_left < out.0 {
            continue;
        }

        if (visited_mask & (1 << out.1)) != 0 {
            continue;
        }

        let next_valve = &valves[out.1];
        let visited_mask = visited_mask | (1 << out.1);
        let time_left = time_left - out.0;
        let curr_released = curr_released + next_valve.rate * time_left;
        let released = find_best_path(valves, out.1, visited_mask, time_left, curr_released);
        max_released = max_released.max(released);
    }

    max_released
}

#[derive(Clone)]
struct Actor {
    target_valve: usize,
    steps_remaining: i32,
}

fn find_best_path2(
    valves: &[MergedValve],
    mut actors: [Actor; 2],
    curr_actor: usize,
    visited_mask: usize,
    time_left: i32,
    curr_released: i32,
) -> i32 {
    let mut max_released = curr_released;

    if time_left <= 0 {
        return max_released;
    }

    if curr_actor == 0 {
        if actors[0].steps_remaining <= 0 && actors[1].steps_remaining <= 0 {
            return max_released;
        }

        actors[0].steps_remaining -= 1;
        actors[1].steps_remaining -= 1;
    }

    let next_actor = 1 - curr_actor;
    let next_time_left = if curr_actor == 0 {
        time_left
    } else {
        time_left - 1
    };

    // arrived at dest? -> pick next target
    let mut checked_sub_paths = false;
    if actors[curr_actor].steps_remaining == 0 {
        let valve = &valves[actors[curr_actor].target_valve];
        let curr_released = curr_released + valve.rate * time_left;

        for out in &valve.out {
            if next_time_left < out.0 {
                continue;
            }

            if (visited_mask & (1 << out.1)) != 0 {
                continue;
            }

            checked_sub_paths = true;

            let mut actors = actors.clone();
            actors[curr_actor].target_valve = out.1;
            actors[curr_actor].steps_remaining = out.0;

            let visited_mask = visited_mask | (1 << out.1);
            let released = find_best_path2(
                valves,
                actors,
                next_actor,
                visited_mask,
                next_time_left,
                curr_released,
            );
            max_released = max_released.max(released);
        }

        if !checked_sub_paths {
            let released = find_best_path2(
                valves,
                actors,
                next_actor,
                visited_mask,
                next_time_left,
                curr_released,
            );
            max_released = max_released.max(released);
        }
    } else {
        let released = find_best_path2(
            valves,
            actors,
            next_actor,
            visited_mask,
            next_time_left,
            curr_released,
        );
        max_released = max_released.max(released);
    }

    max_released
}

pub fn solve() {
    let data = include_str!("inputs/16.txt");

    // 1. read input
    let mut input_valves = HashMap::new();
    for line in data.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let id = line[6..8].to_string();

        let semicolon = line.chars().position(|c| c == ';').unwrap();
        let rate = line[23..semicolon].parse().unwrap();

        let valve = line.find("valve").unwrap();
        let valves = line[valve + 6..].trim();
        let out = valves.split(", ").map(|s| s.to_string()).collect();

        input_valves.insert(id.clone(), InputValve { id, rate, out });
    }

    // 2. eliminate null valves
    let mut merged_index = HashMap::new();
    let mut merged_valves = Vec::new();
    for input in &input_valves {
        if input.0 == "AA" || input.1.rate > 0 {
            merged_index.insert(input.0, merged_valves.len());
            merged_valves.push(MergedValve {
                id: input.0.clone(),
                rate: input.1.rate,
                out: Vec::new(),
            })
        }
    }

    // 3. find closest paths between any valves
    for valve in &mut merged_valves {
        let mut connections = HashMap::new();
        get_connections(&input_valves, &valve.id, 0, &mut connections);

        for conn in &connections {
            if valve.id != *conn.0 && input_valves[conn.0].rate > 0 {
                if let Some(idx) = merged_index.get(conn.0) {
                    valve.out.push((*conn.1 + 1, *idx));
                }
            }
        }
    }

    let start_index = *merged_index.get(&String::from("AA")).unwrap();
    let start_mask = 1 << start_index;
    let task1 = find_best_path(&merged_valves, start_index, start_mask, 30, 0);
    println!("[day 16] task 1: {}", task1);

    // let start = Actor { target_valve: start_index, steps_remaining: 1 };
    // let actors = [start.clone(), start.clone()];
    // let task2 = find_best_path2(&merged_valves, actors, 0, start_mask, 26, 0);
    // println!("[day 16] task 2: {}", task2);
}
