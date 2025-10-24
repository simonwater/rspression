use rspression::{DefaultEnvironment, Environment, RspRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut srcs = Vec::new();
    srcs.push("x = a + b * c");
    srcs.push("a = m + n");
    srcs.push("b = a * 2");
    srcs.push("c = n + w + b");

    let mut runner = RspRunner::new();
    let mut env = DefaultEnvironment::new();
    env.put("m".to_string(), Value::Integer(2));
    env.put("n".to_string(), Value::Integer(4));
    env.put("w".to_string(), Value::Integer(6));

    runner.execute_multiple_with_env(&srcs, &mut env).unwrap();
    println!("x = {}", env.get("x").unwrap().as_integer());
    println!("a = {}", env.get("a").unwrap().as_integer());
    println!("b = {}", env.get("b").unwrap().as_integer());
    println!("c = {}", env.get("c").unwrap().as_integer());

    Ok(())
}
