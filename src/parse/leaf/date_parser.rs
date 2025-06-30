use crate::{Error, Pattern, Result};

pub(crate) fn parse_date_content(content: String) -> Result<Pattern> {
    // Parse the dcbor-pattern date syntax: iso-8601, iso-8601...iso-8601, etc.

    // Check if it's a regex pattern /regex/
    if content.starts_with('/') && content.ends_with('/') {
        let regex_str = &content[1..content.len() - 1];
        let regex = regex::Regex::new(regex_str)
            .map_err(|_| Error::InvalidRegex(0..content.len()))?;
        return Ok(Pattern::date_regex(regex));
    }

    // Check for range patterns
    if content.contains("...") {
        let parts: Vec<&str> = content.split("...").collect();
        if parts.len() == 2 {
            let start_str = parts[0];
            let end_str = parts[1];

            if start_str.is_empty() {
                // ...iso-8601 (latest)
                let date = dcbor::Date::from_string(end_str)
                    .map_err(|_| Error::InvalidDateFormat(0..content.len()))?;
                return Ok(Pattern::date_latest(date));
            } else if end_str.is_empty() {
                // iso-8601... (earliest)
                let date = dcbor::Date::from_string(start_str)
                    .map_err(|_| Error::InvalidDateFormat(0..content.len()))?;
                return Ok(Pattern::date_earliest(date));
            } else {
                // iso-8601...iso-8601 (range)
                let start_date = dcbor::Date::from_string(start_str)
                    .map_err(|_| Error::InvalidDateFormat(0..content.len()))?;
                let end_date = dcbor::Date::from_string(end_str)
                    .map_err(|_| Error::InvalidDateFormat(0..content.len()))?;
                return Ok(Pattern::date_range(start_date..=end_date));
            }
        }
    }

    // Single ISO-8601 date
    let date = dcbor::Date::from_string(&content)
        .map_err(|_| Error::InvalidDateFormat(0..content.len()))?;
    Ok(Pattern::date(date))
}
