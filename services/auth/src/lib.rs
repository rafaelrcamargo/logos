use std::io::Write;

use colored::*;

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

pub fn check_env(vars: Vec<(String, String)>) {
    // Check for all the required environment variables
    let required_env = vec![
        "REDIS_URL",
        "GITHUB_CLIENT_ID",
        "GITHUB_CLIENT_SECRET",
        "REDDIT_CLIENT_ID",
        "REDDIT_CLIENT_SECRET",
        "DISCORD_CLIENT_ID",
        "DISCORD_CLIENT_SECRET",
        "SESSION_KEY",
    ];

    for var in vars {
        if !required_env.contains(&var.0.as_str()) {
            panic!("Missing the {} environment variable.", var.0);
        }
    }
}
