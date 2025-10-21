use crate::Field;
use crate::RspResult;
use crate::Value;
use crate::chunk::Chunk;
use crate::environment::{DefaultEnvironment, Environment};
use crate::expr::Expr;
use crate::ir::{Analyzer, ExprInfo};
use crate::parser::Parser;
use crate::visitors::{Evaluator, OpCodeCompiler};
use crate::vm::VM;

use std::collections::HashSet;
use std::rc::Rc;

pub struct RspRunner {
    need_sort: bool,
    execute_mode: ExecuteMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecuteMode {
    SyntaxTree,
    ChunkVM,
}

impl RspRunner {
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

    pub fn execute(&mut self, expression: &str) -> RspResult<Value> {
        let mut env = DefaultEnvironment::new();
        self.execute_with_env(expression, &mut env)
    }

    pub fn execute_with_env<E: Environment>(
        &mut self,
        expression: &str,
        env: &mut E,
    ) -> RspResult<Value> {
        let results = self.execute_multiple_with_env(&[expression], env)?;
        Ok(results.into_iter().next().unwrap_or(Value::Null))
    }

    pub fn execute_multiple(&mut self, expressions: &[&str]) -> RspResult<Vec<Value>> {
        let mut env = DefaultEnvironment::new();
        self.execute_multiple_with_env(expressions, &mut env)
    }

    pub fn execute_multiple_with_env<E: Environment>(
        &mut self,
        expressions: &[&str],
        env: &mut E,
    ) -> RspResult<Vec<Value>> {
        let exprs = self.parse(expressions)?;

        let ana = Analyzer::new(exprs, self.need_sort);
        let expr_infos = ana.analyze()?;

        let results = if self.execute_mode == ExecuteMode::ChunkVM {
            let chunk = self.compile_ir(&expr_infos)?;
            self.run_chunk(&chunk, env)
        } else {
            self.run_ir(&expr_infos, env)
        };
        results
    }

    pub fn run_ir<E: Environment>(
        &mut self,
        expr_infos: &[&ExprInfo],
        env: &mut E,
    ) -> RspResult<Vec<Value>> {
        // let mut variables = HashSet::new();
        // for info in expr_infos {
        //     variables.union(info.get_reads());
        //     variables.union(info.get_writes());
        // }
        // let fields = self.get_fields(&variables);
        // let flag = env.before_execute(variables.into_iter().collect());
        // if !flag {
        //     return Ok(Vec::new());
        // }

        let n = expr_infos.len();
        let mut result = vec![Value::default(); n];
        for info in expr_infos {
            let expr = info.get_expr();
            let mut evtor = Evaluator::new(env);
            let v = evtor.evaluate(expr)?;
            result[info.get_index()] = v;
        }
        Ok(result)
    }

    pub fn run_chunk<E: Environment>(
        &mut self,
        chunk: &Chunk,
        env: &mut E,
    ) -> RspResult<Vec<Value>> {
        // let chunk_reader = ChunkReader::new(chunk, self.context.get_tracer());
        // let fields = self.get_fields(&chunk_reader.get_variables());
        // let flag = env.before_execute(&strs.iter().map(|str| str).collect());
        // if !flag {
        //     return None;
        // }

        let mut vm = VM::new();
        let ex_results = vm.execute_with_env(chunk, env)?;
        let mut result = vec![Value::default(); ex_results.len()];
        for res in ex_results {
            let r = res.result;
            let index = res.index;
            result[index as usize] = r;
        }
        Ok(result)
    }

    pub fn parse<'a>(&mut self, expressions: &[&'a str]) -> RspResult<Vec<Expr<'a>>> {
        let mut exprs = Vec::new();
        for expr in expressions {
            let mut parser = Parser::new(expr);
            let expr = parser.parse()?;
            exprs.push(expr);
        }
        Ok(exprs)
    }

    pub fn compile_source(&mut self, expressions: &[&str]) -> RspResult<Chunk> {
        let exprs = self.parse(expressions)?;
        let ana = Analyzer::new(exprs, self.need_sort);
        let expr_infos = ana.analyze()?;
        self.compile_ir(&expr_infos)
    }

    pub fn compile_ir(&mut self, expr_infos: &[&ExprInfo]) -> RspResult<Chunk> {
        let mut compiler = OpCodeCompiler::new();
        compiler.begin_compile();
        for expr_info in expr_infos {
            compiler.compile(expr_info).unwrap();
        }
        let result = compiler.end_compile();
        Ok(result)
    }

    fn _get_fields(&self, strs: &HashSet<String>) -> Vec<Rc<Field>> {
        strs.iter().map(|str| Field::with_str(str)).collect()
    }
}

impl Default for RspRunner {
    fn default() -> Self {
        Self::new()
    }
}
