#![allow(unused_imports, unused_variables)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use cfg_if::cfg_if;

use duid_vm::Compile;
cfg_if! {
    if #[cfg(feature = "interpreter")] {
        use duid_vm::Interpreter as Engine;
    }
    else if #[cfg(feature = "vm")]{
        use duid_vm::vm::bytecode::Interpreter as Engine;
        use duid_vm::VM;
    }
}

// ANCHOR: repl
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("duid_vm prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                cfg_if! {
                    if #[cfg(any(feature = "interpreter"))] {
                        match Engine::from_source(&line) {
                            Ok(result) => println!("{}", result),
                            Err(e) => eprintln!("{}", e),
                        };
                    }
                    else if #[cfg(feature = "vm")] {
                        let byte_code = Engine::from_source(&line);
                        //println!("byte code: {:?}", byte_code);
                        let mut vm = VM::new(byte_code);
                        vm.run();
                        println!("{}", vm.pop_last());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break Ok(());
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break Ok(());
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break Ok(());
            }
        }
    }
    // ANCHOR_END: repl
}
