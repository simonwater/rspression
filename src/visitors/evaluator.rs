use crate::environment::Environment;
use crate::error::{RspError, RspResult};
use crate::functions::{Callable, FunctionManager};
use crate::ir::ExprInfo;
use std::rc::Rc;

use crate::TokenType;
use crate::expr::Visitor;
use crate::values::{Value, value_helper};

use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GetExpr, IdExpr, IfExpr, LiteralExpr, LogicExpr,
    SetExpr, UnaryExpr,
};

pub struct Evaluator<'a, E: Environment> {
    environment: &'a mut E,
    function_manager: FunctionManager,
    expr_order: usize,
}

impl<'a, E: Environment> Evaluator<'a, E> {
    pub fn new(environment: &'a mut E) -> Self {
        Self {
            environment,
            function_manager: FunctionManager::new(),
            expr_order: 0,
        }
    }

    pub fn excute(&mut self, expr_info: &ExprInfo) -> RspResult<Value> {
        let expr = expr_info.get_expr();
        let order = expr_info.get_index();
        self.expr_order = order;
        self.evaluate(expr)
    }

    fn evaluate(&mut self, expr: &Expr) -> RspResult<Value> {
        expr.accept(self)
    }

    fn call_function(&mut self, name: &str, arguments: Vec<Value>) -> RspResult<Value> {
        let function = self.get_function(name)?;
        function.call(arguments, self.environment)
    }

    fn get_function(&mut self, name: &str) -> RspResult<Rc<dyn Callable>> {
        if let Some(value) = self.environment.get(name) {
            if let Some(function) = value.as_function() {
                return Ok(function);
            } else {
                return Err(RspError::RuntimeError {
                    message: format!(
                        "Value: {} is not callable, order: {}",
                        name, self.expr_order
                    ),
                });
            }
        } else if let Some(function) = self.function_manager.get(name) {
            return Ok(function);
        } else {
            return Err(RspError::RuntimeError {
                message: format!("Undefined function: {}, order: {}", name, self.expr_order),
            });
        }
    }
}

impl<'a, E: Environment> Visitor<RspResult<Value>> for Evaluator<'a, E> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> RspResult<Value> {
        let BinaryExpr {
            left,
            operator,
            right,
        } = expr;
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;
        value_helper::evaluate_binary(&left_val, &right_val, &operator.token_type)
    }

    fn visit_logic(&mut self, expr: &LogicExpr) -> RspResult<Value> {
        let LogicExpr {
            left,
            operator,
            right,
        } = expr;

        let left_val = self.evaluate(left)?;
        match operator.token_type {
            TokenType::Or => {
                if left_val.is_truthy() {
                    Ok(Value::Boolean(true))
                } else {
                    self.evaluate(right)
                }
            }
            TokenType::And => {
                if !left_val.is_truthy() {
                    Ok(Value::Boolean(false))
                } else {
                    self.evaluate(right)
                }
            }
            _ => Err(crate::error::RspError::RuntimeError {
                message: format!("Invalid logical operator, order: {}", self.expr_order),
            }),
        }
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> RspResult<Value> {
        let LiteralExpr { value } = expr;
        Ok(value.clone())
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> RspResult<Value> {
        let UnaryExpr { operator, right } = expr;
        let right_val = self.evaluate(right)?;
        value_helper::evaluate_unary(&right_val, &operator.token_type)
    }

    fn visit_id(&mut self, expr: &IdExpr) -> RspResult<Value> {
        let IdExpr { name } = expr;
        Ok(self
            .environment
            .get(&name.lexeme)
            .unwrap_or(&Value::Null)
            .clone())
    }

    fn visit_assign(&mut self, expr: &AssignExpr) -> RspResult<Value> {
        let AssignExpr { left, right, .. } = expr;
        if let Expr::Id(IdExpr { name }) = &**left {
            // Variable assignment
            let value = self.evaluate(right)?;
            self.environment.put(name.lexeme.into(), value.clone());
            return Ok(value);
        } else {
            Err(RspError::RuntimeError {
                message: format!("Invalic assign expression, order: {}", self.expr_order),
            })
        }
    }

    fn visit_call(&mut self, expr: &CallExpr) -> RspResult<Value> {
        if let Expr::Id(id_expr) = &*expr.callee {
            let name = id_expr.name.lexeme;
            let mut args = Vec::new();
            for arg in &expr.arguments {
                args.push(self.evaluate(arg)?);
            }
            self.call_function(name, args)
        } else {
            Err(RspError::RuntimeError {
                message: format!(
                    "Invalic function call expression, order: {}",
                    self.expr_order
                ),
            })
        }
    }

    fn visit_if(&mut self, expr: &IfExpr) -> RspResult<Value> {
        let IfExpr {
            condition,
            then_branch,
            else_branch,
        } = expr;

        let condition_val = self.evaluate(condition)?;
        if condition_val.is_truthy() {
            self.evaluate(then_branch)
        } else if let Some(else_expr) = else_branch {
            self.evaluate(else_expr)
        } else {
            Ok(Value::Null)
        }
    }

    fn visit_get(&mut self, expr: &GetExpr) -> RspResult<Value> {
        let GetExpr { object, name } = expr;
        let object_val = self.evaluate(object)?;
        if let Some(o) = object_val.as_object() {
            match o.borrow().get(name.lexeme) {
                Some(val) => Ok(val.clone()),
                None => Ok(Value::Null),
            }
        } else {
            Err(crate::error::RspError::RuntimeError {
                message: format!("Only instances have properties, order: {}", self.expr_order),
            })
        }
    }

    fn visit_set(&mut self, expr: &SetExpr) -> RspResult<Value> {
        let SetExpr {
            object,
            name,
            value,
        } = expr;

        let object_val = self.evaluate(object)?;
        let value_val = self.evaluate(value)?;
        if let Some(instance) = object_val.as_object() {
            instance
                .borrow_mut()
                .insert(name.lexeme.to_string(), value_val.clone());
            Ok(value_val)
        } else {
            Err(crate::error::RspError::RuntimeError {
                message: format!("Only objects have fields, order: {}", self.expr_order),
            })
        }
    }
}
