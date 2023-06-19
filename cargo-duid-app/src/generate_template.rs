use std::process::Command;
use std::io::{self, Write};


pub(crate) fn generate_template(name: &str) {

    match Command::new("cargo")
            .args(["generate", "--git", "https://github.com/djedou/duid_template", "--name", name])
            .output() 
        {
            Ok(output) => {
                match output.status.success() {
                    true => {
                        io::stdout().write_all(&output.stdout).unwrap();
                    },
                    false => {
                        // TODO: check on for this error message (error: no such command: `generate`)
                        //      and make a decision base on it

                        io::stderr().write_all(&output.stderr).unwrap();
                        io::stdout().write_all(&output.stdout).unwrap();
                        println!("########## Please hold on, installing some packages!! ############");
                        match Command::new("cargo")
                                .args(["install", "cargo-generate"])
                                .output() 
                            {
                                Ok(output) => {
                                    io::stdout().write_all(&output.stdout).unwrap();
                                    io::stderr().write_all(&output.stderr).unwrap();
                                },
                                Err(output) => {
                                    println!("Err: {:#?}", output);
                                }
                            }
                        println!("########## Please re-run again!! ############");
                    }
                }
            },
            Err(err) => {
                println!("Err: {:#?}", err);
            }
        }
}