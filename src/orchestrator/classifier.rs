/// Classifies incoming text commands to route them correctly.
/// Used by the Orchestrator Router to decide execution path.

#[derive(Debug, Clone, PartialEq)]
pub enum CommandClass {
    Build,
    Status,
    AiQuery,
    RagSearch,
    Unknown,
}

/// Classify raw user input into a command category.
pub fn classify_command(input: &str) -> CommandClass {
    if input.trim().is_empty() {
        return CommandClass::Unknown;
    }

    let lower = input.to_lowercase();
    let trimmed = lower.trim_start_matches('/').trim();

    // ---------- BUILD ----------
    if trimmed.starts_with("build")
        || trimmed.starts_with("cargo")
        || trimmed.contains("compile")
        || trimmed.contains("rebuild")
    {
        return CommandClass::Build;
    }

    // ---------- STATUS ----------
    if trimmed.starts_with("status")
        || trimmed.starts_with("stat")
        || trimmed.contains("worker")
        || trimmed.contains("health")
    {
        return CommandClass::Status;
    }

    // ---------- RAG SEARCH ----------
    if trimmed.starts_with("search")
        || trimmed.starts_with("find")
        || trimmed.starts_with("rag")
        || trimmed.contains("lookup")
    {
        return CommandClass::RagSearch;
    }

    // ---------- AI QUERY ----------
    // Heuristic: questions or natural sentences
    if trimmed.ends_with('?') || trimmed.contains(' ') || trimmed.len() > 20 {
        return CommandClass::AiQuery;
    }

    CommandClass::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_build() {
        assert_eq!(classify_command("/build axon_core"), CommandClass::Build);

        assert_eq!(
            classify_command("cargo build --release"),
            CommandClass::Build
        );
    }

    #[test]
    fn test_classify_status() {
        assert_eq!(classify_command("/status"), CommandClass::Status);

        assert_eq!(classify_command("worker health"), CommandClass::Status);
    }

    #[test]
    fn test_classify_rag() {
        assert_eq!(
            classify_command("search for error in logs"),
            CommandClass::RagSearch
        );
    }

    #[test]
    fn test_classify_ai() {
        assert_eq!(
            classify_command("Why is the linker failing?"),
            CommandClass::AiQuery
        );
    }

    #[test]
    fn test_unknown() {
        assert_eq!(classify_command("build"), CommandClass::Build);
    }
}
