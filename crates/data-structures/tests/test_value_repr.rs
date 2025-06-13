use minilisp_data_structures::{
    assert_debug_equal, assert_display_equal, Cell, Value,
};

#[test]
fn test_nil() {
    assert_display_equal!(Value::Nil, "nil");
    assert_debug_equal!(Value::Nil, "nil");
}
#[test]
fn test_t() {
    assert_display_equal!(Value::T, "t");
    assert_debug_equal!(Value::T, "t");
}
#[test]
fn test_string() {
    assert_display_equal!(Value::string("string"), r#""string""#);
    assert_debug_equal!(Value::string("string"), r#""string""#);
}
#[test]
fn test_symbol() {
    assert_display_equal!(Value::symbol("symbol"), "symbol");
    assert_debug_equal!(Value::symbol("symbol"), "symbol");
}
#[test]
fn test_quotedsymbol() {
    assert_display_equal!(Value::quoted_symbol("symbol"), "'symbol");
    assert_debug_equal!(Value::quoted_symbol("symbol"), "'symbol");
}
#[test]
fn test_byte() {
    assert_display_equal!(Value::byte(0xF1), "0xf1");
    assert_debug_equal!(Value::byte(0xF1), "0xf1");
}
#[test]
fn test_unsignedinteger() {
    assert_display_equal!(Value::unsigned_integer(808u64), "808");
    assert_debug_equal!(Value::unsigned_integer(808u64), "808");
}
#[test]
fn test_integer() {
    assert_display_equal!(Value::integer(-808i64), "-808");
    assert_debug_equal!(Value::integer(808i64), "808");
}
#[test]
fn test_float() {
    assert_display_equal!(
        Value::float(2.718281828459045),
        "2.718281828459045"
    );
    assert_debug_equal!(Value::float(3.141592653589793), "3.141592653589793");
}
#[test]
fn test_list() {
    assert_display_equal!(
        Value::list({
            let mut cell = Cell::from("head");
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "(head 10 x)"
    );
    assert_debug_equal!(
        Value::list({
            let mut cell = Cell::from("head");
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "(head 10 x)"
    );

    assert_display_equal!(
        Value::list({
            let mut cell = Cell::nil();
            cell.add(&Cell::from("head"));
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "(head 10 x)"
    );
    assert_debug_equal!(
        Value::list({
            let mut cell = Cell::nil();
            cell.add(&Cell::from("head"));
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "(head 10 x)"
    );
}

#[test]
fn test_quotedlist() {
    assert_display_equal!(
        Value::quoted_list({
            let mut cell = Cell::from("head");
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "'(head 10 x)"
    );
    assert_debug_equal!(
        Value::quoted_list({
            let mut cell = Cell::from("head");
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "'(head 10 x)"
    );

    assert_display_equal!(
        Value::quoted_list({
            let mut cell = Cell::nil();
            cell.add(&Cell::from("head"));
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "'(head 10 x)"
    );
    assert_debug_equal!(
        Value::quoted_list({
            let mut cell = Cell::nil();
            cell.add(&Cell::from("head"));
            cell.add(&Cell::from(10i64));
            cell.add(&Cell::from("x"));
            cell
        }),
        "'(head 10 x)"
    );
}
#[test]
fn test_emptylist() {
    assert_display_equal!(Value::empty_list(), "()");
    assert_debug_equal!(Value::empty_list(), "()");
    assert_display_equal!(Value::EmptyList, "()");
    assert_debug_equal!(Value::EmptyList, "()");
}
#[test]
fn test_emptyquotedlist() {
    assert_display_equal!(Value::empty_quoted_list(), "'()");
    assert_debug_equal!(Value::empty_quoted_list(), "'()");
    assert_display_equal!(Value::EmptyQuotedList, "'()");
    assert_debug_equal!(Value::EmptyQuotedList, "'()");
}
