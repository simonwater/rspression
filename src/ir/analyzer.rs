use bitvec::index;

use super::digraph::{Digraph, TopologicalSort};
use crate::expr::Expr;
use crate::ir::ExprInfo;
use crate::ir::node_set::{Node, NodeSet};
use crate::{LoxError, LoxResult};

pub struct Analyzer {
    expr_infos: Vec<ExprInfo>,
    node_set: NodeSet<usize>,
    graph: Digraph,
    need_sort: bool,
}

impl Analyzer {
    pub fn new(exprs: Vec<Expr>, need_sort: bool) -> Self {
        let expr_infos = Self::init_infos(exprs);
        let node_set = Self::init_node_set(&expr_infos);
        let graph = Self::init_graph(&node_set, &expr_infos);
        Analyzer {
            expr_infos,
            node_set,
            graph,
            need_sort,
        }
    }

    fn init_infos(exprs: Vec<Expr>) -> Vec<ExprInfo> {
        let n = exprs.len();
        let mut expr_infos = Vec::with_capacity(n);
        for (i, expr) in exprs.into_iter().enumerate() {
            let expr_info = ExprInfo::new(expr, i);
            expr_infos.push(expr_info);
        }
        expr_infos
    }

    fn init_node_set(expr_infos: &Vec<ExprInfo>) -> NodeSet<usize> {
        let mut node_set = NodeSet::new();
        for expr_info in expr_infos {
            if !expr_info.is_assign() {
                continue;
            }

            for name in expr_info.get_reads() {
                node_set.add_node(name);
            }

            let mut flag = true;
            for name in expr_info.get_writes() {
                let node = node_set.add_node(name);
                if flag {
                    node.info = Some(expr_info.get_index());
                    flag = false;
                }
            }
        }
        node_set
    }

    fn init_graph(node_set: &NodeSet<usize>, expr_infos: &Vec<ExprInfo>) -> Digraph {
        let mut graph = Digraph::new(node_set.size());
        if node_set.size() == 0 {
            return graph;
        }
        for info in expr_infos {
            if !info.is_assign() {
                continue;
            }
            for prec in info.get_reads() {
                let pre_node = node_set.get_node(prec).expect("pre node not found");
                let u = pre_node.index;
                for succ in info.get_writes() {
                    let succ_node = node_set.get_node(succ).expect("succ node not found");
                    let v = succ_node.index;
                    graph.add_edge(u, v);
                }
            }
        }
        graph
    }

    pub fn analyze(&self) -> LoxResult<Vec<&ExprInfo>> {
        if self.need_sort && !self.expr_infos.is_empty() && self.has_assign() {
            return self.sort();
        }
        return Ok(self
            .expr_infos
            .iter()
            .map(|item| -> &ExprInfo { item })
            .collect());
    }

    fn has_assign(&self) -> bool {
        self.graph.v() > 0
    }

    fn sort(&self) -> LoxResult<Vec<&ExprInfo>> {
        let mut result = Vec::new();
        if self.graph.v() == 0 {
            return Ok(result);
        }

        let mut top_sorter = TopologicalSort::new(&self.graph);

        if !top_sorter.sort() {
            return Err(LoxError::AnalyzeError {
                message: format!("{}", "公式列表存在循环引用！"),
            });
        }

        let node_orders = match top_sorter.get_orders() {
            Some(orders) => orders,
            None => return Ok(result),
        };

        for &node_index in node_orders {
            if let Some(Node { info, .. }) = self.node_set.get_node_by_index(node_index) {
                if let Some(index) = info {
                    let expr_info = &self.expr_infos[*index];
                    result.push(expr_info);
                }
            }
        }

        for expr in &self.expr_infos {
            if !expr.is_assign() {
                result.push(expr);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DefaultEnvironment, Environment, LoxRunner};

    #[test]
    fn test1() {
        let srcs = vec![
            "x = y = a + b * c",
            "a = m + n",
            "b = a * 2",
            "c = n + w + b",
        ];
        println!("拓扑排序测试：");
        for expression in &srcs {
            println!("{}", expression);
        }

        let mut runner = LoxRunner::new();
        let exprs = runner.parse(&srcs).unwrap();
        let ana = Analyzer::new(exprs, true);
        let expr_infos = ana.analyze().unwrap();
        println!("排序结果为：");
        for info in &expr_infos {
            println!("{}", srcs[info.get_index()]);
        }
        assert!(expr_infos.len() == 4);
        assert_eq!("a = m + n", srcs[expr_infos[0].get_index()]);
        assert_eq!("b = a * 2", srcs[expr_infos[1].get_index()]);
        assert_eq!("c = n + w + b", srcs[expr_infos[2].get_index()]);
        assert_eq!("x = y = a + b * c", srcs[expr_infos[3].get_index()]);

        runner = LoxRunner::new();
        let mut env = DefaultEnvironment::new();
        env.put("m".to_string(), 2.into());
        env.put("n".to_string(), 4.into());
        env.put("w".to_string(), 6.into());
        runner.execute_multiple_with_env(&srcs, &mut env).unwrap();
        assert_eq!(270, env.get("x").unwrap().as_integer());
        assert_eq!(270, env.get("y").unwrap().as_integer());
        assert_eq!(6, env.get("a").unwrap().as_integer());
        assert_eq!(12, env.get("b").unwrap().as_integer());
        assert_eq!(22, env.get("c").unwrap().as_integer());
        println!("==========");
    }

    #[test]
    fn test2() {
        let srcs = vec![
            "b * 2 + 1",
            "a * b + c",
            "x = y = a + b * c",
            "a = m + n",
            "b = a * 2",
            "c = n + w + b",
        ];
        println!("拓扑排序测试：");
        for expression in &srcs {
            println!("{}", expression);
        }

        let mut runner = LoxRunner::new();
        let exprs = runner.parse(&srcs).unwrap();
        let ana = Analyzer::new(exprs, true);
        let expr_infos = ana.analyze().unwrap();
        println!("排序结果为：");
        for info in &expr_infos {
            println!("{}", srcs[info.get_index()]);
        }
        assert!(expr_infos.len() == 6);
        assert_eq!("a = m + n", srcs[expr_infos[0].get_index()]);
        assert_eq!("b = a * 2", srcs[expr_infos[1].get_index()]);
        assert_eq!("c = n + w + b", srcs[expr_infos[2].get_index()]);
        assert_eq!("x = y = a + b * c", srcs[expr_infos[3].get_index()]);
        assert_eq!("b * 2 + 1", srcs[expr_infos[4].get_index()]);
        assert_eq!("a * b + c", srcs[expr_infos[5].get_index()]);

        runner = LoxRunner::new();
        let mut env = DefaultEnvironment::new();
        env.put("m".to_string(), 2.into());
        env.put("n".to_string(), 4.into());
        env.put("w".to_string(), 6.into());
        let result = runner.execute_multiple_with_env(&srcs, &mut env).unwrap();
        assert_eq!(270, env.get("x").unwrap().as_integer());
        assert_eq!(270, env.get("y").unwrap().as_integer());
        assert_eq!(6, env.get("a").unwrap().as_integer());
        assert_eq!(12, env.get("b").unwrap().as_integer());
        assert_eq!(22, env.get("c").unwrap().as_integer());

        assert_eq!(12 * 2 + 1, result[0].as_integer());
        assert_eq!(6 * 12 + 22, result[1].as_integer());
        assert_eq!(270, result[2].as_integer());
        assert_eq!(6, result[3].as_integer());
        assert_eq!(12, result[4].as_integer());
        assert_eq!(22, result[5].as_integer());
        println!("==========");
    }
}
