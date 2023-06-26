use duid_app::duid_core::compiler::compile;

fn main() {
    println!("########### started compiling...... ############");
    compile("./dist");
    println!("########### ended compiling...... ############");
}