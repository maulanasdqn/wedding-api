use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: i32,
    pub name: String,
}
