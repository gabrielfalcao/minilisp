#![allow(unused)]
use k9::assert_equal;
use minilisp_parser::{Item, Value};
use minilisp_vm::{Result, VirtualMachine};
use minilisp_util::{vec_deque};

#[test]
fn test_print() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = Item::List(vec_deque![
        Item::symbol("print"),
        Item::Value(Value::UnsignedInteger(2)),
        Item::Value(Value::UnsignedInteger(2)),
    ]);

    let val = vm.eval_ast(ast)?;
    assert_equal!(val, Value::from("2 2"));
    Ok(())
}
