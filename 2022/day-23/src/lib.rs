use std::{collections::{BTreeSet, BTreeMap}, ops::Add};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
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

fn find_valid_move(elf: &Point, elves: &BTreeSet<Point>, check_directions: &[[(i32, i32); 3]; 4], round: usize) -> Option<Point> {
    (0..4)
        .find_map(|idx| {
            let idx = (idx + round) % check_directions.len();
            let valid_move = check_directions[idx].into_iter()
                .map(|offset| *elf + offset)
                .all(|point| !elves.contains(&point));
            if valid_move {
                let dst = *elf + check_directions[idx][0];
                Some(dst)
            } else {
                None
            }
        })
}

fn bounding_box(elves: &BTreeSet<Point>) -> (i32, i32, i32, i32) {
    elves.iter()
        .fold(
            (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
            |(x0, y0, x1, y1), Point {x, y}| (x0.min(*x), y0.min(*y), x1.max(*x), y1.max(*y))
        )
}

fn free_space(elves: &BTreeSet<Point>) -> i32 {
    let (xmin, ymin, xmax, ymax) = bounding_box(elves);
    let area = (xmax - xmin + 1) * (ymax - ymin + 1);
    area - elves.len() as i32
}

fn display_map(elves: &BTreeSet<Point>) {
    let (xmin, ymin, xmax, ymax) = bounding_box(elves);
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let c = if elves.contains(&Point {x, y}) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

const CHECK_DIRECTIONS: [[(i32, i32); 3]; 4] = 
    [
        [(0, -1), (-1, -1), (1, -1)], // N, NE, NW
        [(0, 1), (-1, 1), (1, 1)], // S, SE, SW
        [(-1, 0), (-1, -1), (-1, 1)], // W, NW, SW
        [(1, 0), (1, -1), (1, 1)], // E, NE, SE
    ];

fn should_look_for_move(elf: &Point, elves: &BTreeSet<Point>) -> bool {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|(x, y)| *elf + (x, y))
        .any(|point| elves.contains(&point))
}

fn process_round(elves: &mut BTreeSet<Point>, round: usize) -> bool {
    let mut destinations: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
    let mut new_elves = BTreeSet::new();
    elves.iter()
        .for_each(|elf| {
// todo rethink this overcomplicated flow:
            if should_look_for_move(elf, elves) {
                find_valid_move(elf, elves, &CHECK_DIRECTIONS, round)
                    .map(|dst| {
                        destinations.entry(dst)
                            .or_default()
                            .push(*elf);
                    })
                    .unwrap_or_else(|| {
                        new_elves.insert(*elf);
                    });
            } else {
                new_elves.insert(*elf);
            }
        });
    let no_elf_moved = destinations.is_empty();
    destinations.into_iter()
        .for_each(|(dst, candidates)| {
            if candidates.len() == 1 {
                new_elves.insert(dst);
            } else {
                candidates.into_iter()
                    .for_each(|c| {
                        new_elves.insert(c);
                    });
            }
        });
    *elves = new_elves;
    no_elf_moved
}

pub fn process_part1(input: &str) -> String {
    let mut elves = parse_input(input);
    println!("==== Initial state:");
    display_map(&elves);
    for round in 0..10 {
        process_round(&mut elves, round);
        println!("==== Round {}:", round + 1);
        display_map(&elves);
    }

    free_space(&elves).to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut elves = parse_input(input);
    let mut round = 0;
    while !process_round(&mut elves, round) {
        round += 1;
    }
    (round + 1).to_string()
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
    fn part2_works() {
        assert_eq!("20", process_part2(INPUT));
    }
}
