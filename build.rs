use std::path::{PathBuf};
use std::process::{exit, Command};
use std::{env, fs};

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("cargo:rerun-if-changed=tailwind/tailwind.config.js");
    let path = PathBuf::from("static/css/output.css");
    
    if !path.exists() {
        println!("cargo:rerun-if-changed=static/css/output.css");
    }
    
    if !is_program_in_path("npx") {
        eprintln!("npx is missing");
        exit(1);
    }

    match Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./tailwind/input.css",
            "-o",
            "./static/css/output.css",
        ])
        .spawn()
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
