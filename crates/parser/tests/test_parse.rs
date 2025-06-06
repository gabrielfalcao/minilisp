#![allow(unused)]
use k9::assert_equal;
use minilisp_parser::ast::{Item, Value};
use minilisp_parser::test::stub_input;
use minilisp_parser::{parse_source, Result};
use minilisp_util::{vec_deque};

#[test]
fn test_cons_of_literal_strings() -> Result<'static, ()> {
    // (cons "a" "b")
    let items = parse_source(r#"(cons "a" "b")"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("cons"),
            Item::Value(Value::String("a")),
            Item::Value(Value::String("b")),
        ])]
    );
    Ok(())
}

#[test]
fn test_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b")
    let items = parse_source(r#"(list "a" "b")"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("list"),
            Item::Value(Value::String("a")),
            Item::Value(Value::String("b")),
        ])]
    );
    Ok(())
}

#[test]
fn test_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b")
    let items = parse_source(r#"'("a" "b")"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("list"),
            Item::Value(Value::String("a")),
            Item::Value(Value::String("b")),
        ])]
    );
    Ok(())
}

#[test]
fn test_call_to_function_add_two_numbers() -> Result<'static, ()> {
    // (+ 1 2)
    let items = parse_source(r#"(+ 1 2)"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("+"),
            Item::Value(Value::UnsignedInteger(1u64)),
            Item::Value(Value::UnsignedInteger(2u64)),
        ])]
    );
    Ok(())
}

#[test]
fn test_list_of_literal_strings_and_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b" '("b" "c"))
    let items = parse_source(r#"(list "a" "b" '("c" "d"))"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("list"),
            Item::Value(Value::String("a")),
            Item::Value(Value::String("b")),
            Item::List(vec_deque![
                Item::Symbol("list"),
                Item::Value(Value::String("c")),
                Item::Value(Value::String("d")),
            ]),
        ])]
    );
    Ok(())
}

#[test]
fn test_cons_of_car_literal_string_and_cdr_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (cons "a" '("b" "c"))
    let items = parse_source(r#"(cons "a" '("b" "c"))"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("cons"),
            Item::Value(Value::String("a")),
            Item::List(vec_deque![
                Item::Symbol("list"),
                Item::Value(Value::String("b")),
                Item::Value(Value::String("c")),
            ]),
        ])]
    );
    Ok(())
}

#[test]
fn test_print() -> Result<'static, ()> {
    let items = parse_source(r#"(print "t")"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![Item::Symbol("print"), Item::Value(Value::String("t")),])]
    );
    Ok(())
}

#[test]
fn test_defun() -> Result<'static, ()> {
    // (defun myfun() (cons "a" '("b" "c")))
    let items = parse_source(r#"(defun myfun() (cons "a" '("b" "c")))"#)?;
    assert_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::Symbol("defun"),
            Item::Symbol("myfun"),
            Item::List(vec_deque![]),
            Item::List(vec_deque![
                Item::Symbol("cons"),
                Item::Value(Value::String("a")),
                Item::List(vec_deque![
                    Item::Symbol("list"),
                    Item::Value(Value::String("b")),
                    Item::Value(Value::String("c")),
                ])
            ])
        ])]
    );
    Ok(())
}
