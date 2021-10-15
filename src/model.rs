use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct FizzBuzzParams {
    pub limit: u8,
    #[serde(rename = "str1")]
    pub fizz_name: String,
    #[serde(rename = "str2")]
    pub buzz_name: String,
    #[serde(rename = "int1")]
    pub fizz_value: u8,
    #[serde(rename = "int2")]
    pub buzz_value: u8,
}

#[derive(Deserialize, Default, Debug)]
pub struct State {
    // Note: in real life, a database would be better
    pub fizz_buzz_stats: Mutex<HashMap<FizzBuzzParams, usize>>,
}

impl State {
    pub fn register(&self, params: &FizzBuzzParams) {
        println!("pouet");
        let mut counter = self.fizz_buzz_stats.lock().unwrap();
        *counter.entry(params.clone()).or_default() += 1;
    }

    pub fn most_used_query(&self) -> Option<(FizzBuzzParams, usize)> {
        println!("stats");
        self.fizz_buzz_stats
            .lock()
            .unwrap()
            .iter()
            .max_by_key(|(_params, uses)| *uses)
            .map(|(params, uses)| (params.clone(), *uses))
    }
}
