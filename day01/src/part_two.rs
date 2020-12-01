// Part Two

pub fn check_expense_report(data: &[usize]) -> (usize, usize) {
    for (i, item1) in data.iter().enumerate() {
        for (j, item2) in data.iter().enumerate() {
            for (k, item3) in data.iter().enumerate() {
                if i != j && i != k && j != k && item1 + item2 + item3 == 2020 {
                    return (item1 + item2 + item3, item1 * item2 * item3);
                }
            }
        }
    }
    return (0, 0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_expense_report() {
        let data = vec![1721, 979, 366, 299, 675, 1456];
        let (sum, _multiply) = check_expense_report(&data);
        assert_eq!(2020, sum);
    }
}
