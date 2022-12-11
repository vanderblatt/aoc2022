const DISK_BYTES: usize = 70_000_000;
const DESIRED_FREE_BYTES: usize = 30_000_000;

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

fn smallest_to_delete(tree: &DirTree, bytes_to_delete: usize) -> usize {
    let smallest_child = tree
        .dirs
        .iter()
        .map(|sub| smallest_to_delete(sub, bytes_to_delete))
        .min()
        .unwrap_or(usize::MAX);

    if tree.total_bytes >= bytes_to_delete {
        if tree.total_bytes < smallest_child {
            tree.total_bytes
        } else {
            smallest_child
        }
    } else {
        smallest_child
    }
}

fn main() {
    let input = include_str!("input.txt");

    let tree = construct_directory_tree(input);
    let bytes_to_delete = DESIRED_FREE_BYTES - (DISK_BYTES - tree.total_bytes);

    println!("{}", smallest_to_delete(&tree, bytes_to_delete));
}
