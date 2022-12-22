
pub fn process_part1(input: &str) -> String {
    let nums = input.split("\n")
        // .inspect(|s| println!("s: {}", s))
        .filter_map(|l| l.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    
    // let (low, high) = nums.iter()
    //     .fold((i32::MAX, i32::MIN), |(l, h ), &n| (l.min(n), h.max(n)));
    // dbg!(low, high); // +/- 10k

    // let uniq: BTreeSet<i32> = BTreeSet::from_iter(nums.iter().copied());
    // dbg!(uniq.len()); // not unique
    
    // let v = vec![1, 2, -3, 4, 0, 3, -2];
    // let l = v.len();
    // let pos0 = 4;
    // dbg!(v[(pos0 + 1000) % l]);
    // dbg!(v[(pos0 + 2000) % l]);
    // dbg!(v[(pos0 + 3000) % l]); // 4, -3, 2 => correct
    // [1, 2, 3, 4] -> [1, 3, 4, 2] -> 
    // [1, 2, 3, 4] -> [1, 3, 2, 4] ->  
    // challenge 1: how to know which number is next after move?
    // challenge 2: efficiently shift numbers


    // observation1: moving by n (mod M) is the same as moving by (M - n) (mod M)
    let mut v = nums.iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, i32)>>();

    let size = v.len() as i32;
    for i in 0..size {
        // let &(pos, num) = v.iter().find(|(p, _)| *p == i as usize).unwrap();
        let pos = v.iter().position(|(p, _)| *p == i as usize).unwrap();
        let (_, num) = v[pos];
        let mut shift = match num {
            0 => 0,
            1.. => num % size,
            _ => size - (-num % size) - 1,
        };
        if shift == 0 {
            continue;
        }
        if (pos as i32) + shift >= size {
            shift -= size;
        }
        if shift > 0 {
            let tmp_el = v[pos];
            let from = pos + 1;
            let to = pos + 1 + (shift as usize);
            let tmp_slice = &v[from..to];
            let mut tmp_v = Vec::with_capacity(tmp_slice.len());
            tmp_v.extend_from_slice(tmp_slice);
            v[(from - 1)..(to - 1)].clone_from_slice(&tmp_v);
            v[to - 1] = tmp_el;
        } else {
            let tmp_el = v[pos];
            let to = pos - 1;
            let from = ((pos as i32) + shift - 1) as usize;
            let tmp_slice = &v[from..to];
            let mut tmp_v = Vec::with_capacity(tmp_slice.len());
            tmp_v.extend_from_slice(tmp_slice);
            v[(from + 1)..(to + 1)].clone_from_slice(&tmp_v);
            v[from] = tmp_el;
        }
        dbg!(v.iter().map(|x| x.1).collect::<Vec<_>>());
    }
    let pos0 = v.iter().position(|(_, num)| *num == 0).unwrap();

    [1000, 2000, 3000].iter()
        .map(|coord_pos| v[(coord_pos + pos0) % v.len()].1)
        .sum::<i32>()
        .to_string()

}

pub fn process_part2(input: &str) -> String {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_works() {
        assert_eq!("3x", process_part1(INPUT));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!("works", process_part2(INPUT));
    }
}
