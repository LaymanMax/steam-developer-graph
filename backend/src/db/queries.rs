use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewSummary {
    pub total_games: i64,
    pub total_developers: i64,
    pub total_publishers: i64,
    pub total_genres: i64,
    pub dev_pub_relationships: i64,
    pub dev_collab_relationships: i64,
    pub complete_credit_games: i64,
    pub sparse_records: i64,
    pub likely_playtests: i64,
    pub total_aliases: i64,
    pub outstanding_candidates: i64,
    pub top_publishers_by_devs: Vec<RankedEntity>,
    pub top_developers_by_pubs: Vec<RankedEntity>,
    pub top_developers_by_games: Vec<RankedEntity>,
    pub top_publishers_by_entropy: Vec<RankedEntity>,
    pub top_collaborations: Vec<RankedCollab>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedEntity {
    pub id: i64,
    pub name: String,
    pub metric_value: f64,
    pub metric_label: String,
    pub secondary_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedCollab {
    pub dev1_id: i64,
    pub dev1_name: String,
    pub dev2_id: i64,
    pub dev2_name: String,
    pub shared_games: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub entity_type: String, // "developer", "publisher", "game"
    pub id: i64,
    pub label: String,
    pub sub_label: Option<String>,
    pub matched_alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperListItem {
    pub id: i64,
    pub canonical_name: String,
    pub game_count: i64,
    pub publisher_count: i64,
    pub genre_count: i64,
    pub collaborator_count: i64,
    pub earliest_release_year: Option<i32>,
    pub latest_release_year: Option<i32>,
    pub total_owner_midpoint: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperProfile {
    pub id: i64,
    pub canonical_name: String,
    pub normalized_name: String,
    pub aliases: Vec<String>,
    pub game_count: i64,
    pub publisher_count: i64,
    pub genre_count: i64,
    pub collaborator_count: i64,
    pub earliest_release_year: Option<i32>,
    pub latest_release_year: Option<i32>,
    pub total_owner_lower: i64,
    pub total_owner_upper: i64,
    pub total_owner_midpoint: i64,
    pub catalog: Vec<GameSummaryItem>,
    pub publishers: Vec<ConnectedPublisherItem>,
    pub collaborators: Vec<ConnectedDeveloperItem>,
    pub timeline: Vec<YearGameCount>,
    pub genres: Vec<GenreCountItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherListItem {
    pub id: i64,
    pub canonical_name: String,
    pub game_count: i64,
    pub developer_count: i64,
    pub genre_count: i64,
    pub earliest_release_year: Option<i32>,
    pub latest_release_year: Option<i32>,
    pub total_owner_midpoint: i64,
    pub shannon_entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherProfile {
    pub id: i64,
    pub canonical_name: String,
    pub normalized_name: String,
    pub aliases: Vec<String>,
    pub game_count: i64,
    pub developer_count: i64,
    pub genre_count: i64,
    pub earliest_release_year: Option<i32>,
    pub latest_release_year: Option<i32>,
    pub total_owner_lower: i64,
    pub total_owner_upper: i64,
    pub total_owner_midpoint: i64,
    pub shannon_entropy: f64,
    pub developer_relationships: Vec<ConnectedDeveloperItem>,
    pub portfolio: Vec<GameSummaryItem>,
    pub genre_distribution: Vec<GenreCountItem>,
    pub timeline: Vec<YearGameCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummaryItem {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub release_date: Option<String>,
    pub release_year: Option<i32>,
    pub estimated_owners_raw: Option<String>,
    pub owner_lower: i64,
    pub owner_upper: i64,
    pub owner_midpoint: i64,
    pub price: f64,
    pub header_image: Option<String>,
    pub is_sparse: bool,
    pub is_likely_playtest: bool,
    pub developers: Vec<EntityRef>,
    pub publishers: Vec<EntityRef>,
    pub genres: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDetail {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub release_date: Option<String>,
    pub release_year: Option<i32>,
    pub estimated_owners_raw: Option<String>,
    pub owner_lower: i64,
    pub owner_upper: i64,
    pub owner_midpoint: i64,
    pub price: f64,
    pub peak_ccu: i64,
    pub positive: i64,
    pub negative: i64,
    pub recommendations: i64,
    pub average_playtime_forever: i64,
    pub median_playtime_forever: i64,
    pub header_image: Option<String>,
    pub metacritic_score: i64,
    pub is_sparse: bool,
    pub is_likely_playtest: bool,
    pub steam_store_url: String,
    pub developers: Vec<EntityRef>,
    pub publishers: Vec<EntityRef>,
    pub genres: Vec<String>,
    pub related_games: Vec<GameSummaryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedPublisherItem {
    pub publisher_id: i64,
    pub publisher_name: String,
    pub shared_games_count: i64,
    pub first_year: Option<i32>,
    pub latest_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedDeveloperItem {
    pub developer_id: i64,
    pub developer_name: String,
    pub shared_games_count: i64,
    pub first_year: Option<i32>,
    pub latest_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreCountItem {
    pub genre_name: String,
    pub game_count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearGameCount {
    pub year: i32,
    pub game_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionCandidateItem {
    pub id: i64,
    pub entity_type: String,
    pub entity_a_id: i64,
    pub entity_a_name: String,
    pub entity_a_games: i64,
    pub entity_b_id: i64,
    pub entity_b_name: String,
    pub entity_b_games: i64,
    pub similarity_score: f64,
    pub matching_explanation: String,
    pub status: String,
    pub example_games_a: Vec<String>,
    pub example_games_b: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeHistoryItem {
    pub id: i64,
    pub entity_type: String,
    pub source_entity_id: i64,
    pub target_entity_id: i64,
    pub source_canonical_name: String,
    pub target_canonical_name: String,
    pub reason: Option<String>,
    pub created_at: String,
    pub is_undone: bool,
    pub undone_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionReport {
    pub total_observed_names: i64,
    pub total_canonical_developers: i64,
    pub total_canonical_publishers: i64,
    pub auto_resolved_aliases: i64,
    pub manual_confirmed_aliases: i64,
    pub outstanding_candidates: i64,
    pub rejected_candidates: i64,
    pub merged_entities_count: i64,
}

pub struct DbQueries;

impl DbQueries {
    /// Overview summary figures and top rankings
    pub fn get_overview_summary(conn: &Connection) -> Result<OverviewSummary> {
        let total_games: i64 = conn.query_row("SELECT COUNT(*) FROM games", [], |r| r.get(0))?;
        let total_developers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM developers WHERE is_merged = 0",
            [],
            |r| r.get(0),
        )?;
        let total_publishers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM publishers WHERE is_merged = 0",
            [],
            |r| r.get(0),
        )?;
        let total_genres: i64 = conn.query_row("SELECT COUNT(*) FROM genres", [], |r| r.get(0))?;

        let dev_pub_relationships: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT gd.developer_id || '-' || gp.publisher_id)
             FROM game_developers gd
             JOIN game_publishers gp ON gd.game_id = gp.game_id",
            [],
            |r| r.get(0),
        )?;

        let dev_collab_relationships: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT gd1.developer_id || '-' || gd2.developer_id)
             FROM game_developers gd1
             JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id < gd2.developer_id",
            [],
            |r| r.get(0),
        )?;

        let complete_credit_games: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT g.id) FROM games g
             WHERE EXISTS (SELECT 1 FROM game_developers gd WHERE gd.game_id = g.id)
               AND EXISTS (SELECT 1 FROM game_publishers gp WHERE gp.game_id = g.id)
               AND EXISTS (SELECT 1 FROM game_genres gg WHERE gg.game_id = g.id)
               AND g.release_year IS NOT NULL",
            [],
            |r| r.get(0),
        )?;

        let sparse_records: i64 =
            conn.query_row("SELECT COUNT(*) FROM games WHERE is_sparse = 1", [], |r| {
                r.get(0)
            })?;
        let likely_playtests: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games WHERE is_likely_playtest = 1",
            [],
            |r| r.get(0),
        )?;

        let total_aliases: i64 =
            conn.query_row("SELECT COUNT(*) FROM entity_aliases", [], |r| r.get(0))?;
        let outstanding_candidates: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entity_resolution_candidates WHERE status = 'pending'",
            [],
            |r| r.get(0),
        )?;

        // Top publishers by developer count
        let mut pub_dev_stmt = conn.prepare(
            "SELECT id, canonical_name, developer_count, game_count 
             FROM publishers WHERE is_merged = 0 
             ORDER BY developer_count DESC LIMIT 8",
        )?;
        let top_publishers_by_devs: Vec<RankedEntity> = pub_dev_stmt
            .query_map([], |r| {
                Ok(RankedEntity {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    metric_value: r.get::<_, i64>(2)? as f64,
                    metric_label: "developers".to_string(),
                    secondary_label: Some(format!("{} games", r.get::<_, i64>(3)?)),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Top developers by associated publishers
        let mut dev_pub_stmt = conn.prepare(
            "SELECT id, canonical_name, publisher_count, game_count 
             FROM developers WHERE is_merged = 0 
             ORDER BY publisher_count DESC LIMIT 8",
        )?;
        let top_developers_by_pubs: Vec<RankedEntity> = dev_pub_stmt
            .query_map([], |r| {
                Ok(RankedEntity {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    metric_value: r.get::<_, i64>(2)? as f64,
                    metric_label: "publishers".to_string(),
                    secondary_label: Some(format!("{} games", r.get::<_, i64>(3)?)),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Top developers by catalog size
        let mut dev_cat_stmt = conn.prepare(
            "SELECT id, canonical_name, game_count, collaborator_count 
             FROM developers WHERE is_merged = 0 
             ORDER BY game_count DESC LIMIT 8",
        )?;
        let top_developers_by_games: Vec<RankedEntity> = dev_cat_stmt
            .query_map([], |r| {
                Ok(RankedEntity {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    metric_value: r.get::<_, i64>(2)? as f64,
                    metric_label: "games".to_string(),
                    secondary_label: Some(format!("{} collaborators", r.get::<_, i64>(3)?)),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Top publishers by Shannon entropy (genre diversity)
        let mut pub_ent_stmt = conn.prepare(
            "SELECT id, canonical_name, shannon_entropy, genre_count, game_count
             FROM publishers 
             WHERE is_merged = 0 AND game_count >= 8
             ORDER BY shannon_entropy DESC LIMIT 8",
        )?;
        let top_publishers_by_entropy: Vec<RankedEntity> = pub_ent_stmt
            .query_map([], |r| {
                Ok(RankedEntity {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    metric_value: r.get(2)?,
                    metric_label: "entropy".to_string(),
                    secondary_label: Some(format!(
                        "{} genres across {} games",
                        r.get::<_, i64>(3)?,
                        r.get::<_, i64>(4)?
                    )),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Top collaboration pairs
        let mut collab_stmt = conn.prepare(
            "SELECT gd1.developer_id, d1.canonical_name,
                    gd2.developer_id, d2.canonical_name,
                    COUNT(DISTINCT gd1.game_id) as shared_cnt
             FROM game_developers gd1
             JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id < gd2.developer_id
             JOIN developers d1 ON gd1.developer_id = d1.id
             JOIN developers d2 ON gd2.developer_id = d2.id
             WHERE d1.is_merged = 0 AND d2.is_merged = 0
             GROUP BY gd1.developer_id, gd2.developer_id
             ORDER BY shared_cnt DESC LIMIT 8",
        )?;
        let top_collaborations: Vec<RankedCollab> = collab_stmt
            .query_map([], |r| {
                Ok(RankedCollab {
                    dev1_id: r.get(0)?,
                    dev1_name: r.get(1)?,
                    dev2_id: r.get(2)?,
                    dev2_name: r.get(3)?,
                    shared_games: r.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(OverviewSummary {
            total_games,
            total_developers,
            total_publishers,
            total_genres,
            dev_pub_relationships,
            dev_collab_relationships,
            complete_credit_games,
            sparse_records,
            likely_playtests,
            total_aliases,
            outstanding_candidates,
            top_publishers_by_devs,
            top_developers_by_pubs,
            top_developers_by_games,
            top_publishers_by_entropy,
            top_collaborations,
        })
    }

    /// Global search across developers, publishers, games, and alias mappings
    pub fn global_search(
        conn: &Connection,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }
        let term = format!("%{}%", trimmed.to_lowercase());
        let mut results = Vec::new();

        // 1. Search Developers by canonical or alias
        let mut dev_stmt = conn.prepare(
            "SELECT d.id, d.canonical_name, d.game_count, a.raw_name as matched_alias
             FROM developers d
             LEFT JOIN entity_aliases a ON a.canonical_entity_id = d.id AND a.entity_type = 'developer' AND LOWER(a.raw_name) LIKE ?1
             WHERE d.is_merged = 0 AND (LOWER(d.canonical_name) LIKE ?1 OR a.raw_name IS NOT NULL)
             GROUP BY d.id
             ORDER BY d.game_count DESC LIMIT ?2",
        )?;
        let devs = dev_stmt.query_map(params![term, (limit / 2).max(5) as i64], |r| {
            let id: i64 = r.get(0)?;
            let canonical: String = r.get(1)?;
            let games: i64 = r.get(2)?;
            let matched_alias: Option<String> = r.get(3)?;

            let alias_info = if let Some(ref a) = matched_alias {
                if !a.eq_ignore_ascii_case(&canonical) {
                    Some(format!("Matched alias: '{}'", a))
                } else {
                    None
                }
            } else {
                None
            };

            Ok(SearchResultItem {
                entity_type: "developer".to_string(),
                id,
                label: canonical,
                sub_label: Some(format!("{} games", games)),
                matched_alias: alias_info,
            })
        })?;
        for d in devs.flatten() {
            results.push(d);
        }

        // 2. Search Publishers
        let mut pub_stmt = conn.prepare(
            "SELECT p.id, p.canonical_name, p.game_count, a.raw_name as matched_alias
             FROM publishers p
             LEFT JOIN entity_aliases a ON a.canonical_entity_id = p.id AND a.entity_type = 'publisher' AND LOWER(a.raw_name) LIKE ?1
             WHERE p.is_merged = 0 AND (LOWER(p.canonical_name) LIKE ?1 OR a.raw_name IS NOT NULL)
             GROUP BY p.id
             ORDER BY p.game_count DESC LIMIT ?2",
        )?;
        let pubs = pub_stmt.query_map(params![term, (limit / 2).max(5) as i64], |r| {
            let id: i64 = r.get(0)?;
            let canonical: String = r.get(1)?;
            let games: i64 = r.get(2)?;
            let matched_alias: Option<String> = r.get(3)?;

            let alias_info = if let Some(ref a) = matched_alias {
                if !a.eq_ignore_ascii_case(&canonical) {
                    Some(format!("Matched alias: '{}'", a))
                } else {
                    None
                }
            } else {
                None
            };

            Ok(SearchResultItem {
                entity_type: "publisher".to_string(),
                id,
                label: canonical,
                sub_label: Some(format!("{} games", games)),
                matched_alias: alias_info,
            })
        })?;
        for p in pubs.flatten() {
            results.push(p);
        }

        // 3. Search Games
        let mut game_stmt = conn.prepare(
            "SELECT id, name, release_year, app_id
             FROM games 
             WHERE LOWER(name) LIKE ?1 OR CAST(app_id AS TEXT) = ?2
             ORDER BY owner_midpoint DESC LIMIT ?3",
        )?;
        let games = game_stmt.query_map(params![term, trimmed, limit as i64], |r| {
            let id: i64 = r.get(0)?;
            let name: String = r.get(1)?;
            let year: Option<i32> = r.get(2)?;
            let appid: i64 = r.get(3)?;

            Ok(SearchResultItem {
                entity_type: "game".to_string(),
                id,
                label: name,
                sub_label: Some(format!(
                    "AppID: {} • {}",
                    appid,
                    year.map_or("N/A".to_string(), |y| y.to_string())
                )),
                matched_alias: None,
            })
        })?;
        for g in games.flatten() {
            results.push(g);
        }

        results.truncate(limit);
        Ok(results)
    }

    /// List developers with filtering, sorting, pagination
    pub fn list_developers(
        conn: &Connection,
        search: Option<&str>,
        genre: Option<&str>,
        min_games: Option<i64>,
        sort_by: Option<&str>,
        sort_desc: bool,
        page: usize,
        page_size: usize,
    ) -> Result<(Vec<DeveloperListItem>, i64)> {
        let mut where_clauses = vec!["is_merged = 0".to_string()];

        if let Some(s) = search {
            if !s.trim().is_empty() {
                let escaped = s.trim().replace('\'', "''");
                where_clauses.push(format!(
                    "LOWER(canonical_name) LIKE '%{}%'",
                    escaped.to_lowercase()
                ));
            }
        }
        if let Some(min_g) = min_games {
            where_clauses.push(format!("game_count >= {}", min_g));
        }
        if let Some(g) = genre {
            let escaped = g.trim().replace('\'', "''");
            where_clauses.push(format!(
                "EXISTS (
                    SELECT 1 FROM game_developers gd
                    JOIN game_genres gg ON gd.game_id = gg.game_id
                    JOIN genres gen ON gg.genre_id = gen.id
                    WHERE gd.developer_id = developers.id AND gen.name = '{}'
                )",
                escaped
            ));
        }

        let where_sql = where_clauses.join(" AND ");

        let total_count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM developers WHERE {}", where_sql),
            [],
            |r| r.get(0),
        )?;

        let sort_column = match sort_by.unwrap_or("game_count") {
            "name" | "canonical_name" => "canonical_name",
            "publisher_count" => "publisher_count",
            "collaborator_count" => "collaborator_count",
            "earliest_year" => "earliest_release_year",
            "latest_year" => "latest_release_year",
            "ownership" | "total_owner_midpoint" => "total_owner_midpoint",
            _ => "game_count",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let offset = (page.max(1) - 1) * page_size;
        let sql = format!(
            "SELECT id, canonical_name, game_count, publisher_count, genre_count,
                    collaborator_count, earliest_release_year, latest_release_year, total_owner_midpoint
             FROM developers
             WHERE {}
             ORDER BY {} {} NULLS LAST
             LIMIT {} OFFSET {}",
            where_sql, sort_column, dir, page_size, offset
        );

        let mut stmt = conn.prepare(&sql)?;
        let items = stmt
            .query_map([], |r| {
                Ok(DeveloperListItem {
                    id: r.get(0)?,
                    canonical_name: r.get(1)?,
                    game_count: r.get(2)?,
                    publisher_count: r.get(3)?,
                    genre_count: r.get(4)?,
                    collaborator_count: r.get(5)?,
                    earliest_release_year: r.get(6)?,
                    latest_release_year: r.get(7)?,
                    total_owner_midpoint: r.get(8)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok((items, total_count))
    }

    /// Full developer profile
    pub fn get_developer_profile(
        conn: &Connection,
        dev_id: i64,
    ) -> Result<Option<DeveloperProfile>> {
        let dev_opt = conn.query_row(
            "SELECT id, canonical_name, normalized_name, game_count, publisher_count,
                    genre_count, collaborator_count, earliest_release_year, latest_release_year,
                    total_owner_lower, total_owner_upper, total_owner_midpoint
             FROM developers WHERE id = ?1",
            params![dev_id],
            |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, i64>(3)?,
                    r.get::<_, i64>(4)?,
                    r.get::<_, i64>(5)?,
                    r.get::<_, i64>(6)?,
                    r.get::<_, Option<i32>>(7)?,
                    r.get::<_, Option<i32>>(8)?,
                    r.get::<_, i64>(9)?,
                    r.get::<_, i64>(10)?,
                    r.get::<_, i64>(11)?,
                ))
            },
        );

        let dev = match dev_opt {
            Ok(d) => d,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e),
        };

        // Aliases
        let mut alias_stmt = conn.prepare(
            "SELECT raw_name FROM entity_aliases WHERE entity_type = 'developer' AND canonical_entity_id = ?1",
        )?;
        let aliases: Vec<String> = alias_stmt
            .query_map(params![dev_id], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        // Catalog of games
        let mut game_stmt = conn.prepare(
            "SELECT g.id, g.app_id, g.name, g.release_date, g.release_year,
                    g.estimated_owners_raw, g.owner_lower, g.owner_upper, g.owner_midpoint,
                    g.price, g.header_image, g.is_sparse, g.is_likely_playtest
             FROM games g
             JOIN game_developers gd ON g.id = gd.game_id
             WHERE gd.developer_id = ?1
             ORDER BY g.release_year DESC NULLS LAST, g.owner_midpoint DESC",
        )?;
        let catalog_rows = game_stmt.query_map(params![dev_id], |r| {
            let gid: i64 = r.get(0)?;
            Ok(GameSummaryItem {
                id: gid,
                app_id: r.get(1)?,
                name: r.get(2)?,
                release_date: r.get(3)?,
                release_year: r.get(4)?,
                estimated_owners_raw: r.get(5)?,
                owner_lower: r.get(6)?,
                owner_upper: r.get(7)?,
                owner_midpoint: r.get(8)?,
                price: r.get(9)?,
                header_image: r.get(10)?,
                is_sparse: r.get::<_, i64>(11)? == 1,
                is_likely_playtest: r.get::<_, i64>(12)? == 1,
                developers: Vec::new(),
                publishers: Vec::new(),
                genres: Vec::new(),
            })
        })?;
        let mut catalog: Vec<GameSummaryItem> = catalog_rows.filter_map(|r| r.ok()).collect();

        // Populate publishers and genres for catalog games
        for game in &mut catalog {
            let mut pub_s = conn.prepare(
                "SELECT p.id, p.canonical_name FROM publishers p
                 JOIN game_publishers gp ON p.id = gp.publisher_id
                 WHERE gp.game_id = ?1",
            )?;
            game.publishers = pub_s
                .query_map(params![game.id], |r| {
                    Ok(EntityRef {
                        id: r.get(0)?,
                        name: r.get(1)?,
                    })
                })?
                .filter_map(|r| r.ok())
                .collect();

            let mut gen_s = conn.prepare(
                "SELECT gen.name FROM genres gen
                 JOIN game_genres gg ON gen.id = gg.genre_id
                 WHERE gg.game_id = ?1",
            )?;
            game.genres = gen_s
                .query_map(params![game.id], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect();
        }

        // Connected publishers
        let mut conn_pub_stmt = conn.prepare(
            "SELECT p.id, p.canonical_name, COUNT(DISTINCT gp.game_id) as shared_cnt,
                    MIN(g.release_year), MAX(g.release_year)
             FROM game_developers gd
             JOIN game_publishers gp ON gd.game_id = gp.game_id
             JOIN publishers p ON gp.publisher_id = p.id
             JOIN games g ON gd.game_id = g.id
             WHERE gd.developer_id = ?1 AND p.is_merged = 0
             GROUP BY p.id
             ORDER BY shared_cnt DESC",
        )?;
        let publishers = conn_pub_stmt
            .query_map(params![dev_id], |r| {
                Ok(ConnectedPublisherItem {
                    publisher_id: r.get(0)?,
                    publisher_name: r.get(1)?,
                    shared_games_count: r.get(2)?,
                    first_year: r.get(3)?,
                    latest_year: r.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Collaborators
        let mut collab_stmt = conn.prepare(
            "SELECT d.id, d.canonical_name, COUNT(DISTINCT gd1.game_id) as shared_cnt,
                    MIN(g.release_year), MAX(g.release_year)
             FROM game_developers gd1
             JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id != gd2.developer_id
             JOIN developers d ON gd2.developer_id = d.id
             JOIN games g ON gd1.game_id = g.id
             WHERE gd1.developer_id = ?1 AND d.is_merged = 0
             GROUP BY d.id
             ORDER BY shared_cnt DESC",
        )?;
        let collaborators = collab_stmt
            .query_map(params![dev_id], |r| {
                Ok(ConnectedDeveloperItem {
                    developer_id: r.get(0)?,
                    developer_name: r.get(1)?,
                    shared_games_count: r.get(2)?,
                    first_year: r.get(3)?,
                    latest_year: r.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Timeline: game counts by year
        let mut timeline_stmt = conn.prepare(
            "SELECT g.release_year, COUNT(DISTINCT g.id)
             FROM games g
             JOIN game_developers gd ON g.id = gd.game_id
             WHERE gd.developer_id = ?1 AND g.release_year IS NOT NULL
             GROUP BY g.release_year
             ORDER BY g.release_year ASC",
        )?;
        let timeline = timeline_stmt
            .query_map(params![dev_id], |r| {
                Ok(YearGameCount {
                    year: r.get(0)?,
                    game_count: r.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Genres distribution
        let total_game_count = dev.3.max(1) as f64;
        let mut genre_stmt = conn.prepare(
            "SELECT gen.name, COUNT(DISTINCT gg.game_id) as cnt
             FROM game_developers gd
             JOIN game_genres gg ON gd.game_id = gg.game_id
             JOIN genres gen ON gg.genre_id = gen.id
             WHERE gd.developer_id = ?1
             GROUP BY gen.id
             ORDER BY cnt DESC",
        )?;
        let genres = genre_stmt
            .query_map(params![dev_id], |r| {
                let cnt: i64 = r.get(1)?;
                Ok(GenreCountItem {
                    genre_name: r.get(0)?,
                    game_count: cnt,
                    percentage: (cnt as f64 / total_game_count) * 100.0,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(Some(DeveloperProfile {
            id: dev.0,
            canonical_name: dev.1,
            normalized_name: dev.2,
            aliases,
            game_count: dev.3,
            publisher_count: dev.4,
            genre_count: dev.5,
            collaborator_count: dev.6,
            earliest_release_year: dev.7,
            latest_release_year: dev.8,
            total_owner_lower: dev.9,
            total_owner_upper: dev.10,
            total_owner_midpoint: dev.11,
            catalog,
            publishers,
            collaborators,
            timeline,
            genres,
        }))
    }

    /// List publishers with filtering, sorting, pagination
    pub fn list_publishers(
        conn: &Connection,
        search: Option<&str>,
        genre: Option<&str>,
        min_games: Option<i64>,
        sort_by: Option<&str>,
        sort_desc: bool,
        page: usize,
        page_size: usize,
    ) -> Result<(Vec<PublisherListItem>, i64)> {
        let mut where_clauses = vec!["is_merged = 0".to_string()];

        if let Some(s) = search {
            if !s.trim().is_empty() {
                let escaped = s.trim().replace('\'', "''");
                where_clauses.push(format!(
                    "LOWER(canonical_name) LIKE '%{}%'",
                    escaped.to_lowercase()
                ));
            }
        }
        if let Some(min_g) = min_games {
            where_clauses.push(format!("game_count >= {}", min_g));
        }
        if let Some(g) = genre {
            let escaped = g.trim().replace('\'', "''");
            where_clauses.push(format!(
                "EXISTS (
                    SELECT 1 FROM game_publishers gp
                    JOIN game_genres gg ON gp.game_id = gg.game_id
                    JOIN genres gen ON gg.genre_id = gen.id
                    WHERE gp.publisher_id = publishers.id AND gen.name = '{}'
                )",
                escaped
            ));
        }

        let where_sql = where_clauses.join(" AND ");

        let total_count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM publishers WHERE {}", where_sql),
            [],
            |r| r.get(0),
        )?;

        let sort_column = match sort_by.unwrap_or("game_count") {
            "name" | "canonical_name" => "canonical_name",
            "developer_count" => "developer_count",
            "genre_count" => "genre_count",
            "earliest_year" => "earliest_release_year",
            "latest_year" => "latest_release_year",
            "ownership" | "total_owner_midpoint" => "total_owner_midpoint",
            "entropy" | "shannon_entropy" => "shannon_entropy",
            _ => "game_count",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let offset = (page.max(1) - 1) * page_size;
        let sql = format!(
            "SELECT id, canonical_name, game_count, developer_count, genre_count,
                    earliest_release_year, latest_release_year, total_owner_midpoint, shannon_entropy
             FROM publishers
             WHERE {}
             ORDER BY {} {} NULLS LAST
             LIMIT {} OFFSET {}",
            where_sql, sort_column, dir, page_size, offset
        );

        let mut stmt = conn.prepare(&sql)?;
        let items = stmt
            .query_map([], |r| {
                Ok(PublisherListItem {
                    id: r.get(0)?,
                    canonical_name: r.get(1)?,
                    game_count: r.get(2)?,
                    developer_count: r.get(3)?,
                    genre_count: r.get(4)?,
                    earliest_release_year: r.get(5)?,
                    latest_release_year: r.get(6)?,
                    total_owner_midpoint: r.get(7)?,
                    shannon_entropy: r.get(8)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok((items, total_count))
    }

    /// Full publisher profile
    pub fn get_publisher_profile(
        conn: &Connection,
        pub_id: i64,
    ) -> Result<Option<PublisherProfile>> {
        let pub_opt = conn.query_row(
            "SELECT id, canonical_name, normalized_name, game_count, developer_count,
                    genre_count, earliest_release_year, latest_release_year,
                    total_owner_lower, total_owner_upper, total_owner_midpoint, shannon_entropy
             FROM publishers WHERE id = ?1",
            params![pub_id],
            |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, i64>(3)?,
                    r.get::<_, i64>(4)?,
                    r.get::<_, i64>(5)?,
                    r.get::<_, Option<i32>>(6)?,
                    r.get::<_, Option<i32>>(7)?,
                    r.get::<_, i64>(8)?,
                    r.get::<_, i64>(9)?,
                    r.get::<_, i64>(10)?,
                    r.get::<_, f64>(11)?,
                ))
            },
        );

        let pub_row = match pub_opt {
            Ok(p) => p,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e),
        };

        // Aliases
        let mut alias_stmt = conn.prepare(
            "SELECT raw_name FROM entity_aliases WHERE entity_type = 'publisher' AND canonical_entity_id = ?1",
        )?;
        let aliases: Vec<String> = alias_stmt
            .query_map(params![pub_id], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        // Associated developers
        let mut dev_stmt = conn.prepare(
            "SELECT d.id, d.canonical_name, COUNT(DISTINCT gp.game_id) as shared_cnt,
                    MIN(g.release_year), MAX(g.release_year)
             FROM game_publishers gp
             JOIN game_developers gd ON gp.game_id = gd.game_id
             JOIN developers d ON gd.developer_id = d.id
             JOIN games g ON gp.game_id = g.id
             WHERE gp.publisher_id = ?1 AND d.is_merged = 0
             GROUP BY d.id
             ORDER BY shared_cnt DESC",
        )?;
        let developer_relationships = dev_stmt
            .query_map(params![pub_id], |r| {
                Ok(ConnectedDeveloperItem {
                    developer_id: r.get(0)?,
                    developer_name: r.get(1)?,
                    shared_games_count: r.get(2)?,
                    first_year: r.get(3)?,
                    latest_year: r.get(4)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Portfolio of games
        let mut game_stmt = conn.prepare(
            "SELECT g.id, g.app_id, g.name, g.release_date, g.release_year,
                    g.estimated_owners_raw, g.owner_lower, g.owner_upper, g.owner_midpoint,
                    g.price, g.header_image, g.is_sparse, g.is_likely_playtest
             FROM games g
             JOIN game_publishers gp ON g.id = gp.game_id
             WHERE gp.publisher_id = ?1
             ORDER BY g.release_year DESC NULLS LAST, g.owner_midpoint DESC",
        )?;
        let port_rows = game_stmt.query_map(params![pub_id], |r| {
            let gid: i64 = r.get(0)?;
            Ok(GameSummaryItem {
                id: gid,
                app_id: r.get(1)?,
                name: r.get(2)?,
                release_date: r.get(3)?,
                release_year: r.get(4)?,
                estimated_owners_raw: r.get(5)?,
                owner_lower: r.get(6)?,
                owner_upper: r.get(7)?,
                owner_midpoint: r.get(8)?,
                price: r.get(9)?,
                header_image: r.get(10)?,
                is_sparse: r.get::<_, i64>(11)? == 1,
                is_likely_playtest: r.get::<_, i64>(12)? == 1,
                developers: Vec::new(),
                publishers: Vec::new(),
                genres: Vec::new(),
            })
        })?;
        let mut portfolio: Vec<GameSummaryItem> = port_rows.filter_map(|r| r.ok()).collect();

        for game in &mut portfolio {
            let mut dev_s = conn.prepare(
                "SELECT d.id, d.canonical_name FROM developers d
                 JOIN game_developers gd ON d.id = gd.developer_id
                 WHERE gd.game_id = ?1",
            )?;
            game.developers = dev_s
                .query_map(params![game.id], |r| {
                    Ok(EntityRef {
                        id: r.get(0)?,
                        name: r.get(1)?,
                    })
                })?
                .filter_map(|r| r.ok())
                .collect();

            let mut gen_s = conn.prepare(
                "SELECT gen.name FROM genres gen
                 JOIN game_genres gg ON gen.id = gg.genre_id
                 WHERE gg.game_id = ?1",
            )?;
            game.genres = gen_s
                .query_map(params![game.id], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect();
        }

        // Genre distribution
        let total_game_count = pub_row.3.max(1) as f64;
        let mut gen_stmt = conn.prepare(
            "SELECT gen.name, COUNT(DISTINCT gg.game_id) as cnt
             FROM game_publishers gp
             JOIN game_genres gg ON gp.game_id = gg.game_id
             JOIN genres gen ON gg.genre_id = gen.id
             WHERE gp.publisher_id = ?1
             GROUP BY gen.id
             ORDER BY cnt DESC",
        )?;
        let genre_distribution = gen_stmt
            .query_map(params![pub_id], |r| {
                let cnt: i64 = r.get(1)?;
                Ok(GenreCountItem {
                    genre_name: r.get(0)?,
                    game_count: cnt,
                    percentage: (cnt as f64 / total_game_count) * 100.0,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Timeline
        let mut timeline_stmt = conn.prepare(
            "SELECT g.release_year, COUNT(DISTINCT g.id)
             FROM games g
             JOIN game_publishers gp ON g.id = gp.game_id
             WHERE gp.publisher_id = ?1 AND g.release_year IS NOT NULL
             GROUP BY g.release_year
             ORDER BY g.release_year ASC",
        )?;
        let timeline = timeline_stmt
            .query_map(params![pub_id], |r| {
                Ok(YearGameCount {
                    year: r.get(0)?,
                    game_count: r.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(Some(PublisherProfile {
            id: pub_row.0,
            canonical_name: pub_row.1,
            normalized_name: pub_row.2,
            aliases,
            game_count: pub_row.3,
            developer_count: pub_row.4,
            genre_count: pub_row.5,
            earliest_release_year: pub_row.6,
            latest_release_year: pub_row.7,
            total_owner_lower: pub_row.8,
            total_owner_upper: pub_row.9,
            total_owner_midpoint: pub_row.10,
            shannon_entropy: pub_row.11,
            developer_relationships,
            portfolio,
            genre_distribution,
            timeline,
        }))
    }

    /// List games with multi-attribute filtering
    pub fn list_games(
        conn: &Connection,
        search: Option<&str>,
        genre: Option<&str>,
        dev_id: Option<i64>,
        pub_id: Option<i64>,
        min_year: Option<i32>,
        max_year: Option<i32>,
        min_owners: Option<i64>,
        max_owners: Option<i64>,
        exclude_sparse: bool,
        exclude_playtests: bool,
        sort_by: Option<&str>,
        sort_desc: bool,
        page: usize,
        page_size: usize,
    ) -> Result<(Vec<GameSummaryItem>, i64)> {
        let mut where_clauses = Vec::new();

        if let Some(s) = search {
            if !s.trim().is_empty() {
                let escaped = s.trim().replace('\'', "''");
                where_clauses.push(format!(
                    "(LOWER(name) LIKE '%{}%' OR CAST(app_id AS TEXT) = '{}')",
                    escaped.to_lowercase(),
                    escaped
                ));
            }
        }
        if let Some(g) = genre {
            let escaped = g.trim().replace('\'', "''");
            where_clauses.push(format!(
                "EXISTS (SELECT 1 FROM game_genres gg JOIN genres gen ON gg.genre_id = gen.id WHERE gg.game_id = games.id AND gen.name = '{}')",
                escaped
            ));
        }
        if let Some(did) = dev_id {
            where_clauses.push(format!(
                "EXISTS (SELECT 1 FROM game_developers gd WHERE gd.game_id = games.id AND gd.developer_id = {})",
                did
            ));
        }
        if let Some(pid) = pub_id {
            where_clauses.push(format!(
                "EXISTS (SELECT 1 FROM game_publishers gp WHERE gp.game_id = games.id AND gp.publisher_id = {})",
                pid
            ));
        }
        if let Some(min_y) = min_year {
            where_clauses.push(format!("release_year >= {}", min_y));
        }
        if let Some(max_y) = max_year {
            where_clauses.push(format!("release_year <= {}", max_y));
        }
        if let Some(min_o) = min_owners {
            where_clauses.push(format!("owner_midpoint >= {}", min_o));
        }
        if let Some(max_o) = max_owners {
            where_clauses.push(format!("owner_midpoint <= {}", max_o));
        }
        if exclude_sparse {
            where_clauses.push("is_sparse = 0".to_string());
        }
        if exclude_playtests {
            where_clauses.push("is_likely_playtest = 0".to_string());
        }

        let where_sql = if where_clauses.is_empty() {
            "1=1".to_string()
        } else {
            where_clauses.join(" AND ")
        };

        let total_count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM games WHERE {}", where_sql),
            [],
            |r| r.get(0),
        )?;

        let sort_column = match sort_by.unwrap_or("owners") {
            "name" => "name",
            "release_date" | "year" => "release_year",
            "price" => "price",
            "positive" => "positive",
            _ => "owner_midpoint",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let offset = (page.max(1) - 1) * page_size;
        let sql = format!(
            "SELECT id, app_id, name, release_date, release_year, estimated_owners_raw,
                    owner_lower, owner_upper, owner_midpoint, price, header_image, is_sparse, is_likely_playtest
             FROM games
             WHERE {}
             ORDER BY {} {} NULLS LAST
             LIMIT {} OFFSET {}",
            where_sql, sort_column, dir, page_size, offset
        );

        let mut stmt = conn.prepare(&sql)?;
        let items: Vec<GameSummaryItem> = stmt
            .query_map([], |r| {
                let gid: i64 = r.get(0)?;
                Ok(GameSummaryItem {
                    id: gid,
                    app_id: r.get(1)?,
                    name: r.get(2)?,
                    release_date: r.get(3)?,
                    release_year: r.get(4)?,
                    estimated_owners_raw: r.get(5)?,
                    owner_lower: r.get(6)?,
                    owner_upper: r.get(7)?,
                    owner_midpoint: r.get(8)?,
                    price: r.get(9)?,
                    header_image: r.get(10)?,
                    is_sparse: r.get::<_, i64>(11)? == 1,
                    is_likely_playtest: r.get::<_, i64>(12)? == 1,
                    developers: Vec::new(),
                    publishers: Vec::new(),
                    genres: Vec::new(),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Hydrate brief developer and publisher names
        let mut hydrated = items;
        for game in &mut hydrated {
            let mut dev_s = conn.prepare(
                "SELECT d.id, d.canonical_name FROM developers d
                 JOIN game_developers gd ON d.id = gd.developer_id
                 WHERE gd.game_id = ?1 LIMIT 3",
            )?;
            game.developers = dev_s
                .query_map(params![game.id], |r| {
                    Ok(EntityRef {
                        id: r.get(0)?,
                        name: r.get(1)?,
                    })
                })?
                .filter_map(|r| r.ok())
                .collect();

            let mut pub_s = conn.prepare(
                "SELECT p.id, p.canonical_name FROM publishers p
                 JOIN game_publishers gp ON p.id = gp.publisher_id
                 WHERE gp.game_id = ?1 LIMIT 3",
            )?;
            game.publishers = pub_s
                .query_map(params![game.id], |r| {
                    Ok(EntityRef {
                        id: r.get(0)?,
                        name: r.get(1)?,
                    })
                })?
                .filter_map(|r| r.ok())
                .collect();
        }

        Ok((hydrated, total_count))
    }

    /// Full game details
    pub fn get_game_details(
        conn: &Connection,
        game_id_or_appid: i64,
    ) -> Result<Option<GameDetail>> {
        let game_opt = conn.query_row(
            "SELECT id, app_id, name, release_date, release_year, estimated_owners_raw,
                    owner_lower, owner_upper, owner_midpoint, price, peak_ccu, positive, negative,
                    recommendations, average_playtime_forever, median_playtime_forever,
                    header_image, metacritic_score, is_sparse, is_likely_playtest
             FROM games WHERE id = ?1 OR app_id = ?1",
            params![game_id_or_appid],
            |r| {
                let id: i64 = r.get(0)?;
                let app_id: i64 = r.get(1)?;
                Ok(GameDetail {
                    id,
                    app_id,
                    name: r.get(2)?,
                    release_date: r.get(3)?,
                    release_year: r.get(4)?,
                    estimated_owners_raw: r.get(5)?,
                    owner_lower: r.get(6)?,
                    owner_upper: r.get(7)?,
                    owner_midpoint: r.get(8)?,
                    price: r.get(9)?,
                    peak_ccu: r.get(10)?,
                    positive: r.get(11)?,
                    negative: r.get(12)?,
                    recommendations: r.get(13)?,
                    average_playtime_forever: r.get(14)?,
                    median_playtime_forever: r.get(15)?,
                    header_image: r.get(16)?,
                    metacritic_score: r.get(17)?,
                    is_sparse: r.get::<_, i64>(18)? == 1,
                    is_likely_playtest: r.get::<_, i64>(19)? == 1,
                    steam_store_url: format!("https://store.steampowered.com/app/{}", app_id),
                    developers: Vec::new(),
                    publishers: Vec::new(),
                    genres: Vec::new(),
                    related_games: Vec::new(),
                })
            },
        );

        let mut game = match game_opt {
            Ok(g) => g,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e),
        };

        // Credited developers
        let mut dev_stmt = conn.prepare(
            "SELECT d.id, d.canonical_name FROM developers d
             JOIN game_developers gd ON d.id = gd.developer_id
             WHERE gd.game_id = ?1",
        )?;
        game.developers = dev_stmt
            .query_map(params![game.id], |r| {
                Ok(EntityRef {
                    id: r.get(0)?,
                    name: r.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Credited publishers
        let mut pub_stmt = conn.prepare(
            "SELECT p.id, p.canonical_name FROM publishers p
             JOIN game_publishers gp ON p.id = gp.publisher_id
             WHERE gp.game_id = ?1",
        )?;
        game.publishers = pub_stmt
            .query_map(params![game.id], |r| {
                Ok(EntityRef {
                    id: r.get(0)?,
                    name: r.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Genres
        let mut gen_stmt = conn.prepare(
            "SELECT gen.name FROM genres gen
             JOIN game_genres gg ON gen.id = gg.genre_id
             WHERE gg.game_id = ?1",
        )?;
        game.genres = gen_stmt
            .query_map(params![game.id], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        // Related games by same developer
        if let Some(primary_dev) = game.developers.first() {
            let mut rel_stmt = conn.prepare(
                "SELECT g.id, g.app_id, g.name, g.release_year, g.owner_midpoint, g.header_image
                 FROM games g
                 JOIN game_developers gd ON g.id = gd.game_id
                 WHERE gd.developer_id = ?1 AND g.id != ?2
                 ORDER BY g.owner_midpoint DESC LIMIT 6",
            )?;
            let rel_games = rel_stmt.query_map(params![primary_dev.id, game.id], |r| {
                Ok(GameSummaryItem {
                    id: r.get(0)?,
                    app_id: r.get(1)?,
                    name: r.get(2)?,
                    release_date: None,
                    release_year: r.get(3)?,
                    estimated_owners_raw: None,
                    owner_lower: 0,
                    owner_upper: 0,
                    owner_midpoint: r.get(4)?,
                    price: 0.0,
                    header_image: r.get(5)?,
                    is_sparse: false,
                    is_likely_playtest: false,
                    developers: Vec::new(),
                    publishers: Vec::new(),
                    genres: Vec::new(),
                })
            })?;
            game.related_games = rel_games.filter_map(|r| r.ok()).collect();
        }

        Ok(Some(game))
    }

    /// List resolution candidates with game comparison snippets
    pub fn list_resolution_candidates(
        conn: &Connection,
        entity_type: Option<&str>,
        status: Option<&str>,
        min_similarity: Option<f64>,
        page: usize,
        page_size: usize,
    ) -> Result<(Vec<ResolutionCandidateItem>, i64)> {
        let mut where_clauses = Vec::new();

        if let Some(et) = entity_type {
            where_clauses.push(format!("c.entity_type = '{}'", et.replace('\'', "''")));
        }
        if let Some(st) = status {
            where_clauses.push(format!("c.status = '{}'", st.replace('\'', "''")));
        }
        if let Some(score) = min_similarity {
            where_clauses.push(format!("c.similarity_score >= {}", score));
        }

        let where_sql = if where_clauses.is_empty() {
            "1=1".to_string()
        } else {
            where_clauses.join(" AND ")
        };

        let total_count: i64 = conn.query_row(
            &format!(
                "SELECT COUNT(*) FROM entity_resolution_candidates c WHERE {}",
                where_sql
            ),
            [],
            |r| r.get(0),
        )?;

        let offset = (page.max(1) - 1) * page_size;
        let sql = format!(
            "SELECT c.id, c.entity_type, c.entity_a_id, c.entity_b_id, c.similarity_score, c.matching_explanation, c.status
             FROM entity_resolution_candidates c
             WHERE {}
             ORDER BY c.similarity_score DESC, c.id DESC
             LIMIT {} OFFSET {}",
            where_sql, page_size, offset
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
                r.get::<_, f64>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
            ))
        })?;

        let mut items = Vec::new();
        for r in rows.flatten() {
            let (cid, etype, a_id, b_id, score, exp, st) = r;
            let table = if etype == "developer" {
                "developers"
            } else {
                "publishers"
            };
            let rel_table = if etype == "developer" {
                "game_developers"
            } else {
                "game_publishers"
            };
            let fk_col = if etype == "developer" {
                "developer_id"
            } else {
                "publisher_id"
            };

            let (a_name, a_games): (String, i64) = conn
                .query_row(
                    &format!(
                        "SELECT canonical_name, game_count FROM {} WHERE id = ?1",
                        table
                    ),
                    params![a_id],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap_or_else(|_| ("Unknown".to_string(), 0));

            let (b_name, b_games): (String, i64) = conn
                .query_row(
                    &format!(
                        "SELECT canonical_name, game_count FROM {} WHERE id = ?1",
                        table
                    ),
                    params![b_id],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap_or_else(|_| ("Unknown".to_string(), 0));

            // Fetch top 3 example games for A and B
            let mut g_a_stmt = conn.prepare(&format!(
                "SELECT g.name FROM games g JOIN {} rel ON g.id = rel.game_id WHERE rel.{} = ?1 LIMIT 3",
                rel_table, fk_col
            ))?;
            let games_a: Vec<String> = g_a_stmt
                .query_map(params![a_id], |row| row.get(0))?
                .filter_map(|res| res.ok())
                .collect();

            let mut g_b_stmt = conn.prepare(&format!(
                "SELECT g.name FROM games g JOIN {} rel ON g.id = rel.game_id WHERE rel.{} = ?1 LIMIT 3",
                rel_table, fk_col
            ))?;
            let games_b: Vec<String> = g_b_stmt
                .query_map(params![b_id], |row| row.get(0))?
                .filter_map(|res| res.ok())
                .collect();

            items.push(ResolutionCandidateItem {
                id: cid,
                entity_type: etype,
                entity_a_id: a_id,
                entity_a_name: a_name,
                entity_a_games: a_games,
                entity_b_id: b_id,
                entity_b_name: b_name,
                entity_b_games: b_games,
                similarity_score: score,
                matching_explanation: exp,
                status: st,
                example_games_a: games_a,
                example_games_b: games_b,
            });
        }

        Ok((items, total_count))
    }

    /// Entity resolution report & audit trail
    pub fn get_resolution_report(conn: &Connection) -> Result<ResolutionReport> {
        let total_observed_names: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT raw_name) FROM entity_aliases",
            [],
            |r| r.get(0),
        )?;
        let total_canonical_developers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM developers WHERE is_merged = 0",
            [],
            |r| r.get(0),
        )?;
        let total_canonical_publishers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM publishers WHERE is_merged = 0",
            [],
            |r| r.get(0),
        )?;
        let auto_resolved_aliases: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entity_aliases WHERE matching_method = 'exact_normalized'",
            [],
            |r| r.get(0),
        )?;
        let manual_confirmed_aliases: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entity_aliases WHERE matching_method = 'manual_assignment'",
            [],
            |r| r.get(0),
        )?;
        let outstanding_candidates: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entity_resolution_candidates WHERE status = 'pending'",
            [],
            |r| r.get(0),
        )?;
        let rejected_candidates: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entity_resolution_candidates WHERE status IN ('rejected', 'distinct')",
            [],
            |r| r.get(0),
        )?;
        let merged_entities_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entity_merge_events WHERE is_undone = 0",
            [],
            |r| r.get(0),
        )?;

        Ok(ResolutionReport {
            total_observed_names,
            total_canonical_developers,
            total_canonical_publishers,
            auto_resolved_aliases,
            manual_confirmed_aliases,
            outstanding_candidates,
            rejected_candidates,
            merged_entities_count,
        })
    }

    /// Get list of merge events (history for audit and undo)
    pub fn get_merge_history(conn: &Connection, limit: usize) -> Result<Vec<MergeHistoryItem>> {
        let mut stmt = conn.prepare(
            "SELECT id, entity_type, source_entity_id, target_entity_id,
                    source_canonical_name, target_canonical_name, reason, created_at, is_undone, undone_at
             FROM entity_merge_events
             ORDER BY id DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |r| {
            Ok(MergeHistoryItem {
                id: r.get(0)?,
                entity_type: r.get(1)?,
                source_entity_id: r.get(2)?,
                target_entity_id: r.get(3)?,
                source_canonical_name: r.get(4)?,
                target_canonical_name: r.get(5)?,
                reason: r.get(6)?,
                created_at: r.get(7)?,
                is_undone: r.get::<_, i64>(8)? == 1,
                undone_at: r.get(9)?,
            })
        })?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }
}
