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

fn move_left(head_pos: &mut (i32, i32), tail_pos: &mut (i32, i32), d: i32) -> Vec<(i32, i32)> {
    let mut visited = vec![];
    head_pos.0 -= d;
    let distance = dist(head_pos, tail_pos);
    if distance > 1 {
        for i in head_pos.0 + 1..tail_pos.0 {
            visited.push((i, head_pos.1));
        }
        tail_pos.0 = head_pos.0 + 1;
        tail_pos.1 = head_pos.1;
    }

    return visited;
}

fn move_right(head_pos: &mut (i32, i32), tail_pos: &mut (i32, i32), d: i32) -> Vec<(i32, i32)> {
    let mut visited = vec![];
    head_pos.0 += d;
    let distance = dist(head_pos, tail_pos);
    if distance > 1 {
        for i in tail_pos.0 + 1..head_pos.0 {
            visited.push((i, head_pos.1));
        }
        tail_pos.0 = head_pos.0 - 1;
        tail_pos.1 = head_pos.1;
    }

    return visited;
}

fn move_up(head_pos: &mut (i32, i32), tail_pos: &mut (i32, i32), d: i32) -> Vec<(i32, i32)> {
    let mut visited = vec![];
    head_pos.1 += d;
    let distance = dist(head_pos, tail_pos);
    if distance > 1 {
        for i in tail_pos.1 + 1..head_pos.1 {
            visited.push((head_pos.0, i));
        }
        tail_pos.0 = head_pos.0;
        tail_pos.1 = head_pos.1 - 1;
    }

    return visited;
}

fn move_down(head_pos: &mut (i32, i32), tail_pos: &mut (i32, i32), d: i32) -> Vec<(i32, i32)> {
    let mut visited = vec![];
    head_pos.1 -= d;
    let distance = dist(head_pos, tail_pos);
    if distance > 1 {
        for i in head_pos.1 + 1..tail_pos.1 {
            visited.push((head_pos.0, i));
        }
        tail_pos.0 = head_pos.0;
        tail_pos.1 = head_pos.1 + 1;
    }

    return visited;
}

fn make_move(
    mut head_pos: (i32, i32),
    mut tail_pos: (i32, i32),
    mv: (Direction, i32),
    total_visited: &mut HashSet<(i32, i32)>,
) -> ((i32, i32), (i32, i32)) {
    let now_visited = match mv.0 {
        Direction::Up => move_up(&mut head_pos, &mut tail_pos, mv.1),
        Direction::Down => move_down(&mut head_pos, &mut tail_pos, mv.1),
        Direction::Left => move_left(&mut head_pos, &mut tail_pos, mv.1),
        Direction::Right => move_right(&mut head_pos, &mut tail_pos, mv.1),
    };

    for point in now_visited {
        total_visited.insert(point);
    }

    (head_pos, tail_pos)
}

fn part1(content: String) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);

    let moves: Vec<(Direction, i32)> = parse_input(content);
    for mv in moves {
        (head_position, tail_position) = make_move(head_position, tail_position, mv, &mut visited);
    }

    println!("total visited: {}", visited.len());
}

// fn make_move_v2(
//     points_pos: &mut [(i32, i32)],
//     mv: (Direction, i32),
//     total_visited: &mut HashSet<(i32, i32)>,
// ) {
//     for i in 0..points_pos.len() - 1 {
//         let mut d = 0;
//         if i == 0 {
//             d = mv.1;
//         }

//         let mut head = points_pos[i];
//         let mut tail = points_pos[i + 1];

//         let now_visited = match mv.0 {
//             Direction::Up => move_up(&mut head, &mut tail, d),
//             Direction::Down => move_down(&mut head, &mut tail, d),
//             Direction::Left => move_left(&mut head, &mut tail, d),
//             Direction::Right => move_right(&mut head, &mut tail, d),
//         };

//         points_pos[i] = head;
//         points_pos[i + 1] = tail;

//         //last pair - these are points visited by tail
//         if i == points_pos.len() - 2 {
//             for point in now_visited {
//                 total_visited.insert(point);
//                 println!("TAIL POINT: {:?}", point);
//             }
//         }
//     }
// }

// fn part2(content: String) {
//     let mut visited: HashSet<(i32, i32)> = HashSet::new();
//     visited.insert((0, 0));
//     let mut points_pos = [
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//         (0, 0),
//     ];
//     let moves: Vec<(Direction, i32)> = parse_input(content);
//     for mv in moves {
//         make_move_v2(&mut points_pos, mv, &mut visited);
//         println!("{:?}", points_pos);
//     }

//     println!("total visited: {}", visited.len());
// }

fn main() {
    let content = fs::read_to_string("input2").expect("Failed to open file!");
    part1(content.clone());
    // part2(content.clone());
}
