use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plant {
    pub(crate) id: i64,
    pub(crate) name: String,
}

impl Plant {
    pub fn new(id: i64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

pub type Humidity = f64;

#[derive(Debug, PartialEq)]
pub struct Water {
    pub id: i64,
    pub plant: i64,
    pub humidity: Humidity,
    pub stamp: NaiveDateTime,
}
