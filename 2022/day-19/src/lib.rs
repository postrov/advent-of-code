use std::ops::{Add, Mul};

use nom::{IResult, sequence::{preceded, terminated, delimited}, bytes::complete::tag, character::complete::{multispace0, self, line_ending}, multi::separated_list1 };

// implementation heavily influenced by: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day19/main.rs

#[derive(Debug, Clone, Copy, Default)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Resources {
    fn checked_sub(self, other: Resources) -> Option<Self> {
        Some(
            Self {
                ore: self.ore.checked_sub(other.ore)?,
                clay: self.clay.checked_sub(other.clay)?,
                obsidian: self.obsidian.checked_sub(other.obsidian)?,
            }
        )
    }
}

type Cost = Resources;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
}


#[derive(Debug, Clone, Copy)]
struct State {
    time_left: i32,
    geodes: i32,
    resources_rate: Resources,
    resources: Resources,
}

const ONE_ORE: Cost = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
};

const ONE_CLAY: Cost = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
};

const ONE_OBSIDIAN: Cost = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
};

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl Mul<usize> for Resources {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
        }
    }
}

impl State {
    fn upper_bound(&self, blueprint: &Blueprint) -> i32 {
        // assume ore and clay are unlimited, just check obsidian rate
        let geode_cost = blueprint.geode_cost.obsidian;
        let (_, _, predicted_geodes) = (0..self.time_left).rev()
            .fold(
                (
                    self.resources.obsidian,
                    self.resources_rate.obsidian,
                    self.geodes,
                ),
                |(obsidian, obsidian_rate, geodes), time_left| {
                    if obsidian >= geode_cost {
                        (
                            obsidian - geode_cost + obsidian_rate,
                            obsidian_rate,
                            geodes + time_left,
                        )
                    } else {
                        (
                            obsidian + obsidian_rate,
                            obsidian_rate + 1,
                            geodes,
                        )
                    }
                }
            );
        predicted_geodes
    }

    fn wait_and_build(self, cost: Cost, rate_change: Resources) -> Option<Self> {
        (1..self.time_left).rev().zip(0..).find_map(
            |(time_left, time_passed)| {
                let resources = self.resources + self.resources_rate * time_passed;
                resources.checked_sub(cost).map(|resources| {
                    Self {
                        time_left,
                        resources_rate: self.resources_rate + rate_change,
                        resources: resources + self.resources_rate,
                        ..self
                    }
                })
            })
    }

    fn branch(&self, blueprint: &Blueprint) -> Vec<Self> {
        // heuristics:
        // want to build ore? (only if ore rate < each robot cost, can only build one per round)
        // want to build clay? (clay is only needed for obsidian bots, no point making more than obsidian clay cost / round)
        // want to build obsidian? (only if obsidian rate < geode obsidian cost, clay rate must be > 0)
        // always build geode (if obsidian rate > 0)
        //   map the above to: wait for resources & build, possibly giving None
        let max_ore_cost = [blueprint.ore_cost.ore, blueprint.clay_cost.ore, blueprint.obsidian_cost.ore, blueprint.geode_cost.ore]
            .into_iter()
            .max()
            .unwrap_or(0);

        let ore_bot_viable = self.resources_rate.ore < max_ore_cost;
        let clay_bot_viable = self.resources_rate.clay < blueprint.obsidian_cost.clay;
        let obsidian_bot_viable = self.resources_rate.clay > 0 && self.resources_rate.obsidian < blueprint.geode_cost.obsidian;
        let geode_bot_viable = self.resources_rate.obsidian > 0; // todo
        [
            ore_bot_viable.then(|| self.wait_and_build(blueprint.ore_cost, ONE_ORE)),
            clay_bot_viable.then(|| self.wait_and_build(blueprint.clay_cost, ONE_CLAY)),
            obsidian_bot_viable.then(|| self.wait_and_build(blueprint.obsidian_cost, ONE_OBSIDIAN)),
            geode_bot_viable.then(||
                self.wait_and_build(blueprint.geode_cost, Resources::default())
                .map(|s| Self {
                        geodes: s.geodes + s.time_left,
                        ..s
                    }
                )
            ),
        ].into_iter()
        .rev()
        .flatten()
        .flatten()
        .collect()
    }
}


fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, u) = complete::u32(input)?;
    Ok((input, u as usize))
}

fn obsidian_cost(input: &str) -> IResult<&str, Resources> {
    let (input, ore) = preceded(multispace0,
        delimited(tag("Each obsidian robot costs "), parse_usize, tag(" ore "))
    )(input)?;
    let (input, clay) = delimited(tag("and "), parse_usize, tag(" clay."))(input)?;
    Ok((input, ONE_ORE * ore + ONE_CLAY * clay))
}

fn geode_cost(input: &str) -> IResult<&str, Resources> {
    let (input, ore) = preceded(multispace0,
        delimited(tag("Each geode robot costs "), parse_usize, tag(" ore "))
    )(input)?;
    let (input, obsidian) = delimited(tag("and "), parse_usize, tag(" obsidian."))(input)?;
    Ok((input, ONE_ORE * ore + ONE_OBSIDIAN * obsidian))
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
        ore_cost: ONE_ORE * ore,
        clay_cost: ONE_ORE * clay,
        obsidian_cost: obsidian,
        geode_cost: geode,
    };
    Ok((input, blueprint))
}

fn blueprint_max_geodes(blueprint: &Blueprint, time: i32) -> i32 {
    let initial_state = State {
        time_left: time,
        geodes: 0,
        resources_rate: ONE_ORE,
        resources: Resources::default(),
    };

    let mut best = 0;
    let mut queue = vec![initial_state];

    while let Some(state) = queue.pop() {
        best = best.max(state.geodes);

        state.branch(blueprint).iter()
            .for_each(|branch| {
                if branch.upper_bound(blueprint) > best {
                    queue.push(*branch);
                }
            });

    }
    best
}

pub fn process_part1(input: &str) -> String {
    let (_input, blueprints) = separated_list1(line_ending, blueprint)(input).unwrap();
    blueprints.iter()
        .map(|blueprint| blueprint_max_geodes(blueprint, 24) * blueprint.id as i32)
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, blueprints) = separated_list1(line_ending, blueprint)(input).unwrap();
    blueprints.iter()
        .take(3)
        .map(|blueprint| blueprint_max_geodes(blueprint, 32))
        .product::<i32>()
        .to_string()
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
        assert_eq!("33", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("3472", process_part2(INPUT));
    }
}
