use super::digraph::{Digraph, TopologicalSort};
use crate::expr::Expr;
use crate::ir::ExprInfo;
use crate::ir::node_set::NodeSet;
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
        for (i, expr_info) in expr_infos.iter().enumerate() {
            if !expr_info.is_assign() {
                continue;
            }
            let mut flag = true;
            for name in expr_info.get_successors() {
                if flag {
                    node_set.add_node_with_info(name, Some(i));
                    flag = true;
                } else {
                    node_set.add_node_with_info(name, None);
                }
            }

            for name in expr_info.get_precursors() {
                node_set.add_node(name);
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
            for prec in info.get_precursors() {
                let pre_node = node_set.get_node_by_name(prec).expect("pre node not found");
                let u = pre_node.index;
                for succ in info.get_successors() {
                    let succ_node = node_set
                        .get_node_by_name(succ)
                        .expect("succ node not found");
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
            let node = self.node_set.get_node_by_index(node_index);
            if let Some(ref index) = node.info {
                let info = &self.expr_infos[*index];
                result.push(info);
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
