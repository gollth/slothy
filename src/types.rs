use chrono::{NaiveDateTime, SubsecRound, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plant {
    pub iot: i64,
    pub sensor: i64,
    pub name: String,
}

impl Plant {
    pub fn new(iot: i64, sensor: i64, name: &str) -> Self {
        Self {
            iot,
            sensor,
            name: name.to_string(),
        }
    }
}

pub type Humidity = f64;

#[derive(Debug, PartialEq)]
pub struct Observation {
    pub id: i64,
    pub plant: String,
    pub humidity: Humidity,
    pub stamp: NaiveDateTime,
}

impl Observation {
    pub fn new(id: i64, plant: &str, humidity: Humidity) -> Self {
        Self {
            id,
            plant: plant.to_string(),
            humidity,
            stamp: Utc::now().naive_local().trunc_subsecs(0),
        }
    }
}
