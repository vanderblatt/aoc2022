fn num_visible(grid: &[&[u8]]) -> usize {
    let m: usize = grid.len();
    let n: usize = grid[0].len();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; n]; m];

    for i in 0..m {
        let mut h = 0;
        for j in 0..n {
            if grid[i][j] > h {
                h = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    for i in 0..m {
        let mut h = 0;
        for j in (0..n).rev() {
            if grid[i][j] > h {
                h = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    for j in 0..n {
        let mut h = 0;
        for i in 0..m {
            if grid[i][j] > h {
                h = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    for j in 0..n {
        let mut h = 0;
        for i in (0..m).rev() {
            if grid[i][j] > h {
                h = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    visible
        .into_iter()
        .map(|c| c.into_iter().map(|x| x as usize).sum::<usize>())
        .sum()
}

fn main() {
    let grid: Vec<&[u8]> = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    println!("{}", num_visible(&grid));
}
