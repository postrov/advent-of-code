use std::{collections::BTreeSet, ops::Add};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

fn parse_input(input: &str) -> BTreeSet<Point> {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(
                move |(x, c)| if c == '#' {
                    Some(Point {
                        x: x as i32,
                        y: y as i32 
                    })
                }
                else {
                    None 
                }
            )
        )
        .collect()
}

pub fn process_part1(input: &str) -> String {
    // let elves = parse_input(input);
    let check_directions = 
    [
        [(0, -1), (-1, -1), (1, -1)], // N, NE, NW
        [(0, 1), (-1, 1), (1, 1)], // S, SE, SW
        [(-1, 0), (-1, -1), (-1, 1)], // W, NW, SW
        [(1, 0), (1, -1), (1, 1)], // E, NE, SE

    ];
    // on each step dest point -> list of elves trying to move into it
    let x: BTreeMap<Point, Vec<Point>> = new BTreeMap();
    // for each elf, consider movement options, pick first available from check_directions
    //   insert or append into x
    
    // iterate over all entries in x
    //   if entry has 1 value, put key in new map
    //   if entry has more than 1 value, put all values in new map

    // cycle change directions (modulo)

    dbg!(&elves);
    "N/A".into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_works() {
        assert_eq!("110", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
