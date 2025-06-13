#![allow(unused)]
use k9::assert_equal;
use minilisp_data_structures::{Value, list};
use minilisp_vm::{Result, VirtualMachine};

#[test]
fn test_print() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("print"),
        Value::unsigned_integer(2u64),
        Value::unsigned_integer(2u64),
    ]);

    let val = vm.eval_ast(ast)?;
    assert_equal!(val, Value::from("2 2"));
    Ok(())
}
