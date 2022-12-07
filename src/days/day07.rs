use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::common::{AocError, AocResult, IntoAocResult};

#[repr(u8)]
#[derive(PartialEq)]
enum NodeType {
    File,
    Directory,
}

struct Node<'a> {
    pub name: &'a str,
    pub node_type: NodeType,
    pub contents_size: u64,
    pub parent: Option<Rc<RefCell<Node<'a>>>>,
    pub children: HashMap<&'a str, Rc<RefCell<Node<'a>>>>,
}

impl<'a> Node<'a> {
    pub fn new_dir(name: &'a str, parent: Option<Rc<RefCell<Node<'a>>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name,
            node_type: NodeType::Directory,
            contents_size: 0,
            parent,
            children: HashMap::new(),
        }))
    }

    pub fn new_file(
        name: &'a str,
        size: u64,
        parent: Option<Rc<RefCell<Node<'a>>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name,
            node_type: NodeType::File,
            contents_size: size,
            parent,
            children: HashMap::new(),
        }))
    }

    pub fn size(&self) -> u64 {
        match self.node_type {
            NodeType::File => self.contents_size,
            NodeType::Directory => self.children.values().map(|n| n.borrow().size()).sum(),
        }
    }
}

struct NodeTreeIterator<'a> {
    stack: Vec<Rc<RefCell<Node<'a>>>>,
}

impl<'a> NodeTreeIterator<'a> {
    pub fn new(root: &Rc<RefCell<Node<'a>>>) -> Self {
        Self {
            stack: vec![root.clone()],
        }
    }
}

impl<'a> Iterator for NodeTreeIterator<'a> {
    type Item = Rc<RefCell<Node<'a>>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(node) => {
                self.stack
                    .extend(node.borrow().children.values().map(|n| n.clone()));
                Some(node)
            }
        }
    }
}

struct Command<'a> {
    pub cmd: &'a str,
    pub args: Option<&'a str>,
}

impl<'a> Command<'a> {
    fn from_line(line: &'a str) -> Self {
        match line.split_once(' ') {
            Some((first, second)) => Self {
                cmd: first,
                args: Some(second),
            },
            None => Self {
                cmd: line,
                args: None,
            },
        }
    }
}

fn read_directory_tree<'a>(input: &'a str) -> AocResult<Rc<RefCell<Node>>> {
    let root = Node::new_dir("/", None);
    let mut current = root.clone();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with('$') {
            let command = Command::from_line(line[1..].trim_start());
            match command.cmd {
                "cd" => match command.args.into_aoc_result_msg("missing args for cd")? {
                    "/" => current = root.clone(),
                    ".." => {
                        current = current
                            .clone()
                            .borrow()
                            .parent
                            .as_ref()
                            .into_aoc_result_msg("cannot traverse past root")?
                            .clone();
                    }
                    name => {
                        current = current
                            .clone()
                            .borrow()
                            .children
                            .get(name)
                            .into_aoc_result_msg(&format!(
                                "file {name} does not exist in directory {}",
                                current.borrow().name
                            ))?
                            .clone();
                    }
                },
                "ls" => (),
                cmd => return Err(AocError::new(&format!("unknown command {cmd}"))),
            };
        } else {
            match line.split_once(' ') {
                Some(("dir", name)) => {
                    current
                        .borrow_mut()
                        .children
                        .insert(name, Node::new_dir(name, Some(current.clone())));
                }
                Some((size, name)) => {
                    let size = size.parse::<u64>().into_aoc_result_msg("invalid size")?;
                    current
                        .borrow_mut()
                        .children
                        .insert(name, Node::new_file(name, size, Some(current.clone())));
                }

                None => return Err(AocError::new(&format!("invalid output line: {}", line))),
            }
        }
    }
    Ok(root)
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let root = read_directory_tree(input)?;
    Ok(NodeTreeIterator::new(&root)
        .filter_map(|n| {
            let node = n.borrow();
            let size = node.size();
            if node.node_type == NodeType::Directory && size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const TOTAL_DISK_SPACE: u64 = 70000000;
    const NEEDED_UNUSED_SPACE: u64 = 30000000;
    let root = read_directory_tree(input)?;
    let currently_used = root.borrow().size();
    let currently_unused = TOTAL_DISK_SPACE - currently_used;
    if (currently_unused >= NEEDED_UNUSED_SPACE) {
        return Err(AocError::new("already have enough unused disk space"));
    }
    let min_to_remove = NEEDED_UNUSED_SPACE - currently_unused;
    NodeTreeIterator::new(&root)
        .filter_map(|n| {
            let node = n.borrow();
            let size = node.size();
            if node.node_type == NodeType::Directory && size >= min_to_remove {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .into_aoc_result_msg("no directory can be deleted")
}
