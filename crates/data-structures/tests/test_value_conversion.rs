#![allow(unused)]
use std::borrow::Cow;
use minilisp_data_structures::*;
use k9::assert_equal;


#[macro_export]
macro_rules! assert_value_conversion {
    ($variant:ident, $from:expr , $convert:expr, $display:literal, $debug:literal) => {
        assert_equal!(Value::from($from), Value::Symbol(Cow::from("static-str")));
        assert_display_equal!(Value::from($from), $display);
        assert_debug_equal!(Value::from($from), $debug);
    };
}

#[test]
fn value_symbol_conversion() {
    assert_value_conversion!(Symbol, "static-str", Cow::from, "static-str", "\"static-str\"");
}
#[test]
fn value_symbol() {
    assert_value_conversion!(Symbol, "static-str", Cow::from, "static-str", "\"static-str\"");
    assert_equal!(Value::from("static-str"), Value::Symbol(Cow::from("static-str")));
    assert_display_equal!(Value::from("static-str"), "static-str");
    assert_debug_equal!(Value::from("static-str"), "\"static-str\"");
}
#[test]
fn value_string() {
    assert_equal!(Value::from("string".to_string()), Value::String(String::from("string")));
    assert_display_equal!(Value::from("string".to_string()), "string");
    assert_debug_equal!(Value::from("string".to_string()), "\"string\"");
}
#[test]
fn value_nil() {
    assert_equal!(Value::from(false), Value::T);
    assert_display_equal!(Value::Nil, "nil");
    assert_debug_equal!(Value::Nil, "nil");
}
