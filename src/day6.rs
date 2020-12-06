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

pub fn day6_part2(input: &str) -> i32 {
    let (all_answers, count, group) = input.lines().fold(
        (Vec::new(), 0, 0),
        |(mut all_answers, count, group), line| {
            if line.is_empty() {
                let common_answers = find_common_answers(group, all_answers);
                return (Vec::new(), count + common_answers, 0);
            }
            line.chars().for_each(|answer| all_answers.push(answer));
            (all_answers, count, group + 1)
        },
    );

    let common_answers = find_common_answers(group, all_answers);
    return count + common_answers;
}

fn find_common_answers(group: usize, all_answers: Vec<char>) -> i32 {
    let common_answers = ('a'..='z')
        .filter(|letter| all_answers.contains(letter))
        .fold(0, |common_answers, letter| {
            let count = all_answers
                .iter()
                .filter(|&&answer| answer == letter)
                .count();
            if count == group {
                return common_answers + 1;
            }
            common_answers
        });

    common_answers
}
