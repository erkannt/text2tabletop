pub fn parse_spells(input: &str) -> Option<Vec<String>> {
    if input.trim().is_empty() {
        return None;
    }
    let spells = input
        .lines()
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect();
    Some(spells)
}
