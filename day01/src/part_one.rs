// part_one.rs

use std::collections::HashSet;

// Unoptimized
pub fn check_expense_report(data: &[i64]) -> (i64, i64) {
    for (i, item1) in data.iter().enumerate() {
        for (j, item2) in data.iter().enumerate() {
            if i != j && item1 + item2 == 2020 {
                return (item1 + item2, item1 * item2);
            }
        }
    }
    return (0, 0);
}

// Optimized
pub fn check_expense_report_op(data: HashSet<i64>, sum: i64) -> Option<i64> {
    for d in data.iter() {
        let rest = sum - d;
        if data.contains(&rest) {
            return Some(rest * d);
        }
    }
    Some(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_expense_report() {
        let data: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        let (sum, _multiply) = check_expense_report(&data);
        assert_eq!(2020, sum);

        let data: HashSet<i64> = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        let multiply = check_expense_report_op(data, 2020).unwrap();
        assert_eq!(514579, multiply);
    }
}
