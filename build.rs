use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

fn is_program_in_path(program: &str) -> bool {
	if let Ok(path) = env::var("PATH") {
		for p in path.split(':') {
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
	let path = PathBuf::from("static/css/bundle.css");
	if !path.exists() {
		println!("cargo:rerun-if-changed=static/css/bundle.css");
	}

	if !is_program_in_path("yarn") {
		eprintln!("yarn is missing");
		exit(1);
	}
	env::set_current_dir("static_src").unwrap();

	//in development mode we compile without minification
	#[cfg(debug_assertions)]
	match Command::new("yarn").args(["compile"]).spawn() {
		Ok(_) => (),
		Err(e) => {
			eprintln!("{e}");
		}
	}

	//in release mode this compiles tailwind minified and then uses teser and post css to minify everything else
	#[cfg(not(debug_assertions))]
	match Command::new("yarn").args(["compile:prod"]).spawn() {
		Ok(_) => (),
		Err(e) => {
			eprintln!("{e}");
		}
	}
}
