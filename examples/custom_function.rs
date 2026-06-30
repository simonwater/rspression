use rspression::{
    Callable, DefaultEnvironment, Environment, ExecuteMode, RspError, RspResult, RspRunner, Value,
};

pub struct MyAbsFunction;
impl Callable for MyAbsFunction {
    fn call(&self, arguments: Vec<Value>, _: &mut dyn Environment) -> RspResult<Value> {
        if let Some(value) = arguments.get(0) {
            match value {
                Value::Integer(i) => Ok(Value::from(i.abs())),
                Value::Double(d) => Ok(Value::from(d.abs())),
                _ => Err(RspError::CallableError {
                    message: format!("Value: {} can not call abs function", value),
                }),
            }
        } else {
            return Err(RspError::CallableError {
                message: format!("abs function must take 1 argument"),
            });
        }
    }
}

pub struct MySumFunction;
impl Callable for MySumFunction {
    fn call(&self, arguments: Vec<Value>, _: &mut dyn Environment) -> RspResult<Value> {
        let mut sum = 0;
        for value in &arguments {
            let val = value.to_integer()?;
            sum += val;
        }
        Ok(Value::from(sum))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = DefaultEnvironment::new();
    env.put("my_abs".into(), Value::from(MyAbsFunction {}));
    env.put("my_sum".into(), Value::from(MySumFunction {}));

    let mut runner = RspRunner::new();
    runner.set_execute_mode(ExecuteMode::SyntaxTree);
    println!(
        "my_abs(1 - 2 * 3) = {}",
        runner.execute_with_env("my_abs(1 - 2 * 3)", &mut env)?
    );

    runner.set_execute_mode(ExecuteMode::ChunkVM);
    println!(
        "my_abs(1 - 2 * 3) = {}",
        runner.execute_with_env("my_abs(1 - 2 * 3)", &mut env)?
    );

    println!(
        "my_sum(1, 2, 3, 4, 5, my_abs(-6)) + 7 = {}",
        runner.execute_with_env("my_sum(1, 2, 3, 4, 5, my_abs(-6)) + 7", &mut env)?
    );
    Ok(())
}
