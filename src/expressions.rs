#[derive(Debug)]
pub enum Operation<S> {
    Empty,
    Exit,
    Insert(S),
    Select,
}

#[derive(Debug)]
pub struct Expression<S> {
    kind: Operation<S>,
}

impl<S> Expression<S> {
    pub fn new(kind: Operation<S>) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &Operation<S> {
        &self.kind
    }

    pub fn consume(self) -> Operation<S> {
        self.kind
    }
}