use minilisp_parser::parse_source;
use minilisp_util::color;
use minilisp_vm::{Result, VirtualMachine};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn print_error<T: std::fmt::Display>(error: T) {
    eprintln!(
        "{}",
        [
            color::ansi("Error:", 196, 16),
            color::ansi(error.to_string(), 220, 16),
        ]
        .join(" ")
    );
}
fn main() -> Result<()> {
    Ok(repl()?)
}
fn repl<'a>() -> Result<()> {
    let mut vm = VirtualMachine::new();
    print!("\x1b[2J\x1b[3J\x1b[H");
    println!("minilisp VM version {}", env!("CARGO_PKG_VERSION"));
    let mut rl = DefaultEditor::new().unwrap();

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
                let line: &'a str = line.clone().leak();
                rl.add_history_entry(line).unwrap();
                match line.clone().trim() {
                    "@" => {
                        println!("{:#?}", vm.symbols());
                        continue;
                    },
                    _ => match parse_source(line) {
                        Ok(value) => {
                            println!("{}", vm.eval_ast(value)?);
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
    rl.save_history(".minilisp.history").unwrap();
    Ok(())
}
