use std::collections::HashMap;
use std::fmt;

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

    pub fn _set_info(&mut self, info: T) {
        self.info = Some(info)
    }
}

pub struct NodeSet<T> {
    nodes_map: HashMap<String, usize>,
    nodes: Vec<Node<T>>,
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

    pub fn add_node(&mut self, name: &str) -> &mut Node<T> {
        self.add_node_with_info(name, None)
    }

    pub fn add_node_with_info(&mut self, name: &str, info: Option<T>) -> &mut Node<T> {
        if let Some(&index) = self.nodes_map.get(name) {
            return &mut self.nodes[index];
        }
        let node = Node::new(name, self.cnt, info);
        self.nodes_map.insert(name.to_string(), self.cnt);
        self.nodes.push(node);
        self.cnt += 1;
        &mut self.nodes[self.cnt - 1]
    }

    pub fn _get_mut_node(&mut self, name: &str) -> Option<&mut Node<T>> {
        if let Some(&index) = self.nodes_map.get(name) {
            self.nodes.get_mut(index)
        } else {
            None
        }
    }

    pub fn get_node(&self, name: &str) -> Option<&Node<T>> {
        self.nodes_map
            .get(name)
            .and_then(|&index| self.nodes.get(index))
    }

    pub fn _get_node_mut_by_index(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.validate_index(index).ok()?;
        self.nodes.get_mut(index)
    }

    pub fn get_node_by_index(&self, index: usize) -> Option<&Node<T>> {
        self.validate_index(index).ok()?;
        self.nodes.get(index)
    }

    fn validate_index(&self, index: usize) -> Result<(), String> {
        if index >= self.cnt {
            Err(format!(
                "index {} is not between 0 and {}",
                index,
                self.cnt - 1
            ))
        } else {
            Ok(())
        }
    }

    pub fn size(&self) -> usize {
        self.cnt
    }
}

impl<'a, T> fmt::Display for NodeSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.cnt {
            if let Some(node) = self.get_node_by_index(i) {
                writeln!(f, "{}: {}({})", i, node.name, node.index)?;
            }
        }
        Ok(())
    }
}
