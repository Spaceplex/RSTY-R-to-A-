use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct RowID {
    pub id: String,
}
