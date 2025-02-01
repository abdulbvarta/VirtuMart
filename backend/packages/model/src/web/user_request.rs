

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct  User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}