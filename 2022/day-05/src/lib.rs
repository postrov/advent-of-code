mod parser;
mod types;

use crate::parser::{parse_move, parse_stacks};
use crate::types::Stacks;

fn top_of_stacks(stacks: Stacks) -> String {
    stacks.iter().flat_map(|s| s.last()).collect::<String>()
}

pub fn process_part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = parse_stacks(&mut lines);

    lines.for_each(|line| {
        let m = parse_move(line);
        (0..m.count).for_each(|_| {
            let item = stacks[m.from - 1]
                .pop()
                .expect("bad move sequence, move from empty stack");
            stacks[m.to - 1].push(item);
        })
    });
    top_of_stacks(stacks)
}

pub fn process_part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = parse_stacks(&mut lines);

    lines.for_each(|line| {
        let m = parse_move(line);
        let from = &mut stacks[m.from - 1];
        let mut drained = from.drain((from.len() - m.count)..).collect::<Vec<char>>();
        let to = &mut stacks[m.to - 1];
        to.append(&mut drained);
    });

    top_of_stacks(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        assert_eq!("CMZ", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("MCD", process_part2(INPUT));
    }
}
