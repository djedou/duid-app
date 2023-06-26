use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};


pub fn compile(root: &str) {
    create_dist_folder(&root);
    //compile_engine();
}

fn create_dist_folder(root: &str) {
    match Path::new(root).is_dir() {
        true => {},
        false => {
            match fs::create_dir(root) {
                Ok(_) => {},
                Err(_) => {}
            }
        }
    };
}

/*fn compile_engine() {
    match Command::new("rustc")
            .args(["watch", "-w", "src", "-x", "build --release", "-s", "cargo make --makefile ./duid.toml dev"])
            .stdout(Stdio::inherit())
            .status()
        {
            Ok(_) => {},
            Err(err) => {
                println!("Err: {:#?}", err);
            }
        }
}*/