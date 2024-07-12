use regex::Regex;

const URL_REGEX: &str = r#"(?:linovelib|bilinovel)\\.com/novel/(\\d+)"#;
const DOMAIN: &str = "https://linovelib.com";