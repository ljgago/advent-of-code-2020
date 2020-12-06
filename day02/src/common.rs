// common.rs

use regex::Regex;

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct PasswordPolicy {
    pub min: usize,
    pub max: usize,
    pub letter: char,
    pub password: String,
}

impl FromStr for PasswordPolicy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"-|:|\s").unwrap();
        let password_policy: Vec<&str> = re.split(s).collect();

        let min: usize = password_policy[0].parse().unwrap();
        let max: usize = password_policy[1].parse().unwrap();
        let letter: char = password_policy[2].parse().unwrap();
        let password: String = password_policy[4].parse().unwrap();

        Ok(PasswordPolicy {
            min: min,
            max: max,
            letter: letter,
            password: password,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_policy_format() {
        let policy1 = PasswordPolicy::from_str("1-3 a: abcde");
        let policy2 = PasswordPolicy {
            min: 1,
            max: 3,
            letter: 'a',
            password: "abcde".to_string(),
        };
        assert_eq!(policy1.unwrap(), policy2);

        let policy1 = PasswordPolicy::from_str("1-3 b: cdefg");
        let policy2 = PasswordPolicy {
            min: 1,
            max: 3,
            letter: 'b',
            password: "cdefg".to_string(),
        };
        assert_eq!(policy1.unwrap(), policy2);

        let policy1 = PasswordPolicy::from_str("2-9 c: ccccccccc");
        let policy2 = PasswordPolicy {
            min: 2,
            max: 9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        };
        assert_eq!(policy1.unwrap(), policy2);

        let policy1 = PasswordPolicy::from_str("2-11 c: ccccccccccc");
        let policy2 = PasswordPolicy {
            min: 2,
            max: 11,
            letter: 'c',
            password: "ccccccccccc".to_string(),
        };
        assert_eq!(policy1.unwrap(), policy2);
    }
}
