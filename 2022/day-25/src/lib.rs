use std::{str::FromStr, iter::Sum, fmt::Display};

use nom::{character::complete::{line_ending, one_of}, multi::{many1, separated_list1}, IResult, Parser};

struct Snafu {
    value: i64,
}

impl FromStr for Snafu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = 0;
        let mut base = 1;
        for digit in s.chars().rev() {
            result += match digit {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => return Err("Invalid digit".into()),
            } * base;
            base *= 5;
        }
        Ok(Snafu { value: result })
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Snafu {
            value: iter
                .map(|s| s.value)
                .sum::<i64>(),
        }
    }
}

impl Display  for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut remaining = self.value;
        let mut digits = Vec::new();
        while remaining > 0 {
            let (digit, carry_over) = match remaining % 5 { 
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 1),
                4 => ('-', 1),
                _ => unreachable!(),
            };
            digits.push(digit);
            remaining = remaining / 5 + carry_over;
        }
        let str: String = digits.iter().rev()
            .collect();
        f.write_str(&str)
    }
}

fn snafu_nums(input: &str) -> IResult<&str, Vec<String>> {
    let (_input, snafu_nums) = separated_list1(line_ending, many1(one_of("012-=")).map(|v| v.iter().collect::<String>()))(input)?;
    Ok((input, snafu_nums))
}

pub fn process_part1(input: &str) -> String {
    let (_input, snafu_nums) = snafu_nums(input).unwrap();

    snafu_nums.iter()
        .map(|s| s.parse::<Snafu>().unwrap())
        .sum::<Snafu>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[rstest]
    #[case("2", 2)] 
    #[case("1=", 3)] 
    #[case("1-", 4)] 
    #[case("10", 5)] 
    #[case("11", 6)] 
    #[case("12", 7)] 
    #[case("2=", 8)] 
    #[case("2-", 9)] 
    #[case("20", 10)] 
    #[case("1=0", 15)] 
    #[case("1-0", 20)] 
    #[case("1=11-2", 2022)] 
    #[case("1-0---0", 12345)] 
    #[case("1121-1110-1=0", 314159265)] 
    fn to_snafu_works(#[case] expected: &str, #[case] input: i64) {
        assert_eq!(Snafu { value: input }.to_string(), expected);
    }

    #[rstest]
    #[case("2", 2)] 
    #[case("1=", 3)] 
    #[case("1-", 4)] 
    #[case("10", 5)] 
    #[case("11", 6)] 
    #[case("12", 7)] 
    #[case("2=", 8)] 
    #[case("2-", 9)] 
    #[case("20", 10)] 
    #[case("1=0", 15)] 
    #[case("1-0", 20)] 
    #[case("1=11-2", 2022)] 
    #[case("1-0---0", 12345)] 
    #[case("1121-1110-1=0", 314159265)] 
    fn from_snafu_works(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(input.parse::<Snafu>().unwrap().value, expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!("2=-1=0", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
