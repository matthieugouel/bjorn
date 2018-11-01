extern crate bjorn;

#[test]
fn if_statement() {
    assert_eq!(bjorn::interpret("if true:\n    a = 1\nelse:\n    a = 2\na"), String::from("1"));
}

#[test]
fn while_statement() {
    assert_eq!(bjorn::interpret("i = 0\nwhile i < 2:\n    i = i + 1\ni"), String::from("2"))
}
