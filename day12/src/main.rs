use std::{collections::HashMap, fs};

fn char_to_int(c: char) -> usize {
    return c as usize - 97;
}

fn parse_input(content: String) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let mut start_point = (0, 0);
    let mut end_point = (0, 0);
    let mut heightmap = vec![];

    for (y, line) in content.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_point = (y, x);
                    row.push(char_to_int('a'))
                }
                'E' => {
                    end_point = (y, x);
                    row.push(char_to_int('z'))
                }
                _ => row.push(char_to_int(c)),
            }
        }
        heightmap.push(row);
    }

    return (heightmap, start_point, end_point);
}

fn check_neighbors(hmap: &Vec<Vec<usize>>, sp: (usize, usize)) -> Vec<(usize, usize)> {
    let point_height = hmap[sp.0][sp.1];
    let new_points = [
        (sp.0 < hmap.len() - 1, (sp.0 as i32 + 1, sp.1 as i32)),
        (sp.0 > 0, (sp.0 as i32 - 1, sp.1 as i32)),
        (sp.1 < hmap[0].len() - 1, (sp.0 as i32, sp.1 as i32 + 1)),
        (sp.1 > 0, (sp.0 as i32, sp.1 as i32 - 1)),
    ];

    let next_points = new_points
        .iter()
        .filter(|(cond, _p)| *cond == true)
        .map(|(_cond, p)| (p.0 as usize, p.1 as usize))
        .filter(|p| hmap[p.0][p.1] <= point_height + 1);

    return next_points.collect();
}

fn dijkstra(
    heightmap: &Vec<Vec<usize>>,
    start_point: (usize, usize),
    end_point: (usize, usize),
) -> Option<i32> {
    let mut d = HashMap::new();
    let mut p = HashMap::new();
    let mut points = vec![];
    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            points.push((i, j));
        }
    }

    for point in &points {
        if *point == start_point {
            d.insert(*point, 0);
        } else {
            d.insert(*point, i32::MAX);
        }
        p.insert(*point, None);
    }

    let mut queue = points.clone();

    while !queue.is_empty() {
        queue.sort_by(|a, b| Ord::cmp(&d.get(b), &d.get(a)));
        let current = queue.pop().unwrap();
        if *d.get(&current).unwrap() == i32::MAX {
            //can't get to other points
            return d.get(&end_point).map(|v| *v);
        }
        let neighbors = check_neighbors(&heightmap, current);
        for neighbor in neighbors {
            //manhattan/taxi metric
            let distance = (neighbor.0 as i32 - current.0 as i32).abs()
                + (neighbor.1 as i32 - current.1 as i32).abs();

            if *d.get(&neighbor).unwrap() as i64
                > *d.get(&current).unwrap() as i64 + distance as i64
            {
                d.insert(neighbor, *d.get(&current).unwrap() + distance);
                p.insert(neighbor, Some(current));
            }
        }
    }

    return d.get(&end_point).map(|v| *v);
}

fn part1(content: String) {
    let (heightmap, start_point, end_point) = parse_input(content);
    println!(
        "Path length: {}",
        dijkstra(&heightmap, start_point, end_point).unwrap()
    );
}

fn part2(content: String) {
    let (heightmap, _start_point, end_point) = parse_input(content);
    let mut starting_points = vec![];
    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            if heightmap[i][j] == 0 {
                starting_points.push((i, j));
            }
        }
    }

    println!("{} Starting points", starting_points.len());

    let paths = starting_points.iter().enumerate()
        .map(|(i,sp)| {
            // println!("Now running point {} : {:?}", i, *sp);
            dijkstra(&heightmap, *sp, end_point)
        })
        .map(|p| p.unwrap());

    println!("Part 2 shortest: {}", paths.min().unwrap())
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content.clone());
    part2(content);
}
