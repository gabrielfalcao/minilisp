#![allow(unused)]
use k9::assert_equal;
use minilisp_parser::{Item, Value};
use minilisp_vm::{Result, VirtualMachine};
use minilisp_util::{vec_deque};


#[test]
fn test_multiply_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = Item::List(vec_deque![
        Item::symbol("*"),
        Item::Value(Value::Integer(3)),
        Item::Value(Value::Integer(7)),
    ]);

    let val = vm.eval_ast(ast)?;
    assert_equal!(val, Value::Integer(21));
    Ok(())
}

#[test]
fn test_add_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = Item::List(vec_deque![
        Item::symbol("+"),
        Item::Value(Value::UnsignedInteger(2)),
        Item::Value(Value::UnsignedInteger(2)),
    ]);

    let val = vm.eval_ast(ast)?;
    assert_equal!(val, Value::UnsignedInteger(4));
    Ok(())
}


#[test]
fn test_subtract_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = Item::List(vec_deque![
        Item::symbol("-"),
        Item::Value(Value::UnsignedInteger(5)),
        Item::Value(Value::UnsignedInteger(2)),
    ]);

    let val = vm.eval_ast(ast)?;
    assert_equal!(val, Value::UnsignedInteger(3));
    Ok(())
}



#[test]
fn test_divide_numbers() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = Item::List(vec_deque![
        Item::symbol("/"),
        Item::Value(Value::Float(30.0)),
        Item::Value(Value::Float(3.0)),
    ]);

    let val = vm.eval_ast(ast)?;
    assert_equal!(val, Value::Float(10.0));
    Ok(())
}
