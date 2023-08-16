use binuid::Compile;
use binuid::{
    compiler::bytecode::Interpreter as Engine,
    DuidVm
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }

    let byte_code = Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap());
    //println!("byte_code dump: {:?}", byte_code);
    let mut vm = DuidVm::<256>::new();
    vm.load_memory(&byte_code.code);
    vm.load_instructions(&byte_code.instructions);
    vm.run();
    //println!("vm dump: {:?}", vm);
}
