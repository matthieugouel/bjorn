extern crate bjorn;

#[test]
fn simple_statements_with_integers() {
    assert_eq!(bjorn::interpret("2 + 2"), String::from("4"));
}
