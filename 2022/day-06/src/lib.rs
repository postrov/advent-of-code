mod cyclic_buffer;

use std::collections::HashSet;
use crate::cyclic_buffer::CyclicBuffer;

// todo: this is sufficient, but quite naive implementation. By holding previous n-1 information,
// this check could be done more efficiently
fn is_unique<const N: usize>(window: &CyclicBuffer<char, N>) -> bool {
    let unique_signals = window.into_iter()
        .copied()
        .collect::<HashSet<char>>();
     
    unique_signals.len() == N
}

fn get_start_signal_position<const N: usize>(input: &str) -> usize {
    let mut chars = input.chars();
    let mut init: [char; N] = ['_'; N];

    (0..N).for_each(|i| init[i] = chars.next().expect("input too short"));
    let mut window = CyclicBuffer::<char, N>::new(init);
    if is_unique(&window) {
        return N;
    }

    for (pos, c) in chars.enumerate() {
        window.push(c);
        if is_unique(&window) {
            return pos + N + 1;
        };
    }
    0 
}

pub fn process_part1(input: &str) -> String {
    let pos = get_start_signal_position::<4>(input);
    pos.to_string()
}

pub fn process_part2(input: &str) -> String {
    // let pos = get_start_signal_position::<14>(input);
    // pos.to_string()
    let window_size = 14;
    let chars = input.chars().collect::<Vec<char>>();

    let indexed_window = chars.windows(window_size)
        .enumerate()
        .find(|(_i, win)| {
            let s = win.iter().collect::<HashSet<&char>>();
            s.len() == window_size
        })
        .unwrap();
    (indexed_window.0 + window_size).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA1: [(&str, u32);4] = [
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn part1_works() {
        for (input, expected_output) in TEST_DATA1 {
            assert_eq!(expected_output.to_string(), process_part1(input));
        }
    }

    const TEST_DATA2: [(&str, u32);5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];
    #[test]
    fn part2_works() {
        for (input, expected_output) in TEST_DATA2 {
            assert_eq!(expected_output.to_string(), process_part2(input));
        }
    }
}
