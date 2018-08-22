extern crate bjorn;

#[test]
fn statements_with_integers() {
    assert_eq!(bjorn::interpret("2 + 2"), String::from("4"));
    assert_eq!(bjorn::interpret("2 - 2"), String::from("0"));
    assert_eq!(bjorn::interpret("2 + 2 - 3"), String::from("1"));
    assert_eq!(bjorn::interpret("2 * 2"), String::from("4"));
    assert_eq!(bjorn::interpret("2 + 2 * 3"), String::from("8"));
    assert_eq!(bjorn::interpret("2 / 2"), String::from("1"));
    assert_eq!(bjorn::interpret("2 / 2 * 2"), String::from("2"));
}

#[test]
fn statements_with_floats() {
    assert_eq!(bjorn::interpret("2.0 + 2.0"), String::from("4"));
    assert_eq!(bjorn::interpret("2.0 - 2.0"), String::from("0"));
    assert_eq!(bjorn::interpret("2.5 + 2.5 - 3.5"), String::from("1.5"));
    assert_eq!(bjorn::interpret("2.0 * 2.0"), String::from("4"));
    assert_eq!(bjorn::interpret("2.5 + 2.5 * 3"), String::from("10"));
    assert_eq!(bjorn::interpret("2.0 / 2.0"), String::from("1"));
    assert_eq!(bjorn::interpret("2.0 / 2.0 * 2.5"), String::from("2.5"));
    assert_eq!(bjorn::interpret("2.0 + 2"), String::from("4"));
    assert_eq!(bjorn::interpret("2.5 + 2"), String::from("4.5"));
    assert_eq!(bjorn::interpret("2 + 2.5"), String::from("4.5"));
    assert_eq!(bjorn::interpret("4.0 - 2"), String::from("2"));
    assert_eq!(bjorn::interpret("4 - 2.0"), String::from("2"));
    assert_eq!(bjorn::interpret("4.0 * 2"), String::from("8"));
    assert_eq!(bjorn::interpret("3 * 1.5"), String::from("4.5"));
    assert_eq!(bjorn::interpret("4.0 / 2"), String::from("2"));
    assert_eq!(bjorn::interpret("4 / 2.0"), String::from("2"));
    assert_eq!(bjorn::interpret("5 / 2.0"), String::from("2.5"));
}

#[test]
fn statements_with_unary_expressions() {
    assert_eq!(bjorn::interpret("+2 + +2"), String::from("4"));
    assert_eq!(bjorn::interpret("+2 + -2"), String::from("0"));
    assert_eq!(bjorn::interpret("-2 + +2"), String::from("0"));
    assert_eq!(bjorn::interpret("-2 + -2"), String::from("-4"));
    assert_eq!(bjorn::interpret("++2"), String::from("2"));
    assert_eq!(bjorn::interpret("-+2"), String::from("-2"));
    assert_eq!(bjorn::interpret("+-2"), String::from("-2"));
    assert_eq!(bjorn::interpret("--2"), String::from("2"));
}
