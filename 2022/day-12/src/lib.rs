use std::iter::repeat;

use nom::{IResult, multi::{separated_list1, many1}, character::complete::{newline, one_of}};

type Pos = (usize, usize);
type Board = Vec<Vec<char>>;

struct Map {
    map: Board,
    start: Pos,
    end: Pos,
}

fn find_value(board: &Board, value: char) -> Option<Pos> {
    board
        .iter()
        .enumerate() // (y, row)
        .flat_map(|(y, row)| row.iter().enumerate().zip(repeat(y)).collect::<Vec<_>>()) // ((x, c), y)
        .find_map(|((x, c), y)| if *c == value { Some((x, y)) } else { None })
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let allowed_characters = ('a'..='z')
        .chain(vec!['S', 'E'])
        .collect::<String>();
    

    let (input, mut rows) = separated_list1(newline, many1(one_of(allowed_characters.as_str())))(input)?;
    let start = find_value(&rows, 'S')
        .expect("There is no start position!");
    let end = find_value(&rows, 'E')
        .expect("There is no end position!");
    
    rows[start.1][start.0] = 'a';
    rows[end.1][end.0] = 'z';

    let map = Map {
        map: Vec::new(),
        start,
        end,
    };
    Ok((input, map))
}

pub fn process_part1(input: &str) -> String {
    let (input, map) = parse_map(input).expect("Nice input bro!");
    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

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
