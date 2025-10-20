use crate::{
    LoxResult,
    chunk::{Chunk, ChunkReader},
    environment::{DefaultEnvironment, Environment},
    error::LoxError,
    functions::FunctionManager,
    parser::TokenType,
    values::{Value, value_helper},
    vm::OpCode,
};

pub struct ExResult {
    pub result: Value,
    pub index: i32,
}

pub struct VM {
    stack: Vec<Value>,
    function_manager: FunctionManager,
}

impl VM {
    const STACK_MAX: usize = 256;

    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(Self::STACK_MAX),
            function_manager: FunctionManager::new(),
        }
    }

    fn reset(&mut self) {
        self.stack.clear();
    }

    fn push(&mut self, value: Value) {
        if self.stack.len() >= Self::STACK_MAX {
            panic!("Stack overflow");
        }
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::Null)
    }

    fn peek(&self) -> &Value {
        self.stack.last().unwrap_or(&Value::Null)
    }

    fn peek_distance(&self, distance: usize) -> &Value {
        if distance >= self.stack.len() {
            &Value::Null
        } else {
            &self.stack[self.stack.len() - 1 - distance]
        }
    }

    pub fn execute(&mut self, chunk: &Chunk) -> LoxResult<Vec<ExResult>> {
        let mut reader = ChunkReader::new(&chunk.codes, &chunk.constants, &chunk.vars);
        let mut env = DefaultEnvironment::new();
        self.run(&mut reader, &mut env)
    }

    pub fn execute_reader(&mut self, reader: &mut ChunkReader) -> LoxResult<Vec<ExResult>> {
        self.reset();
        let mut env = DefaultEnvironment::new();
        self.run(reader, &mut env)
    }

    pub fn execute_with_env<E: Environment>(
        &mut self,
        chunk: &Chunk,
        env: &mut E,
    ) -> LoxResult<Vec<ExResult>> {
        let mut reader = ChunkReader::new(&chunk.codes, &chunk.constants, &chunk.vars);
        self.run(&mut reader, env)
    }

    pub fn execute_reader_with_env<E: Environment>(
        &mut self,
        reader: &mut ChunkReader,
        env: &mut E,
    ) -> LoxResult<Vec<ExResult>> {
        self.run(reader, env)
    }

    fn run<E: Environment>(
        &mut self,
        reader: &mut ChunkReader,
        env: &mut E,
    ) -> LoxResult<Vec<ExResult>> {
        let mut result = Vec::new();
        let mut exp_order = 0;
        self.reset();

        loop {
            let op = self.read_code(reader);
            match op {
                OpCode::Begin => {
                    exp_order = self.read_int(reader) as i32;
                }
                OpCode::End => {
                    let v = self.pop();
                    result.push(ExResult {
                        result: v,
                        index: exp_order,
                    });
                }
                OpCode::Constant => {
                    let constant = self.read_constant(reader);
                    self.push(constant.clone());
                }
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::Null => {
                    self.push(Value::Null);
                }
                OpCode::True => {
                    self.push(Value::Boolean(true));
                }
                OpCode::False => {
                    self.push(Value::Boolean(false));
                }
                OpCode::GetGlobal => {
                    let name = self.read_str(reader);
                    if let Some(value) = env.get(name) {
                        self.push(value.clone());
                    } else {
                        return Err(LoxError::RuntimeError {
                            message: format!("Undefined variable: {}, order: {}", name, exp_order),
                        });
                    }
                }
                OpCode::SetGlobal => {
                    let name = self.read_str(reader);
                    let value = self.peek().clone();
                    if !env.put(name.to_string(), value) {
                        return Err(LoxError::RuntimeError {
                            message: format!("Undefined variable: {}, order: {}", name, exp_order),
                        });
                    }
                }
                OpCode::GetProperty => {
                    let name = self.read_str(reader);
                    let object = self.pop();
                    if let Value::Instance(instance) = object {
                        if let Some(value) = instance.get(&name) {
                            self.push(value.clone());
                        } else {
                            return Err(LoxError::RuntimeError {
                                message: format!(
                                    "Undefined property: {}, order: {}",
                                    name, exp_order
                                ),
                            });
                        }
                    } else {
                        return Err(LoxError::RuntimeError {
                            message: format!(
                                "Only instances have properties. error: {}, order: {}",
                                name, exp_order
                            ),
                        });
                    }
                }
                OpCode::SetProperty => {
                    let name = self.read_str(reader);
                    let object = self.pop();
                    if let Value::Instance(mut instance) = object {
                        let value = self.peek().clone();
                        instance.set(name.to_string(), value);
                    } else {
                        return Err(LoxError::RuntimeError {
                            message: format!(
                                "Only instances have properties. error: {}, order: {}",
                                name, exp_order
                            ),
                        });
                    }
                }
                OpCode::Add => self.binary_op(TokenType::Plus)?,
                OpCode::Subtract => self.binary_op(TokenType::Minus)?,
                OpCode::Multiply => self.binary_op(TokenType::Star)?,
                OpCode::Divide => self.binary_op(TokenType::Slash)?,
                OpCode::Mode => self.binary_op(TokenType::Percent)?,
                OpCode::Power => self.binary_op(TokenType::StarStar)?,
                OpCode::Greater => self.binary_op(TokenType::Greater)?,
                OpCode::GreaterEqual => self.binary_op(TokenType::GreaterEqual)?,
                OpCode::Less => self.binary_op(TokenType::Less)?,
                OpCode::LessEqual => self.binary_op(TokenType::LessEqual)?,
                OpCode::EqualEqual => self.binary_op(TokenType::EqualEqual)?,
                OpCode::BangEqual => self.binary_op(TokenType::BangEqual)?,
                OpCode::Not => self.pre_unary_op(TokenType::Bang)?,
                OpCode::Negate => self.pre_unary_op(TokenType::Minus)?,
                OpCode::Call => {
                    let name = self.read_str(reader);
                    self.call_function(&name)?;
                }
                OpCode::JumpIfFalse => {
                    let offset = self.read_int(reader) as usize;
                    if !self.peek().is_truthy() {
                        self.goto_offset(reader, offset);
                    }
                }
                OpCode::Jump => {
                    let offset = self.read_int(reader) as usize;
                    self.goto_offset(reader, offset);
                }
                OpCode::Return => {
                    // Return from function
                }
                OpCode::Exit => {
                    if !self.stack.is_empty() {
                        return Err(LoxError::RuntimeError {
                            message: format!(
                                "VM state error, stack not empty: {}",
                                self.stack.len()
                            ),
                        });
                    }
                    return Ok(result);
                }
                _ => {
                    return Err(LoxError::RuntimeError {
                        message: format!("Unknown instruction: {:?}, order: {}", op, exp_order),
                    });
                }
            }
        }
    }

    fn call_function(&mut self, name: &str) -> LoxResult<()> {
        let arity = if let Some(function) = self.function_manager.get(name) {
            function.arity()
        } else {
            return Err(LoxError::RuntimeError {
                message: format!("Undefined function: {}", name),
            });
        };

        let mut arguments = Vec::new();
        for _ in 0..arity {
            arguments.push(self.pop());
        }
        arguments.reverse(); // Arguments are pushed in reverse order

        if let Some(function) = self.function_manager.get(name) {
            let result = function.call(arguments);
            self.push(result);
        }
        LoxResult::Ok(())
    }

    fn binary_op(&mut self, op_type: TokenType) -> LoxResult<()> {
        let b = self.pop();
        let a = self.pop();
        let result = value_helper::evaluate_binary(&a, &b, &op_type)?;
        self.push(result);
        LoxResult::Ok(())
    }

    fn pre_unary_op(&mut self, op_type: TokenType) -> LoxResult<()> {
        let operand = self.pop();
        let result = value_helper::evaluate_unary(&operand, &op_type)?;
        self.push(result);
        LoxResult::Ok(())
    }

    fn read_str<'a>(&mut self, reader: &'a mut ChunkReader) -> &'a str {
        let value = self.read_constant(reader);
        value.as_str()
    }

    fn read_constant<'a>(&mut self, reader: &'a mut ChunkReader) -> &'a Value {
        let index = reader.read_int() as usize;
        reader.read_const(index)
    }

    fn read_code(&mut self, reader: &mut ChunkReader) -> OpCode {
        let code = reader.read_byte();
        OpCode::try_from(code).unwrap()
    }

    fn read_int(&mut self, reader: &mut ChunkReader) -> i32 {
        reader.read_int()
    }

    fn goto_offset(&mut self, reader: &mut ChunkReader, offset: usize) {
        let cur_pos = reader.position();
        reader.new_position(cur_pos + offset);
    }
}
