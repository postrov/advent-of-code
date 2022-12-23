// this can be done easier/faster with just .remove(pos) ;)
fn shift_pos(v: &mut [(usize, i64)], pos: usize, size: i64) {
    let (_, num) = v[pos];
    let mut shift = num.rem_euclid(size - 1);
    if shift == 0 {
        return;
    }
    if (pos as i64) + shift >= size {
        shift -= size - 1;
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
        let to = pos;
        let from = ((pos as i64) + shift) as usize;
        let tmp_slice = &v[from..to];
        let mut tmp_v = Vec::with_capacity(tmp_slice.len());
        tmp_v.extend_from_slice(tmp_slice);
        v[(from + 1)..(to + 1)].clone_from_slice(&tmp_v);
        v[from] = tmp_el;
    }

}

fn get_coords_output(v: &[(usize, i64)]) -> String {
    let size = v.len();
    let pos0 = v.iter().position(|(_, num)| *num == 0).unwrap();

    [1000, 2000, 3000].iter()
        .map(|coord_pos| v[(coord_pos + pos0) % size].1)
        .inspect(|num| println!("coord: {}", num))
        .sum::<i64>()
        .to_string()
}

fn mix(v: &mut [(usize, i64)]) {
    let size = v.len() as i64;
    for i in 0..size {
        let pos = v.iter().position(|(p, _)| *p == i as usize).unwrap();
        shift_pos(v, pos, size);
    }
}

// expected answer for part1: 7713, actual 15744
pub fn process_part1(input: &str) -> String {
    let nums = input.split('\n')
        .filter_map(|l| l.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let mut v = nums.iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, i64)>>();

    mix(&mut v);

    get_coords_output(&v)
}

// expected answer for part2: 1664569352803
pub fn process_part2(input: &str) -> String {
    const ENCRYPTION_KEY: i64 = 811589153;
    let nums = input.split('\n')
        .filter_map(|l| l.parse::<i64>().ok())
        .map(|num| num * ENCRYPTION_KEY)
        .collect::<Vec<i64>>();

    let mut v = nums.iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    for _ in 0..10 {
        mix(&mut v);
    }
    get_coords_output(&v)
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
        assert_eq!("3", process_part1(INPUT));
    }

    fn prepare_test_vec<const N: usize>(v: &[i64; N]) -> Vec<(usize, i64)> {
        let test_vec = v.iter()
            .copied()
            .enumerate()
            .collect::<Vec<_>>();
        test_vec
    }

    #[test]
    fn shift_one_on_right_edge_works() {
        let mut v = prepare_test_vec(&[2, 3, 4, 1]);
        let size = v.len() as i64;
        shift_pos(&mut v, 3, size);
        let result = v.iter().map(|(_p, n)| *n).collect::<Vec<_>>();
        assert_eq!(vec![2, 1, 3, 4], result);
    }

    #[test]
    fn shift_two_on_right_edge_works() {
        let mut v = prepare_test_vec(&[1, 3, 4, 2]);
        let size = v.len() as i64;
        shift_pos(&mut v, 3, size);
        let result = v.iter().map(|(_p, n)| *n).collect::<Vec<_>>();
        assert_eq!(vec![1, 3, 2, 4], result);
    }

    #[test]
    fn shift_one_on_left_edge_works() {
        let mut v = prepare_test_vec(&[-1,  2, 3, 4]);
        let size = v.len() as i64;
        shift_pos(&mut v, 0, size);
        let result = v.iter().map(|(_p, n)| *n).collect::<Vec<_>>();
        assert_eq!(vec![2, 3, -1, 4], result);
    }

    #[test]
    fn shift_two_on_left_edge_works() {
        let mut v = prepare_test_vec(&[-2,  2, 3, 4]);
        let size = v.len() as i64;
        shift_pos(&mut v, 0, size);
        let result = v.iter().map(|(_p, n)| *n).collect::<Vec<_>>();
        assert_eq!(vec![2, -2, 3, 4], result);
    }

    #[test]
    fn part2_works() {
        assert_eq!("1623178306", process_part2(INPUT));
    }
}
