#![allow(unused)]
use k9::assert_equal;
use minilisp_data_structures::{
    car, cdr, cons, list, Cell, Value
};

#[test]
fn test_cons() {
    let cell = cons("head", &mut Cell::from("tail"));
    assert_equal!(cell.values(), vec![Value::symbol("head"), Value::symbol("tail")]);
}

#[test]
fn test_list() {
    let cell = list([Value::from("head"), Value::from("middle"), Value::from(33u8), Value::from("tail")]);
    assert_equal!(
        cell.values(),
        vec![
            Value::from("head"),
            Value::from("middle"),
            Value::Byte(33),
            Value::from("tail"),
        ]
    );
}

#[test]
fn test_car() {
    let value = list([Value::from("head"), Value::from("middle"), Value::from(33u8), Value::from("tail")]);
    assert_equal!(value.head(), Some(Value::from("head")));
    assert_equal!(car(&value), Value::from("head"));
}

#[test]
fn test_cdr() {
    let value = list([Value::symbol("a"), Value::symbol("b"), Value::symbol("c")]);
    assert_equal!(cdr(&value), list([Value::symbol("b"), Value::symbol("c")]));
}
