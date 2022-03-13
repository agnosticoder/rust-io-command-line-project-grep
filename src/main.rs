use rust_io_command_line_project::{run, Config};
use std::env;
use std::process;

fn main() {
    //config variables (extracted)
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argruments: {}", err);
        process::exit(1);
    });

    // running app code which need Config instance as arg
    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
