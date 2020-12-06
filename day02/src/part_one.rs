// part_one.rs

use crate::common::PasswordPolicy;

fn valid_password(p: &PasswordPolicy) -> bool {
    let number = p.password.matches(p.letter).count();

    if p.min <= number && p.max >= number {
        true
    } else {
        false
    }
}

pub fn count_valid_password(data: &[PasswordPolicy]) -> Option<usize> {
    let mut count: usize = 0;

    for d in data.iter() {
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
        assert_eq!(true, valid_password(&policy));
    }
}
