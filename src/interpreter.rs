use anyhow::{Result, anyhow};

use crate::schema::Schema;
use crate::expressions::{Expression, Operation::*};

pub fn parse(buff: &str) -> Result<Expression<Schema>> {
    let mut words = buff.split(" ");
    let command = words.next().ok_or(anyhow!("Missing command"))?;

    if command.len() < 1 { return Ok(Expression::new(Empty)); }

    if command == ".exit" { return Ok(Expression::new(Exit)); }
    
    match &command.to_lowercase()[..] {
        "insert" => {
            let item = interpret_item(words)?; 

            return Ok(Expression::new(Insert(item)));       
        },
        "select" => {
            return Ok(Expression::new(Select));
        },
        _ => {}
    }

    Err(anyhow!("Unrecognized command '{}'", buff))
} 

fn interpret_item<'a>(mut words: impl Iterator<Item = &'a str>) -> Result<Schema> {
    let id_as_string = words.next().ok_or(anyhow!("Missing ID"))?;
    let id = id_as_string.parse::<u32>().map_err(|_| anyhow!("Invalid ID: '{}'. ID should be of type u32", id_as_string))?;

    let raw_username = words.next().ok_or(anyhow!("Missing username"))?;
    if raw_username.len() > 32 { return Err(anyhow!("Username is too long; maximum length is 32 characters")) }
    let mut username = [0u8; 32];
    username[..raw_username.len()].clone_from_slice(raw_username.as_bytes());

    let raw_email = words.next().ok_or(anyhow!("Missing email"))?;
    if raw_email.len() > 255 { return Err(anyhow!("Email is too long; maximum length is 255 characters")) }
    let mut email = [0u8; 255];
    email[..raw_email.len()].clone_from_slice(raw_email.as_bytes());

    Ok(Schema { id, username, email })
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert_item_with_max_length_fields() {
        use crate::interpreter::parse;

        let id = std::u32::MAX;
        let username = (0..32).map(|_| "x").collect::<String>();
        let email = (0..255).map(|_| "x").collect::<String>();

        let result = parse(&format!("insert {} {} {}", id, username, email));

        assert!(result.is_ok());
    }
    
    #[test]
    fn fail_to_insert_with_non_u32_id() {
        use crate::interpreter::parse;

        let result = parse("insert nonu32value a b");

        assert!(result.is_err());
    }

    #[test]
    fn fail_to_insert_with_username_too_long() {
        use crate::interpreter::parse;

        let id = std::u32::MAX;
        let username = (0..33).map(|_| "x").collect::<String>();
        let email = (0..255).map(|_| "x").collect::<String>();

        let result = parse(&format!("insert {} {} {}", id, username, email));

        assert!(result.is_err());
    }

    #[test]
    fn fail_to_insert_with_email_too_long() {
        use crate::interpreter::parse;

        let id = std::u32::MAX;
        let username = (0..32).map(|_| "x").collect::<String>();
        let email = (0..256).map(|_| "x").collect::<String>();

        let result = parse(&format!("insert {} {} {}", id, username, email));

        assert!(result.is_err());
    }

    #[test]
    fn fail_to_insert_with_missing_data() {
        use crate::interpreter::parse;

        let result = parse("insert 1 a");

        assert!(result.is_err());
    }

    #[test]
    fn fail_at_invalid_command() {
        use crate::interpreter::parse;

        let result = parse("a");

        assert!(result.is_err());
    }

    #[test]
    fn dot_exit_is_ok() {
        use crate::interpreter::parse;

        let result = parse(".exit");

        assert!(result.is_ok());
    }
}