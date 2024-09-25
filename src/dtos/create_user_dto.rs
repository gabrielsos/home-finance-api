use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
}