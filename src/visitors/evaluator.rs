use crate::environment::Environment;
use crate::error::{LoxError, LoxResult};

use crate::TokenType;
use crate::expr::Visitor;
use crate::values::{Value, value_helper};

use crate::expr::{
    AssignExpr, BinaryExpr, CallExpr, Expr, GetExpr, IdExpr, IfExpr, LiteralExpr, LogicExpr,
    SetExpr, UnaryExpr,
};

pub struct Evaluator<'a, E: Environment> {
    environment: &'a mut E,
}

impl<'a, E: Environment> Evaluator<'a, E> {
    pub fn new(environment: &'a mut E) -> Self {
        Self { environment }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> LoxResult<Value> {
        expr.accept(self)
    }

    fn call_function(&self, _callee: Value, _arguments: Vec<Value>) -> LoxResult<Value> {
        // For now, we'll implement basic function calling
        // In a full implementation, this would handle built-in functions
        Err(crate::error::LoxError::RuntimeError {
            message: "Function calling not implemented".to_string(),
        })
    }
}

impl<'a, E: Environment> Visitor<LoxResult<Value>> for Evaluator<'a, E> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> LoxResult<Value> {
        let BinaryExpr {
            left,
            operator,
            right,
        } = expr;
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;
        value_helper::evaluate_binary(&left_val, &right_val, &operator.token_type)
    }

    fn visit_logic(&mut self, expr: &LogicExpr) -> LoxResult<Value> {
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
            _ => Err(crate::error::LoxError::RuntimeError {
                message: "Invalid logical operator".to_string(),
            }),
        }
    }

    fn visit_literal(&mut self, expr: &LiteralExpr) -> LoxResult<Value> {
        let LiteralExpr { value } = expr;
        Ok(value.clone())
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> LoxResult<Value> {
        let UnaryExpr { operator, right } = expr;
        let right_val = self.evaluate(right)?;
        value_helper::evaluate_unary(&right_val, &operator.token_type)
    }

    fn visit_id(&mut self, expr: &IdExpr) -> LoxResult<Value> {
        let IdExpr { name } = expr;
        Ok(self
            .environment
            .get_or_default(&name.lexeme, &Value::Null)
            .unwrap()
            .clone())
    }

    fn visit_assign(&mut self, expr: &AssignExpr) -> LoxResult<Value> {
        let AssignExpr { left, right, .. } = expr;
        if let Expr::Id(IdExpr { name }) = &**left {
            // Variable assignment
            let value = self.evaluate(right)?;
            self.environment.put(name.lexeme.clone(), value.clone());
            return Ok(value);
        } else {
            Err(LoxError::RuntimeError {
                message: "Invalic assign expression".to_string(),
            })
        }
    }

    fn visit_call(&mut self, expr: &CallExpr) -> LoxResult<Value> {
        let CallExpr {
            callee, arguments, ..
        } = expr;
        let callee_val = self.evaluate(callee)?;
        let mut arg_values = Vec::new();
        for arg in arguments {
            arg_values.push(self.evaluate(arg)?);
        }
        self.call_function(callee_val, arg_values)
    }

    fn visit_if(&mut self, expr: &IfExpr) -> LoxResult<Value> {
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

    fn visit_get(&mut self, expr: &GetExpr) -> LoxResult<Value> {
        let GetExpr { object, name } = expr;
        let object_val = self.evaluate(object)?;
        if let Some(instance) = object_val.as_instance() {
            Ok(instance.get(&name.lexeme).unwrap().clone())
        } else {
            Err(crate::error::LoxError::RuntimeError {
                message: "Only instances have properties".to_string(),
            })
        }
    }

    fn visit_set(&mut self, expr: &SetExpr) -> LoxResult<Value> {
        let SetExpr {
            object,
            name,
            value,
        } = expr;

        let mut object_val = self.evaluate(object)?;
        let value_val = self.evaluate(value)?;
        if let Some(instance) = object_val.as_instance_mut() {
            instance.set(name.lexeme.clone(), value_val.clone());
            Ok(value_val)
        } else {
            Err(crate::error::LoxError::RuntimeError {
                message: "Only instances have fields".to_string(),
            })
        }
    }
}
