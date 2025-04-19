use regex::Regex;

/// Cleans a URL by trimming and removing common trailing punctuation.
pub fn clean_link(link: &str) -> String {
    link.trim()
        .trim_end_matches(|c: char| matches!(c, ')' | ']' | '}' | '>' | '"' | '\'' | ';' | ','))
        .to_string()
}
/// Validates whether a link is a valid HTTP/HTTPS URL.
pub fn validate_link(link: &str) -> bool {
    // Compiled once at runtime; if performance is critical, make it a lazy_static or once_cell
    Regex::new(r#"^https?://[^\s"'>)]+"#).unwrap().is_match(link)
}
