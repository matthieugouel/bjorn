extern crate bjorn;

#[test]
fn empty_statement() {
    assert_eq!(bjorn::interpret(""), String::from(""));
}

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

#[test]
#[should_panic]
fn statements_with_booleans() {

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
fn statements_with_comparison_expressions() {
    // Equal
    assert_eq!(bjorn::interpret("1 == 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 == 2"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 == 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 == 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 == 2"), String::from("false"));
    assert_eq!(bjorn::interpret("1 == 2.0"), String::from("false"));

    // Not Equal
    assert_eq!(bjorn::interpret("1 != 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 != 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 != 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 != 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 != 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1 != 2.0"), String::from("true"));

    // Lower or Equal
    assert_eq!(bjorn::interpret("1 <= 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1 <= 1"), String::from("true"));
    assert_eq!(bjorn::interpret("2 <= 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 <= 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 <= 1"), String::from("true"));
    assert_eq!(bjorn::interpret("2.0 <= 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 <= 2.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1 <= 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("2 <= 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 <= 2.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 <= 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("2.0 <= 1.0"), String::from("false"));

    // Greater or Equal
    assert_eq!(bjorn::interpret("2 >= 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 >= 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 >= 2"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 >= 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 >= 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 >= 2"), String::from("false"));
    assert_eq!(bjorn::interpret("2 >= 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1 >= 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1 >= 2.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 >= 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 >= 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 >= 2.0"), String::from("false"));

    // Lower Than
    assert_eq!(bjorn::interpret("1 < 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1 < 1"), String::from("false"));
    assert_eq!(bjorn::interpret("2 < 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 < 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 < 1"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 < 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 < 2.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1 < 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2 < 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 < 2.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 < 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 < 1.0"), String::from("false"));

    // Greater Than
    assert_eq!(bjorn::interpret("2 > 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 > 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 > 2"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 < 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 < 1"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 < 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 < 2.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1 < 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2 < 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 < 2.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 < 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 < 1.0"), String::from("false"));
}
