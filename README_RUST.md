# Rsppression Rust Implementation

This is a Rust port of the Rsppression Java expression evaluation engine. The Rust version maintains the same core functionality while leveraging Rust's type system and memory safety features.

## Features

- **High Performance**: Zero-cost abstractions and efficient memory management
- **Type Safety**: Compile-time guarantees for expression evaluation
- **Memory Safe**: No null pointer dereferences or memory leaks
- **Concurrent**: Safe concurrent access to expression evaluators
- **Serializable**: Values and expressions can be serialized/deserialized

## Architecture

The Rust implementation follows the same architecture as the Java version:

### Core Components

1. **Scanner** (`src/scanner.rs`): Lexical analysis - converts expression strings to tokens
2. **Parser** (`src/parser.rs`): Syntax analysis - builds Abstract Syntax Trees (AST)
3. **Visitor** (`src/visitor.rs`): Expression evaluation using visitor pattern
4. **Environment** (`src/environment.rs`): Variable storage and management
5. **VM** (`src/vm.rs`): Bytecode virtual machine (future implementation)
6. **Runner** (`src/runner.rs`): Main API for expression evaluation

### Key Differences from Java Version

1. **Ownership Model**: Rust's ownership system eliminates the need for garbage collection
2. **Trait-based Design**: Uses traits instead of inheritance for extensibility
3. **Error Handling**: Uses `Result<T, E>` for explicit error handling
4. **Type Safety**: Compile-time type checking prevents runtime errors
5. **Memory Safety**: No null pointer exceptions or memory leaks

## Usage

### Basic Expression Evaluation

```rust
use rsppression::{RspRunner, Value};

let mut runner = RspRunner::new();
let result = runner.execute("1 + 2 * 3")?;
println!("Result: {}", result); // Output: 7
```

### With Variables

```rust
use rsppression::{RspRunner, DefaultEnvironment, Value, Environment};

let mut runner = RspRunner::new();
let mut env = DefaultEnvironment::new();

// Define variables
env.define("a".to_string(), Value::Integer(5))?;
env.define("b".to_string(), Value::Integer(3))?;

// Evaluate expression
let result = runner.execute_with_env("a + b", &mut env)?;
println!("Result: {}", result); // Output: 8
```

### Multiple Expressions

```rust
let results = runner.execute_multiple(&[
    "x = a + b",
    "y = x * 2",
    "z = y + 10"
])?;
println!("Results: {:?}", results);
```

## Supported Operations

### Arithmetic
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Modulo: `%`
- Exponentiation: `**`

### Comparison
- Equal: `==`
- Not Equal: `!=`
- Greater: `>`
- Greater Equal: `>=`
- Less: `<`
- Less Equal: `<=`

### Logical
- And: `&&`
- Or: `||`
- Not: `!`

### Conditional
- If expression: `if(condition, then, else)`

## Performance Characteristics

The Rust implementation provides several performance advantages:

1. **Zero-cost Abstractions**: Trait-based design with no runtime overhead
2. **Efficient Memory Management**: Stack allocation for small values, heap for large ones
3. **No Garbage Collection**: Deterministic memory management
4. **SIMD Support**: Can leverage SIMD instructions for vectorized operations
5. **Compile-time Optimizations**: Aggressive inlining and optimization

## Error Handling

The Rust version uses explicit error handling with the `Result` type:

```rust
use rsppression::{RspRunner, RspError};

match runner.execute("invalid expression") {
    Ok(result) => println!("Result: {}", result),
    Err(RspError::ParseError { line, message }) => {
        eprintln!("Parse error at line {}: {}", line, message);
    }
    Err(RspError::RuntimeError { message }) => {
        eprintln!("Runtime error: {}", message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Thread Safety

The Rust implementation is designed to be thread-safe:

```rust
use std::sync::Arc;
use rsppression::{RspRunner, DefaultEnvironment, Value, Environment};

let runner = Arc::new(RspRunner::new());
let env = Arc::new(std::sync::Mutex::new(DefaultEnvironment::new()));

// Safe to use across threads
let runner_clone = Arc::clone(&runner);
let env_clone = Arc::clone(&env);
```

## Future Enhancements

1. **Bytecode VM**: Complete implementation of the virtual machine
2. **JIT Compilation**: Just-in-time compilation for hot expressions
3. **Parallel Evaluation**: Multi-threaded expression evaluation
4. **SIMD Operations**: Vectorized arithmetic operations
5. **Custom Functions**: User-defined function support
6. **Async Support**: Asynchronous expression evaluation

## Testing

Run the test suite:

```bash
cargo test
```

Run examples:

```bash
cargo run --example simple
cargo run --example basic
```

## Benchmarking

The project includes Criterion benchmarks for performance testing:

```bash
cargo bench
```

## Comparison with Java Version

| Feature | Java Version | Rust Version |
|---------|-------------|--------------|
| Memory Management | GC | Ownership |
| Type Safety | Runtime | Compile-time |
| Performance | Good | Excellent |
| Memory Usage | Higher | Lower |
| Startup Time | Slower | Faster |
| Concurrency | Thread-safe with locks | Lock-free where possible |
| Error Handling | Exceptions | Result types |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `cargo test`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
