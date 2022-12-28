use std::collections::BTreeSet;
#[allow(unused)]
use std::{collections::{BTreeMap, HashMap, VecDeque, HashSet}, fs::File, io::Write};

use nom::{IResult, sequence::{delimited, preceded, tuple}, character::complete::{self, alpha1, line_ending}, bytes::complete::tag, multi::separated_list1,  branch::alt};
#[allow(unused)]
use petgraph::{prelude::DiGraphMap, algo::dijkstra, dot::{Dot, Config}};
#[allow(unused)]
use itertools::Itertools;
#[allow(unused)]
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
type DistanceMap<'a> = HashMap<(Vertex<'a>, Vertex<'a>), i32>; 

fn build_graph<'a>(node_list: &'a [Node], nodes: &BTreeMap<&str, &Node>) -> DiGraphMap<(&'a str, u32), ()>{
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

    DiGraphMap::<(&str, u32), ()>::from_edges(&edges)
    // let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes()).unwrap();
}

fn calculate_distances<'a>(start_node: &Vertex<'a>, pos_vertices: &Vec<Vertex<'a>>, zero_vertices: &Vec<Vertex<'a>>, graph: &'a DiGraphMap<(&str, u32), ()>) -> DistanceMap<'a> {
    let mut pos_vertices = pos_vertices.clone();
    let sources = if start_node.1 > 0 {
        pos_vertices
    } else {
        let mut start_and_pos = vec![*start_node];
        start_and_pos.append(&mut pos_vertices);
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
        res.iter()
            .for_each(|(k, v)| {
                distances.insert((*s, *k), *v);
            });
    });
    distances
}

#[derive(Hash, Eq, PartialEq)]
struct State<'a> {
    remaining_time: i32,
    flow_rate: i32,
    relieved: i32,
    remaining_valves: Vec<Vertex<'a>>,
    current_valve: Vertex<'a>,
}

fn calculate_max_release<'a>(start_node: Vertex<'a>, pos_vertices: Vec<Vertex<'a>>, distances: &DistanceMap, remaining_time: i32) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        remaining_time,
        flow_rate: 0,
        relieved: 0,
        remaining_valves: pos_vertices,
        current_valve: start_node,
    });

    let mut relieved_max = 0;
    let mut seen_states = HashSet::new();
    while let Some(state) = queue.pop_front() {
        let relieved_total = state.remaining_time * state.flow_rate + state.relieved;
        relieved_max = relieved_max.max(relieved_total);

        for i in 0..state.remaining_valves.len() {
            let mut new_valves = state.remaining_valves.clone();
            let next_valve = new_valves.remove(i);
            let time_needed = distances.get(&(state.current_valve, next_valve)).unwrap() + 1;
            if time_needed < state.remaining_time {
                let next_state = State {
                    remaining_time: state.remaining_time - time_needed,
                    flow_rate: state.flow_rate + next_valve.1 as i32,
                    relieved: state.relieved + state.flow_rate * time_needed,
                    remaining_valves: new_valves,
                    current_valve: next_valve,
                };
                if !seen_states.contains(&next_state) {
                    queue.push_back(next_state);
                }
            } 
        }

        seen_states.insert(state);
    }
    relieved_max
}

pub fn process_part1(input: &str) -> String {
    let (_input, node_list) = separated_list1(line_ending, node)(input).unwrap();
    let nodes: BTreeMap<&str, &Node> = node_list.iter()
        .map(|n| (n.name, n))
        .collect();

    let start_node = nodes.get("AA")
        .map(|n| (n.name, n.rate))
        .unwrap();

    let graph = build_graph(&node_list, &nodes);
    let (pos_vertices, zero_vertices): (Vec<Vertex>, Vec<Vertex>) = node_list.iter()
        .map(|n| (n.name, n.rate))
        .partition(|n| n.1 > 0);
    let distances = calculate_distances(&start_node, &pos_vertices, &zero_vertices, &graph);


    calculate_max_release(start_node, pos_vertices, &distances, 30).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, node_list) = separated_list1(line_ending, node)(input).unwrap();
    let nodes: BTreeMap<&str, &Node> = node_list.iter()
        .map(|n| (n.name, n))
        .collect();

    let start_node = nodes.get("AA")
        .map(|n| (n.name, n.rate))
        .unwrap();

    let graph = build_graph(&node_list, &nodes);
    let (pos_vertices, zero_vertices): (Vec<Vertex>, Vec<Vertex>) = node_list.iter()
        .map(|n| (n.name, n.rate))
        .partition(|n| n.1 > 0);
    let distances = calculate_distances(&start_node, &pos_vertices, &zero_vertices, &graph);
   
    let valve_count = pos_vertices.len();
    let all_valves = BTreeSet::from_iter(pos_vertices.iter());
    let mut max_relieved = 0;
    for k in 2..=(valve_count / 2) {
        all_valves.iter().combinations(k)
            .for_each(|valves1| {
                let valves1: BTreeSet<&Vertex> = valves1.iter().copied().copied().collect();
                let valves2: BTreeSet<&Vertex> = all_valves.difference(&valves1).copied().collect();
                let relieved1 = calculate_max_release(start_node, valves1.iter().copied().copied().collect(), &distances, 26);
                let relieved2 = calculate_max_release(start_node, valves2.iter().copied().copied().collect(), &distances, 26);
                max_relieved = max_relieved.max(relieved1 + relieved2);
            });
    }
    
    max_relieved.to_string()
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
    fn part2_works() {
        assert_eq!("1707", process_part2(INPUT));
    }
}

