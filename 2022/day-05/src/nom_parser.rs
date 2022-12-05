use crate::types::{Move, Stacks};
use nom::branch::alt;
use nom::character::complete::{self, alpha1, digit1, multispace1, newline, space0};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::{bytes::complete::tag, IResult};

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;
    let result = match c {
        "   " => None,
        value => Some(value.chars().next().unwrap()),
    };
    Ok((input, result))
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, crates) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, crates))
}

pub fn parse_stacks(input: &str) -> IResult<&str, Stacks> {
    let mut result: Stacks = Vec::new();
    let (input, mut layers) = separated_list1(newline, parse_crate_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space0, digit1))(input)?;
    layers.reverse();
    for _ in 0..layers[0].len() {
        result.push(Vec::new());
    }

    for layer in layers {
        for (idx, c) in layer.iter().enumerate() {
            if let Some(item) = c {
                result[idx].push(*item);
            }
        }
    }
    let (input, _) = multispace1(input)?;
    Ok((input, result))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    let result = Move {
        from: from as usize - 1,
        to: to as usize - 1,
        count: count as usize,
    };
    Ok((input, result))
}

pub fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, moves) = separated_list1(newline, parse_move)(input)?;
    Ok((input, moves))
}
