use crate::expr::{AssignExpr, Expr, SetExpr};
use crate::visitors::{VariableSet, VarsQuery};
use std::collections::HashSet;

pub struct ExprInfo {
    precursors: HashSet<String>, // 依赖的变量 read
    successors: HashSet<String>, // 被赋值的变量 write
    expr: Expr,
    index: usize,
}

impl ExprInfo {
    pub fn new(expr: Expr, index: usize) -> Self {
        let mut info = ExprInfo {
            precursors: HashSet::new(),
            successors: HashSet::new(),
            expr,
            index,
        };
        info.init_variables();
        info
    }

    fn init_variables(&mut self) {
        let mut var_query = VarsQuery::new();
        if let Some(var_set) = var_query.execute(&self.expr) {
            self.precursors = var_set.get_depends().clone();
            self.successors = var_set.get_assigns().clone();
        }
    }

    pub fn is_assign(&self) -> bool {
        matches!(self.expr, Expr::Assign(_) | Expr::Set(_))
    }

    pub fn get_precursors(&self) -> &HashSet<String> {
        &self.precursors
    }

    pub fn set_precursors(&mut self, precursors: HashSet<String>) {
        self.precursors = precursors;
    }

    pub fn get_successors(&self) -> &HashSet<String> {
        &self.successors
    }

    pub fn set_successors(&mut self, successors: HashSet<String>) {
        self.successors = successors;
    }

    pub fn get_expr(&self) -> &Expr {
        &self.expr
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}
