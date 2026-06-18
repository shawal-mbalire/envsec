use colored::Colorize;

pub fn header(text: &str) -> String {
    text.bold().cyan().to_string()
}

pub fn success(text: &str) -> String {
    text.green().to_string()
}

pub fn warning(text: &str) -> String {
    text.yellow().to_string()
}

pub fn error(text: &str) -> String {
    text.red().to_string()
}

pub fn dim(text: &str) -> String {
    text.dimmed().to_string()
}

pub fn bold(text: &str) -> String {
    text.bold().to_string()
}

pub fn key_name(text: &str) -> String {
    text.bold().white().to_string()
}

pub fn value_masked(text: &str) -> String {
    text.dimmed().to_string()
}
