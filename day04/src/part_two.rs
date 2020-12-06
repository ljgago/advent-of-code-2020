// part_two.rs

use regex::Regex;

use crate::common::Passport;

fn check_fields(passport: &Passport) -> bool {
    // Birth Year
    let byr = match passport
        .fields
        .get("byr")
        .unwrap()
        .as_str()
        .parse::<usize>()
    {
        Ok(i) => i,
        Err(_) => return false,
    };
    if byr < 1920 || byr > 2002 {
        // println!("byr");
        return false;
    }

    // Issue Year
    let iyr = match passport
        .fields
        .get("iyr")
        .unwrap()
        .as_str()
        .parse::<usize>()
    {
        Ok(i) => i,
        Err(_) => return false,
    };
    if iyr < 2010 || iyr > 2020 {
        // println!("iyr");
        return false;
    }

    // Expiration Year
    let eyr = match passport
        .fields
        .get("eyr")
        .unwrap()
        .as_str()
        .parse::<usize>()
    {
        Ok(i) => i,
        Err(_) => return false,
    };
    if eyr < 2020 || eyr > 2030 {
        // println!("eyr");
        return false;
    }

    // Height
    let hgt_str = passport.fields.get("hgt").unwrap().as_str();
    let (hgt, unit): (usize, &str) = if hgt_str.contains("cm") {
        let num: usize = hgt_str
            .strip_suffix("cm")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        (num, "cm")
    } else if hgt_str.contains("in") {
        let num: usize = hgt_str
            .strip_suffix("in")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        (num, "in")
    } else {
        // println!("hgt");
        return false;
    };
    match unit {
        "cm" => {
            if hgt < 150 || hgt > 193 {
                // println!("hgt cm");
                return false;
            }
        }
        "in" => {
            if hgt < 59 || hgt > 76 {
                // println!("hgt in");
                return false;
            }
        }
        _ => {
            // println!("ecl");
            return false;
        }
    }

    // Hair Color
    let hcl = passport.fields.get("hcl").unwrap().as_str();
    let re = Regex::new("^#[0-9a-fA-F]{6}$").unwrap();
    if !re.is_match(hcl) {
        // println!("hcl");
        return false;
    }

    // Eye Color
    let ecl = passport.fields.get("ecl").unwrap().as_str();
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
        _ => {
            // println!("ecl");
            return false;
        }
    }

    // Pasport ID
    let pid = passport.fields.get("pid").unwrap().as_str();
    let re = Regex::new("^[0-9]{9}$").unwrap();
    if !re.is_match(pid) {
        // println!("pid");
        return false;
    }

    return true;
}

fn valid_passport(passport: &Passport) -> bool {
    let valid_fields = [
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
    ];

    for field in valid_fields.iter() {
        if !passport.fields.contains_key(field.to_owned()) {
            return false;
        }
    }
    return check_fields(passport);
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
        // Invalid passports
        let passports = vec![
            r##"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        "##,
            r##"
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
        "##,
            r##"
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        "##,
            r##"
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "##,
        ];

        for text_fields in passports.iter() {
            let passport = Passport::from_str(text_fields).unwrap();
            assert_eq!(false, valid_passport(&passport));
        }

        // Valid passports
        let passports = vec![
            r##"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
        "##,
            r##"
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        "##,
            r##"
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
        "##,
            r##"
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "##,
        ];

        for text_fields in passports.iter() {
            let passport = Passport::from_str(text_fields).unwrap();
            assert_eq!(true, valid_passport(&passport));
        }
    }
}
