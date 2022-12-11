use std::fs;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    MulOldSelf,
}

#[derive(Debug)]
struct Monke {
    pub monke_id: usize,
    pub items: Vec<u128>,
    pub op: Operation,
    pub test: (usize, usize, usize),
    inspected_count: usize,
}

fn parse_op(tokens: Vec<&str>) -> Operation {
    let mut val = 0;

    match tokens[2].parse() {
        Ok(v) => val = v,
        Err(_) => return Operation::MulOldSelf,
    }

    match tokens[1] {
        "*" => Operation::Mul(val),
        "+" => Operation::Add(val),
        _ => panic!("Error while parsing operation"),
    }
}

fn parse_monke(set: &[&str]) -> Monke {
    let line_1: Vec<&str> = set[0].split(' ').collect();
    let monke_id = line_1[1].strip_suffix(':').unwrap().parse().unwrap();
    let mut items = vec![];
    let line_2 = set[1].split_whitespace().skip(2);
    for number in line_2 {
        items.push(number.strip_suffix(',').unwrap_or(number).parse().unwrap());
    }
    let line_3 = set[2].split_whitespace().skip(3).collect();
    let op = parse_op(line_3);
    let test_div = set[3].split_whitespace().last().unwrap().parse().unwrap();
    let if_true = set[4].split_whitespace().last().unwrap().parse().unwrap();
    let if_false = set[5].split_whitespace().last().unwrap().parse().unwrap();

    Monke {
        monke_id,
        items,
        op,
        test: (test_div, if_true, if_false),
        inspected_count: 0,
    }
}

fn run_monkes(content: String, rounds: usize, divide: bool, use_common_div: bool) {
    let lines: Vec<&str> = content.lines().collect();
    let mut monkeys = vec![];

    for set in lines.chunks(7) {
        let monke = parse_monke(set);
        println!("{:?}", &monke);
        monkeys.push(monke);
    }

    let mut common_divisor = 1;
    for monke in monkeys.iter() {
        common_divisor *= monke.test.0 as u128;
    }

    for _ in 0..rounds{
        for i in 0..monkeys.len() {
            for item_i in 0..monkeys[i].items.len() {
                //inspection
                let mut worry_level = monkeys[i].items[item_i];
                monkeys[i].inspected_count += 1;
                match monkeys[i].op {
                    Operation::Add(val) => worry_level += val as u128,
                    Operation::Mul(val) => worry_level *= val as u128,
                    Operation::MulOldSelf => worry_level = worry_level * worry_level,
                }
                if use_common_div{
                    worry_level = worry_level % common_divisor;
                }
                //end of inspection - div by 3
                if divide{
                    worry_level /= 3;
                }
                //Perform test and decide where to throw item
                let remainder = worry_level % monkeys[i].test.0 as u128;
                let recipient = if remainder == 0 {
                    monkeys[i].test.1
                } else {
                    monkeys[i].test.2
                };
                monkeys[recipient].items.push(worry_level);
            }
            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by(|a, b| Ord::cmp(&a.inspected_count, &b.inspected_count));
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey : {} Inspected_count: {}", i, monkey.inspected_count);
    }
    let monke_business =
        monkeys[monkeys.len() - 1].inspected_count * monkeys[monkeys.len() - 2].inspected_count;

    println!("Monke business is : {}", monke_business);
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    run_monkes(content.clone(), 20, true, false);
    run_monkes(content, 10000, false, true);
}
