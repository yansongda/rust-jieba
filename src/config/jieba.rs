use jieba_rs::Jieba;
use log::info;
use std::env;

pub struct JieBa;

impl JieBa {
    pub fn init() -> Jieba {
        let mut jieba = Jieba::new();

        load_frequency_words(&mut jieba);

        jieba
    }
}

fn load_frequency_words(j: &mut Jieba) -> () {
    let suggests = env::var("JIEBA_FIXED");

    if suggests.is_err() {
        return;
    }

    for x in suggests.unwrap().split(";") {
        info!("加载自定义分词词语: {}", x);
        j.add_word(x, None, None);
    }
}
