use std::fs;

fn parse_stacks(content: String) -> (Vec<Vec<char>>, usize) {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stack_end = 0;

    'outer: for (l_n, line) in content.lines().enumerate() {
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();

        while i < (chars.len() / 4) + 1 {
            if l_n == 0 {
                stacks.push(vec![]);
            }

            let pos = 4 * i;
            let (a, b) = (chars[pos], chars[pos + 1]);

            if a == '[' && b.is_alphabetic() {
                stacks[i].push(b);
            } else if b.is_numeric() {
                stack_end = l_n;
                break 'outer;
            }

            i += 1;
        }
    }

    //reverse stacks
    stacks = stacks
        .into_iter()
        .map(|stack| stack.into_iter().rev().collect::<Vec<char>>())
        .collect();

    return (stacks, stack_end);
}

fn parse_commands(content: String, stack_end: usize) -> Vec<(usize, usize, usize)> {
    let mut commands = vec![];
    let lines: Vec<&str> = content.lines().collect();
    for line in &lines[(stack_end + 2)..] {
        let tokens: Vec<&str> = line.split(' ').collect();
        let amount = tokens[1].parse::<usize>().unwrap();
        let src = tokens[3].parse::<usize>().unwrap();
        let dst = tokens[5].parse::<usize>().unwrap();

        commands.push((amount, src, dst));
    }

    return commands;
}

fn part1(content: String) {
    let (mut stacks, stack_end) = parse_stacks(content.clone());
    let commands = parse_commands(content, stack_end);
    for command in commands {
        for _i in 0..command.0 {
            let val = stacks[(command.1) - 1].pop().unwrap();
            stacks[(command.2) - 1].push(val);
        }
    }
    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
}

fn part2(content: String) {
    let (mut stacks, stack_end) = parse_stacks(content.clone());
    let commands = parse_commands(content, stack_end);
    for command in commands {
        let mut crates_to_move = vec![];
        for _i in 0..command.0 {
            let val = stacks[(command.1) - 1].pop().unwrap();
            crates_to_move.push(val);
        }
        for c in crates_to_move.iter().rev() {
            stacks[(command.2) - 1].push(*c);
        }
    }
    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content.clone());
    println!("");
    part2(content);
}
