use std::collections::VecDeque;

use minilisp_parser::{parse_source, Item, Value};
use minilisp_util::color;
use minilisp_vm::VirtualMachine;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
fn print_error<T: std::fmt::Display>(error: T) {
    eprintln!(
        "{}",
        [color::ansi("Error:", 196, 16), color::ansi(error.to_string(), 220, 16),].join(" ")
    );
}
fn main() -> Result<()> {
    let mut vm = VirtualMachine::new();
    print!("\x1b[2J\x1b[3J\x1b[H");
    println!("minilisp VM version {}", env!("CARGO_PKG_VERSION"));
    let mut rl = DefaultEditor::new()?;

    if rl.load_history(".minilisp.history").is_err() {
        println!("No previous history.");
    }
    println!("\tHELP:");
    println!("\ttype `@' to see the symbol table");
    println!("\ttry arithmetic expressions such as `(* 4 (+ 3 2))'");
    loop {
        let readline = rl.readline(": ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let line = line.to_string();
                match line.trim() {
                    "@" => {
                        println!("{:#?}", vm.symbols());
                        continue;
                    },
                    _ => match parse_source(&line) {
                        Ok(ast) => {
                            let ast = ast
                                .iter()
                                .map(|node| node.clone().as_item())
                                .collect::<VecDeque<Item<'_>>>();
                            match vm.eval_ast(match ast.len() {
                                0 => Item::Value(Value::Nil),
                                1 => ast.front().map(Clone::clone).unwrap(),
                                _ => Item::List(ast),
                            }) {
                                Ok(value) => {
                                    println!("{}", value);
                                    continue;
                                },
                                Err(error) => {
                                    print_error(error);
                                    continue;
                                },
                            }
                        },
                        Err(error) => {
                            print_error(error);
                            continue;
                        },
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
    rl.save_history(".minilisp.history")?;
    Ok(())
}
