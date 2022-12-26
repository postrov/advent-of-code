use std::{collections::{BTreeMap, HashMap}, fs::File, io::Write};

use nom::{IResult, sequence::{delimited, preceded, tuple}, character::complete::{self, alpha1, line_ending}, bytes::complete::tag, multi::separated_list1,  branch::alt};
use petgraph::{prelude::DiGraphMap, algo::dijkstra, dot::{Dot, Config}};
use itertools::Itertools;
use rayon::prelude::ParallelIterator;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    rate: u32,
    tunnels: Vec<&'a str>,
}

fn node(input: &str) -> IResult<&str, Node> {
    let (input, name) = delimited(tag("Valve "), alpha1, tag(" has flow rate="))(input)?;
    let (input, rate) = complete::u32(input)?;
    let (input, tunnels) = preceded(
        tuple((
            tag("; "),
            alt((tag("tunnels lead"), tag("tunnel leads"))),
            alt((tag(" to valve "), tag(" to valves "))),
        )),
        separated_list1(tag(", "), alpha1)
    )(input)?;

    Ok((input,
        Node {
            name,
            rate,
            tunnels,
        }))
}

type Vertex<'a> = (&'a str, u32);
type DistanceMap<'a> = HashMap<Vertex<'a>, HashMap<Vertex<'a>, i32>>; 

fn get_move_time(distances: &DistanceMap, a: &Vertex, b: &Vertex) -> i32 {
    distances
        .get(a)
        .and_then(|distance_from_a| distance_from_a.get(b).copied())
        .unwrap()
}

pub fn process_part1(input: &str) -> String {
    let (_input, node_list) = separated_list1(line_ending, node)(input).unwrap();
    let nodes: BTreeMap<&str, &Node> = node_list.iter()
        .map(|n| (n.name, n))
        .collect();

    let (pos_vertices, zero_vertices): (Vec<Vertex>, Vec<Vertex>) = node_list.iter()
        .map(|n| (n.name, n.rate))
        .partition(|n| n.1 > 0);

    let start = nodes.get("AA")
        .map(|n| (n.name, n.rate))
        .unwrap();

    let edges: Vec<(Vertex, Vertex)> = node_list.iter()
        .flat_map(|n| {
            n.tunnels.iter()
                .map(|&m_name| {
                    let m = nodes.get(m_name).unwrap();
                    (
                        (n.name, n.rate),
                        (m_name, m.rate)
                    )
                })
        })
        .collect();

    let graph = DiGraphMap::<(&str, u32), ()>::from_edges(&edges);
    // let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes()).unwrap();

    let sources = if start.1 > 0 {
        pos_vertices.clone()
    } else {
        let mut start_and_pos = vec![start];
        start_and_pos.append(&mut pos_vertices.clone());
        start_and_pos
    };
    let mut distances: DistanceMap = HashMap::new();

    sources.iter().for_each(|s| {
        let mut res = dijkstra(
            &graph,
            *s,
            None,
            |_| 1,
        );

        zero_vertices.iter()
            .for_each(|v| {
                res.remove(v);
            });
        distances.insert(*s, res);
    });

    // let p = vec![&("DD", 20), &("BB", 13), &("JJ", 21), &("HH", 22), &("EE", 3), &("CC", 2)];
    pos_vertices.iter().permutations(pos_vertices.len()).map(|p| {
        let mut sum = 0;
        let mut total_rate = 0;
        let mut pi = p.iter();
        let mut cur_pos = start;
        let mut action: Option<(i32, Vertex)> = None; // action is move to action.1 and unlock valve
        for _t in 1..=30 {
            if action.is_none() {
                action = pi.next()
                    .map(|&v| {
                        let move_time = get_move_time(&distances, &cur_pos, v);
                        (move_time, *v)
                    });
            }

            // println!("=== Minute {}", t);
            // println!("Releasing {} pressure", total_rate);
            sum += total_rate;
            match action {
                Some((0, v)) => {
                    // println!("You open valve: {}", v.0);
                    cur_pos = v;
                    total_rate += v.1;
                    action = Some((-1, v));
                },
                Some((-1, _v)) => {
                    // println!("Valve {} is open", v.0);
                    // total_rate += v.1;
                    action = pi.next()
                        .map(|&v| {
                            let move_time = get_move_time(&distances, &cur_pos, v);
                            (move_time - 1, *v)
                        });
                    // action = None;
                },
                Some((move_time, v)) => {
                    // println!("Keep moving to valve {}, {} left", v.0, move_time);
                    action = Some((move_time - 1, v));
                },
                None => {
                }
            };
        }
        sum
    })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_works() {
        assert_eq!("1651", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
