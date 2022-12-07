use std::fs;

fn get_priority(c: char) -> u32 {
    match c.is_uppercase() {
        true => c as u32 - 38,
        false => c as u32 - 96,
    }
}

fn part1(contents: String) -> u32 {
    let mut score = 0;

    for line in contents.lines() {
        let ruck1 = &line[0..line.len() / 2];
        let ruck2 = &line[line.len() / 2..];

        for char in ruck1.chars() {
            //Convert to ascii and subtract offset to get in range of priority values
            if ruck2.contains(char) {
                score += get_priority(char);
                break;
            }
        }
    }

    return score;
}

fn part2(contents: String) -> u32 {
    let mut score = 0;
    let lines: Vec<&str> = contents.lines().collect();

    for triplet in lines.chunks(3) {
        for char in triplet[0].chars() {
            if triplet[1].contains(char) && triplet[2].contains(char) {
                score += get_priority(char);
                break;
            }
        }
    }

    return score;
}

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to open file!");

    println!("Score: {}", part1(contents.clone()));
    println!("part2 Score: {}", part2(contents));
}
