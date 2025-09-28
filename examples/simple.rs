use rspression::{DefaultEnvironment, Environment, LoxRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner = LoxRunner::new();

    // Test simple arithmetic
    let result = runner.execute("1 + 2")?;
    println!("1 + 2 = {}", result);

    // Test with variables
    let mut env = DefaultEnvironment::new();
    env.put("a".to_string(), Value::Integer(5))?;
    env.put("b".to_string(), Value::Integer(3))?;

    let result = runner.execute_with_env("a + b", &mut env)?;
    println!("a + b = {}", result);

    // Test assignment
    let result = runner.execute_with_env("x = a + b", &mut env)?;
    println!("x = a + b = {}", result);

    // Check if x was stored
    let x_value = env.get("x")?;
    println!("x value in environment: {}", x_value);

    Ok(())
}
