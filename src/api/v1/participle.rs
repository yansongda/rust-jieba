use actix_web::{get, web, Responder};
use jieba_rs::Jieba;

use crate::model::participle::{Participle, Query};
use crate::result::{Error, Response};

#[get("/cut")]
pub async fn cut(query: web::Query<Query>, jieba: web::Data<Jieba>) -> impl Responder {
    let text = query.text.clone();

    if text.is_empty() {
        return Err(Error::EmptyParams);
    }

    let words = jieba.cut(&text[..], query.hhm.unwrap_or(false));

    Ok(Response::success(Participle {
        words: words.iter().map(|v| String::from(v.to_owned())).collect(),
        text: text.to_owned(),
    }))
}

#[get("/cut_all")]
pub async fn cut_all(query: web::Query<Query>, jieba: web::Data<Jieba>) -> impl Responder {
    let text = query.text.clone();

    if text.is_empty() {
        return Err(Error::EmptyParams);
    }

    let words = jieba.cut_all(&text[..]);

    Ok(Response::success(Participle {
        words: words.iter().map(|v| String::from(v.to_owned())).collect(),
        text: text.to_owned(),
    }))
}
