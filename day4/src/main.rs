use std::fs;

fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let pairs: Vec<&str> = line.split(",").collect();
    let a: Vec<&str> = pairs[0].split('-').collect();
    let b: Vec<&str> = pairs[1].split('-').collect();

    let a1 = a[0].parse::<i32>().unwrap();
    let a2 = a[1].parse::<i32>().unwrap();
    let b1 = b[0].parse::<i32>().unwrap();
    let b2 = b[1].parse::<i32>().unwrap();

    return (a1, a2, b1, b2);
}

fn part1(content: String) -> i32 {
    let mut counter = 0;
    for line in content.lines() {
        let (a1, a2, b1, b2) = parse_line(line);
        if (b1 >= a1 && b2 <= a2) || (a1 >= b1 && a2 <= b2) {
            counter += 1;
        }
    }

    return counter;
}

fn part2(content: String) -> i32 {
    let mut counter = 0;
    for line in content.lines() {
        let (a1, a2, b1, b2) = parse_line(line);
        if (b2 >= a1 && b1 <= a2) || (a2 >= b1 && a1 <= b2) {
            counter += 1;
        }
    }

    return counter;
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    println!("{}", part1(content.clone()));
    println!("{}", part2(content));
}
