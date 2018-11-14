extern crate bjorn;

#[test]
fn function_calls() {
    assert_eq!(bjorn::interpret("def test():\n    return 1\ntest()"), String::from("1"));
    assert_eq!(bjorn::interpret("def test(x):\n    return x\ntest(1)"), String::from("1"));
    assert_eq!(
        bjorn::interpret("def test(x, y):\n    return x + y\ntest(1, 2)"),
        String::from("3")
    );
    assert_eq!(
        bjorn::interpret("def test():\n    if true:\n        return 1\n    else:\n        return 2\ntest()"),
        String::from("1")
    );
    assert_eq!(
        bjorn::interpret("def test(i):\n    while i < 2:\n        i = i + 1\n    return i\ntest(0)"),
        String::from("2")
    );
}
