use crate::entity_resolution::matcher::evaluate_similarity;
use crate::entity_resolution::normalizer::{candidate_stem, normalize_conservative, normalize_key};
use rusqlite::{Connection, Result, params};
use std::collections::HashMap;
use tracing::info;

pub struct EntityResolutionManager;

impl EntityResolutionManager {
    /// Merges `source_id` into `target_id` for given entity_type ('developer' or 'publisher').
    /// Audit-logged and completely reversible.
    pub fn merge_entities(
        conn: &mut Connection,
        entity_type: &str,
        source_id: i64,
        target_id: i64,
        reason: Option<&str>,
    ) -> Result<()> {
        if source_id == target_id {
            return Ok(());
        }

        let tx = conn.transaction()?;

        let table = if entity_type == "developer" {
            "developers"
        } else {
            "publishers"
        };
        let rel_table = if entity_type == "developer" {
            "game_developers"
        } else {
            "game_publishers"
        };
        let fk_col = if entity_type == "developer" {
            "developer_id"
        } else {
            "publisher_id"
        };

        // Fetch source and target canonical names
        let source_name: String = tx.query_row(
            &format!("SELECT canonical_name FROM {} WHERE id = ?1", table),
            params![source_id],
            |r| r.get(0),
        )?;
        let target_name: String = tx.query_row(
            &format!("SELECT canonical_name FROM {} WHERE id = ?1", table),
            params![target_id],
            |r| r.get(0),
        )?;

        // Capture exact games belonging to source and which ones belong exclusively to source (not already on target)
        let source_game_ids: Vec<i64> = {
            let mut stmt = tx.prepare(&format!(
                "SELECT game_id FROM {} WHERE {} = ?1",
                rel_table, fk_col
            ))?;
            let rows = stmt
                .query_map(params![source_id], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect();
            rows
        };
        let exclusive_source_game_ids: Vec<i64> = {
            let mut stmt = tx.prepare(&format!(
                "SELECT game_id FROM {} WHERE {} = ?1 AND game_id NOT IN (SELECT game_id FROM {} WHERE {} = ?2)",
                rel_table, fk_col, rel_table, fk_col
            ))?;
            let rows = stmt
                .query_map(params![source_id, target_id], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect();
            rows
        };

        let snapshot_json = serde_json::json!({
            "source_games": source_game_ids,
            "exclusive_games": exclusive_source_game_ids,
        })
        .to_string();

        // Re-link relationships: insert target if not already present
        tx.execute(
            &format!(
                "INSERT OR IGNORE INTO {} (game_id, {})
                 SELECT game_id, ?1 FROM {} WHERE {} = ?2",
                rel_table, fk_col, rel_table, fk_col
            ),
            params![target_id, source_id],
        )?;

        // Remove source relationships
        tx.execute(
            &format!("DELETE FROM {} WHERE {} = ?1", rel_table, fk_col),
            params![source_id],
        )?;

        // Update entity_aliases to point to target
        tx.execute(
            "UPDATE entity_aliases 
             SET canonical_entity_id = ?1 
             WHERE entity_type = ?2 AND canonical_entity_id = ?3",
            params![target_id, entity_type, source_id],
        )?;

        // Mark source as merged
        tx.execute(
            &format!(
                "UPDATE {} SET is_merged = 1, merged_into_id = ?1, updated_at = datetime('now') WHERE id = ?2",
                table
            ),
            params![target_id, source_id],
        )?;

        // Record merge event
        tx.execute(
            "INSERT INTO entity_merge_events 
             (entity_type, source_entity_id, target_entity_id, source_canonical_name, target_canonical_name, reason, transferred_game_ids, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))",
            params![
                entity_type,
                source_id,
                target_id,
                source_name,
                target_name,
                reason.unwrap_or("Manual review merge"),
                snapshot_json
            ],
        )?;

        // Update candidate status if one exists
        tx.execute(
            "UPDATE entity_resolution_candidates 
             SET status = 'merged', reviewed_at = datetime('now')
             WHERE entity_type = ?1 
               AND ((entity_a_id = ?2 AND entity_b_id = ?3) OR (entity_a_id = ?3 AND entity_b_id = ?2))",
            params![entity_type, source_id, target_id],
        )?;

        tx.commit()?;

        // Recalculate metrics for target
        Self::recalculate_entity_metrics(conn, entity_type, target_id)?;

        info!(
            "Successfully merged {} '{}' (ID: {}) into '{}' (ID: {})",
            entity_type, source_name, source_id, target_name, target_id
        );

        Ok(())
    }

    /// Undoes a previous merge event.
    pub fn undo_merge(conn: &mut Connection, merge_event_id: i64) -> Result<()> {
        let tx = conn.transaction()?;

        let (entity_type, source_id, target_id, is_undone, transferred_json): (
            String,
            i64,
            i64,
            i64,
            Option<String>,
        ) = tx.query_row(
            "SELECT entity_type, source_entity_id, target_entity_id, is_undone, transferred_game_ids 
             FROM entity_merge_events WHERE id = ?1",
            params![merge_event_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;

        if is_undone == 1 {
            return Ok(());
        }

        let table = if entity_type == "developer" {
            "developers"
        } else {
            "publishers"
        };
        let rel_table = if entity_type == "developer" {
            "game_developers"
        } else {
            "game_publishers"
        };
        let fk_col = if entity_type == "developer" {
            "developer_id"
        } else {
            "publisher_id"
        };

        // Unmerge source
        tx.execute(
            &format!(
                "UPDATE {} SET is_merged = 0, merged_into_id = NULL, updated_at = datetime('now') WHERE id = ?1",
                table
            ),
            params![source_id],
        )?;

        // Restore aliases originally belonging to source
        let source_canonical_name: String = tx.query_row(
            &format!("SELECT canonical_name FROM {} WHERE id = ?1", table),
            params![source_id],
            |r| r.get(0),
        )?;
        let source_norm = normalize_key(&source_canonical_name);

        tx.execute(
            "UPDATE entity_aliases 
             SET canonical_entity_id = ?1 
             WHERE entity_type = ?2 AND normalized_name = ?3",
            params![source_id, entity_type, source_norm],
        )?;

        // Restore exact game relationships from snapshot if available
        if let Some(json_str) = transferred_json {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(src_games) = parsed.get("source_games").and_then(|v| v.as_array()) {
                    for g in src_games {
                        if let Some(gid) = g.as_i64() {
                            tx.execute(
                                &format!(
                                    "INSERT OR IGNORE INTO {} (game_id, {}) VALUES (?1, ?2)",
                                    rel_table, fk_col
                                ),
                                params![gid, source_id],
                            )?;
                        }
                    }
                }
                if let Some(excl_games) = parsed.get("exclusive_games").and_then(|v| v.as_array()) {
                    for g in excl_games {
                        if let Some(gid) = g.as_i64() {
                            tx.execute(
                                &format!(
                                    "DELETE FROM {} WHERE game_id = ?1 AND {} = ?2",
                                    rel_table, fk_col
                                ),
                                params![gid, target_id],
                            )?;
                        }
                    }
                }
            }
        }

        // Mark merge event as undone
        tx.execute(
            "UPDATE entity_merge_events 
             SET is_undone = 1, undone_at = datetime('now') 
             WHERE id = ?1",
            params![merge_event_id],
        )?;

        // Restore candidate status back to pending so it can be re-evaluated
        tx.execute(
            "UPDATE entity_resolution_candidates 
             SET status = 'pending', reviewed_at = NULL
             WHERE entity_type = ?1 
               AND ((entity_a_id = ?2 AND entity_b_id = ?3) OR (entity_a_id = ?3 AND entity_b_id = ?2))",
            params![entity_type, source_id, target_id],
        )?;

        tx.commit()?;

        Self::recalculate_entity_metrics(conn, &entity_type, source_id)?;
        Self::recalculate_entity_metrics(conn, &entity_type, target_id)?;

        info!(
            "Undid merge event {} for {} ID {}",
            merge_event_id, entity_type, source_id
        );

        Ok(())
    }

    /// Mark candidate as rejected
    pub fn reject_candidate(
        conn: &Connection,
        entity_type: &str,
        entity_a_id: i64,
        entity_b_id: i64,
    ) -> Result<()> {
        conn.execute(
            "UPDATE entity_resolution_candidates 
             SET status = 'rejected', reviewed_at = datetime('now')
             WHERE entity_type = ?1 
               AND ((entity_a_id = ?2 AND entity_b_id = ?3) OR (entity_a_id = ?3 AND entity_b_id = ?2))",
            params![entity_type, entity_a_id, entity_b_id],
        )?;
        Ok(())
    }

    /// Mark candidate as distinct entities
    pub fn mark_distinct(
        conn: &Connection,
        entity_type: &str,
        entity_a_id: i64,
        entity_b_id: i64,
    ) -> Result<()> {
        conn.execute(
            "UPDATE entity_resolution_candidates 
             SET status = 'distinct', reviewed_at = datetime('now')
             WHERE entity_type = ?1 
               AND ((entity_a_id = ?2 AND entity_b_id = ?3) OR (entity_a_id = ?3 AND entity_b_id = ?2))",
            params![entity_type, entity_a_id, entity_b_id],
        )?;
        Ok(())
    }

    /// Assign a raw alias directly to a canonical entity
    pub fn assign_alias(
        conn: &Connection,
        entity_type: &str,
        raw_name: &str,
        canonical_id: i64,
    ) -> Result<()> {
        let conservative = normalize_conservative(raw_name);
        let normalized = normalize_key(raw_name);

        conn.execute(
            "INSERT INTO entity_aliases 
             (entity_type, raw_name, normalized_name, canonical_entity_id, matching_method, confidence, is_confirmed, created_at)
             VALUES (?1, ?2, ?3, ?4, 'manual_assignment', 1.0, 1, datetime('now'))
             ON CONFLICT(entity_type, raw_name) DO UPDATE SET
               canonical_entity_id = excluded.canonical_entity_id,
               matching_method = 'manual_assignment',
               is_confirmed = 1",
            params![entity_type, conservative, normalized, canonical_id],
        )?;

        Ok(())
    }

    /// Runs candidate generation pass over entities.
    /// Groups by stem / token bucket to efficiently find high-confidence candidate matches.
    pub fn generate_candidates(
        conn: &mut Connection,
        entity_type: &str,
        limit_pairs: usize,
    ) -> Result<usize> {
        info!(
            "Generating entity resolution candidates for {}",
            entity_type
        );
        let table = if entity_type == "developer" {
            "developers"
        } else {
            "publishers"
        };

        // Query active entities with at least 1 game
        let entities: Vec<(i64, String, i64)> = {
            let mut stmt = conn.prepare(&format!(
                "SELECT id, canonical_name, game_count FROM {} WHERE is_merged = 0 ORDER BY game_count DESC LIMIT 2000",
                table
            ))?;

            stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
                .filter_map(|r| r.ok())
                .collect()
        };

        // Index by stem for rapid bucket lookup
        let mut stem_buckets: HashMap<String, Vec<(i64, String)>> = HashMap::new();
        for (id, name, _) in &entities {
            let stem = candidate_stem(name);
            if stem.len() >= 3 {
                stem_buckets
                    .entry(stem)
                    .or_default()
                    .push((*id, name.clone()));
            }
        }

        let mut candidate_count = 0;
        let tx = conn.transaction()?;

        // 1. Same-stem candidates
        for bucket in stem_buckets.values() {
            if bucket.len() > 1 && bucket.len() <= 20 {
                for i in 0..bucket.len() {
                    for j in (i + 1)..bucket.len() {
                        let (id_a, name_a) = &bucket[i];
                        let (id_b, name_b) = &bucket[j];
                        if let Some((score, exp)) = evaluate_similarity(name_a, name_b) {
                            let (min_id, max_id) = if id_a < id_b {
                                (*id_a, *id_b)
                            } else {
                                (*id_b, *id_a)
                            };

                            let inserted = tx.execute(
                                "INSERT OR IGNORE INTO entity_resolution_candidates 
                                 (entity_type, entity_a_id, entity_b_id, similarity_score, matching_explanation, status, created_at)
                                 VALUES (?1, ?2, ?3, ?4, ?5, 'pending', datetime('now'))",
                                params![entity_type, min_id, max_id, score, exp],
                            )?;
                            if inserted > 0 {
                                candidate_count += 1;
                                if candidate_count >= limit_pairs {
                                    tx.commit()?;
                                    return Ok(candidate_count);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. High-profile cross-checks between top 150 entities
        let top_count = entities.len().min(150);
        for i in 0..top_count {
            for j in (i + 1)..top_count {
                let (id_a, name_a, _) = &entities[i];
                let (id_b, name_b, _) = &entities[j];
                if let Some((score, exp)) = evaluate_similarity(name_a, name_b) {
                    let (min_id, max_id) = if id_a < id_b {
                        (*id_a, *id_b)
                    } else {
                        (*id_b, *id_a)
                    };

                    let inserted = tx.execute(
                        "INSERT OR IGNORE INTO entity_resolution_candidates 
                         (entity_type, entity_a_id, entity_b_id, similarity_score, matching_explanation, status, created_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, 'pending', datetime('now'))",
                        params![entity_type, min_id, max_id, score, exp],
                    )?;
                    if inserted > 0 {
                        candidate_count += 1;
                        if candidate_count >= limit_pairs {
                            tx.commit()?;
                            return Ok(candidate_count);
                        }
                    }
                }
            }
        }

        tx.commit()?;
        Ok(candidate_count)
    }

    /// Recalculates catalog, release years, ownership, and relationship counts for an entity.
    pub fn recalculate_entity_metrics(
        conn: &Connection,
        entity_type: &str,
        entity_id: i64,
    ) -> Result<()> {
        if entity_type == "developer" {
            conn.execute(
                "UPDATE developers SET
                 game_count = (SELECT COUNT(DISTINCT game_id) FROM game_developers WHERE developer_id = ?1),
                 publisher_count = (
                     SELECT COUNT(DISTINCT gp.publisher_id)
                     FROM game_developers gd
                     JOIN game_publishers gp ON gd.game_id = gp.game_id
                     WHERE gd.developer_id = ?1
                 ),
                 genre_count = (
                     SELECT COUNT(DISTINCT gg.genre_id)
                     FROM game_developers gd
                     JOIN game_genres gg ON gd.game_id = gg.game_id
                     WHERE gd.developer_id = ?1
                 ),
                 collaborator_count = (
                     SELECT COUNT(DISTINCT gd2.developer_id)
                     FROM game_developers gd1
                     JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id != gd2.developer_id
                     WHERE gd1.developer_id = ?1
                 ),
                 earliest_release_year = (
                     SELECT MIN(g.release_year)
                     FROM game_developers gd
                     JOIN games g ON gd.game_id = g.id
                     WHERE gd.developer_id = ?1 AND g.release_year IS NOT NULL AND g.release_year > 1970
                 ),
                 latest_release_year = (
                     SELECT MAX(g.release_year)
                     FROM game_developers gd
                     JOIN games g ON gd.game_id = g.id
                     WHERE gd.developer_id = ?1 AND g.release_year IS NOT NULL AND g.release_year > 1970
                 ),
                 total_owner_lower = (
                     SELECT COALESCE(SUM(g.owner_lower), 0)
                     FROM game_developers gd
                     JOIN games g ON gd.game_id = g.id
                     WHERE gd.developer_id = ?1
                 ),
                 total_owner_upper = (
                     SELECT COALESCE(SUM(g.owner_upper), 0)
                     FROM game_developers gd
                     JOIN games g ON gd.game_id = g.id
                     WHERE gd.developer_id = ?1
                 ),
                 total_owner_midpoint = (
                     SELECT COALESCE(SUM(g.owner_midpoint), 0)
                     FROM game_developers gd
                     JOIN games g ON gd.game_id = g.id
                     WHERE gd.developer_id = ?1
                 ),
                 updated_at = datetime('now')
                 WHERE id = ?1",
                params![entity_id],
            )?;
        } else {
            // Publisher metrics including Shannon Entropy
            let entropy = Self::calculate_publisher_shannon_entropy(conn, entity_id)?;

            conn.execute(
                "UPDATE publishers SET
                 game_count = (SELECT COUNT(DISTINCT game_id) FROM game_publishers WHERE publisher_id = ?1),
                 developer_count = (
                     SELECT COUNT(DISTINCT gd.developer_id)
                     FROM game_publishers gp
                     JOIN game_developers gd ON gp.game_id = gd.game_id
                     WHERE gp.publisher_id = ?1
                 ),
                 genre_count = (
                     SELECT COUNT(DISTINCT gg.genre_id)
                     FROM game_publishers gp
                     JOIN game_genres gg ON gp.game_id = gg.game_id
                     WHERE gp.publisher_id = ?1
                 ),
                 earliest_release_year = (
                     SELECT MIN(g.release_year)
                     FROM game_publishers gp
                     JOIN games g ON gp.game_id = g.id
                     WHERE gp.publisher_id = ?1 AND g.release_year IS NOT NULL AND g.release_year > 1970
                 ),
                 latest_release_year = (
                     SELECT MAX(g.release_year)
                     FROM game_publishers gp
                     JOIN games g ON gp.game_id = g.id
                     WHERE gp.publisher_id = ?1 AND g.release_year IS NOT NULL AND g.release_year > 1970
                 ),
                 total_owner_lower = (
                     SELECT COALESCE(SUM(g.owner_lower), 0)
                     FROM game_publishers gp
                     JOIN games g ON gp.game_id = g.id
                     WHERE gp.publisher_id = ?1
                 ),
                 total_owner_upper = (
                     SELECT COALESCE(SUM(g.owner_upper), 0)
                     FROM game_publishers gp
                     JOIN games g ON gp.game_id = g.id
                     WHERE gp.publisher_id = ?1
                 ),
                 total_owner_midpoint = (
                     SELECT COALESCE(SUM(g.owner_midpoint), 0)
                     FROM game_publishers gp
                     JOIN games g ON gp.game_id = g.id
                     WHERE gp.publisher_id = ?1
                 ),
                 shannon_entropy = ?2,
                 updated_at = datetime('now')
                 WHERE id = ?1",
                params![entity_id, entropy],
            )?;
        }

        Ok(())
    }

    /// Calculates Shannon entropy for a publisher's genre distribution:
    /// H = - sum(p_i * ln(p_i))
    pub fn calculate_publisher_shannon_entropy(
        conn: &Connection,
        publisher_id: i64,
    ) -> Result<f64> {
        let mut stmt = conn.prepare(
            "SELECT COUNT(gg.game_id) as genre_count
             FROM game_publishers gp
             JOIN game_genres gg ON gp.game_id = gg.game_id
             WHERE gp.publisher_id = ?1
             GROUP BY gg.genre_id",
        )?;

        let counts: Vec<f64> = stmt
            .query_map(params![publisher_id], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        let total: f64 = counts.iter().sum();
        if total <= 0.0 || counts.len() <= 1 {
            return Ok(0.0);
        }

        let mut entropy = 0.0;
        for c in counts {
            if c > 0.0 {
                let p = c / total;
                entropy -= p * p.ln();
            }
        }

        Ok(entropy)
    }
}
