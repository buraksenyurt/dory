use crate::derror::command_error::CommandError;
use std::str::FromStr;

/// Represents incoming commands to the tcp line.
#[derive(Debug,PartialEq)]
pub enum Command {
    Add,
    Get,
    Del,
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADD" => Ok(Command::Add),
            "GET" => Ok(Command::Get),
            "DEL" => Ok(Command::Del),
            _ => Err(CommandError::Unknown),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::command::Command;
    use std::str::FromStr;

    #[test]
    #[should_panic]
    fn command_test() {
        let cmd = Command::from_str("ADD").unwrap();
        assert_eq!(cmd, Command::Add);
        // Commands are case-sensitive
        Command::from_str("add").expect("Command is case-sensitive");
    }
}
