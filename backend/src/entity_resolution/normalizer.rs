use regex::Regex;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

static MULTI_SPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static PUNCT_CLEAN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[.,'"’“”`´_#*~]+|\s*-\s*"#).unwrap());
static LEGAL_SUFFIXES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(inc|incorporated|llc|ltd|limited|corp|corporation|gmbh|co|s\.?r\.?o\.?|pty|sa|sl|bv|ab|spa|oü|oy|sp\.?\s*z\s*o\.?o\.?|games|studios|studio|interactive|entertainment)\b",
    )
    .unwrap()
});

/// Conservative name normalization for exact matching and display canonicalization.
/// Preserves significant punctuation and case while standardizing whitespace and Unicode.
pub fn normalize_conservative(name: &str) -> String {
    let nfkc: String = name.nfkc().collect();
    let trimmed = nfkc.trim();
    let collapsed = MULTI_SPACE_RE.replace_all(trimmed, " ");
    collapsed.to_string()
}

/// Normalized key used for entity matching and indexing.
/// Lowercases, trims whitespace, standardizes separators.
pub fn normalize_key(name: &str) -> String {
    let conservative = normalize_conservative(name).to_lowercase();
    // Replace smart quotes, apostrophes, dashes with standard ASCII
    let ascii_std = conservative
        .replace(['’', '‘', '`', '´'], "'")
        .replace(['“', '”'], "\"")
        .replace(['–', '—'], "-");
    let collapsed = MULTI_SPACE_RE.replace_all(&ascii_std, " ");
    collapsed.trim().to_string()
}

/// Strip legal suffixes and punctuation for fuzzy candidate discovery.
/// (Notice: this is strictly used to identify candidates for review, NOT for blind automatic merge!)
pub fn candidate_stem(name: &str) -> String {
    let key = normalize_key(name);
    let stripped = LEGAL_SUFFIXES_RE.replace_all(&key, "");
    let punct_cleaned = PUNCT_CLEAN_RE.replace_all(&stripped, " ");
    let collapsed = MULTI_SPACE_RE.replace_all(&punct_cleaned, " ");
    collapsed.trim().to_string()
}

/// Parses raw developer / publisher string into individual entity names.
/// Handles:
/// - JSON arrays like `["Dev A", "Dev B"]`
/// - Delimited strings: `Dev A, Dev B` or `Dev A / Dev B` or `Dev A; Dev B`
/// - Respects names that have internal commas like "Smith, John" if bracketed, or standard delimiters.
pub fn parse_delimited_entities(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "[]" || trimmed == "none" || trimmed == "N/A" {
        return Vec::new();
    }

    // Try parsing as JSON array first
    if (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        if let Ok(items) = serde_json::from_str::<Vec<String>>(trimmed) {
            return items
                .into_iter()
                .map(|s| normalize_conservative(&s))
                .filter(|s| !s.is_empty())
                .collect();
        }
    }

    // Otherwise split by delimiters: semicolons, commas, slashes
    // We split primarily on `;` or ` / ` or `,\s*`
    let parts: Vec<&str> = if trimmed.contains(';') {
        trimmed.split(';').collect()
    } else if trimmed.contains(" / ") {
        trimmed.split(" / ").collect()
    } else if trimmed.contains(',') {
        // Split on comma only if not looking like a single legal name e.g. "Games, LLC"
        // If there are multiple commas or obvious list
        trimmed.split(',').collect()
    } else {
        vec![trimmed]
    };

    let mut result = Vec::new();
    for part in parts {
        let cleaned = normalize_conservative(part);
        // Ignore noise like "etc", "none", "n/a"
        if !cleaned.is_empty()
            && !cleaned.eq_ignore_ascii_case("none")
            && !cleaned.eq_ignore_ascii_case("n/a")
            && !cleaned.eq_ignore_ascii_case("null")
        {
            result.push(cleaned);
        }
    }

    if result.is_empty() && !trimmed.is_empty() {
        result.push(normalize_conservative(trimmed));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservative_normalization() {
        assert_eq!(
            normalize_conservative("  Valve  Corporation  "),
            "Valve Corporation"
        );
        assert_eq!(
            normalize_conservative("Rockstar\u{00A0}Games"),
            "Rockstar Games"
        );
    }

    #[test]
    fn test_normalize_key() {
        assert_eq!(normalize_key(" Valve  Games, LLC "), "valve games, llc");
        assert_eq!(normalize_key("BioWare™"), "biowaretm");
    }

    #[test]
    fn test_candidate_stem() {
        assert_eq!(candidate_stem("Valve Corporation"), "valve");
        assert_eq!(candidate_stem("Valve Corp."), "valve");
        assert_eq!(candidate_stem("Frictional Games Ltd."), "frictional");
    }

    #[test]
    fn test_parse_delimited_entities() {
        assert_eq!(
            parse_delimited_entities("Bethesda Game Studios; id Software"),
            vec!["Bethesda Game Studios", "id Software"]
        );
        assert_eq!(
            parse_delimited_entities("[\"BioWare\", \"Demiurge Studios\"]"),
            vec!["BioWare", "Demiurge Studios"]
        );
        assert_eq!(
            parse_delimited_entities("CD PROJEKT RED / CD PROJEKT"),
            vec!["CD PROJEKT RED", "CD PROJEKT"]
        );
    }
}
