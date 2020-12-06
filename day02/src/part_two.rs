// part_two.rs

use crate::common::PasswordPolicy;

fn valid_password(policy: &PasswordPolicy) -> bool {
    let min_char: char = policy.password.chars().nth(policy.min - 1).unwrap();
    let max_char: char = policy.password.chars().nth(policy.max - 1).unwrap();

    if min_char == max_char {
        false
    } else if min_char == policy.letter || max_char == policy.letter {
        true
    } else {
        false
    }
}

pub fn count_valid_password(data: &[PasswordPolicy]) -> Option<usize> {
    let mut count: usize = 0;

    for d in data {
        if valid_password(d) {
            count += 1
        }
    }

    Some(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_password() {
        let policy: PasswordPolicy = "1-3 a: abcde".parse().unwrap();
        assert_eq!(true, valid_password(&policy));

        let policy: PasswordPolicy = "1-3 b: cdefg".parse().unwrap();
        assert_eq!(false, valid_password(&policy));

        let policy: PasswordPolicy = "2-9 c: ccccccccc".parse().unwrap();
        assert_eq!(false, valid_password(&policy));
    }
}
