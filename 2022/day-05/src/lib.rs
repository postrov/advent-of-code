use std::str::Lines;

struct Stacks1 {
    // 0 -> first stack (last vec item is on top of the stack)
    stacks: Vec<Vec<char>>,
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(lines: &mut Lines) -> Stacks1 {
    let mut first = true;
    let mut result = Stacks1 {
        stacks: Vec::new(),
    };
    let mut num_stacks = 0;
    for line in lines {
        if first {
            num_stacks = (line.len() + 1) / 4;
            (0..num_stacks).for_each(|_| {
                result.stacks.push(Vec::new());
            });
            first = false;
        }
        if line.is_empty() {
            break;
        }
        let mut chars = line.chars();
        (0..num_stacks)
            .for_each(|i| {
                let item = chars.nth(1).unwrap();
                if item.is_uppercase() {
                    result.stacks[i].push(item);
                }
                chars.nth(1);
            })
    }
    (0..num_stacks)
        .for_each(|i| result.stacks[i].reverse());
    result
}

fn parse_move(line: &str) -> Move {
    let parts = line.split(' ')
        .collect::<Vec<&str>>();
    Move {
        count: parts[1].parse::<usize>().unwrap(),
        from: parts[3].parse::<usize>().unwrap(),
        to: parts[5].parse::<usize>().unwrap(),
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

fn top_of_stacks(stacks: Stacks1) -> String {
    stacks
        .stacks
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
            let item = stacks.stacks[m.from - 1].pop().unwrap();
            stacks.stacks[m.to - 1].push(item);
        })

    });
    top_of_stacks(stacks)
}

pub fn process_part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = parse_stacks(&mut lines);

    // lines.for_each(|line| {
    //     let m = parse_move(line);
    //     let from_stack = stacks.stacks[m.from -1];
    //     let to_stack = stacks.stacks[m.to - 1];
    //     let items_to_move = from_stack[from_stack.len() - m.count..].to_vec();
    //     to_stack.append(&mut items_to_move);
    //     // from_stack.resize(from_stack.len() - m.count, '_');
    //     // (0..m.count).for_each(|_| {
    //     //     let item = stacks.stacks[m.from - 1].pop().unwrap();
    //     //     stacks.stacks[m.to - 1].push(item);
    //     // })
    // });
    lines.for_each(|line| {
        let m = parse_move(line);
        let mut tmp = Vec::with_capacity(m.count);
        (0..m.count).for_each(|_| {
            let item = stacks.stacks[m.from - 1].pop().unwrap();
            tmp.push(item);
        });
        tmp.reverse();
        stacks.stacks[m.to - 1].append(&mut tmp);
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
