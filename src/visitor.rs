use crate::environment::Environment;
use crate::error::{LoxError, LoxResult};
use crate::expr::*;
use crate::token::TokenType;
use crate::value::Value;

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> LoxResult<T>;
}

pub struct Evaluator<'a, E: Environment> {
    environment: &'a mut E,
}

impl<'a, E: Environment> Evaluator<'a, E> {
    pub fn new(environment: &'a mut E) -> Self {
        Self { environment }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> LoxResult<Value> {
        match expr {
            Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
            }) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary(left_val, right_val, &operator.token_type)
            }
            Expr::Unary(UnaryExpr { operator, right }) => {
                let right_val = self.evaluate(right)?;
                self.evaluate_unary(right_val, &operator.token_type)
            }
            Expr::Literal(LiteralExpr { value }) => Ok(value.clone()),
            Expr::Id(IdExpr { name }) => self.environment.get_or_default(&name.lexeme, Value::Null),
            Expr::Assign(AssignExpr {
                left,
                operator,
                right,
            }) => {
                if let Expr::Id(IdExpr { name }) = &**left {
                    // Variable assignment
                    let value = self.evaluate(right)?;
                    self.environment.put(name.lexeme.clone(), value.clone())?;
                    return Ok(value);
                } else {
                    Err(LoxError::RuntimeError {
                        message: "Invalic assign expression".to_string(),
                    })
                }
            }
            Expr::Logic(LogicExpr {
                left,
                operator,
                right,
            }) => {
                let left_val = self.evaluate(left)?;

                match operator.token_type {
                    TokenType::Or => {
                        if left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    TokenType::And => {
                        if !left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    _ => Err(crate::error::LoxError::RuntimeError {
                        message: "Invalid logical operator".to_string(),
                    }),
                }
            }
            Expr::Call(CallExpr {
                callee, arguments, ..
            }) => {
                let callee_val = self.evaluate(callee)?;
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate(arg)?);
                }
                self.call_function(callee_val, arg_values)
            }
            Expr::Get(GetExpr { object, name }) => {
                let object_val = self.evaluate(object)?;
                if let Some(instance) = object_val.as_instance() {
                    Ok(instance.get(&name.lexeme))
                } else {
                    Err(crate::error::LoxError::RuntimeError {
                        message: "Only instances have properties".to_string(),
                    })
                }
            }
            Expr::Set(SetExpr {
                object,
                name,
                value,
            }) => {
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
            Expr::If(IfExpr {
                condition,
                then_branch,
                else_branch,
            }) => {
                let condition_val = self.evaluate(condition)?;
                if condition_val.is_truthy() {
                    self.evaluate(then_branch)
                } else if let Some(else_expr) = else_branch {
                    self.evaluate(else_expr)
                } else {
                    Ok(Value::Null)
                }
            }
        }
    }

    fn evaluate_binary(&self, left: Value, right: Value, operator: &TokenType) -> LoxResult<Value> {
        match operator {
            TokenType::Plus => {
                if !left.is_number() && !left.is_string()
                    || !right.is_number() && !right.is_string()
                {
                    return Err(crate::error::LoxError::RuntimeError {
                        message: "Operands must be number or string".to_string(),
                    });
                }
                if left.is_string() || right.is_string() {
                    Ok(Value::String(format!(
                        "{}{}",
                        left.as_string(),
                        right.as_string()
                    )))
                } else {
                    if left.is_double() || right.is_double() {
                        Ok(Value::Double(left.as_double() + right.as_double()))
                    } else {
                        Ok(Value::Integer(left.as_integer() + right.as_integer()))
                    }
                }
            }
            TokenType::Minus => {
                self.check_number_operands(&left, &right)?;
                if left.is_double() || right.is_double() {
                    Ok(Value::Double(left.as_double() - right.as_double()))
                } else {
                    Ok(Value::Integer(left.as_integer() - right.as_integer()))
                }
            }
            TokenType::Star => {
                self.check_number_operands(&left, &right)?;
                if left.is_double() || right.is_double() {
                    Ok(Value::Double(left.as_double() * right.as_double()))
                } else {
                    Ok(Value::Integer(left.as_integer() * right.as_integer()))
                }
            }
            TokenType::Slash => {
                self.check_number_operands(&left, &right)?;
                if right.is_integer() && right.as_integer() == 0 {
                    return Err(crate::error::LoxError::RuntimeError {
                        message: "Division by zero".to_string(),
                    });
                }
                if left.is_double() || right.is_double() {
                    Ok(Value::Double(left.as_double() / right.as_double()))
                } else {
                    Ok(Value::Integer(left.as_integer() / right.as_integer()))
                }
            }
            TokenType::Percent => {
                self.check_number_operands(&left, &right)?;
                if left.is_double() || right.is_double() {
                    Ok(Value::Double(left.as_double() % right.as_double()))
                } else {
                    Ok(Value::Integer(left.as_integer() % right.as_integer()))
                }
            }
            TokenType::StarStar => {
                self.check_number_operands(&left, &right)?;
                Ok(Value::Double(left.as_double().powf(right.as_double())))
            }
            TokenType::Greater => {
                self.check_number_operands(&left, &right)?;
                Ok(Value::Boolean(left.as_double() > right.as_double()))
            }
            TokenType::GreaterEqual => {
                self.check_number_operands(&left, &right)?;
                Ok(Value::Boolean(left.as_double() >= right.as_double()))
            }
            TokenType::Less => {
                self.check_number_operands(&left, &right)?;
                Ok(Value::Boolean(left.as_double() < right.as_double()))
            }
            TokenType::LessEqual => {
                self.check_number_operands(&left, &right)?;
                Ok(Value::Boolean(left.as_double() <= right.as_double()))
            }
            TokenType::BangEqual => Ok(Value::Boolean(!self.is_equal(left, right))),
            TokenType::EqualEqual => Ok(Value::Boolean(self.is_equal(left, right))),
            _ => Err(crate::error::LoxError::RuntimeError {
                message: "Invalid binary operator".to_string(),
            }),
        }
    }

    fn evaluate_unary(&self, right: Value, operator: &TokenType) -> LoxResult<Value> {
        match operator {
            TokenType::Bang => {
                let truthy = right.is_truthy();
                Ok(Value::Boolean(!truthy))
            }
            TokenType::Minus => {
                self.check_number_operand(&right)?;
                if right.is_integer() {
                    Ok(Value::Integer(-right.as_integer()))
                } else {
                    Ok(Value::Double(-right.as_double()))
                }
            }
            _ => Err(crate::error::LoxError::RuntimeError {
                message: "Invalid unary operator".to_string(),
            }),
        }
    }

    fn check_number_operand(&self, operand: &Value) -> LoxResult<()> {
        if operand.is_number() {
            Ok(())
        } else {
            Err(crate::error::LoxError::RuntimeError {
                message: "Operand must be a number".to_string(),
            })
        }
    }

    fn check_number_operands(&self, left: &Value, right: &Value) -> LoxResult<()> {
        if left.is_number() && right.is_number() {
            Ok(())
        } else {
            Err(crate::error::LoxError::RuntimeError {
                message: format!("Operands must be numbers. left: {}, right: {}", left, right),
            })
        }
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
        a == b
    }

    fn call_function(&self, _callee: Value, _arguments: Vec<Value>) -> LoxResult<Value> {
        // For now, we'll implement basic function calling
        // In a full implementation, this would handle built-in functions
        Err(crate::error::LoxError::RuntimeError {
            message: "Function calling not implemented".to_string(),
        })
    }
}

impl<'a, E: Environment> Visitor<Value> for Evaluator<'a, E> {
    fn visit_expr(&mut self, expr: &Expr) -> LoxResult<Value> {
        self.evaluate(expr)
    }
}
