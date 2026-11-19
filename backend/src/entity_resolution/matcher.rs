use crate::entity_resolution::normalizer::{candidate_stem, normalize_key};
use strsim::{jaro_winkler, normalized_levenshtein, sorensen_dice};

#[derive(Debug, Clone)]
pub struct MatchCandidate {
    pub entity_a_id: i64,
    pub entity_b_id: i64,
    pub name_a: String,
    pub name_b: String,
    pub similarity_score: f64,
    pub explanation: String,
}

/// Computes multi-signal string similarity between two entity names.
/// Returns (score, explanation) if similarity meets candidate threshold, or None.
pub fn evaluate_similarity(name_a: &str, name_b: &str) -> Option<(f64, String)> {
    let key_a = normalize_key(name_a);
    let key_b = normalize_key(name_b);

    if key_a == key_b {
        return Some((
            1.0,
            "Exact match under case and whitespace normalization".to_string(),
        ));
    }

    let stem_a = candidate_stem(name_a);
    let stem_b = candidate_stem(name_b);

    // If both have non-empty stems and are identical
    if !stem_a.is_empty() && stem_a == stem_b && stem_a.len() >= 3 {
        return Some((
            0.95,
            format!(
                "Identical root stem '{}' after removing legal/corporate suffixes",
                stem_a
            ),
        ));
    }

    let jaro = jaro_winkler(&key_a, &key_b);
    let lev = normalized_levenshtein(&key_a, &key_b);
    let dice = sorensen_dice(&key_a, &key_b);

    // Also compare stems if available
    let stem_jaro = if !stem_a.is_empty() && !stem_b.is_empty() {
        jaro_winkler(&stem_a, &stem_b)
    } else {
        0.0
    };

    let composite_score = (jaro * 0.45) + (lev * 0.35) + (dice * 0.20);
    let max_score = composite_score.max(stem_jaro * 0.92);

    if max_score >= 0.85 {
        let mut reasons = Vec::new();
        if jaro >= 0.90 {
            reasons.push(format!("Jaro-Winkler similarity {:.2}", jaro));
        }
        if lev >= 0.85 {
            reasons.push(format!("Levenshtein similarity {:.2}", lev));
        }
        if dice >= 0.80 {
            reasons.push(format!("Token dice coefficient {:.2}", dice));
        }
        if stem_jaro >= 0.90 {
            reasons.push(format!("Stem similarity {:.2}", stem_jaro));
        }

        let explanation = if reasons.is_empty() {
            format!("High composite string similarity ({:.2})", max_score)
        } else {
            reasons.join(", ")
        };

        return Some((max_score, explanation));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_matches() {
        let (score, _) = evaluate_similarity("Valve Corp", "valve corp").unwrap();
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_legal_suffix_matches() {
        let (score, exp) =
            evaluate_similarity("Double Fine Productions", "Double Fine Productions, LLC").unwrap();
        assert!(score >= 0.90);
        assert!(exp.contains("Identical root stem") || exp.contains("similarity"));
    }

    #[test]
    fn test_minor_typo_matches() {
        let (score, _) = evaluate_similarity("Bethesda Softworks", "Bethesda Softwork").unwrap();
        assert!(score >= 0.88);
    }

    #[test]
    fn test_different_entities_rejected() {
        assert!(evaluate_similarity("Valve Corporation", "Ubisoft Montreal").is_none());
        assert!(evaluate_similarity("Square Enix", "Capcom").is_none());
    }
}
