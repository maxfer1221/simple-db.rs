use std::error::Error;

trait Expression {
    const OperationType: Operation;
    
    fn new() -> Self;
}

// allowed operations
pub enum Operation<S> {
    Insert(S),
    // Select(),
}