use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Participle {
    pub text: String,
    pub words: Vec<String>,
}
