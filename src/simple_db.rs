use std::sync::Mutex;
use anyhow::{Result, anyhow};

use crate::schema::Schema;

use std::mem::size_of;
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / size_of::<Schema>();
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;
// should probably pass all of this into the config for the db. will fix later

pub struct Table<S> {
    row_count: Mutex<usize>,
    pages: [Mutex<Option<Vec<S>>>; TABLE_MAX_PAGES],
}

pub struct Database<S> {
    table: Table<S>,
}
// for posterity
// Arc<Mutex<Vec<S>>>,


impl Database<Schema> {
    pub fn new() -> Self {
        let row_count = Mutex::new(0);
        const INIT: Option<Vec<Schema>> = None;
        let pages: [Mutex<Option<Vec<Schema>>>; 100] = [(); 100].map(|_| Mutex::new(INIT));
        let table = Table {
            row_count,
            pages,
        };
        Self { table }
    }

    pub fn insert(&self, row_to_insert: Schema) -> Result<()> {
        let table = &self.table;

        let mut row_count = match table.row_count.lock() {
            Ok(rc) => rc,
            _ => return Err(anyhow!("Failed to lock Mutex guard for row counter")),
        };

        if *row_count >= TABLE_MAX_ROWS { return Err(anyhow!("Table at capacity")); }
        let page_index = *row_count / ROWS_PER_PAGE;

        let mut page = match table.pages[page_index].lock() {
            Ok(p) => p,
            _ => return Err(anyhow!("Failed to lock Mutex guard for page")),
        };

        if let Some(ref mut page) = &mut *page {
            page.push(row_to_insert);
        } else {
            *page = Some(Vec::with_capacity(ROWS_PER_PAGE));
            if let Some(ref mut page) = &mut *page { page.push(row_to_insert); }
        }

        *row_count += 1;

        Ok(())
    }

    pub fn select(&self) -> Result<Option<Vec<Schema>>> {
        let table = &self.table;

        let row_count = match table.row_count.lock() {
            Ok(rc) => rc,
            _ => return Err(anyhow!("Failed to lock Mutex guard for row counter")),
        };

        let mut guards = Vec::with_capacity(TABLE_MAX_PAGES);
        for i in 0..TABLE_MAX_PAGES {
            let page = match table.pages[i].lock() {
                Ok(p) => p,
                _ => return Err(anyhow!("Failed to lock Mutex guard for page {}", i)),
            };
            if let Some(p) = &*page { 
                guards.push(p.clone());
            } else { break; }
        }

        let mut results: Vec<Schema> = Vec::with_capacity(*row_count);
        for page in guards {
            results.extend(page);
        }

        Ok(Some(results))
    }
}