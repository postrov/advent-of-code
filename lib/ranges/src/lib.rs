use std::ops::Range;

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

pub fn merge_ranges_copied<T: Ord + Copy>(ranges: &[Range<T>]) -> Vec<Range<T>> {
    let mut copied = ranges.to_vec();
    // copied.sort_by_key(|r| r.start);
    merge_ranges_in_place(&mut copied);
    
    copied
}

pub fn merge_sorted_ranges<T: Ord + Copy>(ranges: &[Range<T>]) -> Vec<Range<T>> {
    let (mut result, last_range_maybe) = ranges.iter()
        .fold((Vec::new(), None::<Range<T>>), |(mut v, current_range_maybe), range| {
            if let Some(current_range) = current_range_maybe {
                if current_range.end >= range.start {
                    (v, Some(current_range.start..current_range.end.max(range.end)))
                } else {
                    v.push(current_range);
                    (v, Some(range.clone()))
                }
            } else {
                (v, Some(range.clone()))
            }
        });
    if let Some(last_range) = last_range_maybe {
        result.push(last_range);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn overlapping_ranges_merged_in_place() {
    //     let mut v = vec![1..5, 2..4, 8..12, 3..6];
    //     merge_ranges_in_place(&mut v);
    //     assert_eq!(vec![1..6, 8..12], v);
    // }

    // #[test]
    // fn adjacent_ranges_merged_in_place() {
    //     let mut v = vec![1..5, 8..12, 5..8];
    //     merge_ranges_in_place(&mut v);
    //     assert_eq!(vec![1..12], v);
    // }

    // #[test]
    // fn overlapping_ranges_merged_copied() {
    //     let v = vec![1..5, 2..4, 8..12, 3..6];
    //     let merged_ranges = merge_ranges_copied(&v);
    //     assert_eq!(vec![1..6, 8..12], merged_ranges);
    // }

    // #[test]
    // fn adjacent_ranges_merged_copied() {
    //     let v = vec![1..5, 8..12, 5..8];
    //     let merged_ranges = merge_ranges_copied(&v);
    //     assert_eq!(vec![1..12], merged_ranges);
    // }

    // #[test]
    // fn overlapping_ranges_merged_fold_sorted() {
    //     let mut v = vec![1..5, 2..4, 8..12, 3..6];
    //     v.sort_by_key(|r| r.start);
    //     let merged_ranges = merge_sorted_ranges(&v);
    //     assert_eq!(vec![1..6, 8..12], merged_ranges);
    // }

    // #[test]
    // fn adjacent_ranges_merged_fold_sorted() {
    //     let mut v = vec![1..5, 8..12, 5..8];
    //     v.sort_by_key(|r| r.start);
    //     let merged_ranges = merge_sorted_ranges(&v);
    //     assert_eq!(vec![1..12], merged_ranges);
    // }

    #[test]
    fn adjacent_ranges_merged_fold_sorted() {
        let mut v = vec![1..5, 8..12, 5..8];
        v.sort_by_key(|r| r.start);
        for _i in 0..5_000_000 {
            // let merged_ranges = merge_sorted_ranges(&v);
            let merged_ranges = merge_ranges_copied(&v);
            assert_eq!(vec![1..12], merged_ranges);
        }
    }
}
