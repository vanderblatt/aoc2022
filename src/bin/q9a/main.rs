use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct State {
    visited: HashSet<Point>,
    head: Point,
    tail: Point,
}

struct Move {
    dir: Dir,
    dist: u32,
}

impl State {
    fn new() -> Self {
        let origin = Point { x: 0, y: 0 };
        Self {
            visited: HashSet::from([origin]),
            head: origin,
            tail: origin,
        }
    }
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<_> = line.split(' ').collect();
    let dist: u32 = parts[1].parse().unwrap();

    match parts[0] {
        "U" => Move { dir: Dir::Up, dist },
        "D" => Move {
            dir: Dir::Down,
            dist,
        },
        "L" => Move {
            dir: Dir::Left,
            dist,
        },
        "R" => Move {
            dir: Dir::Right,
            dist,
        },
        _ => panic!("bad command"),
    }
}

fn signum(v: i32) -> i32 {
    if v > 0 {
        1
    } else if v < 0 {
        -1
    } else {
        0
    }
}

fn update_tail(head: Point, tail: Point) -> Point {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        tail
    } else {
        Point {
            x: tail.x + signum(dx),
            y: tail.y + signum(dy),
        }
    }
}

fn apply_move(state: &mut State, m: Move) {
    for _ in 0..m.dist {
        match m.dir {
            Dir::Down => state.head.y -= 1,
            Dir::Up => state.head.y += 1,
            Dir::Left => state.head.x -= 1,
            Dir::Right => state.head.x += 1,
        }
        state.tail = update_tail(state.head, state.tail);
        state.visited.insert(state.tail);
    }
}

fn main() {
    let mut state = State::new();

    include_str!("input.txt")
        .lines()
        .for_each(|line| apply_move(&mut state, parse_move(line)));

    println!("{}", state.visited.len());
}
