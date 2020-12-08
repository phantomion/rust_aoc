use std::collections::HashMap;

pub fn day7_part1(input: &str) -> i32 {
    let color_graph = input.lines().fold(HashMap::new(), |mut color_graph, line| {
        if line.contains(" no ") {
            let bags = line.split(" bags ").collect::<Vec<&str>>();
            color_graph.insert(bags[0], None);
            return color_graph;
        }
        let bags = line.split(", ").collect::<Vec<&str>>();
        let key_bag = bags[0].split(" bags contain ").collect::<Vec<&str>>();
        let suffix = get_suffix(key_bag[1]).unwrap();
        let first_bag = key_bag[1]
            .strip_prefix(char::is_numeric)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
            .trim();
        let mut all_bags = bags
            .iter()
            .skip(1)
            .map(|bag| {
                let suffix = get_suffix(bag).unwrap();
                let bag = bag
                    .strip_prefix(char::is_numeric)
                    .unwrap()
                    .strip_suffix(suffix)
                    .unwrap()
                    .trim();
                bag
            })
            .collect::<Vec<&str>>();
        all_bags.push(first_bag);
        color_graph.insert(key_bag[0], Some(all_bags));
        color_graph
    });

    let shiny_bags = color_graph.iter().fold(0, |shiny_bags, (key, value)| {
        match value {
            Some(bags_vector) => bags_vector,
            None => return shiny_bags,
        };
        shiny_bags + count_shiny_bags(&color_graph, key, 0)
    });

    shiny_bags
}

fn count_shiny_bags(
    color_graph: &HashMap<&str, Option<Vec<&str>>>,
    key: &str,
    shiny_bags: i32,
) -> i32 {
    let value = color_graph.get(key).unwrap();
    let bags_vector = match value {
        Some(bags_vector) => bags_vector,
        None => return shiny_bags,
    };

    for bag in bags_vector {
        if *bag == "shiny gold" && shiny_bags == 0 {
            return 1;
        }
        if count_shiny_bags(color_graph, bag, shiny_bags) == 1 {
            return 1;
        }
    }

    return shiny_bags;
}

fn get_suffix(bag: &str) -> Option<&str> {
    let suffix = if bag.ends_with("bags") {
        Some("bags")
    } else if bag.ends_with("bags.") {
        Some("bags.")
    } else if bag.ends_with("bag") {
        Some("bag")
    } else if bag.ends_with("bag.") {
        Some("bag.")
    } else {
        None
    };
    suffix
}

pub fn day7_part2(input: &str) -> i32 {
    let color_graph = input.lines().fold(HashMap::new(), |mut color_graph, line| {
        if line.contains(" no ") {
            let bags = line.split(" bags ").collect::<Vec<&str>>();
            color_graph.insert(bags[0], None);
            return color_graph;
        }
        let bags = line.split(", ").collect::<Vec<&str>>();
        let key_bag = bags[0].split(" bags contain ").collect::<Vec<&str>>();
        let suffix = get_suffix(key_bag[1]).unwrap();
        let first_bag = {
            let first_bag_quantity = key_bag[1]
                .chars()
                .take_while(|&c| c != ' ')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let first_bag = key_bag[1]
                .strip_prefix(char::is_numeric)
                .unwrap()
                .strip_suffix(suffix)
                .unwrap()
                .trim();
            (first_bag, first_bag_quantity)
        };
        let mut all_bags = bags
            .iter()
            .skip(1)
            .map(|bag| {
                let suffix = get_suffix(bag).unwrap();
                let bag_quantity = bag
                    .chars()
                    .take_while(|&c| c != ' ')
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                let bag = bag
                    .strip_prefix(char::is_numeric)
                    .unwrap()
                    .strip_suffix(suffix)
                    .unwrap()
                    .trim();
                (bag, bag_quantity)
            })
            .collect::<Vec<(&str, i32)>>();
        all_bags.push(first_bag);
        color_graph.insert(key_bag[0], Some(all_bags));
        color_graph
    });

    count_bags_in_shiny_gold(&color_graph, "shiny gold", 0, 1)
}

fn count_bags_in_shiny_gold(
    color_graph: &HashMap<&str, Option<Vec<(&str, i32)>>>,
    key: &str,
    bags: i32,
    mul_bags: i32,
) -> i32 {
    let value = color_graph.get(key).unwrap();
    let bags_vector = match value {
        Some(bags_vector) => bags_vector,
        None => return bags,
    };

    let bags = bags_vector.iter().fold(bags, |bags, bag| {
        bags + count_bags_in_shiny_gold(color_graph, bag.0, mul_bags * bag.1, mul_bags * bag.1)
    });

    return bags;
}
