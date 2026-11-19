use chrono::{Datelike, NaiveDate};
use regex::Regex;
use std::sync::LazyLock;

static OWNER_RANGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([0-9,]+)\s*-\s*([0-9,]+)").unwrap());
static YEAR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(19\d{2}|20\d{2})\b").unwrap());
static PLAYTEST_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bplaytest\b").unwrap());

#[derive(Debug, Clone, Default)]
pub struct ParsedOwners {
    pub raw: String,
    pub lower: u64,
    pub upper: u64,
    pub midpoint: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedReleaseDate {
    pub raw: String,
    pub date_iso: Option<String>,
    pub year: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct GameQualityFlags {
    pub is_sparse: bool,
    pub is_likely_playtest: bool,
    pub missing_developers: bool,
    pub missing_publishers: bool,
    pub missing_genres: bool,
    pub missing_release_date: bool,
    pub invalid_app_id: bool,
    pub issues: Vec<(String, String, String)>, // (issue_type, severity, details)
}

/// Parse owner range string like "100,000 - 200,000" or "0 - 20000"
pub fn parse_estimated_owners(raw: &str) -> ParsedOwners {
    let trimmed = raw.trim();
    if let Some(caps) = OWNER_RANGE_RE.captures(trimmed) {
        let lower_str = caps[1].replace(',', "");
        let upper_str = caps[2].replace(',', "");
        let lower = lower_str.parse::<u64>().unwrap_or(0);
        let upper = upper_str.parse::<u64>().unwrap_or(0);
        let midpoint = (lower + upper) / 2;
        ParsedOwners {
            raw: trimmed.to_string(),
            lower,
            upper,
            midpoint,
        }
    } else {
        ParsedOwners {
            raw: trimmed.to_string(),
            lower: 0,
            upper: 0,
            midpoint: 0,
        }
    }
}

/// Parse diverse Steam release date formats:
/// "Feb 1, 2018", "1 Feb, 2018", "2018-02-01", "Jul 2015", "1998", "Coming Soon"
pub fn parse_release_date(raw: &str) -> ParsedReleaseDate {
    let trimmed = raw.trim();
    if trimmed.is_empty()
        || trimmed.eq_ignore_ascii_case("coming soon")
        || trimmed.eq_ignore_ascii_case("tba")
    {
        return ParsedReleaseDate {
            raw: trimmed.to_string(),
            date_iso: None,
            year: None,
        };
    }

    // Attempt standard formats
    let formats = [
        "%b %d, %Y",
        "%B %d, %Y",
        "%d %b, %Y",
        "%d %B, %Y",
        "%Y-%m-%d",
        "%d %b %Y",
        "%b %Y",
        "%B %Y",
        "%Y",
    ];

    for fmt in formats {
        if let Ok(d) = NaiveDate::parse_from_str(trimmed, fmt) {
            return ParsedReleaseDate {
                raw: trimmed.to_string(),
                date_iso: Some(d.format("%Y-%m-%d").to_string()),
                year: Some(d.year()),
            };
        }
    }

    // Fallback: extract 4-digit year via regex
    let year = YEAR_RE
        .captures(trimmed)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok());

    ParsedReleaseDate {
        raw: trimmed.to_string(),
        date_iso: None,
        year,
    }
}

/// Assess data quality and detect anomalies, sparsity, and playtest indicators
pub fn assess_game_quality(
    app_id: i64,
    name: &str,
    release_date: &ParsedReleaseDate,
    owners: &ParsedOwners,
    developers: &[String],
    publishers: &[String],
    genres: &[String],
    price: f64,
    playtime: i64,
) -> GameQualityFlags {
    let mut flags = GameQualityFlags::default();

    if app_id <= 0 {
        flags.invalid_app_id = true;
        flags.issues.push((
            "invalid_app_id".to_string(),
            "error".to_string(),
            format!("Invalid or non-positive AppID: {}", app_id),
        ));
    }

    if developers.is_empty() {
        flags.missing_developers = true;
        flags.issues.push((
            "missing_developer".to_string(),
            "warning".to_string(),
            "Record has no credited developers".to_string(),
        ));
    }

    if publishers.is_empty() {
        flags.missing_publishers = true;
        flags.issues.push((
            "missing_publisher".to_string(),
            "warning".to_string(),
            "Record has no credited publishers".to_string(),
        ));
    }

    if genres.is_empty() {
        flags.missing_genres = true;
        flags.issues.push((
            "missing_genres".to_string(),
            "warning".to_string(),
            "Record has no assigned genres".to_string(),
        ));
    }

    if release_date.year.is_none() {
        flags.missing_release_date = true;
        flags.issues.push((
            "missing_release_date".to_string(),
            "info".to_string(),
            format!("Unparseable release date string: '{}'", release_date.raw),
        ));
    }

    // Playtest check: name contains playtest keyword
    if PLAYTEST_RE.is_match(name) {
        flags.is_likely_playtest = true;
        flags.issues.push((
            "likely_playtest".to_string(),
            "info".to_string(),
            format!("Title '{}' contains 'playtest' designation", name),
        ));
    }

    // Sparse record detection:
    // When a game lacks meaningful metadata (e.g. 0 owners, no genres or no devs/pubs, 0 price, 0 playtime)
    let sparse_score = (if developers.is_empty() { 1 } else { 0 })
        + (if publishers.is_empty() { 1 } else { 0 })
        + (if genres.is_empty() { 1 } else { 0 })
        + (if release_date.year.is_none() { 1 } else { 0 })
        + (if owners.upper == 0 { 1 } else { 0 })
        + (if price <= 0.0 && playtime == 0 { 1 } else { 0 });

    if sparse_score >= 4 || (owners.upper == 0 && (developers.is_empty() || publishers.is_empty()))
    {
        flags.is_sparse = true;
        flags.issues.push((
            "sparse_record".to_string(),
            "info".to_string(),
            "Record contains very sparse metadata across key attributes".to_string(),
        ));
    }

    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_owners() {
        let p = parse_estimated_owners("100,000 - 200,000");
        assert_eq!(p.lower, 100_000);
        assert_eq!(p.upper, 200_000);
        assert_eq!(p.midpoint, 150_000);

        let p_zero = parse_estimated_owners("0 - 0");
        assert_eq!(p_zero.lower, 0);
        assert_eq!(p_zero.upper, 0);
        assert_eq!(p_zero.midpoint, 0);
    }

    #[test]
    fn test_parse_release_date() {
        let d1 = parse_release_date("Feb 1, 2018");
        assert_eq!(d1.year, Some(2018));
        assert_eq!(d1.date_iso.as_deref(), Some("2018-02-01"));

        let d2 = parse_release_date("2021-11-15");
        assert_eq!(d2.year, Some(2021));
        assert_eq!(d2.date_iso.as_deref(), Some("2021-11-15"));

        let d3 = parse_release_date("Q4 2024");
        assert_eq!(d3.year, Some(2024));

        let d4 = parse_release_date("Coming Soon");
        assert_eq!(d4.year, None);
    }

    #[test]
    fn test_assess_quality_sparse_playtest() {
        let release = parse_release_date("");
        let owners = parse_estimated_owners("0 - 0");
        let flags = assess_game_quality(
            2539430,
            "Black Dragon Mage Playtest",
            &release,
            &owners,
            &[],
            &[],
            &[],
            0.0,
            0,
        );
        assert!(flags.is_likely_playtest);
        assert!(flags.is_sparse);
        assert!(flags.missing_developers);
        assert!(flags.missing_publishers);
    }
}
