mod common;

use common::TestHelper;
use rand::Rng;
use rspression::Chunk;
use rspression::LoxRunner;
use rspression::ir::{Analyzer, ExprInfo};
use rspression::{DefaultEnvironment, Environment};

const FORMULA_BATCHES: usize = 10000;
const DIRECTORY: &str = "SerializeTest";

#[test]
fn test_serialize() {
    println!("序列化反序列化测试：");
    chunk_serialize_test();
}

fn chunk_serialize_test() {
    let lines = create_formulas();
    let srcs: Vec<&str> = lines.iter().map(String::as_str).collect();
    println!("表达式总数：{}", lines.len());
    println!("开始解析和分析：");
    let start = std::time::Instant::now();
    let mut runner = LoxRunner::new();
    let exprs = runner.parse(&srcs).unwrap();
    let ana = Analyzer::new(exprs, true);
    let expr_infos = ana.analyze().unwrap();
    println!("中间结果生成完成。 耗时:{:?}", start.elapsed());

    let start = std::time::Instant::now();
    let chunk = runner.compile_ir(&expr_infos).unwrap();
    println!("编译成字节码完成。 耗时:{:?}", start.elapsed());

    test_chunk(&chunk);
    test_syntax_tree(&expr_infos);
    println!("==========");
}

fn test_chunk(chunk: &Chunk) {
    println!(
        "开始进行字节码序列化反序列化，字节码大小(KB)：{}",
        chunk.get_byte_size() / 1024
    );
    let start = std::time::Instant::now();
    let path = TestHelper::get_path(DIRECTORY, "Chunks.pb");
    TestHelper::write_chunk_file(chunk, &path);
    println!("字节码已序列化到文件, 耗时:{:?}", start.elapsed());

    let start = std::time::Instant::now();
    let chunk = TestHelper::read_chunk_file(&path).expect("Failed to read chunk file");
    println!("完成从文件反序列化字节码。 耗时:{:?}", start.elapsed());

    println!("开始执行字节码：");
    let start = std::time::Instant::now();
    let mut runner = LoxRunner::new();
    let mut env = get_environment();
    runner.run_chunk(&chunk, &mut env).unwrap();
    check_result(&env);
    println!("字节码执行完成。 耗时:{:?}", start.elapsed());
}

fn test_syntax_tree(expr_infos: &[&ExprInfo]) {
    // todo: 序列化语法树
    // todo: 反序列化语法树

    println!("开始执行语法树");
    let start = std::time::Instant::now();
    let mut env = get_environment();
    let mut runner = LoxRunner::new();
    runner.run_ir(expr_infos, &mut env).unwrap();
    check_result(&env);
    println!("语法树执行完成。 耗时:{:?}", start.elapsed());
}

fn create_formulas() -> Vec<String> {
    let fml = "A! = 1 + 2 * 3 - 6 - 1 + B! + C! * (D! - E! + 10 ** 2 / 5 - (12 + 8)) - F! * G! +  100 / 5 ** 2 ** 1";
    let fml1 = "B! = C! + D! * 2 - 1";
    let fml2 = "C! = D! * 2 + 1";
    let fml3 = "D! = E! + F! * G!";
    let fml4 = "G! = M! + N!";
    let mut lines = Vec::with_capacity(5 * FORMULA_BATCHES);

    for i in 0..FORMULA_BATCHES {
        lines.push(fml.replace("!", &i.to_string()));
        lines.push(fml1.replace("!", &i.to_string()));
        lines.push(fml2.replace("!", &i.to_string()));
        lines.push(fml3.replace("!", &i.to_string()));
        lines.push(fml4.replace("!", &i.to_string()));
    }
    lines
}

fn get_environment() -> DefaultEnvironment {
    let mut env = DefaultEnvironment::new();
    for i in 0..FORMULA_BATCHES {
        env.put(format!("E{}", i), 2.into());
        env.put(format!("F{}", i), 3.into());
        env.put(format!("M{}", i), 4.into());
        env.put(format!("N{}", i), 5.into());
    }
    env
}

fn check_result(env: &DefaultEnvironment) {
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
