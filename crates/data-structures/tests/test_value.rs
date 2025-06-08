#![allow(unused)]
use minilisp_data_structures::*;
use k9::assert_equal;

#[test]
fn value_equals() {
    assert_equal!(Value::from(format!("string")), Value::from(String::from("string")));
    assert_equal!(Value::from("string"), Value::from("string"));
    assert_equal!(Value::from(10.0f64), Value::from(10.0f64));
    assert_equal!(Value::from(0xF1u8), Value::from(0xF1u8));
    assert_equal!(Value::from(0xF1u32), Value::from(0xF1u32));
    assert_equal!(Value::from(7i64), Value::from(7i64));
}
#[test]
fn value_ref_equals() {
    assert_equal!(&Value::from(format!("string")), Value::from(String::from("string")));
    assert_equal!(&Value::from("string"), Value::from("string"));
    assert_equal!(&Value::from(10.0f64), Value::from(10.0f64));
    assert_equal!(&Value::from(0xF1u8), Value::from(0xF1u8));
    assert_equal!(&Value::from(0xF1u32), Value::from(0xF1u32));
    assert_equal!(&Value::from(7i64), Value::from(7i64));
}
#[test]
fn value_option_ref_equals() {
    assert_equal!(Some(Value::from(format!("string"))), Some(&Value::from(String::from("string"))));
    assert_equal!(Some(Value::from("string")), Some(&Value::from("string")));
    assert_equal!(Some(Value::from(10.0f64)), Some(&Value::from(10.0f64)));
    assert_equal!(Some(Value::from(0xF1u8)), Some(&Value::from(0xF1u8)));
    assert_equal!(Some(Value::from(0xF1u32)), Some(&Value::from(0xF1u32)));
    assert_equal!(Some(Value::from(7i64)), Some(&Value::from(7i64)));
}
