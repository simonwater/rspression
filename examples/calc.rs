use rspression::{DefaultEnvironment, Environment, RspRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut srcs = Vec::new();
    srcs.push("x = a + b * c");
    srcs.push("b = a * 2");
    srcs.push("a = m + n");
    srcs.push("c = n + w + b");

    let mut runner = RspRunner::new();
    let mut env = DefaultEnvironment::new();
    env.put("m".into(), Value::Integer(2));
    env.put("n".into(), Value::Integer(4));
    env.put("w".into(), Value::Integer(6));

    runner.execute_multiple_with_env(&srcs, &mut env)?;
    println!("x = {}", env.get("x").unwrap()); // x = 270
    println!("a = {}", env.get("a").unwrap()); // a = 6
    println!("b = {}", env.get("b").unwrap()); // b = 12
    println!("c = {}", env.get("c").unwrap()); // c = 22

    Ok(())
}
