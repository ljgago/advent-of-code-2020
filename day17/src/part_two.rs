// part_two.rs

pub fn generate_hypercube(hypercube: &[Vec<Vec<Vec<u8>>>]) -> Option<Vec<Vec<Vec<Vec<u8>>>>> {
    let w_size = hypercube.len();
    let z_size = hypercube[0].len();
    let y_size = hypercube[0][0].len();
    let x_size = hypercube[0][0][0].len();
    let mut count_hypercube: Vec<Vec<Vec<Vec<u8>>>> =
        vec![vec![vec![vec![0; x_size]; y_size]; z_size]; w_size];

    for w in 0..w_size {
        for z in 0..z_size {
            for y in 0..y_size {
                for x in 0..x_size {
                    for w_offset in -1..=1 as i32 {
                        for z_offset in -1..=1 as i32 {
                            for y_offset in -1..=1 as i32 {
                                for x_offset in -1..=1 as i32 {
                                    if (w_offset != 0
                                        || z_offset != 0
                                        || y_offset != 0
                                        || x_offset != 0)
                                        && (w as i32 + w_offset) >= 0
                                        && (z as i32 + z_offset) >= 0
                                        && (y as i32 + y_offset) >= 0
                                        && (x as i32 + x_offset) >= 0
                                        && (w as i32 + w_offset) < w_size as i32
                                        && (z as i32 + z_offset) < z_size as i32
                                        && (y as i32 + y_offset) < y_size as i32
                                        && (x as i32 + x_offset) < x_size as i32
                                    {
                                        count_hypercube[w][z][y][x] = match hypercube
                                            [(w as i32 + w_offset) as usize]
                                            [(z as i32 + z_offset) as usize]
                                            [(y as i32 + y_offset) as usize]
                                            [(x as i32 + x_offset) as usize]
                                        {
                                            1 => count_hypercube[w][z][y][x] + 1,
                                            _ => count_hypercube[w][z][y][x],
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut new_hypercube: Vec<Vec<Vec<Vec<u8>>>> =
        vec![vec![vec![vec![0; 2 + x_size]; 2 + y_size]; 2 + z_size]; 2 + w_size];
    for w in 0..w_size {
        for z in 0..z_size {
            for y in 0..y_size {
                for x in 0..x_size {
                    if hypercube[w][z][y][x] == 1
                        && (count_hypercube[w][z][y][x] == 2 || count_hypercube[w][z][y][x] == 3)
                    {
                        new_hypercube[1 + w][1 + z][1 + y][1 + x] = 1;
                    } else if hypercube[w][z][y][x] == 0 && count_hypercube[w][z][y][x] == 3 {
                        new_hypercube[1 + w][1 + z][1 + y][1 + x] = 1;
                    } else {
                        new_hypercube[1 + w][1 + z][1 + y][1 + x] = 0;
                    }
                }
            }
        }
    }

    Some(new_hypercube)
}

pub fn active_cubes_after_sixth_cycles(plane: &[String]) -> Option<usize> {
    let size: usize = plane.len();

    let norm_plane: Vec<Vec<u8>> = plane
        .iter()
        .map(|y| y.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        .collect();

    let mut hypercube: Vec<Vec<Vec<Vec<u8>>>> =
        vec![vec![vec![vec![0; size + 2]; size + 2]; 1 + 2]; 1 + 2];

    for w in 1..(hypercube.len() - 1) {
        for z in 1..(hypercube[0].len() - 1) {
            for y in 1..(hypercube[0][0].len() - 1) {
                for x in 1..(hypercube[0][0][0].len() - 1) {
                    hypercube[w][z][y][x] = norm_plane[y - 1][x - 1]
                }
            }
        }
    }

    for _ in 0..6 {
        hypercube = generate_hypercube(&hypercube)?;
    }

    let mut count: usize = 0;
    for w in 0..hypercube.len() {
        for z in 0..hypercube[0].len() {
            for y in 0..hypercube[0][0].len() {
                for x in 0..hypercube[0][0][0].len() {
                    if hypercube[w][z][y][x] == 1 {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_active_cubes_after_sixth_cycles() {
        let plane = vec![
            ".#.".to_owned(),
            "..#".to_owned(),
            "###".to_owned()
        ];
        assert_eq!(Some(848), active_cubes_after_sixth_cycles(&plane));
    }
}
