use std::io::{self, Write};
use anyhow::Result;
// because i'm lazy

mod schema;
use schema::Schema;
// database schema

mod expressions;
use expressions::Operation;
// db language
mod interpreter;
// input processor
mod simple_db;
// virtual machine
mod manager;
// database manager

fn main() -> Result<()> {
    let mut input_buffer = String::new();
    // opted for a string to protect against buffer overflows
    
    let db_manager = manager::Manager::<Schema>::new()?;

    loop {
        print_prompt()?;
        read_input(&mut input_buffer)?;

        let expression = match db_manager.parse(&input_buffer) {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

        use Operation::*;
        let _ = match expression.kind() {
            Empty => continue,
            Exit => break,
            _ => db_manager.execute(expression),
        };
    }

    // exit process
    Ok(())
}

fn read_input(buff: &mut String) -> Result<()> {
    buff.clear();
    io::stdin().read_line(buff)?;
    // read input
    
    buff.pop();
    // remove '\n'
    
    Ok(())
}

fn print_prompt() -> Result<()>{
    print!("db > ");
    io::stdout().flush()?;
    Ok(())
}