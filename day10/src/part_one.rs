// part_one.rs

pub fn jolt_differences_mul(adapters: &[usize]) -> Option<usize> {
    let mut sorted_adapters = adapters.into_iter().collect::<Vec<_>>();
    sorted_adapters.sort();

    // return (1-jolt-counts, 3-jold-counts, previous-jold)
    let count: (usize, usize, usize) =
        sorted_adapters
            .into_iter()
            .fold((0, 0, 0), |acc, &jolt| {
                  match jolt - acc.2 {
                      1 => (acc.0 + 1, acc.1, jolt),
                      2 => (acc.0, acc.1, jolt),
                      3 => (acc.0, acc.1 + 1, jolt),
                      _ => (0, 0, 0),
                  }
            });
    Some(count.0 * (count.1 + 1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let adapters1: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let adapters2: Vec<usize> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(Some(35), jolt_differences_mul(&adapters1));
        assert_eq!(Some(220), jolt_differences_mul(&adapters2));
    }
}
