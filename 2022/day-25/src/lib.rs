use nom::{character::complete::{line_ending, one_of}, multi::{many1, separated_list1}, IResult, Parser};

fn to_snafu(n: i64) -> String {
    let mut remaining = n;
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
    digits.iter().rev()
        .collect()
}

fn from_snafu(s: &str) -> Result<i64, &str> {
    let mut result = 0;
    let mut base = 1;
    for digit in s.chars().rev() {
        result += match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => return Err("Invalid digit"),
        } * base;
        base *= 5;
    }
    Ok(result)
}

fn snafu_nums(input: &str) -> IResult<&str, Vec<String>> {
    let (_input, snafu_nums) = separated_list1(line_ending, many1(one_of("012-=")).map(|v| v.iter().collect::<String>()))(input)?;
    Ok((input, snafu_nums))
}

pub fn process_part1(input: &str) -> String {
    let (_input, snafu_nums) = snafu_nums(input).unwrap();

    let sum = snafu_nums.iter()
        .map(|s| from_snafu(s).unwrap())
        .sum::<i64>();
        

    to_snafu(sum)
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn to_snafu_works() {
        assert_eq!("1=1", to_snafu(16));
        assert_eq!("1=2", to_snafu(17));
        assert_eq!("1-=", to_snafu(18));
        assert_eq!("1--", to_snafu(19));
        assert_eq!("1121-1110-1=0", to_snafu(314159265));
//                 1              1
//         2              2
//         3             1=
//         4             1-
//         5             10
//         6             11
//         7             12
//         8             2=
//         9             2-
//        10             20
//        15            1=0
//        20            1-0
//      2022         1=11-2
//     12345        1-0---0
// 314159265  1121-1110-1=0
    }

    #[test]
    fn from_snafu_works() {
        assert_eq!(from_snafu("1=1").unwrap(), 16);
        assert_eq!(from_snafu("1=2").unwrap(), 17);
        assert_eq!(from_snafu("1-=").unwrap(), 18);
        assert_eq!(from_snafu("1--").unwrap(), 19);
        assert_eq!(from_snafu("1121-1110-1=0").unwrap(), 314159265);
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
