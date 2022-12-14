use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content.clone());
    part2(content);
}

fn part1(content: String) {
    let paths = parse_paths(content);
    let rocks = get_rocks(paths);
    let bottom = rocks
        .iter()
        .max_by(|a, b| Ord::cmp(&a.y, &b.y))
        .expect("Could not find max value of y among rocks")
        .y;

    let mut still_grains = HashSet::new();
    let mut abyss = false;

    while !abyss {
        abyss = falling_grain(&rocks, &mut still_grains, bottom, true);
    }

    println!("{} grains came to rest", still_grains.len());
}

fn part2(content: String) {
    let paths = parse_paths(content);
    let rocks = get_rocks(paths);
    let bottom = rocks
        .iter()
        .max_by(|a, b| Ord::cmp(&a.y, &b.y))
        .expect("Could not find max value of y among rocks")
        .y
        + 2;

    let mut still_grains = HashSet::new();

    while !still_grains.contains(&Point { x: 500, y: 0 }) {
        falling_grain(&rocks, &mut still_grains, bottom, false);
    }

    println!("Part 2 : {} grains came to rest", still_grains.len());
}

fn falling_grain(
    rocks: &HashSet<Point>,
    still_grains: &mut HashSet<Point>,
    bottom: i32,
    inf_abyss: bool,
) -> bool {
    let mut grain = Point { x: 500, y: 0 };

    loop {
        if grain.y >= bottom {
            return true;
        }
        let below = Point {
            x: grain.x,
            y: grain.y + 1,
        };
        let diag_left = Point {
            x: grain.x - 1,
            y: grain.y + 1,
        };
        let diag_right = Point {
            x: grain.x + 1,
            y: grain.y + 1,
        };

        if point_free(&below, &rocks, &still_grains, bottom, inf_abyss) {
            grain = below;
        } else if point_free(&diag_left, &rocks, &still_grains, bottom, inf_abyss) {
            grain = diag_left;
        } else if point_free(&diag_right, &rocks, &still_grains, bottom, inf_abyss) {
            grain = diag_right;
        } else {
            still_grains.insert(grain);
            return false;
        }
    }
}

fn point_free(
    p: &Point,
    rocks: &HashSet<Point>,
    still_grains: &HashSet<Point>,
    bottom: i32,
    inf_abyss: bool,
) -> bool {
    if let Some(_) = rocks.get(p) {
        return false;
    }
    if let Some(_) = still_grains.get(p) {
        return false;
    }
    if !inf_abyss && p.y >= bottom {
        return false;
    }
    return true;
}

fn parse_paths(content: String) -> Vec<Vec<Point>> {
    let mut paths = vec![];

    for line in content.lines() {
        let mut path = vec![];

        let tokens = line.split(" -> ");
        for token in tokens {
            let coords: Vec<_> = token.split(',').collect();
            let new_point = Point {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
            };
            path.push(new_point);
        }
        paths.push(path);
    }

    paths
}

fn get_rocks(paths: Vec<Vec<Point>>) -> HashSet<Point> {
    let mut rocks = HashSet::new();

    for path in paths {
        for window in path.windows(2) {
            let p1 = window[0];
            let p2 = window[1];
            if p1.x == p2.x {
                let start = p1.y.min(p2.y);
                let stop = p1.y.max(p2.y);
                for y in start..stop + 1 {
                    let point = Point { x: p1.x, y };
                    rocks.insert(point);
                }
            } else {
                let start = p1.x.min(p2.x);
                let stop = p1.x.max(p2.x);
                for x in start..stop + 1 {
                    let point = Point { x, y: p1.y };
                    rocks.insert(point);
                }
            }
        }
    }

    return rocks;
}
