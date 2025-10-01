use rspression::{DefaultEnvironment, Environment, LoxRunner, Value};

#[test]
fn test_basic_arithmetic() {
    let mut runner = LoxRunner::new();
    assert_eq!(Value::Integer(3), runner.execute("1 + 2").unwrap());
    assert_eq!(Value::Integer(2), runner.execute("5 - 3").unwrap());
    assert_eq!(Value::Integer(12), runner.execute("4 * 3").unwrap());
    assert_eq!(Value::Integer(4), runner.execute("8 / 2").unwrap());
    assert_eq!(Value::Integer(0), runner.execute("1 + 2 - 3").unwrap());
    assert_eq!(Value::Integer(7), runner.execute("1 + 2 * 3").unwrap());
    assert_eq!(Value::Integer(9), runner.execute("3 * (2 + 1)").unwrap());
    assert_eq!(
        Value::Integer(7),
        runner.execute("1 + 2 * (5 - 2)").unwrap()
    );
    assert_eq!(
        Value::Double(1025.0),
        runner.execute("1 + 2 * 2 ** 3 ** 2").unwrap()
    );
    assert_eq!(Value::Double(9.0), runner.execute("3 * (2 + 1.0)").unwrap());
    assert_eq!(
        Value::Boolean(true),
        runner.execute("3 * (2 + 1.0) > 7").unwrap()
    );
    assert_eq!(
        Value::Double(11138.0),
        runner
            .execute(
                "1000 + 100.0 * 99 - (600 - 3 * 15) / (((68 - 9) - 3) * 2 - 100) + 10000 % 7 * 71"
            )
            .unwrap()
    );
}

#[test]
fn test_variables() {
    let mut runner = LoxRunner::new();
    let mut env = DefaultEnvironment::new();

    env.put("a".to_string(), Value::Integer(1));
    env.put("b".to_string(), Value::Integer(2));
    env.put("c".to_string(), Value::Integer(3));

    let result = runner
        .execute_with_env("x = y = a + b * c", &mut env)
        .unwrap();
    assert_eq!(Value::Integer(7), result);
    assert_eq!(Value::Integer(7), *env.get("x").unwrap());
    assert_eq!(Value::Integer(7), *env.get("y").unwrap());

    assert_eq!(
        Value::Double(3.0),
        runner
            .execute_with_env("a + b * c - 100 / 5 ** 2 ** 1", &mut env)
            .unwrap()
    );
    assert_eq!(
        Value::Boolean(true),
        runner.execute_with_env("a + b * c >= 6", &mut env).unwrap()
    );
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
