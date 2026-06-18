use rspression::{DefaultEnvironment, Environment, RspRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = DefaultEnvironment::new();
    env.put("a".to_string(), Value::Integer(1));
    env.put("b".to_string(), Value::Integer(2));
    env.put("c".to_string(), Value::Integer(3));
    let mut runner = RspRunner::new();
    let r = runner.execute_with_env("a + b * c", &mut env)?;
    println!("{}", r); // 7

    Ok(())
}
