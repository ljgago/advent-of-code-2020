// part_one.rs

pub fn generate_cube_by_rules(cube: &[Vec<Vec<u8>>]) -> Option<Vec<Vec<Vec<u8>>>> {
    let z_size = cube.len();
    let y_size = cube[0].len();
    let x_size = cube[0][0].len();
    let mut count_cube: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; x_size]; y_size]; z_size];

    for z in 0..z_size {
        for y in 0..y_size {
            for x in 0..x_size {
                for z_offset in -1..=1 as i32 {
                    for y_offset in -1..=1 as i32 {
                        for x_offset in -1..=1 as i32 {
                            if (z_offset != 0 || y_offset != 0 || x_offset != 0)
                                && (z as i32 + z_offset) >= 0
                                && (y as i32 + y_offset) >= 0
                                && (x as i32 + x_offset) >= 0
                                && (z as i32 + z_offset) < z_size as i32
                                && (y as i32 + y_offset) < y_size as i32
                                && (x as i32 + x_offset) < x_size as i32
                            {
                                count_cube[z][y][x] = match cube[(z as i32 + z_offset) as usize]
                                    [(y as i32 + y_offset) as usize]
                                    [(x as i32 + x_offset) as usize]
                                {
                                    1 => count_cube[z][y][x] + 1,
                                    _ => count_cube[z][y][x],
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // for z in count_cube.iter() {
    //     for y in z.iter() {
    //         println!("{:?}", y);
    //     }
    //     println!("\n");
    // }

    let mut new_cube: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; 2 + x_size]; 2 + y_size]; 2 + z_size];
    for z in 0..z_size {
        for y in 0..y_size {
            for x in 0..x_size {
                if cube[z][y][x] == 1 && (count_cube[z][y][x] == 2 || count_cube[z][y][x] == 3) {
                    new_cube[1 + z][1 + y][1 + x] = 1;
                } else if cube[z][y][x] == 0 && count_cube[z][y][x] == 3 {
                    new_cube[1 + z][1 + y][1 + x] = 1;
                } else {
                    new_cube[1 + z][1 + y][1 + x] = 0;
                }
            }
        }
    }

    // for z in new_cube.iter() {
    //     for y in z.iter() {
    //         println!("{:?}", y);
    //     }
    //     println!("\n");
    // }

    Some(new_cube)
}

pub fn active_cubes_after_sixth_cycles(plane: &[String]) -> Option<usize> {
    let size: usize = plane.len();

    let norm_plane: Vec<Vec<u8>> = plane
        .iter()
        .map(|y| y.chars().map(|x| if x == '#' { 1 } else { 0 }).collect())
        .collect();

    let mut cube: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; size + 2]; size + 2]; 1 + 2];

    for z in 1..(cube.len() - 1) {
        for y in 1..(cube[0].len() - 1) {
            for x in 1..(cube[0][0].len() - 1) {
                cube[z][y][x] = norm_plane[y - 1][x - 1]
            }
        }
    }

    for _ in 0..6 {
        cube = generate_cube_by_rules(&cube)?;
    }

    let mut count: usize = 0;
    for z in 0..cube.len() {
        for y in 0..cube[0].len() {
            for x in 0..cube[0][0].len() {
                if cube[z][y][x] == 1 {
                    count += 1;
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
    fn test_generate_cube_by_rules() {
        let cube: Vec<Vec<Vec<u8>>> = vec![
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0],
                vec![0, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ],
        ];

        let cube_1: Vec<Vec<Vec<u8>>> = vec![
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ],
        ];
        assert_eq!(Some(cube_1.clone()), generate_cube_by_rules(&cube));

        let cube_2: Vec<Vec<Vec<u8>>> = vec![
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        ];
        assert_eq!(Some(cube_2), generate_cube_by_rules(&cube_1));
    }

    #[test]
    fn test_active_cubes_after_sixth_cycles() {
        let plane = vec![
            ".#.".to_owned(),
            "..#".to_owned(),
            "###".to_owned()
        ];
        assert_eq!(Some(112), active_cubes_after_sixth_cycles(&plane));
    }
}
