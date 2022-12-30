// 0 -> empty
// 1 -> elf
// 2 -> <
// 4 -> >
// 8 -> ^
// 16 -> v
// 32 -> #
// 64 -> entry
// 128 -> exit

use std::{fmt::Display, str::FromStr};

// use nom::{multi::{separated_list1, many1}, character::complete::{line_ending, one_of}};

#[allow(unused)]
const EMPTY: u8 = 0;
#[allow(unused)]
const ELF: u8 = 1;
const LEFT: u8 = 2;
const RIGHT: u8 = 4;
const UP: u8 = 8;
const DOWN: u8 = 16;
const WALL: u8 = 32;
const ENTRY: u8 = 64;
const EXIT: u8 = 128;

#[derive(Hash, Eq, PartialEq)]
struct Map {
    dx: usize,
    dy: usize,
    field: Vec<u8>,
}

impl Map {
    fn left_from(&self, x: usize) -> usize {
        match x {
            1 => self.dx - 2,
            _ => x - 1,
        }
    }

    fn right_from(&self, x: usize) -> usize {
        if x == self.dx - 2 {
            1
        } else {
            x + 1
        }
    }

    fn up_from(&self, y: usize) -> usize {
        match y {
            1 => self.dy - 2,
            _ => y - 1,
        }
    }

    fn down_from(&self, y: usize) -> usize {
        if y == self.dy - 2 {
            1
        } else {
            y + 1
        }
    }

    fn to_idx(&self, x: usize, y: usize) -> usize {
        y * self.dx + x
    }

    fn after_minute(&self) -> Self {
        let mut field = Vec::with_capacity(self.field.len());

        for i in 0..self.field.len() {
            field.push(self.field[i] & 0b11100000); // retain only solid (walls, entrance, exit) objects
        }
        let mut res = Map {
            dx: self.dx,
            dy: self.dy,
            field,
        };

        for y in 0..self.dy {
            for x in 0..self.dx {
                if let Some(v) = self.get_xy(x, y) {
                    if v & LEFT != 0 {
                        res.or_xy(self.left_from(x), y, LEFT);
                    }
                    if v & RIGHT != 0 {
                        res.or_xy(self.right_from(x), y, RIGHT);
                    }
                    if v & UP != 0 {
                        res.or_xy(x, self.up_from(y), UP);
                    }
                    if v & DOWN != 0 {
                        res.or_xy(x, self.down_from(y), DOWN);
                    }
                };
            }
        }
        res
    }

    fn get_xy(&self, x: usize, y: usize) -> Option<u8> {
        self.field.get(self.to_idx(x, y)).copied()
    }

    // fn set_xy(&mut self, x: usize, y: usize, value: u8) {
    //     let idx = self.to_idx(x, y);
    //     if let Some(v) = self.field.get_mut(idx) {
    //         *v = value;
    //     }
    // }

    fn or_xy(&mut self, x: usize, y: usize, bits: u8) {
        let idx = self.to_idx(x, y);
        if let Some(v) = self.field.get_mut(idx) {
            *v |= bits;
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.field.len() {
            let v = self.field[i];
            let repr = match v {
                0 => " ",
                ENTRY => "E",
                EXIT => "X",
                WALL => "#",
                LEFT => "<",
                RIGHT => ">",
                UP => "^",
                DOWN => "v",
                _ => match (1..=4).map(|bit| (v >> bit) & 1).sum::<u8>() {
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    x => unreachable!("dafuq: x: {}, bits: {}", x, v)
                }
            };
            write!(f, "{}", repr)?;
            if i % self.dx == self.dx - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split_whitespace()
            .collect::<Vec<_>>();
        let dx = lines[0].len();
        let dy = lines.len();

        let mut res = Map {
            dx,
            dy,
            field: Vec::with_capacity(dx * dy),
        };
        
        lines
            .into_iter()
            .flat_map(|line| line.chars())
            .for_each(|c| {
                let repr = match c {
                    '#' => WALL,
                    '.' => EMPTY,
                    '<' => LEFT,
                    '>' => RIGHT,
                    '^' => UP,
                    'v' => DOWN,
                    _ => panic!("invalid input: {}", c),
                };
                res.field.push(repr);
            });
        Ok(res)
    }
}

struct State {
    map_variant: usize,
    elf_x: usize,
    elf_y: usize,
    steps: usize,
}

impl State {
    fn bound(&self, exit: (usize, usize)) -> usize {
        // manhattan distance
        self.elf_x.abs_diff(exit.0) + self.elf_y.abs_diff(exit.1) + self.steps
    }

    fn branch(&self, maps: &[Map]) -> Vec<Self> {
        let map_variant = (self.map_variant + 1) % maps.len();
        let next_map = &maps[map_variant];
        let steps = self.steps + 1;

        let mut possible_moves = Vec::new(); 
        let x = self.elf_x;
        let y = self.elf_y;
        possible_moves.push((x, y));
        if x > 0 {
            possible_moves.push((x - 1, y));
        }        
        if x < next_map.dx - 1 {
            possible_moves.push((x + 1, y));
        }
        if y < next_map.dy - 1 {
            possible_moves.push((x, y + 1));
        }
        if y > 0 {
            possible_moves.push((x, y - 1));
        }
        possible_moves.into_iter()
            .filter(|&(x, y)| next_map
                .get_xy(x, y)
                .map(|v| v == EMPTY)
                .unwrap_or(false)
            )
            .map(|(x, y)| State {
                map_variant,
                elf_x: x,
                elf_y: y,
                steps,
            })
            .collect()
    }
}

pub fn process_part1(input: &str) -> String {
    let mut map = input.parse::<Map>().unwrap();
    let possible_state_num = (map.dx - 2) * (map.dy - 2); // todo: lcm
    let mut maps = Vec::with_capacity(possible_state_num);

    for _ in 0..possible_state_num {
        let after_minute = map.after_minute();
        maps.push(map);
        map = after_minute;
    }

    let elf_x = map.field.iter().position(|&b| b == EMPTY).unwrap();
    let exit_y = map.dy - 1;
    let exit_x = map.field[(exit_y * map.dx)..].iter().position(|&b| b == EMPTY).unwrap();

    let mut queue = vec![State {
        map_variant: 0,
        elf_x,
        elf_y: 0,
        steps: 0,
    }];

    let mut best = usize::MAX;
    while let Some(state) = queue.pop() {
        if state.elf_y == map.dy - 1 {
            best = best.min(state.steps);
        }

        if state.bound((exit_x, exit_y)) < best {
            state.branch(&maps).into_iter()
                .for_each(|s| queue.push(s))
        }
    }

    // "N/A".into()
    best.to_string()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

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
