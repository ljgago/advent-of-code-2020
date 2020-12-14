// part_two.rs

fn adjacent_check_first_see(x: usize, y: usize, layout: &[Vec<char>]) -> Option<char> {
    let x_size: usize = layout[0].len();
    let y_size: usize = layout.len();

    let mut count_occupied: usize = 0;

    if layout[y][x] == '.' {
        return Some('.')
    }

    // 000
    // #00
    // 000
    let mut x_offset: usize = 1;
    let mut y_offset: usize = 1;
    while x > x_offset - 1 {
        match layout[y][x - x_offset] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => x_offset += 1,
            _ => break,
        }
    }

    // 000
    // 00#
    // 000
    x_offset = 1;
    // y_offset = 0;
    while x + x_offset < x_size {
        match layout[y][x + x_offset] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => x_offset += 1,
            _ => break,
        }
    }

    // 0#0
    // 000
    // 000
    // x_offset = 0;
    // y_offset = 0;
    while y > y_offset - 1 {
        match layout[y - y_offset][x] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => y_offset += 1,
            _ => break,
        }
    }

    // 000
    // 000
    // 0#0
    // x_offset = 0;
    y_offset = 1;
    while y + y_offset < y_size {
        match layout[y + y_offset][x] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => y_offset += 1,
            _ => break,
        }
    }

    // #00
    // 000
    // 000
    x_offset = 1;
    y_offset = 1;
    while (x > x_offset - 1) && (y > y_offset - 1) {
        match layout[y - y_offset][x - x_offset] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => {
                y_offset += 1;
                x_offset += 1;
            },
            _ => break,
        }
    }

    // 000
    // 000
    // 00#
    x_offset = 1;
    y_offset = 1;
    while (x + x_offset < x_size) && (y + y_offset < y_size) {
        match layout[y + y_offset][x + x_offset] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => {
                y_offset += 1;
                x_offset += 1;
            },
            _ => break,
        }
    }

    // 000
    // 000
    // #00
    x_offset = 1;
    y_offset = 1;
    while (x > x_offset - 1) && (y + y_offset < y_size) {
        match layout[y + y_offset][x - x_offset] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => {
                y_offset += 1;
                x_offset += 1;
            },
            _ => break,
        }
    }

    // 00#
    // 000
    // 000
    x_offset = 1;
    y_offset = 1;
    while (x + x_offset < x_size) && (y > y_offset - 1) {
        match layout[y - y_offset][x + x_offset] {
            '#' => {
                count_occupied += 1;
                break;
            }
            '.' => {
                y_offset += 1;
                x_offset += 1;
            },
            _ => break,
        }
    }

    let mut target: char = layout[y][x];
    if layout[y][x] == 'L' && count_occupied == 0 {
        target = '#';
    }
    if layout[y][x] == '#' && count_occupied >= 5 {
        target = 'L';
    }

    Some(target)
}

fn generate_layout(orig_layout: &[String]) -> Option<Vec<String>> {
    let mut layout: Vec<Vec<char>> = Vec::new();
    for x in orig_layout.iter() {
        layout.push(x.chars().collect())
    }

    let mut new_layout: Vec<Vec<char>> = layout.clone();

    for (y, values) in layout.iter().enumerate() {
        for (x, _) in values.iter().enumerate() {
            new_layout[y][x] = adjacent_check_first_see(x, y, &layout)?;
        }
    }

    let mut return_layout: Vec<String> = Vec::new();
    for x in new_layout.iter() {
        return_layout.push(x.iter().collect());
    }

    Some(return_layout)
}

pub fn count_stabilized_seats(layout: &[String]) -> Option<usize> {
    let mut old_layout = layout.to_vec();

    loop {
        let new_layout: Vec<String> = generate_layout(&old_layout)?;
        if old_layout == new_layout {
            break;
        }
        old_layout = new_layout.clone();
    }

    let mut count: usize = 0;

    for y in old_layout {
        count += y.matches("#").count();
    }

    Some(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_layout() {
        let layout1: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout2: Vec<String> = vec![
            "#.##.##.##",
            "#######.##",
            "#.#.#..#..",
            "####.##.##",
            "#.##.##.##",
            "#.#####.##",
            "..#.#.....",
            "##########",
            "#.######.#",
            "#.#####.##",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout3: Vec<String> = vec![
            "#.LL.LL.L#",
            "#LLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLL#",
            "#.LLLLLL.L",
            "#.LLLLL.L#",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout4: Vec<String> = vec![
            "#.L#.##.L#",
            "#L#####.LL",
            "L.#.#..#..",
            "##L#.##.##",
            "#.##.#L.##",
            "#.#####.#L",
            "..#.#.....",
            "LLL####LL#",
            "#.L#####.L",
            "#.L####.L#",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout5: Vec<String> = vec![
            "#.L#.L#.L#",
            "#LLLLLL.LL",
            "L.L.L..#..",
            "##LL.LL.L#",
            "L.LL.LL.L#",
            "#.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLL#",
            "#.LLLLL#.L",
            "#.L#LL#.L#",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout6: Vec<String> = vec![
            "#.L#.L#.L#",
            "#LLLLLL.LL",
            "L.L.L..#..",
            "##L#.#L.L#",
            "L.L#.#L.L#",
            "#.L####.LL",
            "..#.#.....",
            "LLL###LLL#",
            "#.LLLLL#.L",
            "#.L#LL#.L#",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout7: Vec<String> = vec![
            "#.L#.L#.L#",
            "#LLLLLL.LL",
            "L.L.L..#..",
            "##L#.#L.L#",
            "L.L#.LL.L#",
            "#.LLLL#.LL",
            "..#.L.....",
            "LLL###LLL#",
            "#.LLLLL#.L",
            "#.L#LL#.L#",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        assert_eq!(Some(layout2.clone()), generate_layout(&layout1));
        assert_eq!(Some(layout3.clone()), generate_layout(&layout2));
        assert_eq!(Some(layout4.clone()), generate_layout(&layout3));
        assert_eq!(Some(layout5.clone()), generate_layout(&layout4));
        assert_eq!(Some(layout6.clone()), generate_layout(&layout5));
        assert_eq!(Some(layout7.clone()), generate_layout(&layout6));
    }

    #[test]
    fn test_count_stabilized_seats() {
        let layout: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        assert_eq!(Some(26), count_stabilized_seats(&layout));
    }
}
