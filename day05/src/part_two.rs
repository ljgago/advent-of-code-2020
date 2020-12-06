// part_two.rs

use crate::common::Seat;
use crate::part_one::highest_seat_id;
use std::collections::HashSet;

pub fn get_seat_id(seats: &[Seat]) -> Option<usize> {
    let mut hash_ids = HashSet::new();

    for seat in seats.iter() {
        hash_ids.insert(seat.id);
    }

    let max_id: usize = highest_seat_id(&seats)?;
    let min_id: usize = 1;

    for id in min_id..max_id {
        if !hash_ids.contains(&id) {
            if hash_ids.contains(&(id + 1)) && hash_ids.contains(&(id - 1)) {
                return Some(id);
            }
        }
    }

    Some(0)
}
