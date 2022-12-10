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

    fn contains(&self, v: usize) -> bool {
        self.start <= v && self.end >= v
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}

fn process(line: &str) -> bool {
    let parts: Vec<_> = line.split(',').collect();

    let fst = Range::from_str(parts[0]);
    let snd = Range::from_str(parts[1]);

    fst.overlaps(&snd) || snd.overlaps(&fst)
}

fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| process(line) as usize)
        .sum();
    println!("{}", sum);
}
