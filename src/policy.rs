use crate::config::Config;

pub enum Decision {
    Allow,
    Review(String),
    Block(String),
}

pub fn evaluate(command: &str) -> Decision {
    let cmd = command.trim();
    let config = Config::load();

    if let Some(deny_rules) = config.deny {
        for rule in deny_rules {
            if cmd.contains(&rule) {
                return Decision::Block(format!(
                    "matched deny rule '{}'",
                    rule
                ));
            }
        }
    }

    if let Some(review_rules) = config.review {
        for rule in review_rules {
            if cmd.contains(&rule) {
                return Decision::Review(format!(
                    "matched review rule '{}'",
                    rule
                ));
            }
        }
    }

    Decision::Allow
}