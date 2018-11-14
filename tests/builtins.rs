extern crate bjorn;

#[test]
fn builtin_print() {
    bjorn::interpret("print(1)");
}

#[test]
#[should_panic]
fn builtin_print_wrong_parameters() {
    bjorn::interpret("print()");
    bjorn::interpret("print(1, 2)");
}
