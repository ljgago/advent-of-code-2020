// part_one.rs

fn adjacent_check(x: usize, y: usize, layout: &[Vec<char>]) -> Option<char> {
    let x_size: usize = layout[0].len();
    let y_size: usize = layout.len();

    let mut count_occupied: usize = 0;
    // 000
    // #00
    // 000
    if x > 0 {
        if layout[y][x - 1] == '#' {
            count_occupied += 1;
        }
    }
    // 000
    // 00#
    // 000
    if x + 1 < x_size {
        if layout[y][x + 1] == '#' {
            count_occupied += 1;
        }
    }
    // 0#0
    // 000
    // 000
    if y > 0 {
        if layout[y - 1][x] == '#' {
            count_occupied += 1;
        }
    }
    // 000
    // 000
    // 0#0
    if y + 1 < y_size {
        if layout[y + 1][x] == '#' {
            count_occupied += 1;
        }
    }
    // #00
    // 000
    // 000
    if x > 0 && y > 0 {
        if layout[y - 1][x - 1] == '#' {
            count_occupied += 1;
        }
    }
    // 000
    // 000
    // 00#
    if x + 1 < x_size && y + 1 < y_size {
        if layout[y + 1][x + 1] == '#' {
            count_occupied += 1;
        }
    }
    // 000
    // 000
    // #00
    if x > 0 && y + 1 < y_size {
        if layout[y + 1][x - 1] == '#' {
            count_occupied += 1;
        }
    }
    // 00#
    // 000
    // 000
    if x + 1 < x_size && y > 0 {
        if layout[y - 1][x + 1] == '#' {
            count_occupied += 1;
        }
    }

    let mut target: char = layout[y][x];
    if layout[y][x] == 'L' && count_occupied == 0 {
        target = '#';
    }
    if layout[y][x] == '#' && count_occupied >= 4 {
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
            new_layout[y][x] = adjacent_check(x, y, &layout)?;
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
            "#.LL.L#.##",
            "#LLLLLL.L#",
            "L.L.L..L..",
            "#LLL.LL.L#",
            "#.LL.LL.LL",
            "#.LLLL#.##",
            "..L.L.....",
            "#LLLLLLLL#",
            "#.LLLLLL.L",
            "#.#LLLL.##",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout4: Vec<String> = vec![
            "#.##.L#.##",
            "#L###LL.L#",
            "L.#.#..#..",
            "#L##.##.L#",
            "#.##.LL.LL",
            "#.###L#.##",
            "..#.#.....",
            "#L######L#",
            "#.LL###L.L",
            "#.#L###.##",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout5: Vec<String> = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.L.L..#..",
            "#LLL.##.L#",
            "#.LL.LL.LL",
            "#.LL#L#.##",
            "..L.L.....",
            "#L#LLLL#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let layout6: Vec<String> = vec![
            "#.#L.L#.##",
            "#LLL#LL.L#",
            "L.#.L..#..",
            "#L##.##.L#",
            "#.#L.LL.LL",
            "#.#L#L#.##",
            "..L.L.....",
            "#L#L##L#L#",
            "#.LLLLLL.L",
            "#.#L#L#.##",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        assert_eq!(Some(layout2.clone()), generate_layout(&layout1));
        assert_eq!(Some(layout3.clone()), generate_layout(&layout2));
        assert_eq!(Some(layout4.clone()), generate_layout(&layout3));
        assert_eq!(Some(layout5.clone()), generate_layout(&layout4));
        assert_eq!(Some(layout6.clone()), generate_layout(&layout5));
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

        assert_eq!(Some(37), count_stabilized_seats(&layout));
    }
}
