#![allow(unused_imports, unused_variables)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use cfg_if::cfg_if;

use binuid::Compile;
cfg_if! {
    if #[cfg(feature = "interpreter")] {
        use binuid::Interpreter as Engine;
    }
    else if #[cfg(feature = "vm")]{
        use binuid::compiler::bytecode::Interpreter as Engine;
        use binuid::DuidVm;
    }
}


// ANCHOR: repl
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("binuid prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                cfg_if! {
                    if #[cfg(feature = "interpreter")] {
                        match Engine::from_source(&line) {
                            Ok(result) => println!("{}", result),
                            Err(e) => eprintln!("{}", e),
                        };
                    }
                    else if #[cfg(feature = "vm")] {
                        let byte_code = Engine::from_source(&line);
                        let mut vm = DuidVm::<512>::new();
                        vm.load_memory(&byte_code.code);
                        vm.load_instructions(&byte_code.instructions);
                        vm.run();
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