use super::*;

pub trait CommandProc {
    fn process_command(&self);
}

pub struct AddCmd;
pub struct UpdateCmd;
pub struct DeleteCmd;
pub struct MarkProgressCmd;
pub struct MarkDoneCmd;
pub struct ListCmd;

impl CommandProc for AddCmd {
    fn process_command(&self) {
        let sub_arguments: Vec<String> = parse_sub_arguments();

        println!("Add command received args -> {:?}", sub_arguments);
    }
}

impl CommandProc for UpdateCmd {
    fn process_command(&self) {
        let sub_arguments: Vec<String> = parse_sub_arguments();

        println!("Update command received args -> {:?}", sub_arguments);
    }
}

impl CommandProc for DeleteCmd {
    fn process_command(&self) {
        let sub_arguments: Vec<String> = parse_sub_arguments();

        println!("Delete command received args -> {:?}", sub_arguments);
    }
}

impl CommandProc for MarkProgressCmd {
    fn process_command(&self) {
        let sub_arguments: Vec<String> = parse_sub_arguments();

        println!("Mark-in-progress command received args -> {:?}", sub_arguments);
    }
}

impl CommandProc for MarkDoneCmd {
    fn process_command(&self) {
        let sub_arguments: Vec<String> = parse_sub_arguments();

        println!("Mark-done command received args -> {:?}", sub_arguments);
    }
}

impl CommandProc for ListCmd {
    fn process_command(&self) {
        let sub_arguments: Vec<String> = parse_sub_arguments();

        println!("List command received args -> {:?}", sub_arguments);
    }
}