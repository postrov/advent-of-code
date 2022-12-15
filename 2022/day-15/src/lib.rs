use std::collections::BTreeSet;

use nom::{IResult, multi::separated_list1, character::complete::{line_ending, self}, sequence::{preceded, separated_pair},  bytes::complete::tag, Parser};

#[derive(Debug)]
struct Pos(i32, i32);

#[derive(Debug)]
struct Reading {
    sensor: Pos,
    beacon: Pos,
    distance: i32,
}

fn pos(input: &str) -> IResult<&str, Pos> {
    let (input, pos) = 
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32)
    )(input)?;
  
    Ok((input, Pos(pos.0, pos.1)))
}

fn distance(pos1: &Pos, pos2: &Pos) -> i32 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

fn readings(input: &str) -> IResult<&str, Vec<Reading>> {
    let (input, readings) =
    separated_list1(
        line_ending,
        separated_pair(
            preceded(tag("Sensor at "), pos),
            tag(": "),
            preceded(tag("closest beacon is at "), pos)
        ).map(|(pos1, pos2)| {
                let distance = distance(&pos1, &pos2);
                Reading {
                    sensor: pos1,
                    beacon: pos2,
                    distance
                }
            })
    )(input)?;
    Ok((input, readings))
}

fn part1(input: &str, target_row: i32) -> String {
    let (_input, readings) = readings(input).unwrap();

    let mut no_beacon_positions = readings.iter()
        .map(|reading| {
            let y_dist = (reading.sensor.1 - target_row).abs();
            let x_span = reading.distance - y_dist;
            if x_span >= 0 {
                let x = reading.sensor.0;
                (x - x_span)..=(x + x_span)
            } else {
                0..=-1
            }
        })
        .fold(BTreeSet::<i32>::new(), |mut s, range| {
            range.for_each(|x| {s.insert(x);} );
            s
        });

    readings.iter()
        .map(|reading| &reading.beacon)
        .filter(|beacon| beacon.1 == target_row)
        .for_each(|beacon| {
            no_beacon_positions.remove(&beacon.0);
        });

    no_beacon_positions
        .len()
        .to_string()
}

pub fn process_part1(input: &str) -> String {
    const TARGET_ROW: i32 = 2_000_000;
    part1(input, TARGET_ROW)
}

fn part2(input: &str, coord_upper_bound: i32) -> String {
    let (_input, readings) = readings(input).unwrap();
    for x in 0..=coord_upper_bound {
        for y in 0..=coord_upper_bound {
            let target = Pos(x, y);
            let res = readings.iter()
                .all(|reading| distance(&target, &reading.sensor) > reading.distance);
            if res {
                return (target.0 * 4_000_000 + target.1).to_string();
            }
        }
    }
    "dupa".into()
}

pub fn process_part2(input: &str) -> String {
    part2(input, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_works() {
        assert_eq!("26", part1(INPUT, 10));
    }

    #[test]
    fn part2_works() {
        assert_eq!("56000011", part2(INPUT, 20));
    }
}
