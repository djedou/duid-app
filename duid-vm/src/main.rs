use duid_vm::Compile;
use duid_vm::compiler::vm::{
    bytecode::Interpreter as Engine,
    vm::DuidVm
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }

    let byte_code = Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap());
    println!("byte_code dump: {:?}", byte_code);
    let mut vm = DuidVm::<1024>::new();
    vm.push(&byte_code.code);
    //println!("vm dump: {:?}", vm);
    vm.run();
    //println!("vm dump: {:?}", vm);
}
