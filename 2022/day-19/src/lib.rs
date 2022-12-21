use nom::{IResult, sequence::{preceded, terminated, delimited}, bytes::complete::tag, character::complete::{multispace0, self, line_ending}, Parser, multi::separated_list1 };

#[derive(Debug)]
struct ObsidianCost {
    ore: usize,
    clay: usize,
}

#[derive(Debug)]
struct GeodeCost {
    ore: usize,
    obsidian: usize,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: ObsidianCost,
    geode: GeodeCost,
}

struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, u) = complete::u32(input)?;
    Ok((input, u as usize))
}

fn obsidian_cost(input: &str) -> IResult<&str, ObsidianCost> {
    let (input, ore) = preceded(multispace0,
        delimited(tag("Each obsidian robot costs "), parse_usize, tag(" ore "))
    )(input)?;
    let (input, clay) = delimited(tag("and "), parse_usize, tag(" clay."))(input)?;
    Ok((input, ObsidianCost {
        ore,
        clay,
    }))
}

fn geode_cost(input: &str) -> IResult<&str, GeodeCost> {
    let (input, ore) = preceded(multispace0,
        delimited(tag("Each geode robot costs "), parse_usize, tag(" ore "))
    )(input)?;
    let (input, obsidian) = delimited(tag("and "), parse_usize, tag(" obsidian."))(input)?;
    Ok((input, GeodeCost {
        ore,
        obsidian,
    }))
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = preceded(multispace0, tag("Blueprint "))(input)?;
    let (input, id) = terminated(parse_usize, tag(":"))(input)?;
    let (input, ore) = delimited(
            preceded(multispace0, tag("Each ore robot costs ")),
            parse_usize,
        tag(" ore."))(input)?;
    let (input, clay) = delimited(
            preceded(multispace0, tag("Each clay robot costs ")),
            parse_usize,
        tag(" ore."))(input)?;
    let (input, obsidian) = obsidian_cost(input)?;
    let (input, geode) = geode_cost(input)?;

   
    let blueprint = Blueprint {
        id,
        ore,
        clay,
        obsidian,
        geode,
    };
    Ok((input, blueprint))
}

pub fn process_part1(input: &str) -> String {
    let (input, blueprints) = separated_list1(line_ending, blueprint)(input).unwrap();
    dbg!(&blueprints);
    input.into()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_works() {
        assert_eq!("works", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
