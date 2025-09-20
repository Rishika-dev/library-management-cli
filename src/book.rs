use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BookStatus {
    Available,
    Borrowed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: i16,
    pub title: String,
    pub author: String,
    pub year: i16,
    pub status: BookStatus,
}
