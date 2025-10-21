use crate::expr::{AssignExpr, Expr, SetExpr};
use crate::visitors::{VariableSet, VarsQuery};
use std::collections::HashSet;

pub struct ExprInfo<'a> {
    reads: HashSet<String>,  // 依赖的变量 read
    writes: HashSet<String>, // 被赋值的变量 write
    expr: Expr<'a>,
    index: usize,
}

impl<'a> ExprInfo<'a> {
    pub fn new(expr: Expr<'a>, index: usize) -> Self {
        let mut info = ExprInfo {
            reads: HashSet::new(),
            writes: HashSet::new(),
            expr,
            index,
        };
        info.init_variables();
        info
    }

    fn init_variables(&mut self) {
        let mut var_query = VarsQuery::new();
        if let Some(var_set) = var_query.execute(&self.expr) {
            self.reads = var_set.get_depends().clone();
            self.writes = var_set.get_assigns().clone();
        }
    }

    pub fn is_assign(&self) -> bool {
        matches!(self.expr, Expr::Assign(_) | Expr::Set(_))
    }

    pub fn get_reads(&self) -> &HashSet<String> {
        &self.reads
    }

    pub fn get_writes(&self) -> &HashSet<String> {
        &self.writes
    }

    pub fn get_expr(&self) -> &Expr<'a> {
        &self.expr
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}
