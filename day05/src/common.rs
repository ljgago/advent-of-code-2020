// common.rs

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Seat {
    pub binary_space: String,
    pub row: usize,
    pub column: usize,
    pub id: usize,
}

impl FromStr for Seat {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary_space: String = s.to_owned();

        let (row, column) = s.split_at(7);

        let row = row.replace("F", "0").replace("B", "1");
        let row = usize::from_str_radix(&row, 2).unwrap();

        let column = column.replace("L", "0").replace("R", "1");
        let column = usize::from_str_radix(&column, 2).unwrap();

        let id: usize = row * 8 + column;

        Ok(Seat {
            binary_space: binary_space,
            row: row,
            column: column,
            id: id,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seat() {
        let seat = Seat::from_str("BFFFBBFRRR").unwrap();
        assert_eq!(70, seat.row);
        assert_eq!(7, seat.column);
        assert_eq!(567, seat.id);

        let seat = Seat::from_str("FFFBBBFRRR").unwrap();
        assert_eq!(14, seat.row);
        assert_eq!(7, seat.column);
        assert_eq!(119, seat.id);

        let seat = Seat::from_str("BBFFBBFRLL").unwrap();
        assert_eq!(102, seat.row);
        assert_eq!(4, seat.column);
        assert_eq!(820, seat.id);
    }
}
