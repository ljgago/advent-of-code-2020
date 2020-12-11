// part_two.rs

pub fn encryption_weakness(data: &[usize], target: usize) -> Option<usize> {
    let mut start: usize = 0;
    let mut sum: usize = 0;

    loop {
        for (i, x) in data[start..].iter().enumerate() {
            sum += x;
            if sum == target {
                let min = data[start..=(start + i)].iter().min()?;
                let max = data[start..=(start + i)].iter().max()?;
                return Some(min + max);
            }
            if sum > target {
                sum = 0;
                break;
            }
        }
        start += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encryption_weakness() {
        let data: Vec<usize> = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];
        assert_eq!(Some(62), encryption_weakness(&data, 127));
    }
}
