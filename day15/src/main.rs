use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Sensor {
    pub x: i32,
    pub y: i32,
    closest_beacon: (i32, i32),
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content);
    // part2(content);
}

fn part1(content: String) {
    let sensors = parse_input(content);
    let mut excluded_points = HashSet::new();
    let mut covered_points = HashSet::new();
    let y = 2000000;

    for sensor in sensors.iter() {
        excluded_points.insert((sensor.x, sensor.y));
        excluded_points.insert(sensor.closest_beacon);
    }

    for sensor in sensors {
        let points = coverage_line(sensor, y);
        for point in points {
            if !excluded_points.contains(&point) {
                covered_points.insert(point);
            }
        }
    }

    println!("Covered points : {}", covered_points.len());
}

fn part2(content: String) {

}

//Returns points covered by this sensors. In these points there can be no beacon
fn coverage_line(sensor: Sensor, y: i32) -> Vec<(i32, i32)> {
    let mut points = vec![];
    let sensor_pos = (sensor.x, sensor.y);
    let max_dist = manhattan(&sensor_pos, &sensor.closest_beacon);

    for x in -max_dist..max_dist + 1 {
        let point = (sensor.x + x, y);
        if manhattan(&point, &sensor_pos) <= max_dist {
            points.push(point);
        }
    }

    return points;
}

fn parse_input(content: String) -> Vec<Sensor> {
    let mut sensors = vec![];
    for line in content.lines() {
        let tokens: Vec<_> = line.split(" ").collect();
        let x = extract_val(tokens[2]);
        let y = extract_val(tokens[3]);
        let closest_beacon = (extract_val(tokens[8]), extract_val(tokens[9]));
        sensors.push(Sensor {
            x,
            y,
            closest_beacon,
        });
    }

    return sensors;
}

fn extract_val(string: &str) -> i32 {
    let tokens: Vec<_> = string.split("=").collect();
    let val = tokens[1].strip_suffix(',').unwrap_or(tokens[1]);
    let val = val.strip_suffix(':').unwrap_or(val);
    return val.parse().unwrap();
}

fn manhattan(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    let x = (p2.0 - p1.0).abs();
    let y = (p2.1 - p1.1).abs();
    return x + y;
}
