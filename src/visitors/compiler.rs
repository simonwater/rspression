use std::collections::HashSet;

use crate::{
    RspError, RspResult,
    chunk::{Chunk, ChunkWriter},
    expr::{
        AssignExpr, BinaryExpr, CallExpr, Expr, GetExpr, IdExpr, IfExpr, LiteralExpr, LogicExpr,
        SetExpr, UnaryExpr, Visitor,
    },
    functions::FunctionManager,
    ir::ExprInfo,
    parser::TokenType,
    values::Value,
    vm::OpCode,
};

pub struct OpCodeCompiler {
    chunk_writer: ChunkWriter,
    var_set: HashSet<String>,
    function_manager: FunctionManager,
}

impl OpCodeCompiler {
    pub fn new() -> Self {
        Self {
            chunk_writer: ChunkWriter::new(),
            var_set: HashSet::new(),
            function_manager: FunctionManager::new(),
        }
    }

    pub fn begin_compile(&mut self) {
        self.chunk_writer.clear();
        self.var_set.clear();
    }

    pub fn compile(&mut self, exprInfo: &ExprInfo) -> RspResult<()> {
        let expr = exprInfo.get_expr();
        let order = exprInfo.get_index();
        self.compile_expr(expr, order)?;
        self.var_set.extend(exprInfo.get_reads().clone());
        self.var_set.extend(exprInfo.get_writes().clone());
        Ok(())
    }

    pub fn compile_expr(&mut self, expr: &Expr, order: usize) -> RspResult<()> {
        self.emit_op_with_arg(OpCode::Begin, order as i32);
        self.execute(expr)?;
        self.emit_op(OpCode::End);
        Ok(())
    }

    pub fn end_compile(&mut self) -> Chunk {
        self.emit_op(OpCode::Exit);
        self.chunk_writer
            .set_variables(&self.var_set.iter().cloned().collect::<Vec<_>>());
        self.chunk_writer.flush()
    }

    fn execute(&mut self, expr: &Expr) -> RspResult<()> {
        expr.accept(self)
    }

    fn emit_op(&mut self, op: OpCode) {
        self.chunk_writer.write_code(op);
    }

    fn emit_op_with_arg(&mut self, op: OpCode, arg: i32) {
        self.chunk_writer.write_code(op);
        self.chunk_writer.write_int(arg as i32);
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.make_constant(value);
        self.emit_op_with_arg(OpCode::Constant, index as i32);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        self.chunk_writer.add_constant(value)
    }

    fn emit_jump(&mut self, jump_code: OpCode) -> usize {
        self.emit_op(jump_code);
        self.chunk_writer.write_int(0x3fffffff); // placeholder
        self.chunk_writer.position() - 4
    }

    fn patch_jump(&mut self, index: usize) {
        let offset = self.chunk_writer.position() - index - 4;
        self.chunk_writer.update_int(index, offset as i32);
    }
}

impl Visitor<RspResult<()>> for OpCodeCompiler {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> RspResult<()> {
        self.execute(&expr.left)?;
        self.execute(&expr.right)?;
        let op = match &expr.operator.token_type {
            TokenType::Plus => OpCode::Add,
            TokenType::Minus => OpCode::Subtract,
            TokenType::Star => OpCode::Multiply,
            TokenType::Slash => OpCode::Divide,
            TokenType::Percent => OpCode::Mode,
            TokenType::StarStar => OpCode::Power,
            TokenType::Greater => OpCode::Greater,
            TokenType::GreaterEqual => OpCode::GreaterEqual,
            TokenType::Less => OpCode::Less,
            TokenType::LessEqual => OpCode::LessEqual,
            TokenType::BangEqual => OpCode::BangEqual,
            TokenType::EqualEqual => OpCode::EqualEqual,
            t => {
                return Err(RspError::RuntimeError {
                    message: format!("Unknown binary operator: {:?}", t),
                });
            }
        };
        self.emit_op(op);
        Ok(())
    }

    fn visit_logic(&mut self, expr: &LogicExpr) -> RspResult<()> {
        self.execute(&expr.left)?;
        if expr.operator.token_type == TokenType::And {
            let jumper = self.emit_jump(OpCode::JumpIfFalse);
            self.emit_op(OpCode::Pop);
            self.execute(&expr.right)?;
            self.patch_jump(jumper);
        } else {
            let jumper1 = self.emit_jump(OpCode::JumpIfFalse);
            let jumper2 = self.emit_jump(OpCode::Jump);
            self.patch_jump(jumper1);
            self.emit_op(OpCode::Pop);
            self.execute(&expr.right)?;
            self.patch_jump(jumper2);
        }
        Ok(())
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> RspResult<()> {
        match expr.value {
            Value::Boolean(v) if v => self.emit_op(OpCode::True),
            Value::Boolean(v) if !v => self.emit_op(OpCode::False),
            Value::Null => self.emit_op(OpCode::Null),
            _ => self.emit_constant(expr.value.clone()),
        };
        Ok(())
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> RspResult<()> {
        self.execute(&expr.right)?;
        match &expr.operator.token_type {
            TokenType::Bang => self.emit_op(OpCode::Not),
            TokenType::Minus => self.emit_op(OpCode::Negate),
            t => {
                return Err(RspError::CompileError {
                    message: format!("unsupported unary operator: {:?}", t),
                });
            }
        }
        Ok(())
    }

    fn visit_id(&mut self, expr: &IdExpr) -> RspResult<()> {
        let constant = self.make_constant(Value::String(expr.name.lexeme.to_string()));
        self.emit_op_with_arg(OpCode::GetGlobal, constant as i32);
        Ok(())
    }

    fn visit_assign(&mut self, expr: &AssignExpr) -> RspResult<()> {
        self.execute(&expr.right)?;
        if let Expr::Id(id_expr) = &*expr.left {
            let constant = self.make_constant(Value::String(id_expr.name.lexeme.to_string()));
            self.emit_op_with_arg(OpCode::SetGlobal, constant as i32);
        }
        Ok(())
    }

    fn visit_call(&mut self, expr: &CallExpr) -> RspResult<()> {
        if let Expr::Id(id_expr) = &*expr.callee {
            let name = &id_expr.name.lexeme;
            let func = self
                .function_manager
                .get(name)
                .ok_or(RspError::CompileError {
                    message: format!("Undefined function: {}", name),
                })?;

            if func.arity() != expr.arguments.len() {
                return Err(RspError::CompileError {
                    message: format!(
                        "Expected {} arguments but got {} for function {}",
                        func.arity(),
                        expr.arguments.len(),
                        name
                    ),
                });
            }

            for arg in &expr.arguments {
                self.execute(arg)?;
            }
            let constant = self.make_constant(Value::String(name.to_string()));
            self.emit_op_with_arg(OpCode::Call, constant as i32);
        }
        Ok(())
    }

    fn visit_if(&mut self, expr: &IfExpr) -> RspResult<()> {
        self.execute(&expr.condition)?;
        let else_jumper = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_op(OpCode::Pop);
        self.execute(&expr.then_branch)?;
        let end_jumper = self.emit_jump(OpCode::Jump);
        self.patch_jump(else_jumper);
        self.emit_op(OpCode::Pop);
        if let Some(else_branch) = &expr.else_branch {
            self.execute(else_branch)?;
        } else {
            self.emit_op(OpCode::Null);
        }
        self.patch_jump(end_jumper);
        Ok(())
    }

    fn visit_get(&mut self, expr: &GetExpr) -> RspResult<()> {
        self.execute(&expr.object)?;
        let constant = self.make_constant(Value::String(expr.name.lexeme.to_string()));
        self.emit_op_with_arg(OpCode::GetProperty, constant as i32);
        Ok(())
    }

    fn visit_set(&mut self, expr: &SetExpr) -> RspResult<()> {
        self.execute(&expr.value)?;
        self.execute(&expr.object)?;
        let constant = self.make_constant(Value::String(expr.name.lexeme.to_string()));
        self.emit_op_with_arg(OpCode::SetProperty, constant as i32);
        Ok(())
    }
}
