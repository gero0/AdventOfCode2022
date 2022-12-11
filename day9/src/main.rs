use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(content: String) -> Vec<(Direction, i32)> {
    let mut moves = vec![];
    for line in content.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let dir = match tokens[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Error occured while parsing input file"),
        };
        let range = tokens[1]
            .parse::<i32>()
            .expect("Error occured while parsing input file");

        moves.push((dir, range));
    }

    return moves;
}

fn dist(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let len = ((dx * dx + dy * dy) as f64).sqrt().round() as i32;
    return len;
}

fn follow(head: &mut (i32, i32), tail: &mut (i32, i32)) {
    let distance = dist(head, tail);
    if distance <= 1 {
        return;
    }

    if head.0 == tail.0 {
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    } else if head.1 == tail.1 {
        if head.0 > tail.0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
    } else if head.0 > tail.0 {
        tail.0 += 1;
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    } else {
        tail.0 -= 1;
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    }
}

fn follow_chain(points_pos: &mut [(i32, i32)]) {
    for i in 1..points_pos.len() {
        let mut head = points_pos[i - 1];
        let mut tail = points_pos[i];
        follow(&mut head, &mut tail);
        points_pos[i - 1] = head;
        points_pos[i] = tail;
    }
}

fn move_v2(points_pos: &mut [(i32, i32)], mv: (Direction, i32)) -> Vec<(i32, i32)> {
    let mut tail_positions = vec![];
    match mv.0 {
        Direction::Up => {
            let new_pos_y = points_pos[0].0 + mv.1;
            while points_pos[0].0 != new_pos_y {
                points_pos[0].0 += 1;
                follow_chain(points_pos);
                tail_positions.push(points_pos[points_pos.len() - 1]);
            }
        }
        Direction::Down => {
            let new_pos_y = points_pos[0].0 - mv.1;
            while points_pos[0].0 != new_pos_y {
                points_pos[0].0 -= 1;
                follow_chain(points_pos);
                tail_positions.push(points_pos[points_pos.len() - 1]);
            }
        }
        Direction::Right => {
            let new_pos_x = points_pos[0].1 + mv.1;
            while points_pos[0].1 != new_pos_x {
                points_pos[0].1 += 1;
                follow_chain(points_pos);
                tail_positions.push(points_pos[points_pos.len() - 1]);
            }
        }
        Direction::Left => {
            let new_pos_x = points_pos[0].1 - mv.1;
            while points_pos[0].1 != new_pos_x {
                points_pos[0].1 -= 1;
                follow_chain(points_pos);
                tail_positions.push(points_pos[points_pos.len() - 1]);
            }
        }
    }

    return tail_positions;
}

fn part1(content: String) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut points_pos = [(0, 0), (0, 0)];

    let moves: Vec<(Direction, i32)> = parse_input(content);
    for mv in moves {
        let tail_pos = move_v2(&mut points_pos, mv);
        for pos in tail_pos {
            visited.insert(pos);
        }
    }

    println!("total visited: {}", visited.len());
}

fn part2(content: String) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut points_pos = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let moves: Vec<(Direction, i32)> = parse_input(content);
    for mv in moves {
        let tail_pos = move_v2(&mut points_pos, mv);
        for pos in tail_pos {
            visited.insert(pos);
        }
    }

    println!("total visited: {}", visited.len());
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content.clone());
    part2(content.clone());
}
