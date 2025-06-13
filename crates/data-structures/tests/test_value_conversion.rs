use std::borrow::Cow;

use k9::assert_equal;
use minilisp_data_structures::{assert_debug_equal, assert_display_equal, Value};

#[test]
fn value_from_bool() {
    assert_equal!(Value::from(true), Value::T);
    assert_equal!(Value::from(false), Value::Nil);
}
#[test]
fn value_from_unit() {
    assert_equal!(Value::from(()), Value::Nil);
}
#[test]
fn value_from_str() {
    assert_equal!(Value::from("str"), Value::string(Cow::from("str")));
    assert_display_equal!(Value::from("str"), r#""str""#);
    let value = "str".to_string().leak();
    assert_debug_equal!(Value::from(value), r#""str""#);
}
#[test]
fn value_from_string() {
    let value = "string".to_string();
    assert_equal!(Value::from(value).to_string(), r#""string""#);
    let value = "string".to_string();
    assert_display_equal!(Value::from(value), r#""string""#);
    let value = "string".to_string();
    assert_debug_equal!(Value::from(value), r#""string""#);
}
#[test]
fn value_display_nil() {
    assert_display_equal!(Value::Nil, "nil");
}
#[test]
fn value_debug_nil() {
    assert_debug_equal!(Value::Nil, "nil");
}
