use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let data = fs::read_to_string("inputs/day_06.txt").expect("Unable to read file");

    println!("Part 1: {}", part_1(parse_input(&data)));
    println!("Part 2: {}", part_2(parse_input(&data)));
}

/*
--- Day 6: Custom Customs ---

As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

abcx
abcy
abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    The first group contains one person who answered "yes" to 3 questions: a, b, and c.
    The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
    The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
    The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
    The last group contains one person who answered "yes" to only 1 question, b.

In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
 */
fn part_1(groups: Vec<&str>) -> i32 {
    groups
        .iter()
        .map(|group| {
            group.lines().fold(HashSet::new(), |mut acc, x| {
                x.chars().for_each(|c| {
                    acc.insert(c);
                    return;
                });
                acc
            })
        })
        .map(|hs| hs.len() as i32)
        .fold(0, |acc, x| acc + x)
}

/*
--- Part Two ---

As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
 */
fn part_2(groups: Vec<&str>) -> i32 {
    groups
        .iter()
        .map(|group| {
            let mut lines = group
                .lines()
                .map(|x| HashSet::from_iter(x.chars().into_iter()));

            // intersect the sets onto itself
            lines
                .next()
                .map(|set: HashSet<char>| {
                    lines.fold(set, |set1, set2| {
                        HashSet::from_iter(set1.intersection(&set2).map(|c| c.to_owned()))
                    })
                })
                .unwrap()
        })
        .map(|hs| hs.len() as i32)
        .fold(0, |acc, x| acc + x)
}

fn parse_input(data: &str) -> Vec<&str> {
    return data.split("\n\n").collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(part_1(parse_input(input)), 11);
    }

    #[test]
    fn test_part_2_example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(part_2(parse_input(input)), 6);
    }
}
