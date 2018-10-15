extern crate bjorn;

#[test]
fn empty_statement() {
    assert_eq!(bjorn::interpret(""), String::from(""));
}

#[test]
fn operations_with_integers() {
    assert_eq!(bjorn::interpret("2 + 2"), String::from("4"));
    assert_eq!(bjorn::interpret("2 - 2"), String::from("0"));
    assert_eq!(bjorn::interpret("2 + 2 - 3"), String::from("1"));
    assert_eq!(bjorn::interpret("2 * 2"), String::from("4"));
    assert_eq!(bjorn::interpret("2 + 2 * 3"), String::from("8"));
    assert_eq!(bjorn::interpret("2 / 2"), String::from("1"));
    assert_eq!(bjorn::interpret("2 / 2 * 2"), String::from("2"));
}

#[test]
fn operations_with_floats() {
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
fn operations_with_unary_expressions() {
    assert_eq!(bjorn::interpret("+2 + +2"), String::from("4"));
    assert_eq!(bjorn::interpret("+2 + -2"), String::from("0"));
    assert_eq!(bjorn::interpret("-2 + +2"), String::from("0"));
    assert_eq!(bjorn::interpret("-2 + -2"), String::from("-4"));
    assert_eq!(bjorn::interpret("++2"), String::from("2"));
    assert_eq!(bjorn::interpret("-+2"), String::from("-2"));
    assert_eq!(bjorn::interpret("+-2"), String::from("-2"));
    assert_eq!(bjorn::interpret("--2"), String::from("2"));
}

#[test]
fn operations_logical_with_boolean() {
    // OR
    assert_eq!(bjorn::interpret("true or true"), String::from("true"));
    assert_eq!(bjorn::interpret("true or false"), String::from("true"));
    assert_eq!(bjorn::interpret("false or true"), String::from("true"));
    assert_eq!(bjorn::interpret("false or false"), String::from("false"));

    // AND
    assert_eq!(bjorn::interpret("true and true"), String::from("true"));
    assert_eq!(bjorn::interpret("true and false"), String::from("false"));
    assert_eq!(bjorn::interpret("false and true"), String::from("false"));
    assert_eq!(bjorn::interpret("false and false"), String::from("false"));

    // NOT
    assert_eq!(bjorn::interpret("not true"), String::from("false"));
    assert_eq!(bjorn::interpret("not false"), String::from("true"));
}

#[test]
#[should_panic]
fn invalid_operations_with_booleans() {

    let operands = vec!("+", "-", "*", "/");

    for op in operands {
        // with integers
        bjorn::interpret(&format!("1 {} true", op));
        bjorn::interpret(&format!("1 {} false", op));
        bjorn::interpret(&format!("true {} 1", op));
        bjorn::interpret(&format!("false {} 1", op));

        // with Floats
        bjorn::interpret(&format!("1.0 {} true", op));
        bjorn::interpret(&format!("1.0 {} false", op));
        bjorn::interpret(&format!("true {} 1.0", op));
        bjorn::interpret(&format!("false {} 1.0", op));

        // with boolean
        bjorn::interpret(&format!("true {} true", op));
        bjorn::interpret(&format!("true {} false", op));
        bjorn::interpret(&format!("false {} true", op));
        bjorn::interpret(&format!("false {} false", op));
    }
}

#[test]
#[should_panic]
fn invalid_logical_operations_with_numbers() {

    // OR and AND
    let operands = vec!("or", "and");
    for op in operands {
        // with integers and floats
        bjorn::interpret(&format!("1 {} 1", op));
        bjorn::interpret(&format!("1 {} 1.0", op));
        bjorn::interpret(&format!("1.0 {} 1", op));
        bjorn::interpret(&format!("1.0 {} 1.0", op));

        // with integers and Booleans
        bjorn::interpret(&format!("1 {} true", op));
        bjorn::interpret(&format!("1 {} false", op));
        bjorn::interpret(&format!("true {} 1", op));
        bjorn::interpret(&format!("false {} 1", op));

        // with Floats and Boolean
        bjorn::interpret(&format!("1.0 {} true", op));
        bjorn::interpret(&format!("1.0 {} false", op));
        bjorn::interpret(&format!("true {} 1.0", op));
        bjorn::interpret(&format!("false {} 1.0", op));
    }

    // NOT
    bjorn::interpret("not 1");
    bjorn::interpret("not 1.0");

}
