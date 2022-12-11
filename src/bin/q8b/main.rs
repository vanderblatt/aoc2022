use std::cmp;

struct Vec2d<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<'a, T: Copy> Vec2d<T> {
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn row_iter(&self, start: (usize, usize)) -> impl Iterator<Item = &T> {
        let idx1 = self.idx(start.0, start.1);
        let idx2 = self.idx(start.0, self.cols);
        self.data[idx1..idx2].iter()
    }

    fn row_rev_iter(&self, start: (usize, usize)) -> impl Iterator<Item = &T> {
        let idx1 = self.idx(start.0, 0);
        let idx2 = self.idx(start.0, start.1);
        self.data[idx1..=idx2].iter().rev()
    }

    fn col_iter(&'a self, start: (usize, usize)) -> impl Iterator<Item = &T> {
        let idx = self.idx(start.0, start.1);
        self.data[idx..].iter().step_by(self.cols)
    }

    fn col_rev_iter(&'a self, start: (usize, usize)) -> impl Iterator<Item = &T> {
        let idx = self.idx(start.0, start.1);
        self.data[..=idx].iter().rev().step_by(self.cols)
    }
}

fn score_direction<'a>(mut iter: impl Iterator<Item = &'a u8>) -> usize {
    let mut score = 0;
    let own_height = *iter.nth(0).unwrap();

    for &h in iter {
        score += 1;
        if h >= own_height {
            break;
        }
    }

    score
}

fn scenic_score(grid: &Vec2d<u8>, start: (usize, usize)) -> usize {
    let mut score = 1;

    score *= score_direction(grid.row_iter(start));
    score *= score_direction(grid.row_rev_iter(start));
    score *= score_direction(grid.col_iter(start));
    score *= score_direction(grid.col_rev_iter(start));

    score
}

fn top_scenic_score(grid: &Vec2d<u8>) -> usize {
    let mut max_score = 0;
    for i in 1..grid.rows - 1 {
        for j in 1..grid.cols - 1 {
            max_score = cmp::max(max_score, scenic_score(grid, (i, j)));
        }
    }

    max_score
}

fn main() {
    let data: Vec<&[u8]> = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    let rows = data.len();
    let cols = data[0].len();
    let data_flat: Vec<u8> = data.into_iter().flatten().copied().collect();

    let grid = Vec2d {
        data: data_flat,
        rows,
        cols,
    };
    println!("{}", top_scenic_score(&grid));
}
