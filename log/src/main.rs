#[macro_use]
extern crate log;

use log::{LevelFilter, Record, Level, Metadata};

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        /// 输出到控制台，也可以到文件
        println!("{}:{} - {}", record.level(),record.target(),record.args());
    }
    fn flush(&self) {}
}

fn main() {
}
