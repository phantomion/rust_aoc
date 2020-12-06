use std::collections::HashSet;

pub fn day6_part1(input: &str) -> usize {
    let (answers, count) = input
        .lines()
        .fold((HashSet::new(), 0), |(mut answers, count), line| {
            if line.is_empty() {
                return (HashSet::new(), count + answers.len());
            }
            line.chars().for_each(|answer| {
                answers.insert(answer);
            });
            (answers, count)
        });

    return count + answers.len();
}

pub fn day6_part2(input: &str) -> usize {
    let (unique_answers, all_answers, count, group) = input.lines().fold(
        (HashSet::new(), Vec::new(), 0, 0),
        |(unique_answers, mut all_answers, count, group), line| {
            if line.is_empty() {
                let unique_answers = find_unique_answers(group, unique_answers, all_answers);
                return (HashSet::new(), Vec::new(), count + unique_answers.len(), 0);
            }
            line.chars().for_each(|answer| all_answers.push(answer));
            (unique_answers, all_answers, count, group + 1)
        },
    );

    let unique_answers = find_unique_answers(group, unique_answers, all_answers);
    return count + unique_answers.len();
}

fn find_unique_answers(
    group: usize,
    mut unique_answers: HashSet<char>,
    all_answers: Vec<char>,
) -> HashSet<char> {
    ('a'..='z').for_each(|letter| {
        let count = all_answers
            .iter()
            .filter(|&&answer| answer == letter)
            .count();
        if count == group {
            unique_answers.insert(letter);
        }
    });

    unique_answers
}
