use crate::{Feeder, ShellCore};
use super::simple_command::SimpleCommand;

#[derive(Debug)]
pub struct Pipeline {
    pub commands: Vec<SimpleCommand>,
    pub text: String,
}

impl Pipeline {
    pub fn exec(&mut self, core: &mut ShellCore) {
        for command in self.commands.iter_mut() {
            command.exec(core);
        }
    }

    pub fn parse(text: &mut Feeder, core: &mut ShellCore) -> Option<Pipeline> {
        if let Some(command) = SimpleCommand::parse(text, core) {
            return Some( Pipeline{text: command.text.clone(), commands: vec!(command)} );
        }
        None
    }
}