fn byte_to_priority(b: u8) -> usize {
    if b'a' <= b && b <= b'z' {
        (b - b'a' + 1).into()
    } else {
        (b - b'A' + 27).into()
    }
}

fn find_duplicate(bs: &[u8]) -> usize {
    let n = bs.len();
    let mut seen: [bool; 53] = [false; 53];

    for &b in &bs[..n/2] {
        let priority = byte_to_priority(b);
        seen[priority] = true;
    }

    for &b in &bs[n/2..] {
        let priority = byte_to_priority(b);
        if seen[priority] {
            return priority;
        }
    }

    unreachable!("not found")
}

fn main() {
    let max: usize = include_str!("input.txt")
        .lines()
        .map(|line| find_duplicate(line.as_bytes()))
        .sum();
    println!("{}", max);
}
