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
    let mut required_env = vec![
        ("REDIS_URL", false),
        ("GITHUB_CLIENT_ID", false),
        ("GITHUB_CLIENT_SECRET", false),
        ("REDDIT_CLIENT_ID", false),
        ("REDDIT_CLIENT_SECRET", false),
        ("DISCORD_CLIENT_ID", false),
        ("DISCORD_CLIENT_SECRET", false),
        ("SESSION_KEY", false),
    ];

    vars.iter().for_each(|(key, _)| {
        required_env
            .iter_mut()
            .for_each(|(env, found)| {
                if key == env {
                    *found = true;
                }
            });
    });

    if required_env
        .iter()
        .any(|(_, found)| !found)
    {
        let missing = required_env
            .iter()
            .filter(|(_, found)| !found)
            .map(|(env, _)| env.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        panic!("Missing environment variables: {}", missing);
    }
}
