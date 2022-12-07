use nom::{IResult, multi::separated_list1, character::complete::{newline, alpha1, not_line_ending, space1, self}, branch::alt, bytes::complete::tag, sequence::preceded};

use crate::types::{Cmd, LsOutput, Path};

pub fn parse_terminal(input: &str) -> IResult<&str, Vec<Cmd>> {
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

