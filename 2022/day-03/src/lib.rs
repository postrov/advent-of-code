#![feature(iter_array_chunks)]
use std::collections::HashMap;

fn item_value(item: &char) -> u32 {
    match *item {
        'a'..='z' => *item as u32 - 'a' as u32 + 1,
        'A'..='Z' => *item as u32 - 'A' as u32 + 27,
        _ => panic!("Wrong character {item}"),
    } 
}

#[cfg(test)]
#[test]
fn item_value_works() {
    assert_eq!(1, item_value(&'a'));
    assert_eq!(26, item_value(&'z'));
    assert_eq!(27, item_value(&'A'));
    assert_eq!(52, item_value(&'Z'));
}

pub fn process_part1(input: &str) -> String {
    input.lines()
        .map(|line| {
            let mut ruck: HashMap<char, u32> = HashMap::new();
            let ruck_size = line.len() / 2;
            let compartment1 = line[0..ruck_size].to_string();
            let compartment2 = line[ruck_size..].to_string();
            let mut duplicate = 'a';
            compartment1.chars()
                .for_each(|item| {
                    let count = ruck.get(&item).unwrap_or(&0);
                    ruck.insert(item, count + 1);
                });
            for item in compartment2.chars() {
                if ruck.get(&item).is_some() {
                    duplicate = item;
                    break;
                }
            }
            item_value(&duplicate)
        })
        .sum::<u32>()
        .to_string()
}
// ******************************************************************************  
// part 2

use bit_set::{Bit, BitBuffer, BitSet};

fn letter_to_bit_pos(c: char) -> u8 {
    (item_value(&c) - 1) as u8
}

fn backpack_to_item_set(line: &str) -> BitBuffer<u128> {
    let mut item_set = BitBuffer::default();
    line.chars()
        .map(letter_to_bit_pos)
        .for_each(|b| item_set.set(b));
    item_set
}

// this assumes single bit is set
fn rucksack_value(rucksack: BitBuffer<u128>) -> u32 {
    rucksack.into_iter()
        .find(|(_, b)| *b == Bit::One)
        .map(|t| t.0 as u32 + 1)
        .unwrap()

}
pub fn process_part2(input: &str) -> String {
    input.lines()
        .map(backpack_to_item_set)
        .array_chunks::<3>()
        .map(|[a, b, c]| a.intersection(&b).intersection(&c))
        .map(rucksack_value)
        .sum::<u32>()
        .to_string()
}

pub fn process_part2_old(input: &str) -> String {
    let mut elves = ["", "", ""];
    let mut pos = 0;
    let mut sum = 0;
    for line in input.lines() {
        elves[pos] = line;
        if pos == 2 {
            let common_items = elves.iter()
                .map(|line| backpack_to_item_set(line))
                .reduce(|a, b| a.intersection(&b))
                .unwrap();

            let item_pos = common_items
                .into_iter()
                .find(|(_, b)| *b == Bit::One)
                .map(|t| t.0)
                .unwrap();

            sum += item_pos as u32 + 1; 
        }
        pos = (pos + 1) % 3;
    };
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        assert_eq!("157", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("70", process_part2(INPUT));
    }
}
