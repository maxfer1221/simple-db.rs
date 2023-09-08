use std::error::Error;

pub trait Interpreter<S> {
    type InterpreterError: Error;

    fn parse(string: &mut String) -> Result<Self::Action, Self::InterpreterError>;
}
