use anyhow::Result;
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use regex::Regex;

/// Parser for time strings (relative or absolute)
pub struct TimeParser;

impl TimeParser {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse time string to RFC3339 format
    /// Supports relative times like "1h", "2d", "30m" or absolute RFC3339 timestamps
    pub fn parse(&self, time_str: &str) -> Result<String> {
        // If it's already an RFC3339 timestamp, validate and return it
        if DateTime::parse_from_rfc3339(time_str).is_ok() {
            return Ok(time_str.to_string());
        }

        // Parse relative time like "1h", "2d", "30m"
        if let Some((value, unit)) = self.parse_relative_time(time_str)? {
            let now = Utc::now();
            let past = match unit.as_str() {
                "m" => now - Duration::minutes(value),
                "h" => now - Duration::hours(value),
                "d" => now - Duration::days(value),
                _ => anyhow::bail!("Invalid time unit: {}", unit),
            };

            // Use SecondsFormat::Secs to generate clean timestamps without fractional seconds
            // Result: "2026-01-26T09:24:54Z" instead of "2026-01-26T09:24:54.636914+00:00"
            return Ok(past.to_rfc3339_opts(SecondsFormat::Secs, true));
        }

        anyhow::bail!(
            "Invalid time format: {}. Use RFC3339 (e.g., '2026-01-15T00:00:00Z') or relative (e.g., '1h', '2d')",
            time_str
        )
    }
    
    fn parse_relative_time(&self, time_str: &str) -> Result<Option<(i64, String)>> {
        let re = Regex::new(r"^(\d+)(m|h|d)$")?;
        
        if let Some(caps) = re.captures(time_str) {
            let value: i64 = caps[1].parse()?;
            let unit = caps[2].to_string();
            return Ok(Some((value, unit)));
        }
        
        Ok(None)
    }
}

impl Default for TimeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_relative_time() {
        let parser = TimeParser::new();
        
        // Test valid relative times
        assert!(parser.parse("1h").is_ok());
        assert!(parser.parse("2d").is_ok());
        assert!(parser.parse("30m").is_ok());
        
        // Ensure format is clean (ends with Z, not +00:00)
        let result = parser.parse("1h").unwrap();
        assert!(result.ends_with('Z'));
    }
    
    #[test]
    fn test_parse_absolute_time() {
        let parser = TimeParser::new();
        let timestamp = "2026-01-15T00:00:00Z";
        
        assert_eq!(parser.parse(timestamp).unwrap(), timestamp);
    }
    
    #[test]
    fn test_parse_invalid_time() {
        let parser = TimeParser::new();
        
        assert!(parser.parse("invalid").is_err());
        assert!(parser.parse("1x").is_err());
    }
}
