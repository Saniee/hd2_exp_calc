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

#[derive(Clone)]
pub struct DataHandling {
    ranks: Vec<TableElement>
}

impl DataHandling {
    pub fn new() -> Self {
        DataHandling { ranks: Vec::new() }
    }

    // Load the json table with xp values.
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

// Calculate stuff idk
#[allow(clippy::too_many_arguments)]
pub fn calculate(data_handler: DataHandling, cur_rank: i64, want_xp: i64, cur_xp: i64, rec_xp: i64, mis_time: i64, xp_arr: Vec<i64>, time_arr: Vec<i64>) -> (Vec<i64>, Vec<i64>, String) {
    todo!()
}