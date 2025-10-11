use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Outside {
    pub uuid: String,
    pub name: String,
    pub context: String,
    pub url: String,
    pub percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inside {
    pub uuid: String,
    pub date: String,
    pub context: String,
}
