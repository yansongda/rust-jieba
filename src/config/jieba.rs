use crate::config::{JiebaConfig, CONFIG};
use jieba_rs::Jieba;

pub struct JieBa;

impl JieBa {
    pub fn init() -> Jieba {
        let config: &JiebaConfig = &CONFIG.jieba;
        let mut jieba = Jieba::new();

        load_frequency_words(&mut jieba, config.fixed.to_owned());

        jieba
    }
}

fn load_frequency_words(j: &mut Jieba, words: String) {
    tracing::info!("准备加载自定义分词词语");

    for x in words.split(';') {
        if !x.is_empty() {
            tracing::info!("加载自定义分词词语: {}", x);
            j.add_word(x, None, None);
        }
    }
}
