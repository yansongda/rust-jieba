use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub text: String,
    pub hhm: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Participle {
    pub text: String,
    pub words: Vec<String>,
}
