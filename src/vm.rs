use crate::chunk::Chunk;
use crate::environment::Environment;
use crate::error::LoxResult;
use crate::values::Value;

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    chunk: Option<Chunk>,
    ip: usize, // instruction pointer
}

impl VM {
    const STACK_MAX: usize = 256;

    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(Self::STACK_MAX),
            chunk: None,
            ip: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk, env: &mut dyn Environment) -> LoxResult<Vec<Value>> {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.stack.clear();

        let results = Vec::new();

        loop {
            let instruction = self.read_byte()?;
            match instruction {
                0 => {
                    // OP_CONSTANT
                    let constant_index = self.read_byte()? as usize;
                    let constant = self.get_constant(constant_index)?;
                    self.push(constant.clone());
                }
                1 => {
                    // OP_NULL
                    self.push(Value::Null);
                }
                2 => {
                    // OP_TRUE
                    self.push(Value::Boolean(true));
                }
                3 => {
                    // OP_FALSE
                    self.push(Value::Boolean(false));
                }
                4 => {
                    // OP_POP
                    self.pop()?;
                }
                7 => {
                    // OP_GET_GLOBAL
                    let name = self.read_string()?;
                    let value = env.get(&name)?;
                    self.push(value);
                }
                9 => {
                    // OP_SET_GLOBAL
                    let name = self.read_string()?;
                    let value = self.peek()?;
                    env.put(name, value.clone())?;
                }
                18 => {
                    // OP_ADD
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = self.binary_op(a, b, "+")?;
                    self.push(result);
                }
                19 => {
                    // OP_SUBTRACT
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = self.binary_op(a, b, "-")?;
                    self.push(result);
                }
                20 => {
                    // OP_MULTIPLY
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = self.binary_op(a, b, "*")?;
                    self.push(result);
                }
                21 => {
                    // OP_DIVIDE
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = self.binary_op(a, b, "/")?;
                    self.push(result);
                }
                32 => {
                    // OP_EXIT
                    break;
                }
                _ => {
                    return Err(crate::error::LoxError::RuntimeError {
                        message: format!("Unknown opcode: {}", instruction),
                    });
                }
            }
        }

        Ok(results)
    }

    fn read_byte(&mut self) -> LoxResult<u8> {
        let chunk = self
            .chunk
            .as_ref()
            .ok_or_else(|| crate::error::LoxError::RuntimeError {
                message: "No chunk loaded".to_string(),
            })?;

        let byte =
            chunk
                .read_byte(self.ip)
                .ok_or_else(|| crate::error::LoxError::RuntimeError {
                    message: "Invalid instruction".to_string(),
                })?;

        self.ip += 1;
        Ok(byte)
    }

    fn get_constant(&self, index: usize) -> LoxResult<&Value> {
        let chunk = self
            .chunk
            .as_ref()
            .ok_or_else(|| crate::error::LoxError::RuntimeError {
                message: "No chunk loaded".to_string(),
            })?;

        chunk
            .read_constant(index)
            .ok_or_else(|| crate::error::LoxError::RuntimeError {
                message: "Invalid constant index".to_string(),
            })
    }

    fn read_string(&mut self) -> LoxResult<String> {
        let constant_index = self.read_byte()? as usize;
        let constant = self.get_constant(constant_index)?;
        Ok(constant.as_string())
    }

    fn push(&mut self, value: Value) {
        if self.stack.len() >= Self::STACK_MAX {
            panic!("Stack overflow");
        }
        self.stack.push(value);
    }

    fn pop(&mut self) -> LoxResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| crate::error::LoxError::RuntimeError {
                message: "Stack underflow".to_string(),
            })
    }

    fn peek(&self) -> LoxResult<&Value> {
        self.stack
            .last()
            .ok_or_else(|| crate::error::LoxError::RuntimeError {
                message: "Stack underflow".to_string(),
            })
    }

    fn binary_op(&self, a: Value, b: Value, op: &str) -> LoxResult<Value> {
        match op {
            "+" => {
                if a.is_number() && b.is_number() {
                    Ok(Value::Double(a.as_double() + b.as_double()))
                } else if a.is_string() || b.is_string() {
                    Ok(Value::String(format!("{}{}", a.as_string(), b.as_string())))
                } else {
                    Err(crate::error::LoxError::RuntimeError {
                        message: "Operands must be two numbers or two strings".to_string(),
                    })
                }
            }
            "-" => {
                if a.is_number() && b.is_number() {
                    Ok(Value::Double(a.as_double() - b.as_double()))
                } else {
                    Err(crate::error::LoxError::RuntimeError {
                        message: "Operands must be numbers".to_string(),
                    })
                }
            }
            "*" => {
                if a.is_number() && b.is_number() {
                    Ok(Value::Double(a.as_double() * b.as_double()))
                } else {
                    Err(crate::error::LoxError::RuntimeError {
                        message: "Operands must be numbers".to_string(),
                    })
                }
            }
            "/" => {
                if a.is_number() && b.is_number() {
                    if b.as_double() == 0.0 {
                        Err(crate::error::LoxError::RuntimeError {
                            message: "Division by zero".to_string(),
                        })
                    } else {
                        Ok(Value::Double(a.as_double() / b.as_double()))
                    }
                } else {
                    Err(crate::error::LoxError::RuntimeError {
                        message: "Operands must be numbers".to_string(),
                    })
                }
            }
            _ => Err(crate::error::LoxError::RuntimeError {
                message: "Unknown binary operator".to_string(),
            }),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
