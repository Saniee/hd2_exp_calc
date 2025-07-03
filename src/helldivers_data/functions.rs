// This is here so the compiler doesnt complain about unused values/structs/enums. Will be removed when they are.
#![allow(unused, clippy::if_same_then_else)]

use time::{self, Duration};
use std::{io::Error, path::Path};

use thiserror::Error;
use serde::{Serialize, Deserialize};
use tokio::fs;

use crate::gui::{AvgResult, ExperienceInputs};

// Error handling, define an error when needed here.
// Examples on how: https://docs.rs/thiserror/latest/thiserror/#example
#[derive(Error, Debug)]
pub enum FunctionsError {
    #[error("Couldn't read the xp table file.")]
    ErrorReadingTable {
        fs_error: Error
    },
    #[error("Couldn't find the index of rank {0}!")]
    RankNotFound(i64)
}

// For loading the data table. Defined types etc.
pub type Table = Vec<RankData>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankData {
    #[serde(rename = "level")]
    pub level: i64,
    pub total_experience: i64,
    pub required_experience_for_next: i64,
}

// This is needed cause the json data is hard to store into memory without async.
#[derive(Clone)]
pub struct RankHandling {
    ranks: Vec<RankData>
}

// TODO: Save arrays and values into a json file for later use.
// TODO: Make a function that resets these saved values.
impl RankHandling {
    // Make a new struct.
    pub fn new() -> Self {
        RankHandling { ranks: Vec::new() }
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

    // General function to find the rank number with just xp values.
    pub fn find_rank(&self, xp: i64) -> i64 {
        // println!("{}", cur_exp);
        let ranks = &self.ranks;
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

    pub fn sum_needed_xp(&self, current_rank: i64, wanted_rank: i64) -> Result<i64, FunctionsError> {
        let start_index = match self.ranks.iter().position(|x| x.level == current_rank) {
            Some(index) => index,
            None => return Err(FunctionsError::RankNotFound(current_rank))
        };
        
        let (_, split_ranks) = self.ranks.split_at(start_index);

        let end_index = match split_ranks.iter().position(|x| x.level == wanted_rank) {
            Some(index) => index,
            None => return Err(FunctionsError::RankNotFound(wanted_rank))
        };

        // println!("{:?}", split_ranks);

        let mut sum: i64 = 0;
        for (i, rank) in split_ranks.iter().enumerate() {
            if i == end_index {
                break;
            } else {
                // println!("{:?}", rank.level);
                sum += rank.required_experience_for_next;
            }
        }

        // println!("{:?}", sum);

        Ok(sum)
    }
}

// * Make a function that calculates the avg time/xp to the next rank. - Should be done.
pub fn calculate_avg(mission_time: Duration, recieved_exp: i64, mut xp_arr: Vec<i64>, mut time_arr: Vec<Duration>) -> (Vec<Duration>, Vec<i64>, AvgResult) {
    xp_arr.push(recieved_exp);
    time_arr.push(mission_time);

    let mut avg_time = 0;
    for time in time_arr.clone().iter() {
        avg_time += time.whole_minutes();
    }
    avg_time /= time_arr.len() as i64;

    (time_arr.clone(), xp_arr.clone(), AvgResult { avg_time: Duration::minutes(avg_time), avg_xp: (xp_arr.iter().sum::<i64>() / xp_arr.len() as i64)})
}

// * Create a function that estimates the time needed for the wanted rank, based on avg mission time and xperience. - Done.
pub fn estimate_time_needed(time_arr: Vec<Duration>, needed_xp: i64, avg: AvgResult, rank_handler: RankHandling) -> Duration {
    let missions_needed = needed_xp / avg.avg_xp;
    let est = missions_needed * avg.avg_time.whole_minutes();
    Duration::minutes(est)
}