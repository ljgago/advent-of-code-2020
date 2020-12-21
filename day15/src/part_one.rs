// part_one.rs

use std::collections::HashMap;

pub fn number_spoken(starting_numbers: &[usize], turn_end: usize) -> Option<usize> {
    let mut map_spoken_number: HashMap<usize, usize> = HashMap::new();
    let mut last_spoken_number: usize = 0;
    let size: usize = starting_numbers.len();

    for turn in 1..=turn_end {
        let i = turn - 1;

        // insert initial list of spoken numbers
        if i < size - 1 {
            map_spoken_number.insert(starting_numbers[i], turn);
            last_spoken_number = starting_numbers[i];
        }
        if i == size - 1 {
            last_spoken_number = starting_numbers[i];
        }
        if i >= size {
            let temp = last_spoken_number;
            if map_spoken_number.contains_key(&last_spoken_number) {
                last_spoken_number = turn - 1 - map_spoken_number.get(&last_spoken_number)?;
            } else {
                last_spoken_number = 0;
            }
            map_spoken_number.insert(temp, turn - 1);
        }
        // println!("Turn: {}, Spoken number: {}", turn, last_spoken_number);
    }

    Some(last_spoken_number)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_spoken() {
        assert_eq!(Some(0), number_spoken(&[0, 3, 6], 10));
        assert_eq!(Some(436), number_spoken(&[0, 3, 6], 2020));
        assert_eq!(Some(1), number_spoken(&[1, 3, 2], 2020));
        assert_eq!(Some(10), number_spoken(&[2, 1, 3], 2020));
        assert_eq!(Some(27), number_spoken(&[1, 2, 3], 2020));
        assert_eq!(Some(78), number_spoken(&[2, 3, 1], 2020));
        assert_eq!(Some(438), number_spoken(&[3, 2, 1], 2020));
        assert_eq!(Some(1836), number_spoken(&[3, 1, 2], 2020));
    }
}
