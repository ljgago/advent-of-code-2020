// common.rs

use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Passport {
    pub fields: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter_fields: Vec<&str> = s.split_whitespace().collect();

        let mut fields: HashMap<String, String> = HashMap::new();

        for f in iter_fields.into_iter() {
            let hash: Vec<&str> = f.split(":").collect();
            fields.insert(hash[0].to_owned(), hash[1].to_owned());
        }

        Ok(Passport { fields: fields })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passport_from_str() {
        let f = |s: &str| s.to_owned();

        let text_fields = r##"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
        "##;
        let mut hash: HashMap<String, String> = HashMap::new();
        hash.insert(f("ecl"), f("gry"));
        hash.insert(f("pid"), f("860033327"));
        hash.insert(f("eyr"), f("2020"));
        hash.insert(f("hcl"), f("#fffffd"));
        hash.insert(f("byr"), f("1937"));
        hash.insert(f("iyr"), f("2017"));
        hash.insert(f("cid"), f("147"));
        hash.insert(f("hgt"), f("183cm"));
        let passport = Passport { fields: hash };
        assert_eq!(passport, Passport::from_str(text_fields).unwrap());

        let text_fields = r##"
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
        "##;
        let mut hash: HashMap<String, String> = HashMap::new();
        hash.insert(f("iyr"), f("2013"));
        hash.insert(f("ecl"), f("amb"));
        hash.insert(f("cid"), f("350"));
        hash.insert(f("eyr"), f("2023"));
        hash.insert(f("pid"), f("028048884"));
        hash.insert(f("hcl"), f("#cfa07d"));
        hash.insert(f("byr"), f("1929"));
        let passport = Passport { fields: hash };
        assert_eq!(passport, Passport::from_str(text_fields).unwrap());

        let text_fields = r##"
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
        "##;
        let mut hash: HashMap<String, String> = HashMap::new();
        hash.insert(f("hcl"), f("#ae17e1"));
        hash.insert(f("iyr"), f("2013"));
        hash.insert(f("eyr"), f("2024"));
        hash.insert(f("ecl"), f("brn"));
        hash.insert(f("pid"), f("760753108"));
        hash.insert(f("byr"), f("1931"));
        hash.insert(f("hgt"), f("179cm"));
        let passport = Passport { fields: hash };
        assert_eq!(passport, Passport::from_str(text_fields).unwrap());

        let text_fields = r##"
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "##;
        let mut hash: HashMap<String, String> = HashMap::new();
        hash.insert(f("hcl"), f("#cfa07d"));
        hash.insert(f("eyr"), f("2025"));
        hash.insert(f("pid"), f("166559648"));
        hash.insert(f("iyr"), f("2011"));
        hash.insert(f("ecl"), f("brn"));
        hash.insert(f("hgt"), f("59in"));
        let passport = Passport { fields: hash };
        assert_eq!(passport, Passport::from_str(text_fields).unwrap());
    }
}
