use crate::environment::{DefaultEnvironment, Environment};
use crate::expr::Expr;
use crate::ir::{Analyzer, ExprInfo};
use crate::parser::Parser;
use crate::values::Value;
use crate::visitors::Evaluator;
use crate::{LoxError, LoxResult};

pub struct LoxRunner {
    need_sort: bool,
    execute_mode: ExecuteMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecuteMode {
    SyntaxTree,
    ChunkVM,
}

impl LoxRunner {
    pub fn new() -> Self {
        Self {
            need_sort: true,
            execute_mode: ExecuteMode::SyntaxTree,
        }
    }

    pub fn set_need_sort(&mut self, need_sort: bool) {
        self.need_sort = need_sort;
    }

    pub fn set_execute_mode(&mut self, mode: ExecuteMode) {
        self.execute_mode = mode;
    }

    pub fn execute(&mut self, expression: &str) -> LoxResult<Value> {
        let mut env = DefaultEnvironment::new();
        self.execute_with_env(expression, &mut env)
    }

    pub fn execute_with_env<E: Environment>(
        &mut self,
        expression: &str,
        env: &mut E,
    ) -> LoxResult<Value> {
        let results = self.execute_multiple_with_env(&[expression], env)?;
        Ok(results.into_iter().next().unwrap_or(Value::Null))
    }

    pub fn execute_multiple(&mut self, expressions: &[&str]) -> LoxResult<Vec<Value>> {
        let mut env = DefaultEnvironment::new();
        self.execute_multiple_with_env(expressions, &mut env)
    }

    pub fn execute_multiple_with_env<E: Environment>(
        &mut self,
        expressions: &[&str],
        env: &mut E,
    ) -> LoxResult<Vec<Value>> {
        let exprs = self.parse(expressions)?;

        let ana = Analyzer::new(exprs, self.need_sort);
        let expr_infos = ana.analyze()?;

        // Execute expressions
        let mut results = Vec::new();
        let mut evaluator = Evaluator::new(env);

        for expr_info in expr_infos {
            let result = evaluator.evaluate(expr_info.get_expr())?;
            results.push(result);
        }

        Ok(results)
    }

    pub fn parse(&mut self, expressions: &[&str]) -> LoxResult<Vec<Expr>> {
        let mut exprs = Vec::new();
        for expr_str in expressions {
            let expr = expr_str.to_string();
            let mut parser = Parser::new(&expr);
            let expr = parser.parse()?;
            exprs.push(expr);
        }
        Ok(exprs)
    }
}

impl Default for LoxRunner {
    fn default() -> Self {
        Self::new()
    }
}
