use crate::{
    vec2::{vec2, Vec2u32},
    vec4::{vec4, Vec4u32},
};

struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: Vec2u32,
    geode: Vec2u32,
    max_robots: Vec4u32,
}

#[derive(Hash)]
struct State {
    minutes_left: u32,
    resources: Vec4u32,
    robots: Vec4u32,
}

impl State {
    fn initial(minutes_left: u32) -> Self {
        Self {
            minutes_left,
            resources: vec4(0, 0, 0, 0),
            robots: vec4(1, 0, 0, 0),
        }
    }
    fn next(&self) -> Self {
        Self {
            minutes_left: self.minutes_left - 1,
            resources: self.resources + self.robots,
            robots: self.robots,
        }
    }
    fn next_with_delta(&self, res: Vec4u32, robots: Vec4u32) -> Self {
        Self {
            minutes_left: self.minutes_left - 1,
            resources: self.resources + self.robots - res,
            robots: self.robots + robots,
        }
    }
    fn next_ore(&self, ore: u32) -> Self {
        self.next_with_delta(vec4(ore, 0, 0, 0), vec4(1, 0, 0, 0))
    }
    fn next_clay(&self, clay: u32) -> Self {
        self.next_with_delta(vec4(clay, 0, 0, 0), vec4(0, 1, 0, 0))
    }
    fn next_obsidian(&self, obsidian: Vec2u32) -> Self {
        self.next_with_delta(vec4(obsidian.x, obsidian.y, 0, 0), vec4(0, 0, 1, 0))
    }
    fn next_geode(&self, geode: Vec2u32) -> Self {
        self.next_with_delta(vec4(geode.x, 0, geode.y, 0), vec4(0, 0, 0, 1))
    }
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        let line: String = line
            .trim()
            .chars()
            .filter_map(|c| if c != ':' { Some(c) } else { None })
            .collect();
        let parts: Vec<&str> = line.split(" ").collect();
        let id = parts[1].parse().unwrap();
        let ore: u32 = parts[6].parse().unwrap();
        let clay = parts[12].parse().unwrap();
        let obsidian1 = parts[18].parse().unwrap();
        let obsidian2 = parts[21].parse().unwrap();
        let geode1 = parts[27].parse().unwrap();
        let geode2 = parts[30].parse().unwrap();
        let max_ore = ore.max(clay).max(obsidian1).max(geode1);
        let max_clay = obsidian2;
        let max_obsidian = geode2;
        let max_robots = vec4(max_ore, max_clay, max_obsidian, u32::MAX);
        Self {
            id,
            ore,
            clay,
            obsidian: vec2(obsidian1, obsidian2),
            geode: vec2(geode1, geode2),
            max_robots,
        }
    }

    fn find_best_result(&self, state: &State, curr_best: u32) -> u32 {
        // reached end?
        if state.minutes_left == 1 {
            return state.resources.w + state.robots.w;
        }

        // can't possibly beat top-score?
        let possible_best_additional = state.minutes_left * (state.minutes_left - 1) / 2;
        let possible_best =
            state.resources.w + state.minutes_left * state.robots.w + possible_best_additional;
        if possible_best <= curr_best {
            return state.resources.w;
        }

        // always build geode, if possible
        let has_geode = state.resources.xz().ge(self.geode).all();
        if has_geode {
            return self.find_best_result(&state.next_geode(self.geode), curr_best);
        }

        // build one of these, or none
        let has_ore = state.resources.x >= self.ore && state.robots.x < self.max_robots.x;
        let has_clay = state.resources.x >= self.clay && state.robots.y < self.max_robots.y;
        let has_obsidian =
            state.resources.xy().ge(self.obsidian).all() && state.robots.z < self.max_robots.z;

        let mut this_best = self.find_best_result(&state.next(), curr_best);
        let mut curr_best = curr_best.max(this_best);
        if has_ore {
            this_best = this_best.max(self.find_best_result(&state.next_ore(self.ore), curr_best));
            curr_best = curr_best.max(this_best);
        }
        if has_clay {
            this_best =
                this_best.max(self.find_best_result(&state.next_clay(self.clay), curr_best));
            curr_best = curr_best.max(this_best);
        }
        if has_obsidian {
            this_best = this_best
                .max(self.find_best_result(&state.next_obsidian(self.obsidian), curr_best));
            curr_best = curr_best.max(this_best);
        }

        this_best
    }
}

pub fn solve() {
    let input = String::from(include_str!("inputs/19.txt"));
    let blueprints: Vec<Blueprint> = input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .map(|l| Blueprint::parse(l))
        .collect();

    let task1: u32 = blueprints
        .iter()
        .map(|blueprint| {
            let best = blueprint.find_best_result(&State::initial(24), 0);
            blueprint.id * best
        })
        .sum();
    println!("[day 19] task 1: {}", task1);

    let task2: u32 = blueprints[0..3]
        .iter()
        .map(|blueprint| blueprint.find_best_result(&State::initial(32), 0))
        .product();
    println!("[day 19] task 2: {}", task2);
}
