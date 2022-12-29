use std::{collections::BTreeSet};

use nom::{IResult, character::complete::{space0, one_of, line_ending, self}, multi::{many1, separated_list1}, sequence::preceded, branch::alt, Parser};

#[derive(Debug)]
struct Row {
    start: u16,
    end: u16,
    walls: BTreeSet<u16>,
}

#[derive(Debug)]
enum Movement {
    Go(u16),
    Left,
    Right,
}

fn row(input: &str) -> IResult<&str, Row> {
    let (input, spaces) = space0(input)?;
    let (input, maze) = many1(one_of(".#"))(input)?;

    let start = spaces.len() as u16;
    let end = maze.len() as u16 + start;
    let walls = maze.into_iter()
        .enumerate()
        .flat_map(|(index, ch)| match ch {
            '.' => None,
            '#' => Some(index as u16 + start),
            _ => unreachable!(),
        })
        .collect();
    let row = Row {
        start,
        end,
        walls,
    };

    Ok((input, row))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Row>, Vec<Movement>)> { // todo
    let (input, rows) = separated_list1(line_ending, row)(input)?;
    let (input, _) = preceded(space0, line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, movements) = many1(alt(
        (
            complete::u16.map(|n| Movement::Go(n)),
            one_of("LR").map(|c| if c == 'L' { Movement::Left } else { Movement::Right } ),
        )
    ))(input)?;

    Ok((input, (rows, movements)))
}

fn prepare(input: &str) -> (Vec<Row>, Vec<Row>, Vec<Movement>, u16) {
    let (_input, (rows, movements)) = parse_input(input).unwrap();
    let column_number = rows.iter()
        .map(|row| row.end)
        .max()
        .unwrap();
    let row_number = rows.len();
    let columns: Vec<Row> = (0..column_number).into_iter()
        .map(|index| {
            let start = rows.iter()
                .position(|row| row.start <= index && row.end > index)
                .unwrap();
            let end = rows[start..].iter()
                .position(|row| index < row.start || index >= row.end)
                .map(|pos| pos + start)
                .unwrap_or(row_number);
            let walls = (start..end).into_iter()
                .flat_map(|y| rows[y].walls.get(&index).map(|_| y as u16))
                .collect();

            Row {
                start: start as u16,
                end: end as u16,
                walls,
            }
        })
        .collect();

        let start_x = (rows[0].start..rows[0].end).into_iter()
            .position(|x| !rows[0].walls.contains(&x))
            .map(|x| x as u16 + rows[0].start)
            .unwrap();
        (rows, columns, movements, start_x)
}

fn do_the_walk(row: &Row, pos: &u16, forward: bool, steps: &u16) -> u16 {
    let mut pos = *pos;
    for _i in 0..*steps {
        let next_pos = if forward {
            if pos == row.end - 1 {
                row.start
            } else {
                pos + 1
            }
        } else {
            if pos == row.start {
                row.end - 1
            } else {
                pos - 1
            }
        };
        if row.walls.contains(&next_pos) {
            break;
        }
        pos = next_pos;
    }
    pos
}

pub fn process_part1(input: &str) -> String {
    let (rows, columns, movements, x) = prepare(input);
    let mut x = x;
    let mut y = 0;
    let mut heading = 0;

    for movement in movements.iter() {
        match movement {
            Movement::Go(steps) => {
                if heading % 2 == 0 {
                    // moving left-right
                    let row = &rows[y as usize];
                    let forward = heading == 0;
                    x = do_the_walk(row, &x, forward, steps);
                } else {
                    // moving up-down
                    let row = &columns[x as usize];
                    let forward = heading == 1;
                    y = do_the_walk(row, &y, forward, steps);
                }

            },
            Movement::Left => heading = (heading + 3) % 4,
            Movement::Right => heading = (heading + 1) % 4,
        }

    }

    dbg!(&x, &y, &heading);
    (1000 * (y + 1) + 4 * (x + 1) + heading).to_string()
}


pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_works() {
        assert_eq!("6032", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("5031", process_part2(INPUT));
    }
}
