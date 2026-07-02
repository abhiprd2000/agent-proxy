pub enum Decision {
    Allow,
    Review(String),
    Block(String),
}

pub fn evaluate(command: &str) -> Decision {
    let cmd = command.trim();

    if cmd.starts_with("rm ") {
        return Decision::Block(
            "file deletion command detected".to_string()
        );
    }

    if cmd.starts_with("git push -f")
        || cmd.starts_with("git push --force")
    {
        return Decision::Block(
            "force push detected".to_string()
        );
    }

    if cmd.contains("curl")
        && cmd.contains('|')
        && cmd.contains("bash")
    {
        return Decision::Review(
            "remote script execution".to_string()
        );
    }

    Decision::Allow
}