extern crate bjorn;

#[test]
fn assignment_statement() {
    assert_eq!(bjorn::interpret("a = 1"), String::from(""));
}
