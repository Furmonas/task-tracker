pub trait CommandProc {
    fn process_command(&self);
}

pub struct AddCmd {
    pub sub_arguments: Vec<String>
}
pub struct UpdateCmd {
    pub sub_arguments: Vec<String>
}
pub struct DeleteCmd {
    pub sub_arguments: Vec<String>
}
pub struct MarkProgressCmd {
    pub sub_arguments: Vec<String>
}
pub struct MarkDoneCmd {
    pub sub_arguments: Vec<String>
}
pub struct ListCmd {
    pub sub_arguments: Vec<String>
}

impl CommandProc for AddCmd {
    fn process_command(&self) {
        println!("Add command received args -> {:?}", self.sub_arguments);
    }
}

impl CommandProc for UpdateCmd {
    fn process_command(&self) {
        println!("Update command received args -> {:?}", self.sub_arguments);
    }
}

impl CommandProc for DeleteCmd {
    fn process_command(&self) {
        println!("Delete command received args -> {:?}", self.sub_arguments);
    }
}

impl CommandProc for MarkProgressCmd {
    fn process_command(&self) {
        println!("Mark-in-progress command received args -> {:?}", self.sub_arguments);
    }
}

impl CommandProc for MarkDoneCmd {
    fn process_command(&self) {
        println!("Mark-done command received args -> {:?}", self.sub_arguments);
    }
}

impl CommandProc for ListCmd {
    fn process_command(&self) {
        println!("List command received args -> {:?}", self.sub_arguments);
    }
}