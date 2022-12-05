use std::str::Lines;

type Stacks = Vec<Vec<char>>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(lines: &mut Lines) -> Stacks {
    let mut first = true;
    let mut stacks = Vec::new();
    let mut num_stacks = 0;
    for line in lines {
        if first {
            num_stacks = (line.len() + 1) / 4;
            (0..num_stacks).for_each(|_| {
                stacks.push(Vec::new());
            });
            first = false;
        }
        if line.is_empty() {
            break;
        }
        let mut chars = line.chars();
        (0..num_stacks)
            .for_each(|i| {
                let item = chars.nth(1).expect("bad crate input");
                if item.is_uppercase() {
                    stacks[i].push(item);
                }
                chars.nth(1);
            })
    }
    (0..num_stacks)
        .for_each(|i| stacks[i].reverse());
    stacks
}

fn parse_move(line: &str) -> Move {
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

fn top_of_stacks(stacks: Stacks) -> String {
    stacks
        .iter()
        .flat_map(|s| s.last())
        .collect::<String>()
}

pub fn process_part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = parse_stacks(&mut lines);

    lines.for_each(|line| {
        let m = parse_move(line);
        (0..m.count).for_each(|_| {
            let item = stacks[m.from - 1].pop().expect("bad move sequence, move from empty stack");
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
        let from = &mut stacks[m.from -1];
        let mut drained = from.drain((from.len() - m.count)..)
            .collect::<Vec<char>>();
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
