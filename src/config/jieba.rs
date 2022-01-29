use crate::config::config::JiebaConfig;
use jieba_rs::Jieba;

pub struct JieBa;

impl JieBa {
    pub fn init(config: JiebaConfig) -> Jieba {
        let mut jieba = Jieba::new();

        load_frequency_words(&mut jieba, config.fixed);

        jieba
    }
}

fn load_frequency_words(j: &mut Jieba, words: String) -> () {
    tracing::info!("准备加载自定义分词词语");

    for x in words.split(";") {
        tracing::info!("加载自定义分词词语: {}", x);
        j.add_word(x, None, None);
    }
}
