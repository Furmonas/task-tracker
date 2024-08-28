use std::fmt::Error;

use crate::tasks::*;

pub fn parse_cmd() -> Result<Box<dyn CommandProc>, Error> {
    let command = std::env::args().nth(1);
    let sub_arguments: Vec<String> = std::env::args().skip(2).collect();

    match command {
        Some(command) => find_command(command, sub_arguments),
        None => Err(Error),
    }
}

fn find_command(command: String, sub_arguments: Vec<String>) -> Result<Box<dyn CommandProc>, Error> {
    match command.as_str() {
        "add" => Ok(Box::new(AddCmd {sub_arguments} )),
        "update" => Ok(Box::new(UpdateCmd {sub_arguments} )),
        "delete" => Ok(Box::new(DeleteCmd {sub_arguments} )),
        "mark-in-progress" => Ok(Box::new(MarkProgressCmd {sub_arguments} )),
        "mark-done" => Ok(Box::new(MarkDoneCmd {sub_arguments} )),
        "list" => Ok(Box::new(ListCmd {sub_arguments} )),
        _ => Err(Error),
    }
}