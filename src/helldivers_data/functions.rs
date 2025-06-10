// This is here so the compiler doesnt complain about unused values/structs/enums. Will be removed when they are.
#![allow(unused)]

use std::{io::Error, path::Path};

use thiserror::Error;
use serde::{Serialize, Deserialize};
use tokio::fs;

// Error handling, define an error when needed here.
// Examples on how: https://docs.rs/thiserror/latest/thiserror/#example
#[derive(Error, Debug)]
pub enum FunctionsError {
    #[error("Couldn't read the xp table file.")]
    ErrorReadingTable {
        fs_error: Error
    }
}

// For loading the data table. Defined types etc.
pub type Table = Vec<TableElement>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableElement {
    #[serde(rename = "Level")]
    pub level: i64,
    pub total_experience: i64,
    pub required_experience_for_next: i64,
}

// Calculate stuff idk
pub fn calculate() -> Result<i64, FunctionsError> {
    todo!()
}

// Load the json table with xp values.
pub async fn load_table() -> Result<Table, FunctionsError> {
    // This function will not be public. It's now only for testing purposes.
    let data = match fs::read_to_string("./xp_table.json").await {
        Ok(string) => string,
        Err(err) => return Err(FunctionsError::ErrorReadingTable { fs_error: err })
    };
    let table: Table = serde_json::from_str(&data).unwrap();
    Ok(table)
}