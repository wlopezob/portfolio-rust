/// Converts `DD/MM/YYYY` → `YYYY-MM-DD` for storage.
pub fn to_iso_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('/').collect();
    format!("{}-{}-{}", parts[2], parts[1], parts[0])
}

/// Converts `YYYY-MM-DD` → `DD/MM/YYYY` for responses.
pub fn to_display_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    format!("{}/{}/{}", parts[2], parts[1], parts[0])
}
