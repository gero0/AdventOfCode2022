use std::fs;

enum Instruction {
    Nop,
    Add(i32),
}

fn parse_content(content: String) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in content.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens[0] {
            "noop" => instructions.push(Instruction::Nop),
            "addx" => instructions.push(Instruction::Add(
                tokens[1]
                    .parse::<i32>()
                    .expect("Error parsing addx argument"),
            )),
            _ => panic!("Invalid parser input"),
        }
    }

    return instructions;
}

fn part1(content: String) {
    let mut x: i32 = 1;
    let mut signal_during = vec![];

    let instructions = parse_content(content);
    let mut current_cycle = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Nop => {
                signal_during.push((current_cycle) as i32 * x);
                current_cycle += 1;
            }
            Instruction::Add(val) => {
                signal_during.push((current_cycle) as i32 * x);
                current_cycle += 1;
                x += val;
                signal_during.push(current_cycle as i32 * x);
                current_cycle += 1;
            }
        }
    }

    let sum: i32 = signal_during
        .iter()
        .enumerate()
        .filter(|(i, _c)| (*i >= 19) && ((i + 1 - 20) % 40) == 0)
        .map(|(_i, c)| c)
        .sum();

    println!("sum: {}", sum);
}

fn draw(mut image: [[char; 40]; 8], cycle: usize, x: i32) -> [[char; 40]; 8] {
    let row = cycle / 40;
    let col = cycle % 40;
    if col >= (x - 1) as usize && col <= (x + 1) as usize {
        image[row][col] = '#';
    } else {
        image[row][col] = '.';
    }

    return image;
}

fn part2(content: String) {
    let mut x: i32 = 1;
    let instructions = parse_content(content);
    let mut current_cycle = 0;
    let mut image = [['.'; 40]; 8];

    for instruction in instructions {
        match instruction {
            Instruction::Nop => {
                image = draw(image, current_cycle, x);
                current_cycle += 1;
            }
            Instruction::Add(val) => {
                image = draw(image, current_cycle, x);
                current_cycle += 1;
                image = draw(image, current_cycle, x);
                current_cycle += 1;
                x += val;
            }
        }
    }

    for row in image {
        for c in row {
            print!("{}", c);
        }
        println!()
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content.clone());
    part2(content);
}
