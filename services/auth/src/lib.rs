/// This solves the problem of forgetting to set any of the `env` used only in specific routes
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
