use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

pub struct Logger;

impl Logger {
    pub fn config() -> Config {
        let root = Root::builder().appender("stdout").build(LevelFilter::Info);

        let encoder = PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%f)}|{l}|{X(request_id)(default)}|{M} - {m}{n}",
        );
        let console_append = ConsoleAppender::builder()
            .encoder(Box::new(encoder))
            .build();
        let appender = Appender::builder().build("stdout", Box::new(console_append));

        Config::builder().appender(appender).build(root).unwrap()
    }
}
