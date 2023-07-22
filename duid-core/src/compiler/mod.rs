use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};


/*
pub fn compile(root: &str) {
    create_dist_folder(&root);
}*/

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

#[macro_export]
macro_rules! user_app {
    ($($x:expr),* ) => {
        {
            /*let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec*/
            
            $(
                duid_app::duid_core::console::info!("paths: {:#?}", $x);
            )*
        } 
    };
}