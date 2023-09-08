use std::error::Error;

pub mod language;
// use language::{Insert, Select};

pub mod interpreter;

pub trait Database<S> {
    type DatabaseError: Error;

    fn new() -> Self;
    fn insert(&mut self, insert_statement: impl Insert<S>) -> Result<(), Self::DatabaseError>;
    fn select(&self, select_statement: impl Select<S>) -> Result<Option<S>, Self::DatabaseError>;
}
