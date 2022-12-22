use std::collections::HashMap;

use nom::{IResult, sequence::separated_pair, character::complete::{alpha1, self, line_ending}, bytes::streaming::tag, branch::alt, Parser, multi::separated_list1};

#[derive(Debug)]
enum Op {
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Debug)]
enum Job<'a> {
    Yell(i64),
    Operation(&'a str, Op, &'a str),
}

#[derive(Debug)]
struct Monkey<'a> {
    id: &'a str,
    job: Job<'a>,
}

fn job(input: &str) -> IResult<&str, Job> {
    let (input, job) = alt((
        alt((
            separated_pair(alpha1, tag(" + "), alpha1).map(|(a, b)| Job::Operation(a, Op::Add, b)),
            separated_pair(alpha1, tag(" - "), alpha1).map(|(a, b)| Job::Operation(a, Op::Sub, b)),
            separated_pair(alpha1, tag(" * "), alpha1).map(|(a, b)| Job::Operation(a, Op::Mul, b)),
            separated_pair(alpha1, tag(" / "), alpha1).map(|(a, b)| Job::Operation(a, Op::Div, b)),
        )),
        complete::i64.map(Job::Yell)
    ))(input)?;

    Ok((input, job))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (id, job)) = separated_pair(
        alpha1,
        tag(": "),
        job
    )(input)?;

    Ok((input, Monkey {id, job} ))
}

fn what_monkey_yells(m: &Monkey, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match &m.job {
        Job::Yell(num) => *num,
        Job::Operation(a, op, b) => {
            let a = what_monkey_yells(monkeys.get(a).unwrap(), monkeys);
            let b = what_monkey_yells(monkeys.get(b).unwrap(), monkeys);
            match op {
                Op::Add => a + b,
                Op::Div => a / b,
                Op::Mul => a * b,
                Op::Sub => a - b,
            }
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (_input, monkeys) = separated_list1(line_ending, monkey)(input).unwrap();
    let monkeys = HashMap::<&str, Monkey>::from_iter(
        monkeys.into_iter().map(|m| (m.id, m))
    );

    let root = monkeys.get("root").unwrap();

    what_monkey_yells(root, &monkeys)
        .to_string()
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

fn find_monkey_path(id: &str, m: &Monkey, monkeys: &HashMap<&str, Monkey>) -> Option<Vec<Dir>> {
    if m.id == id {
        return Some(vec![]);
    }

    match m.job {
        Job::Yell(_) => None,
        Job::Operation(left, _, right) => {
            if let Some(mut v) = find_monkey_path(id, monkeys.get(left).unwrap(), monkeys) {
                v.push(Dir::Left);
                return Some(v);
            }

            if let Some(mut v) = find_monkey_path(id, monkeys.get(right).unwrap(), monkeys) {
                v.push(Dir::Right);
                return Some(v);
            }
            None
        },
    }
}

fn what_monkey_needs_to_yell(m: &Monkey, path: &[Dir], wanted: i64, monkeys: &HashMap<&str, Monkey>) -> i64 {
    if path.is_empty() {
        return wanted;
    }

    let search_dir = &path[0];
    let new_path = &path[1..];
    let m = monkeys.get(m.id).unwrap();
    if let Job::Operation(left, op, right) = &m.job {
        let right = monkeys.get(right).unwrap();
        let left = monkeys.get(left).unwrap();

        match search_dir {
            Dir::Left => {
                let known = what_monkey_yells(right, monkeys);
                let new_wanted = match op {
                    Op::Add => wanted - known,
                    Op::Div => wanted * known,
                    Op::Mul => wanted / known,
                    Op::Sub => wanted + known,
                };
                what_monkey_needs_to_yell(left, new_path, new_wanted, monkeys)
            }
            Dir::Right => {
                let known = what_monkey_yells(left, monkeys);
                let new_wanted = match op {
                    Op::Add => wanted - known,
                    Op::Div => known / wanted,
                    Op::Mul => wanted / known,
                    Op::Sub => known - wanted,
                };
                what_monkey_needs_to_yell(right, new_path, new_wanted, monkeys)
            }
        }

    } else {
        -31337 // unreachable
    }
}

pub fn process_part2(input: &str) -> String {
    let (_input, monkeys) = separated_list1(line_ending, monkey)(input).unwrap();
    let monkeys = HashMap::<&str, Monkey>::from_iter(
        monkeys.into_iter().map(|m| (m.id, m))
    );

    let root = monkeys.get("root").unwrap();
    if let Job::Operation(left, _, right) = root.job {
        let mut path = find_monkey_path("humn", root, &monkeys).unwrap();
        let right = monkeys.get(right).unwrap();
        let left = monkeys.get(left).unwrap();
        path.reverse();
        let p0 = &path[0];
        match p0 {
            Dir::Left => {
                let wanted = what_monkey_yells(right, &monkeys);
                what_monkey_needs_to_yell(left, &path[1..], wanted, &monkeys)
            }
            Dir::Right => {
                let wanted = what_monkey_yells(left, &monkeys);
                what_monkey_needs_to_yell(right, &path[1..], wanted, &monkeys)
            }
        }.to_string()
    } else {
        unreachable!() // unreachable with proper input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_works() {
        assert_eq!("152", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("301", process_part2(INPUT));
    }
}
