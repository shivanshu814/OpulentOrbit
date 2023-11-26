use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}
