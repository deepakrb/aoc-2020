use std::fs;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::iter::FromIterator;

fn main() {
    let data = fs::read_to_string("inputs/day7.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(data.clone())));
    println!("Part 2: {}", part2(parse_input(data.clone())));
}

fn parse_input(input: String) -> Vec<(String, Vec<(String, i32)>)> {
    input.lines()
        .map(|l| l.trim())
        .filter(|&l| l.is_empty() || !l.contains("contain no other bags"))
        .map(|l| {
            let instruction: Vec<&str> = l.split("contain").collect();
            let container = instruction[0].split(' ').take(2).collect::<Vec<&str>>().join("_");
            let bags = instruction[1].split(',')
                .map(|contains| {
                    let c = contains.trim().split(' ').collect::<Vec<&str>>();
                    (String::from(c[1..3].join("_")), c[0].parse::<i32>().unwrap())
                }).collect::<Vec<(String, i32)>>();
            (container, bags)
        }).collect()
}

/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
 */
fn part1(input: Vec<(String, Vec<(String, i32)>)>) -> i32 {
    // build a reverse index { bag_x: Set(every bag that can contain bag_x) }
    let mapping: HashMap<String, HashSet<String>> = input.iter()
        .fold(HashMap::new(), |mut acc, (container, bags)| {
            bags.iter().for_each(|(bag, _)| {
                match acc.entry(String::from(bag)) {
                    Entry::Vacant(e) => { e.insert(HashSet::from_iter(vec![container.to_owned()].into_iter())); },
                    Entry::Occupied(mut e) => { e.get_mut().insert(container.to_owned()); }
                }
            });
            acc
        });

    bag_has_containers(&mapping, String::from("shiny_gold"), &mut HashSet::new())
}

fn bag_has_containers(mapping: &HashMap<String, HashSet<String>>, color: String, containers_accumulator: &mut HashSet<String>) -> i32 {
    if !mapping.contains_key(&color) { return containers_accumulator.len() as i32 }

    mapping[&color].iter()
        .filter(|contents| contents.to_string() != color.to_string()) // avoid recursion
        .for_each(|color| {
            containers_accumulator.insert(color.to_string());
            bag_has_containers(mapping, String::from(color), containers_accumulator);
        });

    containers_accumulator.len() as i32
}

/*
--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?
 */
fn part2(input: Vec<(String, Vec<(String, i32)>)>) -> i32 {
    let mapping: HashMap<String, Vec<(String, i32)>> = input.iter()
        .fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.0.clone(), x.1.clone()); acc
        });

    bag_contains(String::from("shiny_gold"), &mapping)
}

fn bag_contains(bag: String, mapping: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    if !mapping.contains_key(&bag) { return 0 }

    mapping[&bag].iter().map(|(contained_bag, count)|
        count + count * bag_contains(String::from(contained_bag), mapping)
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(part1(parse_input(input.parse().unwrap())), 4);
    }

    #[test]
    fn test_part_2_example() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(part2(parse_input(input.parse().unwrap())), 126);
    }
}
