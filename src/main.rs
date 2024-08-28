mod cli;
mod tasks;

use cli::*;

fn main() {
    let cmd_proc = match parse_cmd() {
        Ok(cmd_proc) => cmd_proc,
        Err(e) => return,
    };
    cmd_proc.process_command();
}
