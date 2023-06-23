use std::process::{Command, Stdio};


pub(crate) fn generate_template(name: &str) {
    match Command::new("cargo")
        .args(["install", "cargo-generate"])
        .stdout(Stdio::inherit())
        .status()  
        {
            Ok(_) => {},
            Err(output) => {
                println!("Err: {:#?}", output);
            }
        }


    match Command::new("cargo")
            .args(["install", "cargo-watch"])
            .stdout(Stdio::inherit())
            .status() 
        {
            Ok(_) => {},
            Err(watch) => {
                println!("Err: {:#?}", watch);
            }
        }

    match Command::new("cargo")
        .args(["install", "--no-default-features", "cargo-make"]) 
        .stdout(Stdio::inherit())
        .status() 
    {
        Ok(_) => {},
        Err(watch) => {
            println!("Err: {:#?}", watch);
        }
    }

    match Command::new("cargo")
            .args(["generate", "--git", "https://github.com/djedou/duid_template", "--name", name])
            .stdout(Stdio::inherit())
            .status() 
        {
            Ok(_) => {},
            Err(err) => {
                println!("Err: {:#?}", err);
            }
        }
}