use duid_vm::Compile;
use duid_vm::compiler::vm::bytecode::Interpreter as Engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }

    let byte_code = Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap());
    println!("byte code: {:?}", byte_code);
    //let mut vm = DuidVm::new(byte_code);
    //vm.run();
}
