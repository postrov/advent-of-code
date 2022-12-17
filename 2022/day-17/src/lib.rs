use std::{iter::repeat, fmt::Display};

use nom::{IResult, character::complete::char, multi::many1, branch::alt, Parser};

enum Jet {
    Left,
    Right,
}

impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Jet::Left => "L",
            Jet::Right => "R",
        })
    }
}

fn jets(input: &str) -> IResult<&str, Vec<Jet>> {
    let (input, jets) = many1(
        alt((
            char('<'),
            char('>')
        )).map(|c| match c {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => unreachable!(),
            })
    )(input)?;
    Ok((input, jets))
}

pub fn process_part1(input: &str) -> String {
    let (_input, jets) = jets(input).unwrap();
    repeat(&jets)
        .flat_map(|jets| jets.iter())
        .take(15)
        .for_each(|jet| println!("dupa: {}", jet));

    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_works() {
        assert_eq!("works", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
