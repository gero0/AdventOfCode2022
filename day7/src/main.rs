use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::string::String;

#[derive(Clone, Debug)]
struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Clone, Debug)]
struct Directory {
    pub name: String,
    pub parent: Option<Rc<RefCell<Directory>>>,
    pub content: Vec<Object>,
    pub size: usize,
}

#[derive(Clone, Debug)]
enum Object {
    File(File),
    Directory(Rc<RefCell<Directory>>),
}

fn handle_cd(
    root_dir: Rc<RefCell<Directory>>,
    current_dir: Rc<RefCell<Directory>>,
    dirname: &str,
) -> Rc<RefCell<Directory>> {
    if dirname == "/" {
        return Rc::clone(&root_dir);
    } else if dirname == ".." {
        return Rc::clone(
            current_dir
                .as_ref()
                .borrow()
                .parent
                .as_ref()
                .expect("Tried to get parent of root directory"),
        );
    } else {
        let dir = current_dir.as_ref().borrow();
        let child = dir
            .content
            .iter()
            .find(|obj| match obj {
                Object::File(_) => false,
                Object::Directory(d) => {
                    return d.as_ref().borrow().name == dirname;
                }
            })
            .expect("Tried to cd to not eisting folder!");

        if let Object::Directory(d) = child {
            return Rc::clone(d);
        } else {
            panic!("Tried to cd into file!");
        }
    }
}

fn handle_ls(current_dir: Rc<RefCell<Directory>>, line: &str) -> Rc<RefCell<Directory>> {
    let tokens: Vec<&str> = line.split(" ").collect();
    let current_dir = Rc::clone(&current_dir);

    if tokens[0] == "dir" {
        let new_dir = Rc::new(RefCell::new(Directory {
            name: String::from(tokens[1]),
            content: vec![],
            parent: Some(Rc::clone(&current_dir)),
            size: 0,
        }));

        let object = Object::Directory(new_dir);

        current_dir.borrow_mut().content.push(object);
    } else {
        let new_file = File {
            name: String::from(tokens[1]),
            size: tokens[0].parse::<usize>().unwrap(),
        };
        let object = Object::File(new_file);
        current_dir.borrow_mut().content.push(object);
    }

    return current_dir;
}

fn parse_input(content: String) -> Rc<RefCell<Directory>> {
    let root_dir = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        parent: None,
        content: vec![],
        size: 0,
    }));

    let mut current_dir = Rc::clone(&root_dir);

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let tokens: Vec<&str> = lines[i].split(' ').collect();

        if tokens[1] == "cd" {
            let dirname = tokens[2];
            current_dir = handle_cd(Rc::clone(&root_dir), current_dir, dirname);
            i += 1;
        } else if tokens[1] == "ls" {
            i += 1;
            while i < lines.len() && !(lines[i].starts_with("$")) {
                current_dir = handle_ls(current_dir, lines[i]);
                i += 1;
            }
        }
    }

    return root_dir;
}

fn recursive_print(dir: &Rc<RefCell<Directory>>, depth: usize) {
    let dir = dir.as_ref().borrow();
    //files first
    for element in dir.content.iter() {
        if let Object::File(f) = element {
            println!("{:->pad$} {} ({})", "", f.name, f.size, pad = depth);
        }
    }
    for element in dir.content.iter() {
        if let Object::Directory(d) = element {
            let db = d.as_ref().borrow();
            println!("{:->pad$} dir : {}", "", db.name, pad = depth);
            recursive_print(d, depth + 4);
        }
    }
}

fn calculate_sizes(dir: &Rc<RefCell<Directory>>) -> usize {
    let mut dir = dir.as_ref().borrow_mut();
    let mut size = 0;
    for element in dir.content.iter() {
        match element {
            Object::Directory(d) => {
                size += calculate_sizes(d);
            }
            Object::File(f) => {
                size += f.size;
            }
        }
    }
    dir.size = size;
    return size;
}

fn find_total_under100k(dir: &Rc<RefCell<Directory>>) -> usize {
    let dir = dir.as_ref().borrow();
    let mut size = 0;
    if dir.size <= 100000 {
        size += dir.size;
    }

    for element in dir.content.iter() {
        if let Object::Directory(d) = element {
            size += find_total_under100k(d);
        }
    }

    return size;
}

fn find_smallest_to_delete(
    dir: &Rc<RefCell<Directory>>,
    space_needed: usize,
    best: usize,
) -> usize {
    let dir = dir.as_ref().borrow();
    let mut size = best;

    if dir.size >= space_needed && dir.size < best {
        size = dir.size;
    }

    for element in dir.content.iter() {
        if let Object::Directory(d) = element {
            let s = find_smallest_to_delete(d, space_needed, size);
            if s < size {
                size = s;
            }
        }
    }

    return size;
}

fn main() {
    let content = fs::read_to_string("input").expect("Failed to open file!");
    let root: Rc<RefCell<Directory>> = parse_input(content);
    recursive_print(&root, 1);
    let size = calculate_sizes(&root);
    let total_u100k_size = find_total_under100k(&root);
    println!("Size of root: {}", size);
    println!(
        "Total size of dirs with size <= 10000: {}",
        total_u100k_size
    );
    let unused_space = 70000000 - size;
    let space_needed = 30000000 - unused_space;
    println!(
        "Unused space is: {}, require {} more",
        unused_space, space_needed
    );
    let size_to_delete = find_smallest_to_delete(&root, space_needed, 70000000);
    print!("The smallest dir we can delete has {} size", size_to_delete);
}
