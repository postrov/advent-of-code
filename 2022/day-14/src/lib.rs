// todo: not too happy about usize <-> i32 back and forth conversions

use core::fmt;
use std::{cmp::{min, max}, fmt::{Display, Write}, iter::{repeat, once}};

use nom::{character::complete::{self, newline}, multi::separated_list1, bytes::complete::tag, sequence::separated_pair, IResult};

type Pair = (i32, i32);
type Path = Vec<Pair>;

fn path(input: &str) -> IResult<&str, Path> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::i32, complete::char(','), complete::i32)
    )(input)?;
    Ok((input, pairs))
}

fn parse_cave(input: &str) -> IResult<&str, Cave> {
    let (input, paths) = separated_list1(newline, path)(input)?;
    
    let (xmin, _ymin, xmax, ymax) = paths.iter()
        .flat_map(|path| path.iter())
        .fold((1000, 1000, 0, 0), |(x0, y0, x1, y1), (x, y)| (min(x0, *x), min(y0, *y), max(x1, *x), max(y1, *y)));
    // new coords: x - xmin, y
    // dimensions: xmax - xmin + 1, ymax + 
    let dx = xmax - xmin + 1;
    let dy = ymax + 1;
    let mut cave = Cave {
        dx: dx as usize,
        dy: dy as usize,
        map: Vec::with_capacity(dy as usize),
        spawn: (500 - xmin, 0),
    };
    (0..dy).for_each(|_| {
        let mut row = Vec::with_capacity(dx as usize);
        (0..dx).for_each(|_| row.push(Field::Empty));
        cave.map.push(row);
    });

    for path in paths {
        path
            .windows(2)
            .for_each(|win| {
                match win {
                    [p1, p2] => {
                        let (x1, y1) = ((p1.0 - xmin), p1.1);
                        let (x2, y2) = ((p2.0 - xmin), p2.1);
                        let (x1, x2) = (min(x1, x2), max(x1, x2));
                        let (y1, y2) = (min(y1, y2), max(y1, y2));
                        
                        for x in x1..=x2 {
                            for y in y1..=y2 {
                                cave.map[y as usize][x as usize] = Field::Wall;
                            }
                        }
                    }
                    _ => panic!("unexpected window size"),
                }
        })
    }
    Ok((input, cave))
}

#[derive(Eq, PartialEq, Clone)]
enum Field {
    Empty,
    Sand,
    Wall,
}

struct Cave {
    map: Vec<Vec<Field>>,
    dx: usize,
    dy: usize,
    spawn: Pair,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n').unwrap();
        (0..self.dy).for_each(|y| {
            let row = &self.map[y];
            (0..self.dx).for_each(|x| {
                let field = match row[x] {
                    Field::Empty => '.',
                    Field::Sand => 'o',
                    Field::Wall => '#',
                };
                if (x as i32, y as i32) == self.spawn {
                    f.write_char('+').unwrap();
                } else {
                    f.write_char(field).unwrap();
                }
            });
            f.write_char('\n').unwrap(); // todo: same as above
        });
        Ok(())
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

enum MoveOutcome {
    Abyss,
    Found(Pair),
}

impl Cave {

    fn count_sand(&self) -> usize {
        self.map.iter()
            .flat_map(|row| row.iter())
            .filter(|&field| *field == Field::Sand)
            .count()
    }
    fn set(&mut self, x: i32, y: i32, value: Field) {
        self.map[y as usize][x as usize] = value;
    }

    fn get(&self, x: i32, y: i32) -> &Field {
        &self.map[y as usize][x as usize]
    }

    fn expand_cave(&self) -> Self {
        let new_height = self.dy + 2;
        // need at least this to the left and right of spawn
        let spawn_x = self.spawn.0;
        let safe_margin = new_height - 1;
        let pad_left = max((safe_margin as i32) - spawn_x, 0) as usize;
        let pad_right = max((safe_margin as i32) - ((self.dx as i32) - spawn_x - 1), 0) as usize;
        let new_dx = self.dx + pad_left + pad_right;
        let new_rows = self.map.iter()
            .map(|row| 
                 repeat(Field::Empty).take(pad_left)
                    .chain(row.iter().cloned())
                    .chain(repeat(Field::Empty).take(pad_right))
                    .collect::<Vec<Field>>()
            )
            .chain(once(repeat(Field::Empty).take(new_dx).collect()))
            .chain(once(repeat(Field::Wall).take(new_dx).collect()))
            .collect::<Vec<Vec<Field>>>();
        Self {
            dx: new_dx,
            dy: new_height,
            map: new_rows,
            spawn: (spawn_x + (pad_left as i32), self.spawn.1)
        } 

    }

    fn spawn_sand(&mut self) -> bool {
        let (mut sx, mut sy) = self.spawn;
        if self.map[sy as usize][sx as usize] != Field::Empty {
            return false;
        }

        loop { 
            let possible_moves = [(sx, sy + 1), (sx - 1, sy + 1), (sx + 1, sy + 1)]; // down, left, right
            let found_move = possible_moves.iter()
                .find_map(|(x, y)| {
                    if *x < 0 || (*x as usize) >= self.dx || (*y as usize) >= self.dy {
                        Some(MoveOutcome::Abyss)
                    } else if self.get(*x, *y) == &Field::Empty {
                        Some(MoveOutcome::Found((*x, *y)))
                    } else {
                        None
                    }
                });
            match found_move {
                Some(MoveOutcome::Abyss) => return false,
                Some(MoveOutcome::Found((x, y))) => {
                    sx = x;
                    sy = y;
                }
                None => {
                    self.set(sx, sy, Field::Sand);
                    return true;
                }
            }
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (_input, mut cave) = parse_cave(input).unwrap();
    while cave.spawn_sand() {}; 

    dbg!(&cave);
    cave.count_sand().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, cave) = parse_cave(input).unwrap();
    let mut cave = cave.expand_cave();
    while cave.spawn_sand() {}; 

    dbg!(&cave);
    cave.count_sand().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        assert_eq!("24", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("93", process_part2(INPUT));
    }
}
