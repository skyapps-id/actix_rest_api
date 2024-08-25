use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub title: String,
    pub author: String,
    pub published_year: u32,
}
