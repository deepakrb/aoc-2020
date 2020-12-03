use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");

    let input = data.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
    
    let mut available_inputs: HashSet<i32> = HashSet::new();
    
    input.iter()
    .filter(|&x| x < &2020)
    .map(|&x| { 
        available_inputs.insert(x);
        return x
    }).collect::<Vec<i32>>();

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
