use colored::*;
use std::io::Write;

// ? Logging macros
pub use log::{debug, error, info, trace, warn};

// ? Logger utils
pub fn logger_setup() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now()
                    .format("%H:%M:%S")
                    .to_string()
                    .dimmed(),
                highlight_level(record.level()),
                record.args()
            )
        })
        .init();
}
pub fn highlight_logger() -> String {
    format!(
        "{} - [{}] {} -> {}",
        "%a".dimmed(),
        "%s".bold(),
        "%r".blue(),
        "%T".dimmed(),
    )
}
pub fn highlight_level<T: std::fmt::Display>(
    text: T
) -> colored::ColoredString {
    match text.to_string().as_str() {
        "ERROR" => text.to_string().red(),
        "WARN" => text.to_string().yellow(),
        "INFO" => text.to_string().green(),
        "DEBUG" => text.to_string().blue(),
        "TRACE" => text.to_string().dimmed(),
        _ => text.to_string().normal()
    }
}
