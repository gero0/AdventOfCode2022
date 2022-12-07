use std::collections::HashMap;
use std::fs;

fn part1_check_result(A: &str, B: &str) -> i32 {
    let combinations = HashMap::from([
        (("A", "X"), 3),
        (("A", "Y"), 6),
        (("A", "Z"), 0),
        (("B", "X"), 0),
        (("B", "Y"), 3),
        (("B", "Z"), 6),
        (("C", "X"), 6),
        (("C", "Y"), 0),
        (("C", "Z"), 3),
    ]);

    return *combinations.get(&(A, B)).unwrap();
}

fn part1(contents: String) -> i32 {
    let mut score = 0;

    for line in contents.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        score += part1_check_result(tokens[0], tokens[1]);
        score += match tokens[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
    }

    return score;
}

fn part2_get_points(A: &str, B: &str) -> i32 {
    match A {
        "A" => match B {
            "X" => 3,
            "Y" => 4,
            "Z" => 8,
            _ => 0,
        },
        "B" => match B {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        },
        "C" => match B {
            "X" => 2,
            "Y" => 6,
            "Z" => 7,
            _ => 0,
        },
        _ => 0,
    }
}

// A - Rock, B - Paper, C - Scissors
// Rock - 1, Paper - 2, Scissors - 3

fn part2(contents: String) -> i32 {
    let mut score = 0;

    for line in contents.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        score += part2_get_points(tokens[0], tokens[1]);
    }

    return score;
}

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to open file!");

    println!("Part1: {}", part1(contents.clone()));
    println!("Part2: {}", part2(contents));
}
