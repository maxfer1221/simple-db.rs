use std::{error::Error, fmt};
use self::MetaCommandErrorKind::*;

#[derive(Debug)]
enum MetaCommandErrorKind {
    UnrecognizedCommand,
}

#[derive(Debug)]
pub struct MetaCommandError {
    kind: MetaCommandErrorKind,
    command: String,
}

impl MetaCommandError {
    fn new(kind: MetaCommandErrorKind, command: String) -> Self {
        Self { kind, command }
    }

    pub fn print(&self) {
        print!("{}", self);
    }

    pub fn println(&self) {
        println!("{}", self);
    }
}

impl Error for MetaCommandError {}
impl fmt::Display for MetaCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            UnrecognizedCommand => write!(f, "Unrecognized command '{}'.", self.command),
            _ => write!(f, "Some other error occurred."),
        }
    }
}

pub fn do_meta_command(cmd: &str) -> Result<bool, MetaCommandError> {
    if cmd == ".exit" {
        Ok(false)
    } else { 
        Err(MetaCommandError::new(UnrecognizedCommand, cmd.to_string()))
    }
}
