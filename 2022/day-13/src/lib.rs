use std::cmp::Ordering;

use nom::{
    branch::alt,
    character::complete::{self, char, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), Self::Int(r0)) => l0 == &vec![Packet::Int(*r0)],
            (Self::Int(l0), Self::List(r0)) => &vec![Packet::Int(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(list), Packet::List(other_list)) => list.cmp(other_list),
            (Packet::List(list), Packet::Int(other_num)) => {
                list.cmp(&vec![Packet::Int(*other_num)])
            }
            (Packet::Int(num), Packet::List(other_list)) => vec![Packet::Int(*num)].cmp(other_list),
            (Packet::Int(num), Packet::Int(other_num)) => num.cmp(other_num),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let (input, pairs) = separated_list1(
        pair(newline, newline),
        separated_pair(packet, newline, packet),
    )(input)?;

    Ok((input, pairs))
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, p) = alt((
        delimited(char('['), separated_list0(char(','), packet), char(']')).map(Packet::List),
        complete::u32.map(Packet::Int),
    ))(input)?;

    Ok((input, p))
}

pub fn process_part1(input: &str) -> String {
    let (_input, packet_pairs) = parse_input(input).expect("Input parse failed");
    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (left, right))| if left < right { Some(idx + 1) } else { None })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let div1: Packet = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2: Packet = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let (_input, packet_pairs) = parse_input(input).expect("Input parse failed");
    let mut packets = packet_pairs
        .iter()
        .flat_map(|(left, right)| [left, right])
        .chain([&div1, &div2])
        .collect::<Vec<&Packet>>();
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter(|(_idx, p)| [&div1, &div2].contains(p))
        .map(|(idx, _)| idx + 1)
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        assert_eq!("13", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("140", process_part2(INPUT));
    }
}
