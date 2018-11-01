extern crate bjorn;

#[test]
fn assignment_statement() {
    assert_eq!(bjorn::interpret("a = 1"), String::from(""));
    assert_eq!(bjorn::interpret("a = 1\na"), String::from("1"));
}
