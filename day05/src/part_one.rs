// part_one.rs

use crate::common::Seat;

pub fn highest_seat_id(seats: &[Seat]) -> Option<usize> {
    let max_id = seats
        .iter()
        .fold(0, |acc, seat| if seat.id > acc { seat.id } else { acc });

    Some(max_id)
}
