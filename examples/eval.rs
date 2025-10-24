use rspression::{DefaultEnvironment, Environment, RspRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic arithmetic
    let mut runner = RspRunner::new();

    // Simple expression
    println!("1 + 2 * 3 = {}", runner.execute("1 + 2 * 3")?);

    // With variables
    let mut env = DefaultEnvironment::new();
    env.put("a".to_string(), Value::Integer(1));
    env.put("b".to_string(), Value::Integer(2));
    env.put("c".to_string(), Value::Integer(3));
    println!(
        "a + b * c = {}",
        runner.execute_with_env("a + b * c", &mut env)?
    );
    println!("{}", runner.execute_with_env("a + b * c >= 6", &mut env)?);

    Ok(())
}
