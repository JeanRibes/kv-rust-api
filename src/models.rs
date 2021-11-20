use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Kudos {
    pub count: i64,
    pub slug: String,
}