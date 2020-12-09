use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let data = fs::read_to_string("inputs/day1.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(&data)));
    println!("Part 2 {}", part2(parse_input(&data)).unwrap());
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .filter(|v| *v < 2020)
        .collect()
}

fn does_match(goal: i32, set: &HashSet<&i32>) -> Result<(i32, i32), &'static str> {
    for &input in set.iter() {
        let new_goal = goal - *input;

        if new_goal > 0 && set.contains(&new_goal) {
            return Ok((*input, new_goal));
        }
    }

    return Err("not_found");
}

/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
 */
fn part1(input: Vec<i32>) -> i32 {
    let (x, y) = does_match(2020, &HashSet::from_iter(input.iter())).unwrap();
    x * y
}

/*
--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?
 */
fn part2(input: Vec<i32>) -> Result<i32, &'static str> {
    let all_inputs: HashSet<&i32> = HashSet::from_iter(input.iter());

    for item in input.iter() {
        let goal = 2020 - *item;

        let (x, y) = match does_match(goal, &all_inputs) {
            Ok(f) => f,
            Err(_) => continue,
        };

        return Ok(item * x * y);
    }
    Err("not_found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "1721
979
366
299
675
1456";

        assert_eq!(part1(parse_input(input)), 514579);
    }

    #[test]
    fn test_part_2_example() {
        let input = "1721
979
366
299
675
1456";

        assert_eq!(part2(parse_input(input)).unwrap(), 241861950);
    }
}
