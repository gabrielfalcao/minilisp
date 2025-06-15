#![allow(unused)]
use k9::assert_equal;
use minilisp_data_structures::{list, Value, Symbol};
use minilisp_util::{dbg};
use minilisp_vm::{Result, VirtualMachine};

#[test]
fn test_eval_defun() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.eval_string(r#"(defun sum(a b) (+ a b))"#)?;
    // dbg!(&vm);
    let val = vm.eval_string(r#"(sum 1 1)"#)?;
    assert_equal!(val, Value::unsigned_integer(2u64));
    Ok(())
}
