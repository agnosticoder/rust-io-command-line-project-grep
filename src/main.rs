use rust_io_command_line_project::{get_cmd_line_arg, run, Config};
use std::process;

fn main() {
    //getting command line arguments
    let args = get_cmd_line_arg();

    //config variables (extracted)
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argruments: {}", err);
        process::exit(1);
    });

    // running app code which need Config instance as arg
    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
