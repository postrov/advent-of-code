use parser::parse_input;
use types::{Monkey, Operand, Operation};

mod parser;
mod types;

impl Monkey {
    fn operand_value(v: Operand, cur: &u64) -> u64 {
        match v {
            Operand::Value(n) => n,
            Operand::Old => *cur,
        }
    }
    fn inspect_item(&mut self, relief: bool, lcm: u64) -> u64 {
        let item = self.items.pop_front().expect("item should be here");
       
        let mut worry_level = item;
        worry_level = match self.operation {
            Operation::Mul(v1, v2) => (Self::operand_value(v1, &worry_level) * Self::operand_value(v2, &worry_level)) % lcm,
            Operation::Add(v1, v2) => (Self::operand_value(v1, &worry_level) + Self::operand_value(v2, &worry_level)) % lcm,
        };
        if relief {
            worry_level /= 3;
        }
        self.inspect_count += 1;
        worry_level
    }

    fn throw_to(&self, item: u64) -> usize {
        let throw_target = match item % self.test {
            0 => self.throw_true,
            _ => self.throw_false,
        };
        throw_target as usize
    }
}
pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = parse_input(input).unwrap();
    let lcm = monkeys.iter().fold(1, |prod, m| m.test * prod);
    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_idx].items.len() {
                let monkey = monkeys.get_mut(monkey_idx).unwrap();
                let item = monkey.inspect_item(true, lcm);
                let throw_to = monkey.throw_to(item);
                monkeys.get_mut(throw_to).unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys
        .sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys
        .iter()
        .map(|m| m.inspect_count)
        .take(2)
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = parse_input(input).unwrap();
    let lcm = monkeys.iter().fold(1, |prod, m| m.test * prod);

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_idx].items.len() {
                let monkey = monkeys.get_mut(monkey_idx).unwrap();
                let item = monkey.inspect_item(false, lcm);
                let throw_to = monkey.throw_to(item);
                monkeys.get_mut(throw_to).unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys
        .sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys
        .iter()
        .map(|m| m.inspect_count)
        .take(2)
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        assert_eq!("10605", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("2713310158", process_part2(INPUT));
    }
}
