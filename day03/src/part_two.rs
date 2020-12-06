// part_two.rs

use crate::part_one::count_trajectory_trees;

pub fn multiply_trajectory_trees(data: &[String], slopes: &[(usize, usize)]) -> Option<usize> {
    let mut multiply: usize = 1;

    for slope in slopes.iter() {
        multiply *= count_trajectory_trees(&data, slope)?;
    }

    Some(multiply)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply_trajectory_trees() {
        let data: Vec<String> = vec![
            "..##.......".to_owned(),
            "#...#...#..".to_owned(),
            ".#....#..#.".to_owned(),
            "..#.#...#.#".to_owned(),
            ".#...##..#.".to_owned(),
            "..#.##.....".to_owned(),
            ".#.#.#....#".to_owned(),
            ".#........#".to_owned(),
            "#.##...#...".to_owned(),
            "#...##....#".to_owned(),
            ".#..#...#.#".to_owned(),
        ];
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        assert_eq!(336, multiply_trajectory_trees(&data, &slopes).unwrap());
    }
}
