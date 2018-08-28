use std::process::Command;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	println!("{} {}", args[0], env!("CARGO_PKG_VERSION"));

	Command::new("cargo")
		.arg("install")
		.arg("--path")
		.arg(".")
		.arg("--force")
		.spawn()
		.expect("failed to execute process")
		.wait()
		.expect("command wasn't running");
}
