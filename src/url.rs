use lazy_regex::Lazy;
use lazy_regex::Regex;
use lazy_regex::lazy_regex;
use std::error::Error;

static PATH_PATTERN: Lazy<Regex> = lazy_regex!(r#"\A(/([\w.\-~]|%[0-9A-F]{2})*)*\z"#);

pub fn validate_path(path: &str) -> Result<&str, Box<dyn Error>> {
    if PATH_PATTERN.is_match(path) {
        Ok(path)
    } else {
        Err("Invalid URL path".into())
    }
}
