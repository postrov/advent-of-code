use std::{collections::{BTreeSet, BTreeMap}, ops::Add};

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
    println!("-------------------------");
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

pub fn process_part1(input: &str) -> String {
    let check_directions = [
        [(0, -1), (-1, -1), (1, -1)], // N, NE, NW
        [(0, 1), (-1, 1), (1, 1)], // S, SE, SW
        [(-1, 0), (-1, -1), (-1, 1)], // W, NW, SW
        [(1, 0), (1, -1), (1, 1)], // E, NE, SE
    ];
    let mut elves = parse_input(input);
    display_map(&elves);
    for round in 0..10 {
        let mut destinations: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
        let mut new_elves = BTreeSet::new();
        elves.iter()
            .for_each(|elf| {
                find_valid_move(elf, &elves, &check_directions, round)
                    .map(|dst| {
                       destinations.entry(dst)
                            .or_default()
                            .push(*elf);
                    })
                    .unwrap_or_else(|| {
                        new_elves.insert(*elf);
                    });
            });
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
        display_map(&new_elves);
        elves = new_elves;
    }

    free_space(&elves).to_string()
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
