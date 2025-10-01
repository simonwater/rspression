use rspression::{DefaultEnvironment, Environment, LoxRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic arithmetic
    let mut runner = LoxRunner::new();

    // Simple expression
    let result = runner.execute("1 + 2 * 3")?;
    println!("1 + 2 * 3 = {}", result);

    // With variables
    let mut env = DefaultEnvironment::new();
    env.put("a".to_string(), Value::Integer(1));
    env.put("b".to_string(), Value::Integer(2));
    env.put("c".to_string(), Value::Integer(3));

    let result = runner.execute_with_env("a + b * c", &mut env)?;
    println!("a + b * c = {}", result);

    // Multiple expressions
    let results = runner.execute_multiple(&["x = a + b * c", "y = x * 2", "z = y + 10"])?;

    println!("Multiple expressions results: {:?}", results);

    Ok(())
}
