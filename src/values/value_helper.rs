use crate::RspResult;
use crate::TokenType;
use crate::Value;

pub fn evaluate_binary(left: &Value, right: &Value, operator: &TokenType) -> RspResult<Value> {
    match operator {
        TokenType::Plus => {
            if !left.is_number() && !left.is_string() || !right.is_number() && !right.is_string() {
                return Err(crate::error::RspError::RuntimeError {
                    message: "Operands must be number or string".to_string(),
                });
            }
            if left.is_string() || right.is_string() {
                Ok(Value::String(format!(
                    "{}{}",
                    left.as_str(),
                    right.as_str()
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
            check_number_operands(left, right)?;
            if left.is_double() || right.is_double() {
                Ok(Value::Double(left.as_double() - right.as_double()))
            } else {
                Ok(Value::Integer(left.as_integer() - right.as_integer()))
            }
        }
        TokenType::Star => {
            check_number_operands(left, right)?;
            if left.is_double() || right.is_double() {
                Ok(Value::Double(left.as_double() * right.as_double()))
            } else {
                Ok(Value::Integer(left.as_integer() * right.as_integer()))
            }
        }
        TokenType::Slash => {
            check_number_operands(left, right)?;
            if right.is_integer() && right.as_integer() == 0 {
                return Err(crate::error::RspError::RuntimeError {
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
            check_number_operands(left, right)?;
            if left.is_double() || right.is_double() {
                Ok(Value::Double(left.as_double() % right.as_double()))
            } else {
                Ok(Value::Integer(left.as_integer() % right.as_integer()))
            }
        }
        TokenType::StarStar => {
            check_number_operands(left, right)?;
            Ok(Value::Double(left.as_double().powf(right.as_double())))
        }
        TokenType::Greater => {
            check_number_operands(left, right)?;
            Ok(Value::Boolean(left.as_double() > right.as_double()))
        }
        TokenType::GreaterEqual => {
            check_number_operands(left, right)?;
            Ok(Value::Boolean(left.as_double() >= right.as_double()))
        }
        TokenType::Less => {
            check_number_operands(left, right)?;
            Ok(Value::Boolean(left.as_double() < right.as_double()))
        }
        TokenType::LessEqual => {
            check_number_operands(left, right)?;
            Ok(Value::Boolean(left.as_double() <= right.as_double()))
        }
        TokenType::BangEqual => Ok(Value::Boolean(!left.equals(right))),
        TokenType::EqualEqual => Ok(Value::Boolean(left.equals(right))),
        _ => Err(crate::error::RspError::RuntimeError {
            message: "Invalid binary operator".to_string(),
        }),
    }
}

pub fn evaluate_unary(right: &Value, operator: &TokenType) -> RspResult<Value> {
    match operator {
        TokenType::Bang => {
            let truthy = right.is_truthy();
            Ok(Value::Boolean(!truthy))
        }
        TokenType::Minus => {
            check_number_operand(&right)?;
            if right.is_integer() {
                Ok(Value::Integer(-right.as_integer()))
            } else {
                Ok(Value::Double(-right.as_double()))
            }
        }
        _ => Err(crate::error::RspError::RuntimeError {
            message: "Invalid unary operator".to_string(),
        }),
    }
}

fn check_number_operand(operand: &Value) -> RspResult<()> {
    if operand.is_number() {
        Ok(())
    } else {
        Err(crate::error::RspError::RuntimeError {
            message: "Operand must be a number".to_string(),
        })
    }
}

fn check_number_operands(left: &Value, right: &Value) -> RspResult<()> {
    if left.is_number() && right.is_number() {
        Ok(())
    } else {
        Err(crate::error::RspError::RuntimeError {
            message: format!("Operands must be numbers. left: {}, right: {}", left, right),
        })
    }
}
