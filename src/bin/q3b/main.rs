fn byte_to_priority(b: u8) -> usize {
    if b'a' <= b && b <= b'z' {
        (b - b'a' + 1).into()
    } else {
        (b - b'A' + 27).into()
    }
}

fn find_common<'a, I>(chunk: I) -> usize where I: Iterator<Item=&'a str> {
    let mut count: [u8; 53] = [0; 53];
    let mut seen_round: [usize; 53] = [0; 53];

    for (i, line) in chunk.enumerate() {
        for &b in line.as_bytes() {
            let priority = byte_to_priority(b);
            if seen_round[priority] < i + 1 {
                count[priority] += 1;
                seen_round[priority] = i + 1;
            }
            if count[priority] == 3 {
                return priority;
            }
        }
    }

    unreachable!("not found")
}

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let sum: usize = lines
        .chunks(3)
        .map(|chunk| find_common(chunk.into_iter().copied()))
        .sum();
    println!("{}", sum);
}
