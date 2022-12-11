use std::cmp;

const MARKER_SIZE: usize = 14;

fn start_idx(bytes: &[u8]) -> usize {
    let mut seen_idx_map: [usize; 26] = [0; 26];
    let mut streak: usize = 0;

    for (idx, c) in bytes.into_iter().enumerate() {
        let seen_idx = &mut seen_idx_map[(c - b'a') as usize];

        let next_idx = (idx + 1) as usize;
        let start_idx = cmp::max(next_idx, MARKER_SIZE) - MARKER_SIZE;

        if *seen_idx > start_idx {
            streak = cmp::min(next_idx - *seen_idx, streak + 1);
        } else {
            streak += 1;
        }
        *seen_idx = next_idx;

        if streak == MARKER_SIZE {
            return next_idx;
        }
    }

    unreachable!("no marker")
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", start_idx(input.as_bytes()));
}
