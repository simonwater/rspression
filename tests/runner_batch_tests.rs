mod common;

use common::TestHelper;
use rand::Rng;
use rspression::{Chunk, DefaultEnvironment, Environment, ExecuteMode, RspRunner};
use std::path::PathBuf;
use std::time::Instant;

const FORMULA_BATCHES: usize = 10000;
const DIRECTORY: &str = "BatchRunnerTest";

#[test]
fn test_ir() {
    println!("批量运算测试(解析执行)");
    let lines = get_expressions();
    let srcs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let start = Instant::now();
    let mut runner = RspRunner::new();
    runner.set_execute_mode(ExecuteMode::SyntaxTree);
    let mut env = get_env();
    runner.execute_multiple_with_env(&srcs, &mut env).unwrap();
    check_values(&env);
    println!("用时: {:?}", start.elapsed());
    println!("==========");
}

#[test]
fn test_compile_chunk() {
    println!("批量运算测试(编译+字节码执行)");
    let lines = get_expressions();
    let srcs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let mut runner = RspRunner::new();
    let start = std::time::Instant::now();
    let chunk = runner.compile_source(&srcs).unwrap();
    println!("编译用时: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let mut env = get_env();
    runner.run_chunk(&chunk, &mut env).unwrap();
    check_values(&env);
    println!("字节码执行用时：{:?}", start.elapsed());
    println!("==========");
}

#[test]
fn test_file_chunk() {
    println!("字节码编译到文件再从文件读取执行");
    let file_path = TestHelper::get_path(DIRECTORY, "Chunks.pb");
    let chunk = create_and_get_chunk(&file_path);
    println!(
        "字节码大小：{}KB. code: {}Byte, consts: {}Byte, vars: {}Byte",
        chunk.get_byte_size() / 1024,
        chunk.codes.len(),
        chunk.constants.len(),
        chunk.vars.len(),
    );

    let start = std::time::Instant::now();
    let cnt = 1;
    for _ in 0..cnt {
        let mut runner = RspRunner::new();
        let mut env = get_env();
        runner.run_chunk(&chunk, &mut env).unwrap();
        check_values(&env);
    }
    println!(
        "字节码执行完成，执行次数：{}。 总耗时:{:?}",
        cnt,
        start.elapsed()
    );
    println!("==========");
}

fn create_and_get_chunk(path: &PathBuf) -> Chunk {
    let lines = get_expressions();
    let srcs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let mut runner = RspRunner::new();
    let start = std::time::Instant::now();
    let chunk = runner.compile_source(&srcs).unwrap();
    println!("编译完成，耗时：{:?}", start.elapsed());

    let start = std::time::Instant::now();
    TestHelper::write_chunk_file(&chunk, path);
    println!("序列化到文件完成，耗时：{:?}", start.elapsed());

    let start = std::time::Instant::now();
    let chunk = TestHelper::read_chunk_file(path).expect("Failed to read chunk file");
    println!(
        "从文件反序列化完成，耗时(ms)：{}",
        start.elapsed().as_millis()
    );
    chunk
}

fn get_expressions<'a>() -> Vec<String> {
    let mut lines = Vec::new();
    let fml = "A! = 1 + 2 * 3 - 6 - 1 + B! + C! * (D! - E! + 10 ** 2 / 5 - (12 + 8)) - F! * G! +  100 / 5 ** 2 ** 1";
    let fml1 = "B! = C! + D! * 2 - 1";
    let fml2 = "C! = D! * 2 + 1";
    let fml3 = "D! = E! + F! * G!";
    let fml4 = "G! = M! + N!";

    for i in 0..FORMULA_BATCHES {
        lines.push(fml.replace("!", &i.to_string()));
        lines.push(fml1.replace("!", &i.to_string()));
        lines.push(fml2.replace("!", &i.to_string()));
        lines.push(fml3.replace("!", &i.to_string()));
        lines.push(fml4.replace("!", &i.to_string()));
    }
    lines
}

fn get_env() -> DefaultEnvironment {
    let mut env = DefaultEnvironment::new();
    for i in 0..FORMULA_BATCHES {
        env.put(format!("E{}", i), 2.into());
        env.put(format!("F{}", i), 3.into());
        env.put(format!("M{}", i), 4.into());
        env.put(format!("N{}", i), 5.into());
    }
    env
}

fn check_values(env: &DefaultEnvironment) {
    let mut rng = rand::rng();
    for _ in 0..10 {
        let index = rng.random_range(0..FORMULA_BATCHES);
        assert_eq!(1686.0, env.get(&format!("A{}", index)).unwrap().as_double());
        assert_eq!(116.0, env.get(&format!("B{}", index)).unwrap().as_double());
        assert_eq!(59.0, env.get(&format!("C{}", index)).unwrap().as_double());
        assert_eq!(29.0, env.get(&format!("D{}", index)).unwrap().as_double());
        assert_eq!(9.0, env.get(&format!("G{}", index)).unwrap().as_double());
    }
}
