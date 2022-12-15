
use std::{collections::BTreeSet, ops::{Range}};

use nom::{IResult, multi::separated_list1, character::complete::{line_ending, self}, sequence::{preceded, separated_pair},  bytes::complete::tag, Parser};

#[derive(Debug)]
struct Pos(i64, i64);

#[derive(Debug)]
struct Reading {
    sensor: Pos,
    beacon: Pos,
    distance: i64,
}

pub fn merge_ranges_in_place<T: Ord + Copy>(ranges: &mut Vec<Range<T>>) {
    ranges.sort_by_key(|r| r.start);
    let mut i = 0;
    while i < ranges.len() - 1 {
        let r1 = &ranges[i];
        let r2 = &ranges[i + 1];
        if r1.end >= r2.start {
            ranges[i] = r1.start..r1.end.max(r2.end);
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}


fn pos(input: &str) -> IResult<&str, Pos> {
    let (input, pos) = 
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64)
    )(input)?;
  
    Ok((input, Pos(pos.0, pos.1)))
}

fn distance(pos1: &Pos, pos2: &Pos) -> i64 {
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

fn no_beacon_ranges(readings: &[Reading], target_row: i64) -> Vec<Range<i64>> {
    let mut no_beacon_ranges = readings.iter()
        .filter_map(|reading| {
            let y_dist = (reading.sensor.1 - target_row).abs();
            let x_span = reading.distance - y_dist;
            if x_span >= 0 {
                let x = reading.sensor.0;
                Some((x - x_span)..(x + x_span + 1))
            } else {
                None
            }
        }).collect::<Vec<Range<i64>>>();

    merge_ranges_in_place(&mut no_beacon_ranges);
    no_beacon_ranges
}

fn part1(input: &str, target_row: i64) -> String {
    let (_input, readings) = readings(input).unwrap();

    let no_beacon_ranges = no_beacon_ranges(&readings, target_row);

    let beacons_on_row: BTreeSet<i64> = readings.iter()
        .map(|reading| &reading.beacon)
        .filter(|beacon| beacon.1 == target_row)
        .map(|beacon| beacon.0)
        .collect();

    let no_beacons_count = no_beacon_ranges.iter()
        .fold(0, |acc, range| acc + range.end - range.start);


    let beacons_in_range = beacons_on_row.iter()
        .filter(|x| no_beacon_ranges.iter().any(|range| range.contains(x)))
        .count() as i64;

    (no_beacons_count - beacons_in_range)
        .to_string()
}

pub fn process_part1(input: &str) -> String {
    const TARGET_ROW: i64 = 2_000_000;
    part1(input, TARGET_ROW)
}

fn part2(input: &str, coord_upper_bound: i64) -> String {
    let (_input, readings) = readings(input).unwrap();
    let test_range = 0..=coord_upper_bound;
    // todo: could run below in parallel
    for y in test_range.clone() {
        let no_beacon_ranges = no_beacon_ranges(&readings, y);
        let found = no_beacon_ranges.iter()
            .flat_map(|r| [r.start - 1, r.end])
            .find(|x| test_range.contains(x));

        if let Some(x) = found {
            return (x * 4_000_000 + y).to_string();
        }
    }

    panic!("not found");
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
