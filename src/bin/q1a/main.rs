fn main() {
    let max: usize = include_str!("input.txt")
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    println!("{}", max);
}
