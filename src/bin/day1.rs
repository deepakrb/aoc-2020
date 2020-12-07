use std::collections::HashSet;
use std::fs;
// NOTE: Day 1 and 2 are incorrectly formatted and only contain the last answer, they need to be updated
// to include the working for Part 1

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
fn main() {
    let data = fs::read_to_string("inputs/day1.txt").expect("Unable to read file");

    let input = data.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut available_inputs: HashSet<i32> = HashSet::new();

    let filtered_inputs = input.iter()
        .filter(|&x| x < &2020)
        .map(|&x| { available_inputs.insert(x); x }).collect::<Vec<i32>>();

    for item in filtered_inputs.iter() {
        let goal = 2020 - item;

        let matching = match is_present(goal, &filtered_inputs, &available_inputs) {
            Ok(f) => f,
            Err(_) => continue,
        };

        println!("Total: {}, 1: {}, 2: {}, 3: {}", (item*matching.0*matching.1), item, matching.0, matching.1)
    }
}

fn is_present(max_value: i32, inputs: &Vec<i32>, set: &HashSet<i32>) -> Result<(i32, i32), &'static str> {
    for input in inputs.iter() {
        let goal = max_value - input;

        if goal > 0 && set.contains(&goal) {
            return Ok((*input, goal))
        }
    }

    return Err("not found")
}
