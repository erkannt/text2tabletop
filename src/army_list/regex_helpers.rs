use regex::Regex;

pub fn extract_single_or(default: &str, re: Regex, input: &str) -> String {
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .map(|s| s.to_string())
        .unwrap_or(default.to_string())
}

pub fn extract_single(name: &str, re: Regex, input: &str) -> String {
    re.captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|mat| input.get(mat.range()))
        .map(|s| s.to_string())
        .unwrap_or(format!("[error: can't extract {}]", name))
}
