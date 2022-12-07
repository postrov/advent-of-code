use std::collections::{BTreeMap, BTreeSet};

use parser::parse_terminal;
use types::{Cmd, LsOutput, Path};

mod parser;
mod types;

fn calculate_dir_sizes(input: &str) -> BTreeMap<String, usize> {
    let (_, cmds) = parse_terminal(input).expect("failed to parse input");
    let mut dir_sizes = BTreeMap::new();
    let mut dir_lsed = BTreeSet::<String>::new();
    let mut current_dir =  vec!["/".to_string()];
 
    for cmd in cmds {
        match cmd {
            Cmd::Ls(ls_outs) => {
                let pwd = current_dir.join("/");
                if dir_lsed.contains(&pwd) {
                    continue; // do not count the same directory again
                }
                dir_lsed.insert(pwd.clone());
                let files_size = ls_outs.iter()
                    .filter_map(|ls_out| match ls_out {
                        LsOutput::Dir(_name) => None,
                        LsOutput::File(_, size) => Some(size),
                    })
                    .sum::<usize>();
                dir_sizes.entry(pwd.clone())
                    .and_modify(|size| *size += files_size)
                    .or_insert(files_size);
                for i in 1..current_dir.len() {
                    let dir = current_dir[..i].join("/");
                    dir_sizes.entry(dir)
                        .and_modify(|size| *size += files_size)
                        .or_insert(files_size);
                }
            }
            Cmd::Cd(path) => {
                match path {
                    Path::Root => {
                        current_dir.drain(1..);
                    }
                    Path::Up => {
                        current_dir.pop();
                    }
                    Path::Name(name) => {
                        current_dir.push(name);
                    }
                }
            }
        }
    }
    dir_sizes
}

pub fn process_part1(input: &str) -> String {
    calculate_dir_sizes(input).iter()
        .map(|(_dir, size)| *size)
        .filter(|size| *size <= 100000)
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    let dir_sizes = calculate_dir_sizes(input);
    let root_dir_size = dir_sizes.get("/").unwrap();
    let avail = TOTAL_SPACE - root_dir_size;
    let to_free = NEEDED_SPACE - avail;

    dir_sizes.iter()
        .map(|(_dir, size)| *size)
        .filter(|size| *size >= to_free)
        .min()
        .expect("unable to find directory to delete")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        assert_eq!("95437", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("24933642", process_part2(INPUT));
    }
}
