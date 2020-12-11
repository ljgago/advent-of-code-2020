// part_one.rs

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn num_break_xmas(data: &[usize], preamble: usize) -> Option<usize> {
    let mut low: usize = 0;
    let mut hight: usize = preamble;
    let mut index: usize = preamble;
    let mut found: bool = false;

    loop {
        if index > data.len() - 1 {
            return None;
        }

        for x in data[low..hight].iter() {
            for y in data[low..hight].iter() {
                if *x != *y {
                    if *x + *y == data[index] {
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return Some(data[index]);
        }

        low += 1;
        hight += 1;
        index += 1;
        found = false;
    }
}

// Optimized version
pub fn num_break_xmas_opt(data: &[usize], preamble: usize) -> Option<usize> {
    let mut low: usize = 0;
    let mut hight: usize = preamble;
    let mut index: usize = preamble;
    let mut found: bool = false;

    loop {
        if index > data.len() - 1 {
            return None;
        }

        let hash_set: HashSet<usize> = HashSet::from_iter(data[low..hight].iter().map(|x| {
            if data[index] > *x {
                data[index] - *x
            } else {
                0
            }
        }));

        for x in data[low..hight].iter() {
            if hash_set.contains(&x) {
                found = true;
                break;
            }
        }

        if !found {
            return Some(data[index]);
        }

        low += 1;
        hight += 1;
        index += 1;
        found = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_break_xmas() {
        let data: Vec<usize> = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];
        assert_eq!(Some(127), num_break_xmas(&data, 5));
        assert_eq!(Some(127), num_break_xmas_opt(&data, 5));
    }
}

// 2238,
// 2543,
// 1789,
// 1812,
// 1842,
// 1865,
// 2143,
// 2606,
// 4241,
// 3263,
// 2901,
// 5792,
// 3563,
// 4563,
// 4268,
// 3069,
// 6464,
// 2974,
// 3084,
// 3120,
// 3199,
// 3309,
// 5041,
// 3360,
// 3487,
// 6571,
