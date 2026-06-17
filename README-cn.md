# rspression
# 1. 背景介绍
rspression是一款用rust编写的高性能、轻量级表达式计算引擎，旨在提高用户系统在不同业务场景下的扩展能力。

传统的表达式引擎对表达式的执行过程一般是将表达式解析为语法树，然后直接解释执行语法树，这种方式适合公式表达式数量比较少的情况，每次从头解析、分析、执行，性能上也不会有太大问题。但如果每次需要执行的表达式数量都有成千上万条，那么每执行一次都从0开始做解析，就会造成资源的浪费。如果系统是单机环境，倒是可以把中间表示结构缓存在内存中。但如果系统是集群部署，缓存是类似redis的独立服务，则中间结构所占的空间就太大了，读写缓存时序列化、反序列化、网络传输会占用很多时间。

针对这种背景，rspression提供了两种执行表达式的方式，一是传统的直接执行表达式字符串，适合表达式数量较少的情况。二是字节码运行模式，业务系统在配置好表达式以后，可以先将表达式编译为字节码(Chunk)，然后将字节码放入缓存或者数据库、文件等存储服务中。最后需要执行的时候，从存储/缓存服务中读取出字节码再由虚拟机运行。

字节码运行模式与中间表示对象(IR)序列化/反序列化再运行有本质区别，通常对IR对象做存储或者网络传输前需要先经过序列化的过程，然后经过网络传输或者从存储中读取到序列化结果后，想要运行则又必须先经过反序列化为对象的过程。不同于序列化反序列化，字节码本身就是一个字节数组，字符串格式的表达式被编译成字节码后，无须经过序列化就可以直接进行网络传输，或者写入到存储服务中。而通过网络传输或者存储服务读取到的字节码，其在内存中的形式为一个字节数组，这个字节数组同样不需要反序列化就可以直接被虚拟机识别并运行。

将表达式编译为字节码虽有微小的前期性能损耗，但完美契合了表达式‘单次写入、海量复用’的业务形态。对于高并发、多表达式的业务，在创建或变更时触发单次编译，后续运行完全脱离原始结构、纯靠字节码驱动，这为数据层缓存、网络带宽以及计算节点执行带来了质的性能飞跃。
# 2. 用法说明
## 求值模式
支持+、-、*、/、**【指数运算】、<、>、<=、>=、==、!=、%、&&、||、!、等操作符。支持Excel风格的if(cond, thenBranch, elseBranch)条件函数。
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
## 运算模式
支持表达式变量赋值运算，多个表达式批量进行运算时，支持根据表达式的依赖关系先进行排序，再运算。并且会对运算表达式之间是否有循环依赖进行检测。
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
##  定义环境
表达式求值时，对于遇到的变量，求值器会从环境对象Environment中取值，赋值表达式则会把求值的结果写回到Environment中，因此对于表达式中用到的变量，具体含义需要在Environment中进行定义：
```rust
let mut env = DefaultEnvironment::new();
env.put("a".to_string(), Value::Integer(1));
env.put("b".to_string(), Value::Integer(2));
env.put("c".to_string(), Value::Integer(3));
let mut runner = RspRunner::new();
let r = runner.execute_with_env("a + b * c", &mut env)?;
println!({}, r) // 7
```
系统提供的默认环境对象为DefaultEnvironment，在执行表达式前，对于表达式中需要读取值的变量，都需要在DefaultEnvironment对象中有值。有时候需要执行的表达式数量较多，在对表达式做解析之前，业务层无法高效的把所有变量值都提前准备好，或者表达式中的变量和实际数据之间是间接的关联，这时候便可以根据需要自定义环境对象，只需继承Environment抽象类即可。
## 字节码运行
先把表达式编译为字节码(Chunk)，由业务系统缓存或者存储字节码，后续需要执行时直接运行字节码。
- 编译表达式：
```rust
use rspression::{Chunk, RspRunner};

let mut runner = RspRunner::new();
let chunk = runner.compile_source(&srcs).unwrap();
let bytes: Vec<u8> = chunk.to_bytes();
// write bytes to store or cache
// ...
```
- 运行字节码：
```rust
use rspression::{Chunk, RspRunner};

let mut runner = RspRunner::new();
let env = get_env();
// read bytes from store or cache
// let bytes: Vec<u8> = ...
let chunk = Chunk::from_bytes(&bytes);
runner.run_chunk(&chunk, &mut env).unwrap();
```
Chunk对象只由字节数组构成，序列化、反序列化性能极高，适合集群环境使用redis等缓存服务做缓存的场景。