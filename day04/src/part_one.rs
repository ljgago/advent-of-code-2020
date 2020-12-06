// part_one.rs

use crate::common::Passport;

fn valid_passport(passport: &Passport) -> bool {
    let valid_fields = [
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
               // (Country ID)
    ];

    for field in valid_fields.iter() {
        if !passport.fields.contains_key(field.to_owned()) {
            return false;
        }
    }
    true
}

pub fn count_valid_passport(passports: &[Passport]) -> Option<usize> {
    // functional approach
    let count = passports
        .iter()
        .map(|passport| valid_passport(&passport))
        .filter(|x| *x)
        .count();

    Some(count)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_valid_password() {
        let text_fields = r##"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
        "##;
        let passport = Passport::from_str(text_fields).unwrap();
        assert_eq!(true, valid_passport(&passport));

        let text_fields = r##"
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
        "##;
        let passport = Passport::from_str(text_fields).unwrap();
        assert_eq!(false, valid_passport(&passport));

        let text_fields = r##"
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
        "##;
        let passport = Passport::from_str(text_fields).unwrap();
        assert_eq!(true, valid_passport(&passport));

        let text_fields = r##"
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "##;
        let passport = Passport::from_str(text_fields).unwrap();
        assert_eq!(false, valid_passport(&passport));
    }
}
