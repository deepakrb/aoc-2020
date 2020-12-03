use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt")
        .expect("Unable to read file");

    let inputs: Vec<&str> = data
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut valid_passes: Vec<_> = Vec::new();

    for input in inputs.iter() {
        let parsed_input: Vec<_> = input.split(|c| c == ',').collect();
        let (lower_bound, upper_bound, character, pass) = (
            parsed_input[0].parse::<i32>().unwrap(), 
            parsed_input[1].parse::<i32>().unwrap(), 
            parsed_input[2].chars().next().unwrap(), 
            parsed_input[3]
        );
        
        if is_valid(&pass, character, lower_bound, upper_bound) {
            valid_passes.push(pass)
        }
    }

    println!("Len: {}", valid_passes.iter().count());
}

fn is_valid(pass: &str, character: char, lower_bound: i32, upper_bound: i32) -> bool {
    let pass_vec: Vec<char> = pass.chars().collect();

    let mut contains_lower = false;
    if pass_vec[(lower_bound) as usize - 1] == character {
        contains_lower = true
    }

    let mut contains_upper = false;
    if pass_vec[(upper_bound) as usize - 1] == character {
        contains_upper = true
    }

    if contains_lower && contains_upper {
        return false
    }

    if !contains_lower && !contains_upper {
        return false
    }

    return true
}
