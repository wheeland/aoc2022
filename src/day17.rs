use std::collections::HashMap;

use crate::{
    array2d::Array2D,
    vec2::{vec2, Vec2i32},
};

struct Rock {
    coords: Vec<Vec2i32>,
    grid: Array2D<bool>,
    w: i32,
    h: i32,
}

impl Rock {
    fn from(coords: &[(i32, i32)]) -> Self {
        let coords: Vec<Vec2i32> = coords.iter().map(|c| Vec2i32::new(c.0, c.1)).collect();
        let w = 1 + coords.iter().map(|c| c.x).max().unwrap();
        let h = 1 + coords.iter().map(|c| c.y).max().unwrap();
        let mut grid = Array2D::new(w as usize, h as usize);
        grid.fill(false);

        for coord in &coords {
            grid.set(*coord, true);
        }

        Self { coords, grid, w, h }
    }
}

fn can_move(board: &Array2D<bool>, rock: &Rock, pos: Vec2i32) -> bool {
    if pos.x < 0 || pos.x + rock.w > 7 {
        false
    } else {
        assert!(pos.y >= 0);
        !rock.coords.iter().any(|coord| *board.at(pos + *coord))
    }
}

fn print_board(board: &Array2D<bool>, max_y: i32, rock: &Rock, pos: Vec2i32) {
    for y in (0..max_y).rev() {
        let mut s = String::new();
        for x in 0..7 {
            s += if *board.at((x, y)) {
                "#"
            } else if x >= pos.x
                && y >= pos.y
                && x - pos.x < rock.w
                && y - pos.y < rock.h
                && *rock.grid.at(vec2(x, y) - pos)
            {
                "@"
            } else {
                "."
            };
        }
        println!("|{}|", s);
    }
    println!("+-------+");
    println!("");
}

fn board_height(board: &Array2D<bool>) -> usize {
    let mut height = 0;
    for y in 0..board.height() {
        if board.slice(y).iter().any(|b| *b) {
            height = y;
        } else {
            break;
        }
    }
    return height + 1;
}

fn board_simulate(
    board: &mut Array2D<bool>,
    rocks: &[Rock],
    jets: &[i32],
    start_step: usize,
    rock_count: usize,
) -> usize {
    let mut spawn = board_height(board) as i32 + 3;
    let mut step = start_step;

    for rock in 0..rock_count {
        let rock = &rocks[rock % 5];
        let mut pos = vec2(2, spawn);

        loop {
            let jet = jets[step % jets.len()];
            step += 1;

            if can_move(&board, rock, pos + (jet, 0)) {
                pos += (jet, 0);
            }

            if can_move(&board, rock, pos + (0, -1)) {
                pos += (0, -1);
            } else {
                for c in &rock.coords {
                    board.set(pos + *c, true);
                }
                spawn = spawn.max(pos.y + rock.h + 3);
                break;
            }
        }
    }

    step % jets.len()
}

#[derive(Clone)]
struct BoardLayout {
    step: usize,
    top_rows: Array2D<bool>,
    hash: String,
}

impl BoardLayout {
    fn new(board: &Array2D<bool>, step: usize) -> Self {
        let height = board_height(board);
        let mut top_rows = Array2D::new(7, height);
        for y in 0..height {
            let slice = board.slice(y);
            top_rows.slice_mut(y).clone_from_slice(slice);
        }
        let mut hash: String = top_rows
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect();
        hash += &format!("{}", step);

        Self {
            top_rows,
            hash,
            step,
        }
    }
}

#[derive(Clone)]
struct BoardDropping {
    start_layout: BoardLayout,
    end_layout: BoardLayout,
    height_diff: usize,
    loop_height_diff: Option<usize>,
    loop_blocks: Option<usize>,
}

impl BoardDropping {
    fn new(start_layout: BoardLayout, rocks: &[Rock], jets: &[i32], rock_count: usize) -> Self {
        let mut board = Array2D::new(7, rock_count * 20 + start_layout.top_rows.height());
        board.fill(false);

        // fill board
        for y in 0..start_layout.top_rows.height() {
            board
                .slice_mut(y)
                .clone_from_slice(start_layout.top_rows.slice(y));
        }

        let old_height = board_height(&board);
        let new_step = board_simulate(&mut board, rocks, jets, start_layout.step, rock_count);
        let new_height = board_height(&board);

        let height_diff = new_height - old_height;

        // chop of bottom parts of the board
        let max_height = new_height.min(50);
        let mut new_board = Array2D::new(7, max_height);
        new_board.fill(false);
        for y in 0..max_height {
            new_board
                .slice_mut(y)
                .clone_from_slice(board.slice(new_height - max_height + y));
        }

        let end_layout = BoardLayout::new(&new_board, new_step);

        Self {
            start_layout,
            end_layout,
            height_diff,
            loop_height_diff: None,
            loop_blocks: None,
        }
    }
}

pub fn solve() {
    let data = include_str!("inputs/17.txt");
    let jets: Vec<i32> = data
        .chars()
        .filter_map(|c| match c {
            '<' => Some(-1),
            '>' => Some(1),
            _ => None,
        })
        .collect();

    let rocks = vec![
        Rock::from(&[(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::from(&[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Rock::from(&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::from(&[(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::from(&[(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];

    {
        let mut board = Array2D::new(7, 16000);
        board.fill(false);
        board.slice_mut(0).clone_from_slice(&[true; 7]);
        board_simulate(&mut board, &rocks, &jets, 0, 2022);
        println!("[day 17] task 1: {}", board_height(&board) - 1);
    }

    {
        let mut initial_board = Array2D::new(7, 1);
        initial_board.fill(true);
        let initial_layout = BoardLayout::new(&initial_board, 0);

        let mut droppings = HashMap::new();
        let mut initial_heights = HashMap::new();
        let mut initial_blocks = HashMap::new();
        let mut current_height = 0;
        let mut current_blocks = 0;
        let mut layout = initial_layout.clone();
        loop {
            let dropping = BoardDropping::new(layout.clone(), &rocks, &jets, rocks.len());
            current_height += dropping.height_diff;
            current_blocks += rocks.len();
            if droppings.contains_key(&layout.hash) {
                break;
            }
            initial_heights.insert(layout.hash.clone(), current_height);
            initial_blocks.insert(layout.hash.clone(), current_blocks);
            droppings.insert(layout.hash.clone(), dropping.clone());
            layout = dropping.end_layout;
        }
        let loop_dropping = droppings.get_mut(&layout.hash).unwrap();
        loop_dropping.loop_height_diff =
            Some(current_height - *initial_heights.get(&layout.hash).unwrap());
        loop_dropping.loop_blocks =
            Some(current_blocks - *initial_blocks.get(&layout.hash).unwrap());

        let mut rocks_left = 1000000000000;
        let mut height = 0;
        let mut curr_layout = initial_layout.clone();

        while rocks_left > rocks.len() {
            let dropping = droppings.get(&curr_layout.hash).unwrap();

            let loop_diff = dropping.loop_height_diff.unwrap_or(usize::MAX);
            if rocks_left < loop_diff {
                rocks_left -= rocks.len();
                height += dropping.height_diff;
                curr_layout = dropping.end_layout.clone();
                continue;
            }

            let iterations = rocks_left / dropping.loop_blocks.unwrap();
            rocks_left -= iterations * dropping.loop_blocks.unwrap();
            height += iterations * loop_diff;
        }

        let last_dropping = BoardDropping::new(curr_layout, &rocks, &jets, rocks_left);
        height += last_dropping.height_diff;
        println!("[day 17] task 2: {}", height);
    }
}
