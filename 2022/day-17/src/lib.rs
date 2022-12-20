use std::{iter::repeat, fmt::Display};
use itertools::Itertools;

use nom::{IResult, character::complete::char, multi::many1, branch::alt, Parser};

const WIDTH: i32 = 7;
type Row = [char; WIDTH as usize];

fn init_shapes() -> Vec<Vec<Row>> {
    [
        [
            [' ', ' ', '@', '@', '@', '@', ' '],
        ].to_vec(),
        [
            [' ', ' ', ' ', '@', ' ', ' ', ' '],
            [' ', ' ', '@', '@', '@', ' ', ' '],
            [' ', ' ', ' ', '@', ' ', ' ', ' '],
        ].to_vec(),
        [
            [' ', ' ', ' ', ' ', '@', ' ', ' '],
            [' ', ' ', ' ', ' ', '@', ' ', ' '],
            [' ', ' ', '@', '@', '@', ' ', ' '],
        ].to_vec(),
        [
            [' ', ' ', '@', ' ', ' ', ' ', ' '],
            [' ', ' ', '@', ' ', ' ', ' ', ' '],
            [' ', ' ', '@', ' ', ' ', ' ', ' '],
            [' ', ' ', '@', ' ', ' ', ' ', ' '],
        ].to_vec(),
        [
            [' ', ' ', '@', '@', ' ', ' ', ' '],
            [' ', ' ', '@', '@', ' ', ' ', ' '],
        ].to_vec(),
    ].to_vec()
}

struct Game {
    board: Vec<[char; WIDTH as usize]>,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.iter().rev()
            .for_each(|row| writeln!(f, "{}", row.iter().join("")).unwrap());
        Ok(())
    }
}

impl Display for Game2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.iter().rev()
            .map(|row| [
                row & 0b1000000,
                row & 0b0100000,
                row & 0b0010000,
                row & 0b0001000,
                row & 0b0000100,
                row & 0b0000010,
                row & 0b0000001,
            ].map(|b| if b > 0 { '#' } else { ' ' }).iter().collect::<String>())
            .for_each(|row_str| writeln!(f, "{}", row_str).unwrap());
        Ok(())
    }
}

impl Game {
    fn add_row(&mut self) {
        self.board.push([' '; WIDTH as usize])
    }

    fn new() -> Self {
        let mut board = Vec::new();
        board.push(['#'; WIDTH as usize]);
        for _i in 0..3 {
            board.push([' '; WIDTH as usize]);
        }

        Game {
            board,
        }
    }

    fn ensure_top_rows(&mut self) {
        const REQUIRED: usize = 3;
        let board_len = self.board.len();
        let empty_row_count = (0..board_len).rev()
            .take_while(|i| self.board[*i].iter().all(|pos| pos == &' '))
            .count();
        match empty_row_count {
            0..=REQUIRED => for _ in 0..(REQUIRED - empty_row_count) { self.add_row(); },
            _ => self.board.truncate(board_len - (empty_row_count - REQUIRED))
        }
    }

    fn fall_shape(&mut self, shape: &Vec<Row>, jets: &mut dyn Iterator<Item = &Jet>) {
        let height = shape.len();
        shape.iter().rev()
            .for_each(|row| self.board.push(*row));
        let mut shape_end_row = self.board.len();
        loop {
            let jet = jets.next().unwrap();
            let shape_start_row = shape_end_row - height;
            // (left to right, bottom..up)
            let s: Vec<_> = (0..WIDTH).into_iter()
                .cartesian_product((shape_start_row..shape_end_row).into_iter())
                .filter(|(x, y)| self.board[*y][*x as usize] == '@')
                .collect();

            // get all '@' positions
            match jet {
                Jet::Left => {
                    // if all '@' have free space or '@' to the left, transpose all by 1 to the left
                    let movable = s.iter()
                        .map(|(x, y)| (x - 1, y))
                        .all(|(x, y)| x >= 0 && self.board[*y][x as usize] != '#');
                    if movable {
                        s.iter()
                            .for_each(|(x, y)| {
                                self.board[*y][(x - 1) as usize] = '@';
                                self.board[*y][*x as usize] = ' ';
                            });
                    } 
                },
                Jet::Right => {
                    // if all '@' have free space or '@' to the right, transpose all by 1 to the right
                    let movable = s.iter()
                        .map(|(x, y)| (x + 1, y))
                        .all(|(x, y)| x < WIDTH && self.board[*y][x as usize] != '#');
                    if movable {
                        s.iter().rev()
                            .for_each(|(x, y)| {
                                self.board[*y][(x + 1) as usize] = '@';
                                self.board[*y][*x as usize] = ' ';
                            });
                    } 
                },
            };

            let s: Vec<_> = (0..WIDTH).into_iter()
                .cartesian_product((shape_start_row..shape_end_row).into_iter())
                .filter(|(x, y)| self.board[*y][*x as usize] == '@')
                .collect();
            let movable = s.iter()
                        .map(|(x, y)| (x, y - 1))
                        .all(|(x, y)| self.board[y][*x as usize] != '#');
            if movable {
                s.iter()
                    .for_each(|(x, y)| {
                        self.board[y - 1][*x as usize] = '@';
                        self.board[*y][*x as usize] = ' ';
                    });
                shape_end_row -= 1;
            } else {
                // shape came to rest, @ => #
                s.iter()
                    .for_each(|(x, y)| self.board[*y][*x as usize] = '#');
                break;
            }
        }
    }
}

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

fn tower_height(input: &str, iterations: i64) -> String {
    let (_input, jets) = jets(input).unwrap();
    let mut game = Game::new();
    let mut jet_iter = repeat(&jets)
        .flat_map(|jets| jets.iter());
    let shapes = init_shapes();
    let mut shape_iter = repeat(shapes.iter())
        .flatten();
    for _i in 0..iterations {
        game.fall_shape(shape_iter.next().unwrap(), &mut jet_iter);
        game.ensure_top_rows();
    }

    let empty_rows_on_top = game.board.iter().rev()
        .take_while(|r| r.iter().all(|c| c == &' '))
        .count();

    // println!("{}", &game);
    (game.board.len() - empty_rows_on_top - 1).to_string()
}

fn tower_height2(input: &str, iterations: i64) -> String {
    let (_input, jets) = jets(input).unwrap();
    let mut game2 = Game2::new();
    let mut jet_iter = repeat(&jets)
        .flat_map(|jets| jets.iter());
    let shapes = init_shapes2();
    let mut shape_iter = repeat(shapes.iter())
        .flatten();
    for _i in 0..iterations {
        game2.fall_shape(shape_iter.next().unwrap(), &mut jet_iter);
        game2.ensure_top_rows();
    }

    let empty_rows_on_top = game2.board.iter().rev()
        .take_while(|&&r| r == 0)
        .count();

    // println!("{}", &game2);
    ((game2.board.len() - empty_rows_on_top - 1) as i64 + game2.height_offset).to_string()
}


pub fn process_part1(input: &str) -> String {
    tower_height2(input, 2022i64)
}

type Row2 = u8;

struct Game2 {
    board: Vec<Row2>,
    #[allow(unused)] // todo: for optimizing out unreachable bottom part
    height_offset: i64,
}

type Shape2 = Vec<Row2>;

fn init_shapes2() -> Vec<Shape2> {
    [
        [
            0b0011110
        ].to_vec(),
        [
            0b0001000,
            0b0011100,
            0b0001000,
        ].to_vec(),
        [
            0b0000100,
            0b0000100,
            0b0011100,
        ].to_vec(),
        [
            0b0010000,
            0b0010000,
            0b0010000,
            0b0010000,
        ].to_vec(),
        [
            0b0011000,
            0b0011000,
        ].to_vec(),
    ].to_vec()
}

impl Game2 {
    fn new() -> Self {
        Game2 {
            board: [0b1111111, 0, 0, 0].to_vec(),
            height_offset: 0,
        }
    }

    fn add_row(&mut self) {
        self.board.push(0);
    }

    fn ensure_top_rows(&mut self) {
        const REQUIRED: usize = 3;
        let board_len = self.board.len();
        let empty_row_count = (0..board_len).rev()
            .take_while(|i| self.board[*i] == 0)
            .count();
        match empty_row_count {
            0..=REQUIRED => for _ in 0..(REQUIRED - empty_row_count) { self.add_row(); },
            _ => self.board.truncate(board_len - (empty_row_count - REQUIRED))
        }
    }

    fn collides(&self, shape: &Shape2, row: usize) -> bool {
        let shape_height = shape.len();
        let shape_iter = shape.iter().rev();
        let max_row = self.board.len().min(row + shape_height);
        let board_iter = (row..max_row)
            .map(|i| self.board[i]);
        shape_iter.zip(board_iter)
            .any(|(s, b)| s & b != 0)
    }

    fn rest (&mut self, shape: &Shape2, row: usize) {
        let shape_height = shape.len();
        let shape_iter = shape.iter().rev();
        if row + shape_height > self.board.len() {
            // one shot be enough, it's only for long vertical shape
            self.add_row();
        }
        
        let board_iter = row..self.board.len();
        shape_iter.zip(board_iter)
            .for_each(|(s, board_idx)| self.board[board_idx] |= s);

    }

    fn simplify_board(&mut self, row: usize) {
        if self.board[row] == 0b1111111 {
            self.board.drain(0..row);
            self.height_offset += row as i64;
            // dbg!(row, self.height_offset);
        }
        // if there is a row with all 1s, remove it and add a row on top
    }

    fn fall_shape(&mut self, shape: &Shape2, jets: &mut dyn Iterator<Item = &Jet>) {
        let mut shape = shape.clone();
        // shape.iter().for_each(|_| self.add_row()); // todo: this sucks, will always be empty after coming to rest

        let mut shape_end_row = self.board.len();
        loop {
            let jet = jets.next().unwrap();

            match jet {
                Jet::Left => {
                    let can_shift_left = shape.iter()
                            .all(|row| row & 0b1000000 == 0);
                    
                    let shifted_shape: Shape2 = shape.iter()
                        .map(|row| row << 1)
                        .collect();

                    if can_shift_left && ! self.collides(&shifted_shape, shape_end_row) {
                        shape = shifted_shape;
                    }
                },
                Jet::Right => {
                    let can_shift_right = shape.iter()
                        .all(|row| row & 1 == 0);
                    let shifted_shape: Shape2 = shape.iter()
                        .map(|row| row >> 1)
                        .collect();

                    if can_shift_right && ! self.collides(&shifted_shape, shape_end_row) {
                        shape = shifted_shape;
                    }
                },
            };

            if !self.collides(&shape, shape_end_row - 1) {
                // advance shape position by 1
                shape_end_row -= 1;
            } else {
                self.rest(&shape, shape_end_row);
                self.simplify_board(shape_end_row);
                break;
            }
        }
    }
}

pub fn process_part2(input: &str) -> String {
    tower_height2(input, 1_000_000_000_000i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_works() {
        // let iterations = 2022;
        // assert_eq!(tower_height(INPUT, iterations), tower_height2(INPUT, iterations));
        assert_eq!("3068", process_part1(INPUT));
    }

    #[test]
    #[ignore = "waaay too large number of iterations, will go OOM"]
    fn part2_works() {
        assert_eq!("1514285714288", process_part2(INPUT));
    }
}
