// part_two.rs

// NOTE:
//
// This challeger can be resolf using the
//
// `combinatorics(n, r) + 1`
//
// n: number of 1-jolt consecutive elements >= 2
// r: set size
//
// Example:
//
// (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//
// Calculate the 1-jolt and 3-jolt
//
// 1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3
//      |       |   |    |
//      *---3---*   *-2--*
//
// 8 = (combinatorics(3, 2) + 1) * (combinatorics(2, 2) + 1)
//                    ^                           ^

fn factorial(num: usize) -> Option<usize> {
    let mut result: usize = 1;

    if num == 0 {
        return Some(1);
    }

    for n in 1..=num {
        result *= n
    }
    Some(result)
}

// Combinatorics
fn combinatorics(n: usize, r: usize) -> Option<usize> {
    if n < r {
        return Some(1);
    }
    let result: usize = factorial(n)? / (factorial(r)? * factorial(n - r)?);

    Some(result)
}

pub fn distinct_ways_arrange(adapters: &[usize]) -> Option<usize> {
    let mut sorted_adapters = adapters.into_iter().collect::<Vec<_>>();
    sorted_adapters.sort();

    // create a vector of 1-jolt and 3-jolt
    let secuences: (Vec<usize>, usize) =
        sorted_adapters
            .into_iter()
            .fold((Vec::new(), 0), |acc, &jolt| {
                let mut vec: Vec<usize> = acc.0;
                vec.push(jolt - acc.1);
                (vec, jolt)
            });

    // transform the vector in a String
    let str_sec = secuences
        .0
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();

    // split the 3-jolt, y need only the 1-jolt
    let result: usize =
        str_sec
            .split("3")
            .collect::<Vec<&str>>()
            .iter()
            .fold(1, |acc, &jolt_diff_ones| {
                let count = jolt_diff_ones.chars().count();
                match count {
                    0 => acc, // ignore count = 0
                    1 => acc, // ignore count = 1
                    n => acc * (1 + combinatorics(n, 2).unwrap()),
                }
            });
    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(Some(24), factorial(4));
        assert_eq!(Some(120), factorial(5));
        assert_eq!(Some(5040), factorial(7));
    }

    #[test]
    fn test_comb() {
        assert_eq!(Some(3), combinatorics(3, 2));
        assert_eq!(Some(4), combinatorics(4, 3));
        assert_eq!(Some(10), combinatorics(5, 2));
    }

    #[test]
    fn test_distinct_ways_arrange() {
        let adapters1: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let adapters2: Vec<usize> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(Some(8), distinct_ways_arrange(&adapters1));
        assert_eq!(Some(19208), distinct_ways_arrange(&adapters2));
    }
}
