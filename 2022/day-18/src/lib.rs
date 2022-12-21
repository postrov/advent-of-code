use std::collections::{BTreeSet, VecDeque};

use nom::{IResult, multi::separated_list1, character::complete::{self, line_ending}};

type Point = (i32, i32, i32);

fn point(input: &str) -> IResult<&str, Point> {
    let (input, coords) = separated_list1(nom::character::complete::char(','), complete::i32)(input)?;
    let p = (coords[0], coords[1], coords[2]);
    Ok((input, p))
}

fn points(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points) = separated_list1(line_ending, point)(input)?;
    Ok((input, points))
}

pub fn process_part1(input: &str) -> String {
    let (_input, points) = points(input).unwrap();
    let droplet: BTreeSet<Point> = points.iter()
        .cloned()
        .collect();

    points.iter()
        .copied()
        .map(|(x, y, z)| {
            let adj = [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ];
            
            let covered = adj.iter()
                .filter(|p| droplet.contains(p))
                .count();
            6 - covered
        })
        .sum::<usize>()
        .to_string()
}

fn surround_with_lava(droplet: &BTreeSet<Point>) -> BTreeSet<Point> {
    let (x_min, y_min, z_min) = droplet.iter()
        .copied()
        .reduce(|(x, y, z), (px, py, pz)| (x.min(px), y.min(py), z.min(pz)))
        .map(|(x, y, z)| (x - 1, y - 1, z - 1))
        .unwrap();

    let (x_max, y_max, z_max) = droplet.iter()
        .copied()
        .reduce(|(x, y, z), (px, py, pz)| (x.max(px), y.max(py), z.max(pz)))
        .map(|(x, y, z)| (x + 1, y + 1, z + 1))
        .unwrap();
    let mut res = BTreeSet::new();

    // BFS
    let mut q: VecDeque<Point> = VecDeque::new();
    q.push_back((x_min, y_min, z_min));
    while let Some(p) = q.pop_front() {
        if res.contains(&p) {
            continue;
        }
        let (x, y, z) = p;
        res.insert((x, y, z));
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
            .iter()
            .filter(|(x, y, z)| {
                *x >= x_min && *x <= x_max &&
                *y >= y_min && *y <= y_max &&
                *z >= z_min && *z <= z_max
            })
            .filter(|neighbor| !droplet.contains(neighbor))
            .for_each(|neighbor| q.push_back(*neighbor));
    }

    res
}

pub fn process_part2(input: &str) -> String {
    let (_input, points) = points(input).unwrap();
    let droplet: BTreeSet<Point> = points.iter()
        .cloned()
        .collect();
    
    let lava: BTreeSet<Point> = surround_with_lava(&droplet);

    points.iter()
        .copied()
        .map(|(x, y, z)| {
            let adj = [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ];

            let exposed = adj.iter()
                .filter(|p| lava.contains(p))
                .count();
            exposed
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_works() {
        assert_eq!("64", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("58", process_part2(INPUT));
    }
}
