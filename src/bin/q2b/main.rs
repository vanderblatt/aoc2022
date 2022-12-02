#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Move {
    fn from_byte(b: u8) -> Move {
        match b {
            b'A' => Move::Rock,
            b'B' => Move::Paper,
            b'C' => Move::Scissors,
            _ => panic!("not a move"),
        }
    }
}

impl Outcome {
    fn from_byte(b: u8) -> Outcome {
        match b {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!("not an outcome"),
        }
    }
}

fn losing_move(m: Move) -> Move {
    match m {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn beating_move(m: Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn score_move(m: Move) -> usize {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn score_outcome(outcome: Outcome) -> usize {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn score_total(line: &str) -> usize {
    let bytes = line.as_bytes();

    let theirs = Move::from_byte(bytes[0]);
    let outcome = Outcome::from_byte(bytes[2]);

    let ours = match outcome {
        Outcome::Loss => losing_move(theirs),
        Outcome::Draw => theirs,
        Outcome::Win => beating_move(theirs),
    };

    score_outcome(outcome) + score_move(ours)
}

fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| score_total(line))
        .sum();
    println!("{}", sum);
}
