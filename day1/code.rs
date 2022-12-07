use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to open file!");

    let mut elves: Vec<i32> = vec![];
    let mut calories = vec![];

    for line in contents.lines() {
        if line == "" {
            elves.push(calories.iter().sum());
            calories = vec![];
            continue;
        }

        let num_cal = line.parse::<i32>().expect("Failed parsing a number");

        calories.push(num_cal);
    }

    elves.sort();
    let elves : Vec<i32> = elves.into_iter().rev().collect();

    let top_3_total : i32 = elves[0..3].iter().sum();
    println!("top 3 total: {}", top_3_total);
    println!("top 1: {}", elves[0]);
}
