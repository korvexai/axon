use crate::config::schema::AxonConfig;

/// Checks whether a binary is allowed according to config.toml
pub fn is_allowed(bin: &str, config: &AxonConfig) -> bool {
    config
        .shell
        .allowed_commands
        .iter()
        .any(|allowed| allowed == bin)
}

/// Checks whether a command requires explicit approval
pub fn requires_approval(command: &str, args: &[String], config: &AxonConfig) -> bool {
    let full = format!("{} {}", command, args.join(" ")).to_lowercase();

    config
        .shell
        .require_approval_for
        .iter()
        .any(|pattern: &String| full.contains(&pattern.to_lowercase()))
}