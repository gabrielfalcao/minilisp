#![allow(unused)]
use k9::assert_equal;
use minilisp_data_structures::{Cell, Value};

#[test]
fn test_iterator() {
    let mut head = Cell::new(Value::from("head"));
    head.add(&mut Cell::new(Value::integer(10)));
    head.add(&mut Cell::new(Value::symbol("x")));

    assert_equal!(
        format!("{:#?}", &head),
        "Cell[head=\"head\" | tail=Cell[head=10 | tail=Cell[head=x | tail: null]]]"

    );
    assert_equal!(
        head.into_iter().map(|value| value.to_string()).collect::<Vec<String>>(),
        vec!["head", "10", "x"]
    );
}
