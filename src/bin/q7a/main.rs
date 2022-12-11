const MAX_TOTAL_BYTES: usize = 100_000;

struct DirTree {
    dirs: Vec<DirTree>,
    total_bytes: usize,
}

impl DirTree {
    fn new() -> Self {
        DirTree {
            dirs: Vec::new(),
            total_bytes: 0,
        }
    }
}

fn recur<'a, I>(tree: &mut DirTree, mut lines: I) -> I
where
    I: Iterator<Item = &'a str>,
{
    while let Some(line) = lines.next() {
        let parts: Vec<_> = line.split(' ').collect();

        if parts[0] == "$" {
            if parts[1] == "cd" {
                match parts[2] {
                    "/" => {}
                    ".." => return lines,
                    _ => {
                        tree.dirs.push(DirTree::new());
                        let subdir = tree.dirs.last_mut().unwrap();
                        lines = recur(subdir, lines);
                        tree.total_bytes += subdir.total_bytes;
                    }
                };
            }
        } else if parts[0] != "dir" {
            let size: usize = parts[0].parse().unwrap();
            tree.total_bytes += size;
        }
    }

    lines
}

fn construct_directory_tree(input: &str) -> DirTree {
    let mut root = DirTree::new();
    let _ = recur(&mut root, input.lines());
    root
}

fn small_dir_total_size(tree: &DirTree) -> usize {
    let total_size = if tree.total_bytes < MAX_TOTAL_BYTES {
        tree.total_bytes
    } else {
        0
    };

    total_size
        + tree
            .dirs
            .iter()
            .map(|sub| small_dir_total_size(sub))
            .sum::<usize>()
}

fn main() {
    let input = include_str!("input.txt");

    let tree = construct_directory_tree(input);

    println!("{}", small_dir_total_size(&tree));
}
