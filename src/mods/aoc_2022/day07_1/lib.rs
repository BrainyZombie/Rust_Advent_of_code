// use core::fmt;
use std::cmp::Ordering;

struct DirNode {
    name: String,
    size: usize,
    children: Vec<Node>,
}
struct FileNode {
    name: String,
    size: usize,
}
enum Node {
    Dir(DirNode),
    File(FileNode),
}

fn node_sort(c1: &Node, c2: &Node) -> Ordering {
    match (c1, c2) {
        (Node::File(_), Node::Dir(_)) => Ordering::Less,
        (Node::Dir(_), Node::File(_)) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

impl DirNode {
    fn insert(&mut self, path: &[&str], insert_node: Node) {
        let node_size = match insert_node {
            Node::File(ref insert_file) => insert_file.size,
            Node::Dir(ref insert_directory) => insert_directory.size,
        };

        self.size += node_size;
        if path.is_empty() {
            let existing_node =
                self.children
                    .iter()
                    .find(|child_node| match (*child_node, &insert_node) {
                        (Node::File(child_file), Node::File(insert_file)) => {
                            child_file.name == insert_file.name
                        }
                        (Node::Dir(child_dir), Node::Dir(insert_dir)) => {
                            child_dir.name == insert_dir.name
                        }
                        _ => false,
                    });
            if matches!(existing_node, None) {
                self.children.push(insert_node);
                self.children.sort_by(node_sort);
            }
        } else {
            let next_node = self
                .children
                .iter_mut()
                .find_map(|child_node| match child_node {
                    Node::File(_) => None,
                    Node::Dir(child_node) => {
                        if child_node.name == path[0] {
                            Some(child_node)
                        } else {
                            None
                        }
                    }
                })
                .unwrap();
            next_node.insert(&path[1..], insert_node);
        }
    }
    // fn print(&self, depth: usize) {
    //     let s = String::from_utf8(vec![' ' as u8; depth * 2]).unwrap();
    //     println!("{s} d {} {}", self.name, self.size);
    //     self.children.iter().for_each(|c| c.print(depth + 1));
    // }
}

// impl FileNode {
//     fn print(&self, depth: usize) {
//         let s = String::from_utf8(vec![' ' as u8; depth * 2]).unwrap();
//         println!("{s} f {} {},", self.name, self.size)
//     }
// }

// impl Node {
//     fn print(&self, depth: usize) {
//         match self {
//             Node::File(f) => f.print(depth),
//             Node::Dir(d) => d.print(depth),
//         };
//     }
// }

fn create_node_tree(input: &str) -> DirNode {
    let mut root_node = DirNode {
        name: String::from("/"),
        size: 0,
        children: vec![],
    };

    let mut current_dir: Vec<&str> = vec![];

    input.lines().for_each(|line| {
        let words: Vec<&str> = line.split(' ').collect();
        match words[..] {
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", "/"] => {}
            ["$", "cd", next_dir] => {
                current_dir.push(next_dir);
            }
            ["$", _] => {}
            ["dir", new_dir_name] => {
                root_node.insert(
                    &current_dir,
                    Node::Dir(DirNode {
                        name: new_dir_name.to_string(),
                        size: 0,
                        children: vec![],
                    }),
                );
            }
            [file_size_str, new_file_name] => {
                root_node.insert(
                    &current_dir,
                    Node::File(FileNode {
                        name: new_file_name.to_string(),
                        size: file_size_str.parse().unwrap(),
                    }),
                );
            }
            _ => {}
        }
    });
    root_node
}

fn get_top_dirs<'a>(root_node: &'a DirNode, max_dir_size: usize) -> Vec<&'a DirNode> {
    let mut dir_list: Vec<&'a DirNode> = vec![];

    if root_node.size <= max_dir_size {
        dir_list.push(root_node);
    }

    root_node.children.iter().for_each(|child_node| {
        match child_node {
            Node::File(_) => {}
            Node::Dir(child_dir) => {
                dir_list.append(&mut get_top_dirs(child_dir, max_dir_size));
            }
        };
    });

    dir_list
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    const MAX_DIR_SIZE: usize = 100000;
    let root_node = create_node_tree(input);
    // root_node.print(0);
    let top_dirs = get_top_dirs(&root_node, MAX_DIR_SIZE);
    let total_size = top_dirs.iter().fold(0, |acc, dir| acc + dir.size);

    Ok(total_size.to_string())
}
