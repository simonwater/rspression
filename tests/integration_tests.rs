use rspression::{DefaultEnvironment, Environment, LoxRunner, Value};

#[test]
fn test_basic_arithmetic() {
    let mut runner = LoxRunner::new();

    let result = runner.execute("1 + 2").unwrap();
    assert_eq!(result, Value::Integer(3));

    let result = runner.execute("5 - 3").unwrap();
    assert_eq!(result, Value::Integer(2));

    let result = runner.execute("4 * 3").unwrap();
    assert_eq!(result, Value::Integer(12));

    let result = runner.execute("8 / 2").unwrap();
    assert_eq!(result, Value::Integer(4));
}

#[test]
fn test_variables() {
    let mut runner = LoxRunner::new();
    let mut env = DefaultEnvironment::new();

    env.put("a".to_string(), Value::Integer(5)).unwrap();
    env.put("b".to_string(), Value::Integer(3)).unwrap();

    let result = runner.execute_with_env("a + b", &mut env).unwrap();
    assert_eq!(result, Value::Integer(8));
}

#[test]
fn test_strings() {
    let mut runner = LoxRunner::new();

    let result = runner.execute("\"aa\" + \"bb\"").unwrap();
    assert_eq!(result, Value::String("aabb".to_string()));

    let result = runner.execute("\"你好\" + \"bb！\"").unwrap();
    assert_eq!(result, Value::String("你好bb！".to_string()));
}

#[test]
fn test_comparison() {
    let mut runner = LoxRunner::new();

    let result = runner.execute("5 > 3").unwrap();
    assert_eq!(result, Value::Boolean(true));

    let result = runner.execute("3 < 5").unwrap();
    assert_eq!(result, Value::Boolean(true));

    let result = runner.execute("5 == 5").unwrap();
    assert_eq!(result, Value::Boolean(true));

    let result = runner.execute("5 != 3").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_logical_operations() {
    let mut runner = LoxRunner::new();

    let result = runner.execute("true && false").unwrap();
    assert_eq!(result, Value::Boolean(false));

    let result = runner.execute("true || false").unwrap();
    assert_eq!(result, Value::Boolean(true));

    let result = runner.execute("!true").unwrap();
    assert_eq!(result, Value::Boolean(false));
}
