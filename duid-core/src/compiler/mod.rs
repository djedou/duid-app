use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};


pub fn compile(root: &str) {
    create_dist_folder(&root);
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


