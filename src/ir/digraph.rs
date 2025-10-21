use std::collections::VecDeque;
use std::fmt;

pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>,
    indegree: Vec<usize>,
}

impl Digraph {
    pub fn new(v: usize) -> Self {
        Digraph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
            indegree: vec![0; v],
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v - 1);
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(w);
        self.indegree[w] += 1;
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> &[usize] {
        self.validate_vertex(v);
        &self.adj[v]
    }

    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn indegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.indegree[v]
    }

    pub fn reverse(&self) -> Digraph {
        let mut reverse = Digraph::new(self.v);
        for v in 0..self.v {
            for &w in &self.adj[v] {
                reverse.add_edge(w, v);
            }
        }
        reverse
    }
}

impl fmt::Display for Digraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} vertices, {} edges", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{}: ", v)?;
            for &w in &self.adj[v] {
                write!(f, "{} ", w)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct TopologicalSort<'a> {
    g: &'a Digraph,
    indegree: Vec<usize>,
    order: Option<Vec<usize>>,
}

impl<'a> TopologicalSort<'a> {
    pub fn new(g: &'a Digraph) -> Self {
        let v = g.v();
        TopologicalSort {
            g,
            indegree: vec![0; v],
            order: Some(vec![0; v]),
        }
    }

    pub fn sort(&mut self) -> bool {
        let v_count = self.g.v();
        for v in 0..v_count {
            self.indegree[v] = self.g.indegree(v);
        }

        let mut queue = VecDeque::new();
        for v in 0..v_count {
            if self.indegree[v] == 0 {
                queue.push_back(v);
            }
        }

        let mut count = 0;
        let mut order = vec![0; v_count];
        while let Some(u) = queue.pop_front() {
            order[count] = u;
            count += 1;
            for &v in self.g.adj(u) {
                self.indegree[v] -= 1;
                if self.indegree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if count != v_count {
            self.order = None;
            return false;
        }
        self.order = Some(order);
        true
    }

    pub fn has_order(&self) -> bool {
        self.order.is_some()
    }

    pub fn get_orders(&self) -> Option<&[usize]> {
        self.order.as_deref()
    }
}

impl<'a> fmt::Display for TopologicalSort<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.order {
            None => write!(f, "null"),
            Some(order) => {
                write!(f, "[")?;
                for v in order {
                    write!(f, "{},", v)?;
                }
                write!(f, "]")
            }
        }
    }
}
