use std::fs;

fn get_unique(chars:Vec<char>) -> Vec<char>{
    let mut uniq = vec![];
    for char in chars.iter() {
        if uniq.contains(char) {
            break;
        }
        uniq.push(*char);
    }

    return uniq;
}

fn part1(content: String) {
    let chars: Vec<char> = content.chars().collect();
    for i in 3..chars.len() {
        let four_chars = [chars[i - 3], chars[i - 2], chars[i - 1], chars[i]];
        let uniq = get_unique(four_chars.to_vec());
        if uniq.len() == 4 {
            println!("{}", i + 1);
            return;
        }
    }
}

fn part2(content: String) {
    let chars: Vec<char> = content.chars().collect();
    for i in 13..chars.len() {
        let chars: Vec<char> = ((i - 13)..i+1).map(|x| chars[x]).collect();
        let uniq = get_unique(chars.to_vec());
        if uniq.len() == 14 {
            println!("{}", i + 1);
            return;
        }
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content.clone());
    part2(content.clone());
}
