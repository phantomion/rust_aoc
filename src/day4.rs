use std::collections::HashMap;
pub fn day4_part1(input: &str) -> i32 {
    let passport: HashMap<&str, &str> = HashMap::new();
    let (valid, passport) = input.lines().fold((0, passport), |(valid, mut passport), line| {
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

pub fn day4_part2(_input: &str) -> i32 {
    return 1;
}
