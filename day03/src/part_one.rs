// part_one.rs

pub fn count_trajectory_trees(data: &[String], slope: &(usize, usize)) -> Option<usize> {
    let x_size: usize = data[0].len();
    let y_size: usize = data.len();

    let right: usize = slope.0;
    let down: usize = slope.1;

    let tree: char = '#';

    let mut x: usize = right;
    let mut y: usize = down;
    let mut count: usize = 0;

    loop {
        if y >= y_size {
            return Some(count);
        }

        if data[y].chars().nth(x)? == tree {
            count += 1
        }

        x = (x + right) % x_size;
        y += down;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_trajectory_trees() {
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

        assert_eq!(7, count_trajectory_trees(&data, &(3, 1)).unwrap());
    }
}
