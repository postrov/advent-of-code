use std::str::FromStr;
// ******************************************************************************
// common
#[derive(Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, PartialEq, Eq)]
enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err("Not a valid Shape"),
        }
    }
}

// ******************************************************************************
// part 1

struct Round1 {
    action: Shape,
    response: Shape,
}

impl FromStr for Round1 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err("Invalid line length");
        }
        let action = s[0..1].parse::<Shape>()?;
        let response = s[2..3].parse::<Shape>()?;
        Ok(Round1 { action, response })
    }
}

impl Round1 {
    fn score(&self) -> u32 {
        let response_score = match self.response {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
        let result_score = match self.result() {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        };
        response_score + result_score
    }

    fn result(&self) -> RoundResult {
        let Round1 { action, response } = self;
        if action == response {
            return RoundResult::Draw;
        }
        let win = match response {
            Shape::Rock => *action == Shape::Scissors,
            Shape::Paper => *action == Shape::Rock,
            Shape::Scissors => *action == Shape::Paper,
        };
        if win {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Round1>().unwrap())
        // .map(|line| Round::parse(line).unwrap())
        .map(|round| round.score())
        .sum::<u32>()
        .to_string()
}
// ******************************************************************************
// part 2
struct Round2 {
    action: Shape,
    result: RoundResult,
}

impl FromStr for RoundResult {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err("Not a valid RoundResult"),
        }
    }
}

impl FromStr for Round2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err("Invalid line length");
        }
        let action = s[0..1].parse::<Shape>()?;
        let result = s[2..3].parse::<RoundResult>()?;
        Ok(Round2 { action, result })
    }
}

fn response_shape_for_round_result(action: &Shape, result: RoundResult) -> Shape {
    if result == RoundResult::Draw {
        return action.clone();
    }

    if result == RoundResult::Win {
        match action {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    } else {
        match action {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}
pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let Round2 { action, result } = line.parse::<Round2>().unwrap();
            let response = response_shape_for_round_result(&action, result);
            let round = Round1 { action, response };
            round.score()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        assert_eq!("15", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("12", process_part2(INPUT));
    }
}
