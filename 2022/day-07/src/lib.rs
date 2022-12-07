use nom::branch::alt;
use nom::character::complete::{self, alpha1, digit1, multispace1, newline, space0, space1, not_line_ending};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::{bytes::complete::tag, IResult};

// State:
//  * cur dir: ["a", "e"]
//  *  


#[derive(Debug)]
enum Path {
    Root,
    Up,
    Name(String),
}

#[derive(Debug)]
enum LsOutput {
    File(String, usize),
    Dir(String),
}

#[derive(Debug)]
enum Cmd {
    Ls(Vec<LsOutput>),
    Cd(Path)
}

struct FsFile {
    name: String,
    size: usize,
}

#[allow()]
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

fn parse_terminal(input: &str) -> IResult<&str, Vec<Cmd>> {
    let (input, cmds) = separated_list1(newline, parse_cmd)(input)?;
    Ok((input, cmds)) 
}

fn parse_cmd(input: &str) -> IResult<&str, Cmd> {
    let (input, cmd) = alt((parse_cmd_cd, parse_cmd_ls))(input)?;
    Ok((input, cmd))
}

fn parse_cmd_cd(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, path_str) = alt((tag("/"), tag(".."), alpha1))(input)?;
    let path = match path_str {
        "/" => Path::Root,
        ".." => Path::Up,
        name => Path::Name(name.to_string()),
    };
    Ok((input, Cmd::Cd(path)))
}

fn parse_cmd_ls(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, ls_outs) = separated_list1(newline, parse_cmd_ls_out_line)(input)?; 
    let res = Cmd::Ls(ls_outs);
    Ok((input, res))
}

fn parse_cmd_ls_out_line(input: &str) -> IResult<&str, LsOutput> {
    let (input, ls_out) = alt((
        parse_cmd_ls_out_file,
        parse_cmd_ls_out_dir
    ))(input)?;
    Ok((input, ls_out))
}

fn parse_cmd_ls_out_file(input: &str) -> IResult<&str, LsOutput> {
    let (input, size) = complete::u32(input)?;
    let (input, name) = preceded(space1, not_line_ending)(input)?;
    Ok((input, LsOutput::File(name.into(), size as usize)))
}

fn parse_cmd_ls_out_dir(input: &str) -> IResult<&str, LsOutput> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, LsOutput::Dir(name.into())))
}

pub fn process_part1(input: &str) -> String {
    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
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
        dbg!(parse_terminal(INPUT).unwrap());
        // assert_eq!("works", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
