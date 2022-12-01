fn top_three<I>(iter: I) -> [usize; 3]
where
    I: Iterator<Item = usize>,
{
    let mut result: [usize; 3] = [0; 3];

    for v in iter {
        if v > result[0] {
            result[2] = result[1];
            result[1] = result[0];
            result[0] = v;
        } else if v > result[1] {
            result[2] = result[1];
            result[1] = v;
        } else if v > result[2] {
            result[2] = v;
        }
    }

    result
}

fn main() {
    let iter = include_str!("input.txt").split("\n\n").map(|block| {
        block
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .sum::<usize>()
    });

    let sum: usize = top_three(iter).iter().sum();

    println!("{}", sum);
}
