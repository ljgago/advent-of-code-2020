// part_two.rs

use crate::part_one;

pub fn number_spoken(starting_numbers: &[usize], turn_end: usize) -> Option<usize> {
    part_one::number_spoken(starting_numbers, turn_end)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numer_spoken() {
        // take a lot of time
        // assert_eq!(Some(175594), number_spoken(&[0, 3, 6], 30000000));
        // assert_eq!(Some(2578), number_spoken(&[1, 3, 2], 30000000));
        // assert_eq!(Some(3544142), number_spoken(&[2, 1, 3], 30000000));
        // assert_eq!(Some(261214), number_spoken(&[1, 2, 3], 30000000));
        // assert_eq!(Some(6895259), number_spoken(&[2, 3, 1], 30000000));
        // assert_eq!(Some(18), number_spoken(&[3, 2, 1], 30000000));
        // assert_eq!(Some(362), number_spoken(&[3, 1, 2], 30000000));
    }
}

