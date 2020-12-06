use regex::Regex;
use std::collections::HashMap;

pub fn day4_part1(input: &str) -> i32 {
    let passport: HashMap<&str, &str> = HashMap::new();
    let (valid, passport) = input
        .lines()
        .fold((0, passport), |(valid, mut passport), line| {
            if line.is_empty() {
                if passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid")) {
                    return (valid + 1, HashMap::new());
                }
                return (valid, HashMap::new());
            }
            let pairs = line.split_ascii_whitespace();
            pairs.for_each(|pair| {
                let (key, value) = {
                    let pair = pair.split(':').collect::<Vec<&str>>();
                    (pair[0].trim(), pair[1].trim())
                };
                passport.insert(key, value);
            });
            (valid, passport)
        });

    if passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid")) {
        return valid + 1;
    }
    return valid;
}

pub fn day4_part2(input: &str) -> i32 {
    let passport: HashMap<&str, &str> = HashMap::new();
    let (valid, passport) = input
        .lines()
        .fold((0, passport), |(valid, mut passport), line| {
            if line.is_empty() {
                if (passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid")))
                    && check_validity(passport)
                {
                    return (valid + 1, HashMap::new());
                }
                return (valid, HashMap::new());
            }
            let pairs = line.split_ascii_whitespace();
            pairs.for_each(|pair| {
                let (key, value) = {
                    let pair = pair.split(':').collect::<Vec<&str>>();
                    (pair[0].trim(), pair[1].trim())
                };
                passport.insert(key, value);
            });
            (valid, passport)
        });

    if (passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid")))
        && check_validity(passport)
    {
        return valid + 1;
    }
    return valid;
}

pub fn check_validity(passport: HashMap<&str, &str>) -> bool {
    let four_digits = Regex::new(r"^\d{4}$").unwrap();
    let hgt_regex = Regex::new(r"^[0-9]+(in|cm)$").unwrap();
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    let byr_value = passport.get("byr").unwrap();
    let iyr_value = passport.get("iyr").unwrap();
    let eyr_value = passport.get("eyr").unwrap();
    let hgt_value = passport.get("hgt").unwrap();
    let hcl_value = passport.get("hcl").unwrap();
    let ecl_value = passport.get("ecl").unwrap();
    let pid_value = passport.get("pid").unwrap();
    let byr_value_int: i32 = byr_value.parse().unwrap();
    let iyr_value_int: i32 = iyr_value.parse().unwrap();
    let eyr_value_int: i32 = eyr_value.parse().unwrap();
    if !four_digits.is_match(byr_value) || byr_value_int < 1920 || byr_value_int > 2002 {
        return false;
    } else if !four_digits.is_match(iyr_value) || iyr_value_int < 2010 || iyr_value_int > 2020 {
        return false;
    } else if !four_digits.is_match(eyr_value) || eyr_value_int < 2020 || eyr_value_int > 2030 {
        return false;
    } else if !hgt_regex.is_match(hgt_value) {
        return false;
    } else if !hcl_regex.is_match(hcl_value) {
        return false;
    } else if *ecl_value != "amb"
        && *ecl_value != "blu"
        && *ecl_value != "brn"
        && *ecl_value != "gry"
        && *ecl_value != "grn"
        && *ecl_value != "hzl"
        && *ecl_value != "oth"
    {
        return false;
    } else if !pid_regex.is_match(pid_value) {
        return false;
    }

    if hgt_value.contains("cm") {
        let height: i32 = hgt_value
            .chars()
            .take_while(|&c| c != 'c')
            .collect::<String>()
            .parse()
            .unwrap();
        if height < 150 || height > 193 {
            return false;
        }
    } else if hgt_value.contains("in") {
        let height: i32 = hgt_value
            .chars()
            .take_while(|&c| c != 'i')
            .collect::<String>()
            .parse()
            .unwrap();
        if height < 59 || height > 76 {
            return false;
        }
    }

    true
}
