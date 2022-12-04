fn line_to_nums(line: &str) -> [u32;4] {
    line
        .split([',', '-'])
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

fn completely_overlapping(assignments: &[u32;4]) -> bool {
    let [a, b, c, d] = assignments;
    (a >= c && b <= d) || (c >= a && d <= b)
}

pub fn process_part1(input: &str) -> String {
    input.lines()
        .map(line_to_nums)
        .filter(completely_overlapping)
        .count()
        .to_string()
}

fn partially_overlapping(assignments: &[u32;4]) -> bool {
    let [a, b, c, d] = assignments;
    (b >= c && b <= d) || (d >= a && d <= b)
}

pub fn process_part2(input: &str) -> String {
    input.lines()
        .map(line_to_nums)
        .filter(partially_overlapping)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        assert_eq!("2", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("4", process_part2(INPUT));
    }
}
