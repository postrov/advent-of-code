use std::iter;

use nom::{character::complete::{newline, self}, IResult, multi::separated_list1, branch::alt, sequence::separated_pair, bytes::complete::tag, Parser};


#[derive(Clone)]
enum Op {
    Addx(i32),
    Noop,
}

fn parse_program(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, ops) = separated_list1(newline, alt((
        tag("noop").map(|_| Op::Noop),
        separated_pair(tag("addx"), complete::char(' '), complete::i32)
        .map(|(_, x)| Op::Addx(x)))))(input)?;
    Ok((input, ops))
}

fn cycles(op: &Op) -> usize {
    match op {
        Op::Addx(_) => 2,
        Op::Noop => 1,
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, ops) = parse_program(input).unwrap();

    struct State {
        x: i32,
        sum: i32,
    }
    ops.iter().flat_map(|op| iter::repeat(Op::Noop).take(cycles(op) - 1).chain(Some(op.clone())))
        .enumerate()
        .fold(State { x: 1, sum: 0}, |mut state, (cycle, op)| {
            if cycle % 40 == 19 {
                state.sum += (cycle + 1) as i32 * state.x;
            }
            state.x = match op {
                Op::Addx(x) => state.x + x,
                _ => state.x,
            };
            state
        })
        .sum
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, ops) = parse_program(input).unwrap();
    struct State {
        x: i32,
        crt: String,
    }
    ops.iter().flat_map(|op| iter::repeat(Op::Noop).take(cycles(op) - 1).chain(Some(op.clone())))
        .enumerate()
        .fold(State { x: 1, crt: String::new() }, |mut state, (cycle, op)| {
            let sprite_range = (state.x - 1)..=(state.x + 1);
            let pixel = if sprite_range.contains(&(cycle as i32 % 40)) {
                '#'
            } else {
                ' ' 
            };
            state.crt.push(pixel);
            if cycle % 40 == 39 && cycle != 239 {
                state.crt.push('\n');
            }
            state.x = match op {
                Op::Addx(x) => state.x + x,
                _ => state.x,
            };
            state
        })
        .crt

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!("13140", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ", process_part2(INPUT));
    }
}
