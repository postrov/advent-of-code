use std::collections::HashMap;


#[allow(unused)]
struct Rucksack {
    ruck: HashMap<char, u32>,
    sack: HashMap<char, u32>,
}


fn item_value(item: &char) -> u32 {
    let v = *item as u32;
    let (base, base_value) = if item.is_uppercase() {
        ('A' as u32, 27)
    }
    else {
        ('a' as u32, 1)
    };
    v - base + base_value
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

pub fn process_part2(input: &str) -> String {
    input.into()
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
        assert_eq!(INPUT, process_part2(INPUT));
    }
}
