# rspression
# 1. Background Introduction
rspression is a high-performance, lightweight expression calculation engine written in Rust, designed to enhance the extensibility of user systems in different business scenarios.

Traditional expression engines typically execute expressions by parsing them into an Abstract Syntax Tree (AST) and then directly interpreting and executing the tree. This approach is suitable for scenarios with a relatively small number of formulas and expressions; since they are parsed, analyzed, and executed from scratch each time, it does not pose significant performance issues. However, if there are thousands of expressions to be executed each time, starting the parsing process from scratch for every single execution would result in a massive waste of resources. If the system is in a single-machine environment, the intermediate representation (IR) structure can simply be cached in memory. But if the system is deployed in a cluster where the cache resides in an independent service like Redis, the footprint of the intermediate structure becomes too large, and the serialization, deserialization, and network transmission during cache reads and writes will consume a considerable amount of time.

To address this background, rspression provides two execution modes. The first is the traditional approach of directly executing expression strings, which is ideal for scenarios with a limited number of expressions. The second is the bytecode execution mode: once the expressions are configured, the business system can compile them into bytecode (Chunk) and persist them into storage services such as caches, databases, or files. When execution is subsequently required, the bytecode is retrieved from the storage or cache service and run directly by the virtual machine.

The bytecode execution mode is inherently different from the process of serializing, deserializing, and then executing an Intermediate Representation (IR) object. Typically, before an IR object can be stored or transmitted over a network, it must first undergo serialization. Then, after being transmitted or retrieved from storage, it must be deserialized back into an object before it can be executed. Bytecode, by contrast, is natively a byte array. Once a string-formatted expression is compiled into bytecode, it can be directly transmitted over the network or written into a storage service without any serialization. Furthermore, when the bytecode is retrieved from the network or a storage service, its in-memory form remains a byte array, which can be directly recognized and executed by the virtual machine without the need for deserialization

While compiling expressions into bytecode introduces a slight initial compilation overhead, it perfectly fits the 'write-once, execute-frequently' business pattern. For high-concurrency workloads with large volumes of expressions, compiling once upon creation or modification enables future executions to run entirely on bytecode, completely decoupled from the source structure. This delivers a massive performance boost for data caching, network transmission, and compute node execution alike.

# 2. Usage Guide
## Evaluation Mode
Supports operators such as +, -, *, /, ** (exponentiation), <, >, <=, >=, ==, !=, %, &&, ||, !, etc. Supports Excel-style if(cond, thenBranch, elseBranch) conditional functions.
```rust
use rspression::{DefaultEnvironment, Environment, RspRunner, Value};

let mut runner = RspRunner::new();
// Simple expression
println!("1 + 2 * 3 = {}", runner.execute("1 + 2 * 3")?); // 1 + 2 * 3 = 7

// With variables
let mut env = DefaultEnvironment::new();
env.put("a".to_string(), Value::Integer(1));
env.put("b".to_string(), Value::Integer(2));
env.put("c".to_string(), Value::Integer(3));
println!(
    "a + b * c = {}",
    runner.execute_with_env("a + b * c", &mut env)?
); // a + b * c = 7
println!("{}", runner.execute_with_env("a + b * c >= 6", &mut env)?); // true
```

## Computation Mode
Supports variable assignment operations in expressions. When performing batch computations with multiple expressions, they are first sorted according to their dependency relationships before execution. Additionally, circular dependency detection is performed among the computation expressions.
```rust
use rspression::{DefaultEnvironment, Environment, RspRunner, Value};

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
println!("x = {}", env.get("x").unwrap().as_integer()); // x = 270
println!("a = {}", env.get("a").unwrap().as_integer()); // a = 6
println!("b = {}", env.get("b").unwrap().as_integer()); // b = 12
println!("c = {}", env.get("c").unwrap().as_integer()); // c = 22
```

## Defining Environment
When evaluating expressions, the evaluator retrieves values from the Environment object for variables encountered. Assignment expressions write the evaluation results back to the Environment. Therefore, for variables used in expressions, their specific meanings need to be defined in the Environment:
```rust
let mut env = DefaultEnvironment::new();
env.put("a".to_string(), Value::Integer(1));
env.put("b".to_string(), Value::Integer(2));
env.put("c".to_string(), Value::Integer(3));
let mut runner = RspRunner::new();
let r = runner.execute_with_env("a + b * c", &mut env)?;
println!({}, r) // 7
```

The default environment object provided by the system is DefaultEnvironment. Before executing expressions, all variables that need to read values must have corresponding values in the DefaultEnvironment object. Sometimes there are many expressions to execute, and the business layer cannot efficiently prepare all variable values in advance before parsing expressions. Or the variables in the expressions are indirectly related to the actual data. In such cases, you can define a custom environment object by simply inheriting the Environment abstract class.

## Bytecode Execution
The expressions are first compiled into bytecode (Chunk) and then cached or stored by the business system. When subsequent execution is required, the bytecode is run directly.
- Compile expressions:
```rust
use rspression::{Chunk, RspRunner};

let mut runner = RspRunner::new();
let chunk = runner.compile_source(&srcs).unwrap();
let bytes: Vec<u8> = chunk.to_bytes();
// write bytes to store or cache
// ...
```

- Run bytecode:
```rust
use rspression::{Chunk, RspRunner};

let mut runner = RspRunner::new();
let env = get_env();
// read bytes from store or cache
// let bytes: Vec<u8> = ...
let chunk = Chunk::from_bytes(&bytes);
runner.run_chunk(&chunk, &mut env).unwrap();
```

The Chunk object consists only of byte arrays with extremely high serialization and deserialization performance, making it suitable for cluster environments using caching services like Redis.