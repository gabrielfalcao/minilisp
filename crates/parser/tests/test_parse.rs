#![allow(unused)]
use k9::assert_equal;
use minilisp_parser::ast::{Item, Value};
use minilisp_parser::test::stub_input;
use minilisp_parser::{assert_ast_equal, parse_source, Result};
use minilisp_util::vec_deque;

#[test]
fn test_cons_of_literal_strings() -> Result<'static, ()> {
    // (cons "a" "b")
    let items = parse_source(r#"(cons "a" "b")"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("cons"),
            Item::Value(Value::from("a")),
            Item::Value(Value::from("b")),
        ])]
    );
    Ok(())
}

#[test]
fn test_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b")
    let items = parse_source(r#"(list "a" "b")"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("list"),
            Item::Value(Value::from("a")),
            Item::Value(Value::from("b")),
        ])]
    );
    Ok(())
}

#[test]
fn test_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b")
    let items = parse_source(r#"'("a" "b")"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("quote"),
            Item::Value(Value::from("a")),
            Item::Value(Value::from("b")),
        ])]
    );
    Ok(())
}

#[test]
fn test_call_to_function_add_two_numbers() -> Result<'static, ()> {
    // (+ 1 2)
    let items = parse_source(r#"(+ 1 2)"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("+"),
            Item::Value(Value::UnsignedInteger(1u32)),
            Item::Value(Value::UnsignedInteger(2u32)),
        ])]
    );
    Ok(())
}

#[test]
fn test_list_of_literal_strings_and_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (list "a" "b" '("b" "c"))
    let items = parse_source(r#"(list "a" "b" '("c" "d"))"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("list"),
            Item::Value(Value::from("a")),
            Item::Value(Value::from("b")),
            Item::List(vec_deque![
                Item::symbol("quote"),
                Item::Value(Value::from("c")),
                Item::Value(Value::from("d")),
            ]),
        ])]
    );
    Ok(())
}

#[test]
fn test_cons_of_car_literal_string_and_cdr_quoted_list_of_literal_strings() -> Result<'static, ()> {
    // (cons "a" '("b" "c"))
    let items = parse_source(r#"(cons "a" '("b" "c"))"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("cons"),
            Item::Value(Value::from("a")),
            Item::List(vec_deque![
                Item::symbol("quote"),
                Item::Value(Value::from("b")),
                Item::Value(Value::from("c")),
            ]),
        ])]
    );
    Ok(())
}

#[test]
fn test_print() -> Result<'static, ()> {
    let items = parse_source(r#"(print "t")"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![Item::symbol("print"), Item::Value(Value::from("t")),])]
    );
    Ok(())
}

#[test]
fn test_defun() -> Result<'static, ()> {
    // (defun myfun() (cons "a" '("b" "c")))
    let items = parse_source(r#"(defun myfun() (cons "a" '("b" "c")))"#)?;
    assert_ast_equal!(
        items,
        vec_deque![Item::List(vec_deque![
            Item::symbol("defun"),
            Item::symbol("myfun"),
            Item::List(vec_deque![]),
            Item::List(vec_deque![
                Item::symbol("cons"),
                Item::Value(Value::from("a")),
                Item::List(vec_deque![
                    Item::symbol("quote"),
                    Item::Value(Value::from("b")),
                    Item::Value(Value::from("c")),
                ])
            ])
        ])]
    );
    Ok(())
}
