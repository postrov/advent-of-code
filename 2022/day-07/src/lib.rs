mod parser;
mod types;
// State:
//  * cur dir: ["a", "e"]
//  *  


#[allow(unused)]
struct FsFile {
    name: String,
    size: usize,
}

#[allow(unused)]
struct FsDir {
    name: String,
    dirs: Vec<FsDir>,
    files: Vec<FsFile>,
}

#[allow(unused)]
impl FsDir {
    fn new<S: Into<String>>(name: S) -> Self {
        FsDir {
            name: name.into(),
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

#[allow(unused)]
struct ParsingCtx<'a> {
    result: FsDir,
    pwd: Vec::<&'a FsDir>,
}


pub fn process_part1(input: &str) -> String {
    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_terminal;

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
        dbg!(parse_terminal(INPUT).unwrap());
        // assert_eq!("works", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
