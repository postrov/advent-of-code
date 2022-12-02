pub fn process_part1(input: &str) -> String {
    input.split("\n\n")
        .into_iter()
        .map(|elf| 
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>())
        .max()
        .unwrap_or(0)
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut elf_calories = input.split("\n\n")
        .into_iter()
        .map(|elf| 
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum())
        .collect::<Vec<u32>>();
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories
        .iter()
        .take(3)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!("24000", result);
    }

    #[test]
    fn part2_works() {
        assert_eq!("45000", process_part2(INPUT));
    }
}
