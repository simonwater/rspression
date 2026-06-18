use rspression::{Chunk, DefaultEnvironment, Environment, RspRunner, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile expressions to bytecode
    // 1. get expressions
    let mut srcs = Vec::new();
    srcs.push("x = a + b * c");
    srcs.push("b = a * 2");
    srcs.push("a = m + n");
    srcs.push("c = n + w + b");
    // 2. compile expressions
    let mut runner = RspRunner::new();
    let chunk = runner.compile_source(&srcs)?;
    let bytes: Vec<u8> = chunk.to_bytes();
    // 3. you can write bytes to store or cache
    // write_to_your_storage(bytes);

    // Run bytecode
    // 1. read bytes from store or cache
    // let bytes: Vec<u8> = read_from_your_storage();
    // 2. construct chunk
    let chunk = Chunk::from_bytes(&bytes);
    // 3. define environment
    let mut env = DefaultEnvironment::new();
    env.put("m".to_string(), Value::Integer(2));
    env.put("n".to_string(), Value::Integer(4));
    env.put("w".to_string(), Value::Integer(6));
    // 4. run bytecode
    let mut runner = RspRunner::new();
    runner.run_chunk(&chunk, &mut env)?;

    // 5. check results
    println!("x = {}", env.get("x").unwrap()); // x = 270
    println!("a = {}", env.get("a").unwrap()); // a = 6
    println!("b = {}", env.get("b").unwrap()); // b = 12
    println!("c = {}", env.get("c").unwrap()); // c = 22

    Ok(())
}
