pub fn map_serde_error(source: &str, err: serde_json::Error) -> String {
    let line = source.lines().nth(err.line() - 1).unwrap_or("");
    err.to_string() + "\\n" + line
}
