mod cli;
mod tasks;

use cli::*;

fn main() {
    match parse_cmd() {
        Ok(cmd_proc) => cmd_proc.process_command(),
        Err(_) => println!("No command provided"),
    };
}
