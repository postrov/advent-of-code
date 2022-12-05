use crate::types::{Move, Stacks};
use std::str::Lines;

fn parse_crate_line(line: &str, stacks: &mut Stacks) {
    let mut chars = line.chars();
    (0..stacks.len())
        .for_each(|i| {
            let item = chars.nth(1).expect("bad crate input");
            if item.is_uppercase() { // ignoring line of stack indices
                stacks[i].push(item);
            }
            chars.nth(1);
        });
} 

pub fn parse_stacks(lines: &mut Lines) -> Stacks {
    let mut stacks = Vec::new();
    if let Some(line) = lines.next() {
        let num_stacks = (line.len() + 1) / 4;
        (0..num_stacks).for_each(|_| {
            stacks.push(Vec::new());
        });
        parse_crate_line(line, &mut stacks);
    }
    for line in lines {
        if line.is_empty() {
            break;
        }
        parse_crate_line(line, &mut stacks);
    }
    (0..stacks.len())
        .for_each(|i| stacks[i].reverse());
    stacks
}

pub fn parse_move(line: &str) -> Move {
    let parts = line.split(' ')
        .collect::<Vec<&str>>();
    let values = [1, 3, 5].map(|idx| parts[idx]
        .parse::<usize>()
        .expect("bad move input"));
    Move {
        count: values[0],
        from: values[1],
        to: values[2],
    }
}

#[test]
fn parse_move_works() {
    let input = "move 13 from 3 to 6";
    let m = parse_move(input);
    assert_eq!(13, m.count);
    assert_eq!(3, m.from);
    assert_eq!(6, m.to);
}
