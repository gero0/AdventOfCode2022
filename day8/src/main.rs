use std::{fs, ops::Range};

fn check_range(
    treematrix: &Vec<Vec<u32>>,
    row: usize,
    col: usize,
    range: Range<usize>,
    vertical_sweep: bool,
) -> bool {
    let mut visible = true;
    for i in range {
        if vertical_sweep {
            if treematrix[i][col] >= treematrix[row][col] {
                visible = false;
            }
        } else {
            if treematrix[row][i] >= treematrix[row][col] {
                visible = false;
            }
        }
    }
    return visible;
}

fn check_visible(treematrix: &Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    //always true for edges
    if (row == 0 || row == treematrix.len() - 1) || (col == 0 || col == treematrix[row].len() - 1) {
        return 1;
    }
    //left
    if check_range(treematrix, row, col, 0..col, false)
    //right
        || check_range(treematrix, row, col, col + 1..treematrix[row].len(), false)
    //down
        || check_range(treematrix, row, col, row + 1..treematrix.len(), true)
    //up
        || check_range(treematrix, row, col, 0..row, true)
    {
        return 1;
    }

    return 0;
}

fn process_input(content: String) -> Vec<Vec<u32>> {
    let mut treematrix: Vec<Vec<u32>> = vec![];
    for line in content.lines() {
        let mut heights = vec![];
        for char in line.chars() {
            heights.push(char.to_digit(10).unwrap())
        }
        treematrix.push(heights);
    }

    return treematrix;
}

fn part1(content: String) -> usize {
    let treematrix = process_input(content);
    let mut visible_counter = 0;
    for row in 0..treematrix.len() {
        for col in 0..treematrix[row].len() {
            visible_counter += check_visible(&treematrix, row, col);
        }
    }
    return visible_counter;
}

fn check_range_score(
    treematrix: &Vec<Vec<u32>>,
    row: usize,
    col: usize,
    range: Range<usize>,
    vertical_sweep: bool,
    rev : bool,
) -> usize {
let mut counter = 0;
    let mut r = vec![];
    if rev {
        r = range.rev().collect();
    }else{
        r = range.collect();
    }
    for i in r.iter() {
        counter+=1;
        if vertical_sweep {
            if treematrix[*i][col] >= treematrix[row][col] {
                break;
            }
        } else {
            if treematrix[row][*i] >= treematrix[row][col] {
                break;
            }
        }
    }

    return counter;
}

fn calculate_scenic_score(treematrix: &Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    let (l, r, d, u) = (
        //left
        check_range_score(treematrix, row, col, 0..col, false, true),
        //right
        check_range_score(treematrix, row, col, col + 1..treematrix[row].len(), false, false),
        //down
        check_range_score(treematrix, row, col, row + 1..treematrix.len(), true, false),
        //up
        check_range_score(treematrix, row, col, 0..row, true, true),
    );

    let score = l * r * d * u;

    return score;
}

fn part2(content: String) -> usize {
    let treematrix = process_input(content);
    let mut scenic_scores = vec![];

    for row in 0..treematrix.len() {
        for col in 0..treematrix[row].len() {
            scenic_scores.push(calculate_scenic_score(&treematrix, row, col));
        }
    }

    return *scenic_scores.iter().max().unwrap();
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    println!("{}", part1(content.clone()));
    println!("{}", part2(content));
}
