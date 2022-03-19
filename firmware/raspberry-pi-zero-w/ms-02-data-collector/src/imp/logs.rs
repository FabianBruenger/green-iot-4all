// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use chrono::{Datelike, Timelike, Utc};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::{fs, error::Error};

// -----------------------------------------------------------------------------------------
//                                      Logging
// -----------------------------------------------------------------------------------------
pub fn set_logging(config: &super::config::DeviceConfig) -> Result<(), Box<dyn std::error::Error>> {
    // get current dat and time for logging file
    let now = Utc::now();
    let now_str = format!(
        "{}_{}",
        &config.logging_path.to_owned(),
        &now.format("%Y-%m-%d_%H:%M:%S").to_string()
    );

    // archiv the current log file. If it is not existing, then create new
    if let Err(_e) = fs::rename(&config.logging_path.to_owned(), now_str) {
        fs::File::create(&config.logging_path.to_owned()).unwrap();
    }

    // Set file appender
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}]{h({l})}: {m}\n",
        )))
        .build(&config.logging_path.to_owned())?;

    // Build the config
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    // Final instance
    log4rs::init_config(config)?;

    // Debug
    log::info!("Logging set up initially");

    Ok(())
}