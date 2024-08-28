use std::fmt::Error;

use crate::tasks::*;

pub fn parse_cmd() -> Result<&'static dyn CommandProc, Error> {
    let command = std::env::args().nth(1);

    match command {
        Some(command) => find_command(command),
        None => Err(Error),
    }
}

pub fn parse_sub_arguments() -> Vec<String> {
    std::env::args().skip(2).collect()
}

fn find_command(command: String) -> Result<&'static dyn CommandProc, Error> {
    match command.as_str() {
        "add" => Ok(&AddCmd),
        "update" => Ok(&UpdateCmd),
        "delete" => Ok(&DeleteCmd),
        "mark-in-progress" => Ok(&MarkProgressCmd),
        "mark-done" => Ok(&MarkDoneCmd),
        "list" => Ok(&ListCmd),
        _ => Err(Error),
    }
}