use crate::{
    array2d::Array2D,
    vec2::{vec2, Vec2i32},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Straight(i32),
}

fn parse_directions(s: &str) -> Vec<Direction> {
    let mut s = s.trim();
    let mut ret = Vec::new();
    while !s.is_empty() {
        let (dir, new_s) = match s.chars().next().unwrap() {
            'L' => (Direction::Left, &s[1..]),
            'R' => (Direction::Right, &s[1..]),
            _ => {
                let l = s.find('L').unwrap_or(s.len());
                let r = s.find('R').unwrap_or(s.len());
                let count = l.min(r);
                let steps = s[0..count].parse().unwrap();
                (Direction::Straight(steps), &s[count..])
            }
        };
        ret.push(dir);
        s = new_s;
    }
    ret
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Void,
    Empty,
    Wall,
}

const RIGHT: i32 = 0;
const DOWN: i32 = 1;
const LEFT: i32 = 2;
const UP: i32 = 3;

fn look_diff(look: i32) -> Vec2i32 {
    match look {
        RIGHT => vec2(1, 0),
        DOWN => vec2(0, 1),
        LEFT => vec2(-1, 0),
        UP => vec2(0, -1),
        _ => panic!("nay"),
    }
}

fn parse_map(lines: &[&str]) -> Array2D<Tile> {
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let height = lines.len();
    let mut ret = Array2D::new(width, height);
    ret.fill(Tile::Void);
    for y in 0..height {
        for (x, ch) in lines[y].chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                _ => Tile::Void,
            };
            ret.set((x, y), tile);
        }
    }
    ret
}

fn traverse1(map: &Array2D<Tile>, directions: &[Direction]) -> (Vec2i32, i32) {
    let size = vec2(map.width() as i32, map.height() as i32);
    let start_x = map.slice(0).iter().position(|t| *t == Tile::Empty).unwrap();
    let mut pos = vec2(start_x as i32, 0);
    let mut look = RIGHT;

    for dir in directions {
        match dir {
            Direction::Left => look = (look + 3) % 4,
            Direction::Right => look = (look + 1) % 4,
            Direction::Straight(steps) => {
                let diff = look_diff(look);
                for _ in 0..*steps {
                    let mut new_pos = (pos + diff + size) % size;
                    while *map.at(new_pos) == Tile::Void {
                        new_pos = (new_pos + diff + size) % size;
                    }
                    if *map.at(new_pos) == Tile::Wall {
                        break;
                    } else {
                        pos = new_pos;
                    }
                }
            }
        }
    }

    (pos, look)
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Wrap {
    source_face: Vec2i32,
    source_look: i32,
    target_face: Vec2i32,
    target_look: i32,
}

impl Wrap {
    fn new(source_face: Vec2i32, source_look: i32, target_face: Vec2i32, target_look: i32) -> Self {
        Self {
            source_face,
            source_look,
            target_face,
            target_look,
        }
    }
}

fn traverse2(
    map: &Array2D<Tile>,
    directions: &[Direction],
    faces: &[Vec2i32],
    face_size: i32,
    wraps: &[Wrap],
) -> (Vec2i32, i32) {
    let size = vec2(map.width() as i32, map.height() as i32);
    let start_x = map.slice(0).iter().position(|t| *t == Tile::Empty).unwrap();
    let mut pos = vec2(start_x as i32, 0);
    let mut look = RIGHT;

    for dir in directions {
        match dir {
            Direction::Left => look = (look + 3) % 4,
            Direction::Right => look = (look + 1) % 4,
            Direction::Straight(steps) => {
                for _ in 0..*steps {
                    let mut new_pos = pos + look_diff(look);
                    let mut new_look = look;

                    // wrap around?
                    if new_pos.lt(vec2(0, 0)).any()
                        || new_pos.ge(size).any()
                        || *map.at(new_pos) == Tile::Void
                    {
                        let face = pos / face_size;
                        let ofs = pos - face * face_size;
                        let edge = match new_look {
                            RIGHT => ofs.y,
                            DOWN => face_size - 1 - ofs.x,
                            LEFT => face_size - 1 - ofs.y,
                            UP => ofs.x,
                            _ => panic!("nay"),
                        };

                        let wrap = wraps
                            .iter()
                            .find(|wrap| wrap.source_face == face && wrap.source_look == new_look)
                            .unwrap();
                        let new_ofs = match wrap.target_look {
                            RIGHT => vec2(0, edge),
                            DOWN => vec2(face_size - 1 - edge, 0),
                            LEFT => vec2(face_size - 1, face_size - 1 - edge),
                            UP => vec2(edge, face_size - 1),
                            _ => panic!("nay"),
                        };
                        new_pos = wrap.target_face * face_size + new_ofs;
                        new_look = wrap.target_look;
                    }

                    if *map.at(new_pos) == Tile::Wall {
                        break;
                    } else {
                        pos = new_pos;
                        look = new_look;
                    }
                }
            }
        }
    }

    (pos, look)
}

pub fn solve() {
    let input = include_str!("inputs/22.txt");
    let lines: Vec<&str> = input.split("\n").filter(|l| !l.trim().is_empty()).collect();

    let map = parse_map(&lines[0..lines.len() - 1]);
    let directions = parse_directions(lines.last().unwrap());

    let task1 = traverse1(&map, &directions);
    let task1 = (task1.0.y + 1) * 1000 + (task1.0.x + 1) * 4 + task1.1;
    println!("[day 22] task 1: {}", task1);

    // let face_size = 4;
    // let faces = [vec2(2, 0), vec2(0, 1), vec2(1, 1), vec2(2, 1), vec2(2, 2), vec2(3, 2)];
    // let mut wraps = vec![
    //     Wrap::new(faces[0], RIGHT, faces[5], LEFT),
    //     Wrap::new(faces[0], UP, faces[1], DOWN),
    //     Wrap::new(faces[0], LEFT, faces[2], DOWN),
    //     Wrap::new(faces[1], LEFT, faces[5], UP),
    //     Wrap::new(faces[1], DOWN, faces[4], UP),
    //     Wrap::new(faces[2], DOWN, faces[4], RIGHT),
    //     Wrap::new(faces[3], RIGHT, faces[5], DOWN),
    // ];

    let face_size = 50;
    let faces = [
        vec2(1, 0),
        vec2(2, 0),
        vec2(1, 1),
        vec2(1, 2),
        vec2(0, 2),
        vec2(0, 3),
    ];
    let mut wraps = vec![
        Wrap::new(faces[0], UP, faces[5], RIGHT),
        Wrap::new(faces[0], LEFT, faces[4], RIGHT),
        Wrap::new(faces[1], UP, faces[5], UP),
        Wrap::new(faces[1], RIGHT, faces[3], LEFT),
        Wrap::new(faces[1], DOWN, faces[2], LEFT),
        Wrap::new(faces[2], LEFT, faces[4], DOWN),
        Wrap::new(faces[3], DOWN, faces[5], LEFT),
    ];

    for i in 0..wraps.len() {
        let wrap = wraps[i];
        wraps.push(Wrap::new(
            wrap.target_face.into(),
            (wrap.target_look + 2) % 4,
            wrap.source_face.into(),
            (wrap.source_look + 2) % 4,
        ));
    }

    let task2 = traverse2(&map, &directions, &faces, face_size, &wraps);
    let task2 = (task2.0.y + 1) * 1000 + (task2.0.x + 1) * 4 + task2.1;
    println!("[day 22] task 2: {}", task2);
}
