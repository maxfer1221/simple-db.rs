use anyhow::{Result, anyhow};

use crate::schema::Schema;
use crate::simple_db::Database;
use crate::interpreter;
use crate::expressions::{Expression, Operation};

pub struct Manager<S> {
    database: Database<S>,
    config: Option<String>,
}

impl Manager<Schema> {
    // will eventually allow this to take a config
    pub fn new() -> Result<Self> {
        Ok(Self {
            database: Database::new(),
            config: None
        })
    }
    // will eventually allow this to take a config

    pub fn parse(&self, buff: &str) -> Result<Expression<Schema>> {
        interpreter::parse(buff)
    }

    pub fn execute(&self, expression: Expression<Schema>) -> Result<Option<Vec<Schema>>> {
        use Operation::{Insert, Select};
        let _ = match expression.consume() {
            Select => return Ok(self.database.select()?),
            Insert(row) => self.database.insert(row)?,
            x => return Err(anyhow!("Expression kind can't be executed: {:?}", x)),
        };

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::Manager;
    use crate::simple_db::TABLE_MAX_ROWS;

    #[test]
    fn insert_and_retrieve() {

        let manager = Manager::new().expect("should always succeed");
        let insert_expression = manager.parse("insert 1 user1 person1@example.com").expect("has correct format");
        assert!(manager.execute(insert_expression).is_ok());

        let select_expression = manager.parse("select").expect("has correct format");
        let result = manager.execute(select_expression).expect("assumes database can execute a select command").unwrap();
        assert_eq!(result.len(), 1);

        let result = &result[0];
        assert_eq!(result.id, 1);
        assert_eq!(result.username, to_32_sized_byte_array("user1"));
        assert_eq!(result.email, to_255_sized_byte_array("person1@example.com"));
    }

    #[test]
    fn too_many_inserts_causes_error() {
        let manager = Manager::new().expect("should always succeed");
        for i in 0..TABLE_MAX_ROWS {
            let insert_expression = manager.parse(&format!("insert {i} user{i} person{i}@example.com")).expect("has correct format");
            assert!(manager.execute(insert_expression).is_ok());
        }

        let insert_expression = manager.parse(&format!("insert {} user{} person{}@example.com", TABLE_MAX_ROWS, TABLE_MAX_ROWS, TABLE_MAX_ROWS)).expect("has correct format");
        assert!(manager.execute(insert_expression).is_err());
    }

    #[test]
    fn sanity_check() {
        assert_eq!("\0", std::str::from_utf8(&[0u8]).unwrap());
    }

    fn to_32_sized_byte_array(string: &str) -> [u8; 32] {
        let mut array = [0u8; 32];
        array[..string.len()].clone_from_slice(string.as_bytes());
        array
    }

    fn to_255_sized_byte_array(string: &str) -> [u8; 255] {
        let mut array = [0u8; 255];
        array[..string.len()].clone_from_slice(string.as_bytes());
        array
    }
}