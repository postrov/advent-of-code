use std::collections::BTreeSet;

use nom::{IResult, multi::separated_list1, character::complete::{newline, char, one_of, self}, sequence::separated_pair};

enum Move {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

type Pos = (i32, i32);

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    Ok((input, moves))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, (dir, len)) = separated_pair(one_of("LRUD"), char(' '), complete::i32)(input)?;

    let m = match dir {
        'U' => Move::Up(len),
        'D' => Move::Down(len),
        'R' => Move::Right(len),
        'L' => Move::Left(len),
        _ => panic!("Bro, what a move!") // todo: handle error properly
    };
    Ok((input, m))
}

fn calculate_step(pos: Pos, m: Move) -> Pos {
Match m {
        Move::Up(len) => (pos.0, pos.1 + len),
        Move::Down(len) => (pos.0, pos.1 - len),
        Move::Left(len) => (pos.0 - len, pos.1),
        Move::Right(len) => (pos.0 + len, pos.1),
    }
}

pub fn process_part1(input: &str) -> String {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut trail = BTreeSet::<Pos>::new();

    let moves = parse_moves(input).unwrap();
    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    #[ignore = "not implemented"]
    fn part1_works() {
        assert_eq!("13", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
