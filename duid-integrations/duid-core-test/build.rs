use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("routes.rs");

    let paths = std::fs::read_dir("./src/app").unwrap();

    let path_names: Vec<_> = paths.into_iter().map(|p| {
        let mut path_name = format!("{}", p.unwrap().path().display());
        let path_name = path_name
            .replace("./src", "crate")
            .replace("/", "::")
            .replace("\\", "::")
            .replace(".rs", "");

        if path_name.ends_with("page") {
            path_name
        }
        else {
            format!("{}::page", path_name)
        }
    })
    .filter(|p| !p.contains("::mod::"))
    .collect();

    fs::write(
        &dest_path,
        &format!("
pub fn user_app_routes() -> Vec<&'static str> {{
    let paths = vec!{:?};


    paths
}}
            
        ", path_names)
    ).unwrap();

    //panic!();
}