#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_byte(b: u8) -> Move {
        match b {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("not a move"),
        }
    }

    fn beats(self, other: Move) -> bool {
        match (self, other) {
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            (Move::Rock, Move::Scissors) => true,
            _ => false,
        }
    }
}

fn score_move(m: Move) -> usize {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn score_match(ours: Move, theirs: Move) -> usize {
    if ours.beats(theirs) {
        6
    } else if theirs.beats(ours) {
        0
    } else {
        3
    }
}

fn score_total(line: &str) -> usize {
    let bytes = line.as_bytes();
    let theirs = Move::from_byte(bytes[0] - b'A');
    let ours = Move::from_byte(bytes[2] - b'X');
    score_match(ours, theirs) + score_move(ours)
}

fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| score_total(line))
        .sum();
    println!("{}", sum);
}
