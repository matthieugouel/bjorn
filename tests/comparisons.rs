extern crate bjorn;

#[test]
fn valid_comparisons() {
    // Equal
    assert_eq!(bjorn::interpret("1 == 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 == 2"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 == 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 == 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 == 2"), String::from("false"));
    assert_eq!(bjorn::interpret("1 == 2.0"), String::from("false"));

    assert_eq!(bjorn::interpret("true == true"), String::from("true"));
    assert_eq!(bjorn::interpret("false == true"), String::from("false"));
    assert_eq!(bjorn::interpret("true == false"), String::from("false"));
    assert_eq!(bjorn::interpret("false == false"), String::from("true"));

    // Not Equal
    assert_eq!(bjorn::interpret("1 != 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 != 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 != 1"), String::from("false"));
    assert_eq!(bjorn::interpret("1 != 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 != 2"), String::from("true"));
    assert_eq!(bjorn::interpret("1 != 2.0"), String::from("true"));

    assert_eq!(bjorn::interpret("true != true"), String::from("false"));
    assert_eq!(bjorn::interpret("false != true"), String::from("true"));
    assert_eq!(bjorn::interpret("true != false"), String::from("true"));
    assert_eq!(bjorn::interpret("false != false"), String::from("false"));

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
    assert_eq!(bjorn::interpret("1.0 > 2"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 > 1"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 > 1"), String::from("true"));
    assert_eq!(bjorn::interpret("1 > 2.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1 > 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2 > 1.0"), String::from("true"));
    assert_eq!(bjorn::interpret("1.0 > 2.0"), String::from("false"));
    assert_eq!(bjorn::interpret("1.0 > 1.0"), String::from("false"));
    assert_eq!(bjorn::interpret("2.0 > 1.0"), String::from("true"));
}

#[test]
#[should_panic]
fn invalid_comparisons() {

    let operands = vec!["==", "!=", "<=", ">=", "<", ">"];

    for op in operands {
        bjorn::interpret(&format!("true {} 1", op));
        bjorn::interpret(&format!("1 {} true", op));
        bjorn::interpret(&format!("true {} 1.0", op));
        bjorn::interpret(&format!("1.0 {} true", op));
        bjorn::interpret(&format!("false {} 1", op));
        bjorn::interpret(&format!("1 {} false", op));
        bjorn::interpret(&format!("false {} 1.0", op));
        bjorn::interpret(&format!("1.0 {} false", op));
    }



}
