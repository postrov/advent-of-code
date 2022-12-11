use std::{collections::BTreeSet, iter::repeat, cmp::Ordering};

use nom::{IResult, multi::separated_list1, character::complete::{newline, char, one_of, self}, sequence::separated_pair};

#[derive(Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct Move {
    dir: Dir,
    dist: u32,
}

type Pos = (i32, i32);

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, moves) = separated_list1(newline, parse_move)(input)?;

    Ok((input, moves))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, (dir, dist)) = separated_pair(one_of("LRUD"), char(' '), complete::u32)(input)?;

    let dir = match dir {
        'U' => Dir::Up,
        'D' => Dir::Down,
        'R' => Dir::Right,
        'L' => Dir::Left,
        _ => unreachable!("Bro, what a move!") // todo: handle error properly
    };
    let m = Move {
        dir,
        dist,
    };
    Ok((input, m))
}

fn calculate_step(pos: Pos, dir: &Dir) -> Pos {
    match dir {
        Dir::Up => (pos.0, pos.1 + 1),
        Dir::Down => (pos.0, pos.1 - 1),
        Dir::Left => (pos.0 - 1, pos.1),
        Dir::Right => (pos.0 + 1, pos.1),
    }
}

pub fn process_part1(input: &str) -> String {
    struct State {
        h: Pos, 
        t: Pos,
        trail: BTreeSet<Pos>,
    }

    let state = State {
        h: (0, 0),
        t: (0, 0),
        trail: BTreeSet::new(),
    };

    let (_input, moves) = parse_moves(input).unwrap();
    moves.iter()
        .flat_map(|Move {dir, dist}| repeat(dir).take(*dist as usize))
        .fold(state, |mut state, dir| {
            let new_h = calculate_step(state.h, dir);
            let new_t = calculate_catch_up(new_h, state.t);
            state.trail.insert(new_t);
            State {
                h: new_h,
                t: new_t,
                trail: state.trail,
            }
        }).trail
        .len()
        .to_string()
}

fn calculate_catch_up(h: Pos, t: Pos) -> Pos {
    fn normalize(d: i32) -> i32 {
        match d.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
    let dx = h.0 - t.0;
    let dy = h.1 - t.1;
    let adx = dx.abs(); 
    let ady = dy.abs();

    if adx >= 2 || ady >= 2 {
        (t.0 + normalize(dx), t.1 + normalize(dy))
    } else {
        t
    } 
}

pub fn process_part2(input: &str) -> String {
    const N: usize = 10;
    let mut rope = [(0, 0); N];
    let mut trail: BTreeSet<Pos> = BTreeSet::new();
    let (_input, moves) = parse_moves(input).unwrap();

    moves.iter()
        .flat_map(|Move {dir, dist}| repeat(dir).take(*dist as usize))
        .for_each(|dir| {
            rope[0] = calculate_step(rope[0], dir);
            (1..N).for_each(|i| {
                rope[i] = calculate_catch_up(rope[i - 1], rope[i]);
            });
            trail.insert(rope[N - 1]);
        });
        trail
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        assert_eq!("13", process_part1(INPUT1));
    }

const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    #[test]
    fn part2_works() {
        assert_eq!("36", process_part2(INPUT2));
    }
}
