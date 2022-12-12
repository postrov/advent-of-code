use std::{iter::repeat, collections::{VecDeque, HashSet}};

use nom::{IResult, multi::{separated_list1, many1}, character::complete::{newline, one_of}};

type Pos = (usize, usize);
type Board = Vec<Vec<char>>;

#[derive(Debug)]
struct Map {
    map: Board,
    start: Pos,
    end: Pos,
}

fn find_values(board: &Board, value: char) -> impl Iterator<Item = Pos> + '_ {
    board
        .iter()
        .enumerate() // (y, row)
        .flat_map(|(y, row)| row.iter().enumerate().zip(repeat(y)).collect::<Vec<_>>()) // ((x, c), y)
        .filter_map(move |((x, c), y)| if *c == value { Some((x, y)) } else { None })
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let allowed_characters = ('a'..='z')
        .chain(vec!['S', 'E'])
        .collect::<String>();


    let (input, mut rows) = separated_list1(newline, many1(one_of(allowed_characters.as_str())))(input)?;
    let start = find_values(&rows, 'S').next()
        .expect("There is no start position!");
    let end = find_values(&rows, 'E').next()
        .expect("There is no end position!");

    rows[start.1][start.0] = 'a';
    rows[end.1][end.0] = 'z';

    let map = Map {
        map: rows,
        start,
        end,
    };
    Ok((input, map))
}

// todo: perhaps replace this with some clever .get .. and_then usage
fn neighbors(pos: Pos, dx: usize, dy: usize) -> Vec<Pos> {
    let (x, y) = (pos.0 as i32, pos.1 as i32);
    let dx = dx as i32;
    let dy = dy as i32;

    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter()
        .filter(|(x, y)| x >= &0 && y >= &0 && x < &dx && y < &dy)
        .map(|&(x, y)| (x as usize, y as usize))
        .collect::<Vec<Pos>>()
}

fn find_path_length(map: &Map, start: Pos) -> Option<i32> {
    let mut q = VecDeque::from(vec![(start, 0)]);
    let mut dst = None;
    let mut visited = HashSet::new();
    while let Some((pos, d)) = q.pop_front() {
        if pos == map.end {
            dst = Some(d);
            break;
        }
        let h = map.map[pos.1][pos.0];
        let dy = map.map.len();
        let dx = map.map[0].len();
        neighbors(pos, dx, dy).iter()
            .flat_map(|&(x, y)| if h as usize + 1 >= map.map[y][x] as usize {
                Some(((x, y), d + 1))
            } else {
                None
            }).for_each(|node|
                if !visited.contains(&node.0) {
                    visited.insert(node.0);
                    q.push_back(node)
                }
            );

    }
    dst
}

pub fn process_part1(input: &str) -> String {
    let (_input, map) = parse_map(input).expect("Nice input bro!");
    let dst = find_path_length(&map, map.start);

    dst.map(|x| x.to_string()).unwrap()
}

pub fn process_part2(input: &str) -> String {
    let (_input, map) = parse_map(input).expect("Nice input bro!");
    find_values(&map.map, 'a')
        .flat_map(|start| find_path_length(&map, start))
        .min()
        .unwrap()
        .to_string()
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
        assert_eq!("31", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("29", process_part2(INPUT));
    }
}
