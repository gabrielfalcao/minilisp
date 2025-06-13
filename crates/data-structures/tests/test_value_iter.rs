use k9::assert_equal;
use minilisp_data_structures::{Cell, Value};

#[test]
fn test_value_list_into_iterator() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));

    let value = Value::list(cell);

    assert_equal!(
        value,
        Value::from({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell
        })
    );

    let strings = value
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>();
    assert_equal!(strings, vec!["a", "b", "c"]);
}

#[test]
fn test_value_quoted_list_into_iterator() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));

    let value = Value::quoted_list(cell);

    assert_equal!(
        value,
        Value::quoted_list({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell
        })
    );

    let strings = value
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>();
    assert_equal!(strings, vec!["a", "b", "c"]);
}
