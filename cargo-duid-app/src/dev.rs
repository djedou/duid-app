use std::process::{Command, Stdio};

pub(crate) fn dev() {
    match Command::new("cargo")
            .args(["watch", "-w", "src", "-x", "build --release", "-s", "cargo make --makefile ./duid.toml dev"])
            .stdout(Stdio::inherit())
            .status()
        {
            Ok(_) => {},
            Err(err) => {
                println!("Err: {:#?}", err);
            }
        }
}
