struct State {
    crates: Vec<Vec<u8>>,
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_state(string: &str) -> State {
    let lines: Vec<_> = string.lines().collect();
    let num_crates = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .filter(|x| !x.is_empty())
        .count();

    let mut crates: Vec<Vec<u8>> = vec![vec![]; num_crates];
    for line in lines[..lines.len() - 1].into_iter().rev() {
        let bs = line.as_bytes();
        for i in 0..num_crates {
            let b = bs[4 * i + 1];
            if b != b' ' {
                crates[i].push(b);
            }
        }
    }

    State { crates }
}

fn parse_moves(string: &str) -> Vec<Move> {
    string
        .lines()
        .map(|line| {
            // move \d+ from \d+ to \d+
            let idx: Vec<_> = line.match_indices(' ').collect();

            let count_idx = (idx[0].0 + 1, idx[1].0);
            let from_idx = (idx[2].0 + 1, idx[3].0);
            let to_idx = (idx[4].0 + 1, line.len());

            Move {
                count: line[count_idx.0..count_idx.1].parse().unwrap(),
                from: line[from_idx.0..from_idx.1].parse().unwrap(),
                to: line[to_idx.0..to_idx.1].parse().unwrap(),
            }
        })
        .collect()
}

fn apply_moves(state: &mut State, moves: &[Move]) {
    for m in moves {
        for _ in 0..m.count {
            let v = state.crates[m.from - 1].pop().unwrap();
            state.crates[m.to - 1].push(v);
        }
    }
}

fn top_crates(state: &State) -> String {
    let bs: Vec<u8> = state
        .crates
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect();
    String::from_utf8(bs).unwrap()
}

fn main() {
    let chunks: Vec<_> = include_str!("input.txt").split("\n\n").collect();

    let mut state = parse_state(chunks[0]);
    let moves = parse_moves(chunks[1]);

    apply_moves(&mut state, &moves);

    println!("{}", top_crates(&state));
}
