use chrono::{DateTime, Local};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::path::PathBuf;

pub fn renewm_logger(debug: bool) {
    Builder::new()
        .format(move |buf, record| {
            let level = record.level();
            let timestamp: DateTime<Local> = Local::now();
            if debug {
                writeln!(
                    buf,
                    "[{}] {}: {}: {}",
                    level,
                    record.file().unwrap(),
                    record.line().unwrap(),
                    record.args()
                )
            } else {
                writeln!(
                    buf,
                    "[{}] {}: {}",
                    level,
                    timestamp.format("%Y-%m-%d %H:%M:%S"),
                    record.args()
                )
            }
        })
        .filter(
            None,
            if debug {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init();
}

pub fn run(debug: bool, profile: bool, config_file: Option<PathBuf>) {
    renewm_logger(debug);

    let renewm = super::layout::Layout::new(debug, config_file);
    renewm.run();
}
