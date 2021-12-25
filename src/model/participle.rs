use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Participle {
    pub text: String,
    pub words: Vec<String>
}
