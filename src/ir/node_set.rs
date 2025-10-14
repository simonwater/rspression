use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct Node<T> {
    pub name: String,
    pub info: Option<T>,
    pub index: usize,
}

impl<T> Node<T> {
    pub fn new(name: &str, index: usize, info: Option<T>) -> Self {
        Node {
            name: name.to_string(),
            info,
            index,
        }
    }
}

pub struct NodeSet<T> {
    nodes_map: HashMap<String, Rc<Node<T>>>,
    nodes: Vec<Rc<Node<T>>>,
    cnt: usize,
}

impl<T> NodeSet<T> {
    pub fn new() -> Self {
        NodeSet {
            nodes_map: HashMap::new(),
            nodes: Vec::new(),
            cnt: 0,
        }
    }

    pub fn add_node(&mut self, name: &str) -> Rc<Node<T>> {
        self.add_node_with_info(name, None)
    }

    pub fn add_node_with_info(&mut self, name: &str, info: Option<T>) -> Rc<Node<T>> {
        if let Some(node) = self.nodes_map.get(name) {
            return node.clone();
        }
        let node = Rc::new(Node::new(name, self.cnt, info));
        self.nodes_map.insert(name.to_string(), node.clone());
        self.nodes.push(node.clone());
        self.cnt += 1;
        node
    }

    pub fn get_node_by_name(&self, name: &str) -> Option<Rc<Node<T>>> {
        match self.nodes_map.get(name) {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }

    pub fn get_node_by_index(&self, index: usize) -> Rc<Node<T>> {
        self.validate_index(index);
        self.nodes[index].clone()
    }

    fn validate_index(&self, i: usize) {
        if i >= self.cnt {
            panic!("index {} is not between 0 and {}", i, self.cnt - 1);
        }
    }

    pub fn size(&self) -> usize {
        self.cnt
    }
}

impl<'a, T> fmt::Display for NodeSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.cnt {
            let node = self.get_node_by_index(i);
            writeln!(f, "{}: {}({})", i, node.name, node.index)?;
        }
        Ok(())
    }
}
