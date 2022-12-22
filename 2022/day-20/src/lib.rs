use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> String {
    let nums = input.split("\n")
        // .inspect(|s| println!("s: {}", s))
        .filter_map(|l| l.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    
    // let (low, high) = nums.iter()
    //     .fold((i32::MAX, i32::MIN), |(l, h ), &n| (l.min(n), h.max(n)));
    // dbg!(low, high); // +/- 10k

    // let uniq: BTreeSet<i32> = BTreeSet::from_iter(nums.iter().copied());
    // dbg!(uniq.len()); // not unique
    
    // let v = vec![1, 2, -3, 4, 0, 3, -2];
    // let l = v.len();
    // let pos0 = 4;
    // dbg!(v[(pos0 + 1000) % l]);
    // dbg!(v[(pos0 + 2000) % l]);
    // dbg!(v[(pos0 + 3000) % l]); // 4, -3, 2 => correct

    // challenge 1: how to know which number is next after move?
    // challenge 2: efficiently shift numbers

    "x".into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_works() {
        assert_eq!("3", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
