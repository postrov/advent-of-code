use std::collections::VecDeque;

use nom::{IResult, character::complete::{newline, self, char, multispace0}, multi::{separated_list1, many1}, bytes::complete::tag, sequence::{preceded, delimited}, branch::alt, Parser};

use crate::types::{Monkey, Operation, Operand};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(many1(newline), parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, id) = complete::u64(input)?;
    let (input, _) = preceded(tag(":"), newline)(input)?;
    let (input, items) = preceded(tag("  Starting items: ") , separated_list1(tag(", "), complete::u64))(input)?;
    let (input, _) = newline(input)?;
    let (input, operation) = preceded(tag("  Operation: new = "), parse_operation)(input)?;
    let (input, _) = newline(input)?;
    let (input, test) = preceded(tag("  Test: divisible by "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, throw_true) = preceded(tag("    If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, throw_false) = preceded(tag("    If false: throw to monkey "), complete::u64)(input)?;


    let items = VecDeque::from(items);
    Ok((input, Monkey {
        id,
        items,
        operation,
        test,
        throw_true,
        throw_false,
        inspect_count: 0,
    }))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, operand1) = parse_operand(input)?;
    let (input, op) = delimited(
        multispace0,
        alt((char('*'), char('+'))),
        multispace0)
        (input)?;
    let (input, operand2) = parse_operand(input)?;
    let operation = match op {
        '*' => Operation::Mul(operand1, operand2),
        '+' => Operation::Add(operand1, operand2),
        _ => unreachable!("parser should not allow that"),
    };
    
    Ok((input, operation))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    let (input, result) = alt((
        tag("old").map(|_| Operand::Old),
        complete::u64.map(Operand::Value)
    ))(input)?;
    Ok((input, result))
}
