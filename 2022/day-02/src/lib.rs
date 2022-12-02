use phf::phf_map;

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
    Loss,
}

struct Round {
    action: Shape,
    response: Shape,
}

const SHAPE_CODES: phf::Map<char, Shape> = phf_map! {
    'A' => Shape::Rock,
    'B' => Shape::Paper,
    'C' => Shape::Scissors,
};

const RESPONSE_CODES: phf::Map<char, Shape> = phf_map! {
    'X' => Shape::Rock,
    'Y' => Shape::Paper,
    'Z' => Shape::Scissors,
};

const TASK_CODES: phf::Map<char, RoundResult> = phf_map! {
    'X' => RoundResult::Loss,
    'Y' => RoundResult::Draw,
    'Z' => RoundResult::Win,
};

impl Round {
    fn parse(code: &str) -> Result<Round, &str> {
        if code.len() != 3 {
            return Err("Invalid line length");
        }
        let action_code = code.chars().next().ok_or("no action code")?;
        let response_code = code.chars().nth(2).ok_or("no response code")?;
        let action = SHAPE_CODES.get(&action_code).cloned().ok_or("invalid action code")?;
        let response = RESPONSE_CODES.get(&response_code).cloned().ok_or("invalid response code")?;
        Ok(Round {
            action,
            response,
        })
    }

    fn score(&self) -> u32 {
        let response_score = match self.response {
            Shape::Rock => 1,
            Shape::Paper => 2, 
            Shape::Scissors => 3
        };
        let result_score = match self.result() {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };
        response_score + result_score
    }

    fn result(&self) -> RoundResult {
        let Round {action, response} = self;
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
        }
        else {
            RoundResult::Loss
        }
    }
}

pub fn process_part1(input: &str) -> String {
    input.lines()
        .map(|line| Round::parse(line).unwrap())
        .map(|round| round.score())
        .sum::<u32>()
        .to_string()
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
    input.lines()
        .map(|line| {
            let action_code = line.chars().next().unwrap();
            let task_code = line.chars().nth(2).unwrap();
            let action = SHAPE_CODES.get(&action_code).cloned().unwrap();
            let task = TASK_CODES.get(&task_code).cloned().unwrap();
            let response = response_shape_for_round_result(&action, task);
            let round = Round { action, response };
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
