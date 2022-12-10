struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn from_str(string: &str) -> Self {
        let parts: Vec<_> = string.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        Range { start, end }
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn process(line: &str) -> bool {
    let parts: Vec<_> = line.split(',').collect();

    let fst = Range::from_str(parts[0]);
    let snd = Range::from_str(parts[1]);

    fst.fully_contains(&snd) || snd.fully_contains(&fst)
}

fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| process(line) as usize)
        .sum();
    println!("{}", sum);
}
