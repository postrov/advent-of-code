pub fn process_part1(input: &str) -> String {
    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ".234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8";

    #[test]
    fn part1_works() {
        assert_eq!("works", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
