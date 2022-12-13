use std::fs;

#[derive(Debug)]
enum Content {
    Integer(i32),
    List(Box<List>),
}

enum Order {
    Ok,
    NotOk,
    Equal,
}

#[derive(Debug)]
struct List {
    pub content: Vec<Content>,
}

fn parse_packet(pstring: &str) -> List {
    if pstring.is_empty() {
        return List { content: vec![] };
    }

    let mut content = vec![];

    let pstring = pstring
        .strip_prefix('[')
        .unwrap()
        .strip_suffix("]")
        .unwrap();

    let chars: Vec<_> = pstring.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' {
            let mut closing_br_left = 1;
            let start_pos = i;

            while closing_br_left != 0 {
                i += 1;
                if chars[i] == '[' {
                    closing_br_left += 1;
                } else if chars[i] == ']' {
                    closing_br_left -= 1;
                }
            }

            let list = &pstring[start_pos..i + 1];
            let list = parse_packet(list);
            content.push(Content::List(Box::new(list)));
            i += 2;
        } else {
            println!("{}", chars[i]);
            let mut string = String::new();
            while i < chars.len() && chars[i].is_numeric() {
                string.push(chars[i]);
                i += 1;
            }
            let val = string.parse::<i32>().unwrap();
            content.push(Content::Integer(val));
            i += 1;
        }
    }

    List { content }
}

fn parse_input(content: String) -> Vec<(List, List)> {
    let mut pairs = vec![];
    let lines: Vec<_> = content.lines().filter(|s| *s != "").collect();
    // let lines = ["[1,[2,[3,[4,[5,6,7]]]],8,9]", "[]"];
    for pair in lines.chunks(2) {
        let p1 = parse_packet(pair[0]);
        let p2 = parse_packet(pair[1]);
        pairs.push((p1, p2));
    }

    return pairs;
}

fn recursive_print(list: List, depth: usize) {
    print!("[");
    for element in list.content {
        match element {
            Content::Integer(i) => print!("{},", i),
            Content::List(l) => recursive_print(*l, depth + 1),
        }
    }
    print!("]");
}

fn compare_integers(i: i32, j: i32) -> Order {
    if i < j {
        return Order::Ok;
    } else if i == j {
        return Order::Equal;
    } else {
        return Order::NotOk;
    }
}

fn compare_lists(x: &List, y: &List) -> Order {
    let max_index = x.content.len().min(y.content.len());
    for i in 0..max_index {
        let result = compare_content(&x.content[i], &y.content[i]);
        //Order could not be decided, continue to next loop
        if let Order::Equal = result {
            continue;
        }
        //Order decided, return result
        return result;
    }

    //Could not find order, check which list ran out of items first
    if x.content.len() < y.content.len() {
        return Order::Ok;
    } else if x.content.len() > y.content.len() {
        return Order::NotOk;
    } else {
        return Order::Equal;
    }
}

fn compare_content(c1: &Content, c2: &Content) -> Order {
    match c1 {
        Content::Integer(i) => match c2 {
            Content::Integer(j) => return compare_integers(*i, *j),
            Content::List(l2) => {
                let single_list = List {
                    content: vec![Content::Integer(*i)],
                };
                return compare_lists(&single_list, l2);
            }
        },
        Content::List(l) => match c2 {
            Content::Integer(j) => {
                let single_list = List {
                    content: vec![Content::Integer(*j)],
                };
                return compare_lists(l, &single_list);
            }
            Content::List(l2) => return compare_lists(l, l2),
        },
    }
}

fn part1(content: String) {
    let pairs = parse_input(content);
    let mut sum = 0;
    for (i, (p1, p2)) in pairs.iter().enumerate() {
        let order = compare_lists(p1, p2);
        if let Order::Ok = order {
            println!("Pair {} ok", i + 1);
            sum += (i + 1);
        }
    }

    println!("Sum is: {}", sum);
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    part1(content);
}
