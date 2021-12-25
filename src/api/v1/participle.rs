use actix_web::{get, Responder, web};
use jieba_rs::Jieba;

use crate::model::participle::{Participle, Query};
use crate::result::{Error, Response};

#[get("/do")]
pub async fn index(query: web::Query<Query>, jieba: web::Data<Jieba>) -> impl Responder {
    let text = query.text.as_ref().map_or_else(|| String::from(""), |t| String::from(t));

    if text.is_empty() {
        return Err(Error::ParamsError);
    }

    let words = jieba.cut(&text[..], false);

    Ok(Response::success(Participle {
        words: words.iter().map(|v| String::from(v.clone())).collect(),
        text,
    }))
}
