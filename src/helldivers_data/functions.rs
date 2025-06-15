// This is here so the compiler doesnt complain about unused values/structs/enums. Will be removed when they are.
#![allow(unused, clippy::if_same_then_else)]

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
    #[serde(rename = "level")]
    pub level: i64,
    pub total_experience: i64,
    pub required_experience_for_next: i64,
}

// This is needed cause the json data is hard to store into memory without async.
#[derive(Clone)]
pub struct DataHandling {
    ranks: Vec<TableElement>
}

impl DataHandling {
    // Make a new struct.
    pub fn new() -> Self {
        DataHandling { ranks: Vec::new() }
    }

    // Load the json table with xp values. We use a mutable self to change the ranks variable in the struct.
    // We dont return the values, but we return an error if it happens.
    pub async fn load_table(&mut self) -> Result<(), FunctionsError> {
        let data = match fs::read_to_string("./xp_table.json").await {
            Ok(string) => string,
            Err(err) => return Err(FunctionsError::ErrorReadingTable { fs_error: err })
        };
        let table: Table = serde_json::from_str(&data).unwrap();
        self.ranks = table;
        Ok(())
    }
}

// General function to find the rank number with just xp values.
pub fn find_rank(data_handler: DataHandling, xp: i64) -> i64 {
    // println!("{}", cur_exp);
    let ranks = data_handler.ranks;
    let mut level: i64 = 0;

    for rank in ranks {
        if rank.total_experience > xp {
            continue;
        } else if rank.total_experience == xp {
            level = rank.level
        } else if rank.total_experience < xp {
            level = rank.level
        }
    }
    level
}

// TODO: Make a function that calculates the avg time/xp to the next rank.
// TODO: Make a function that moves forward the current_xp values, and resets the recieved xp values and mission time ones.
// I originally wanted just one function, but the line for calling it is too big.
// So it would be much simpler to make just.. multiple that have dif. purposes. Dont forget to make them public with "pub"!
// pub fn function() {}