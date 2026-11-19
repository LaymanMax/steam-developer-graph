use crate::graph::model::{
    CollaborationGameItem, DeveloperCollaboration, PublisherDeveloperRelation,
};
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalYearActivity {
    pub year: i32,
    pub games_released: i64,
    pub active_developers: i64,
    pub active_publishers: i64,
    pub new_developer_collaborations: i64,
    pub new_publisher_relationships: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreYearTrend {
    pub year: i32,
    pub genre: String,
    pub game_count: i64,
}

pub struct GraphMetricsService;

impl GraphMetricsService {
    /// Retrieve developer collaboration rankings.
    pub fn get_top_collaborations(
        conn: &Connection,
        min_shared_games: i64,
        genre: Option<&str>,
        min_year: Option<i32>,
        max_year: Option<i32>,
        limit: usize,
    ) -> Result<Vec<DeveloperCollaboration>> {
        let mut sql = String::from(
            "SELECT gd1.developer_id, d1.canonical_name, 
                    gd2.developer_id, d2.canonical_name,
                    COUNT(DISTINCT gd1.game_id) as shared_cnt,
                    MIN(g.release_year) as first_yr,
                    MAX(g.release_year) as latest_yr
             FROM game_developers gd1
             JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id < gd2.developer_id
             JOIN developers d1 ON gd1.developer_id = d1.id
             JOIN developers d2 ON gd2.developer_id = d2.id
             JOIN games g ON gd1.game_id = g.id
             WHERE d1.is_merged = 0 AND d2.is_merged = 0",
        );

        if let Some(g) = genre {
            sql.push_str(&format!(
                " AND EXISTS (SELECT 1 FROM game_genres gg JOIN genres gen ON gg.genre_id = gen.id WHERE gg.game_id = g.id AND gen.name = '{}')",
                g.replace('\'', "''")
            ));
        }
        if let Some(min_y) = min_year {
            sql.push_str(&format!(" AND g.release_year >= {}", min_y));
        }
        if let Some(max_y) = max_year {
            sql.push_str(&format!(" AND g.release_year <= {}", max_y));
        }

        sql.push_str(&format!(
            " GROUP BY gd1.developer_id, gd2.developer_id
              HAVING shared_cnt >= ?1
              ORDER BY shared_cnt DESC, latest_yr DESC LIMIT ?2"
        ));

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![min_shared_games, limit as i64], |r| {
            let dev1_id: i64 = r.get(0)?;
            let dev1_name: String = r.get(1)?;
            let dev2_id: i64 = r.get(2)?;
            let dev2_name: String = r.get(3)?;
            let shared_games_count: i64 = r.get(4)?;
            let first_year: Option<i32> = r.get(5)?;
            let latest_year: Option<i32> = r.get(6)?;

            Ok(DeveloperCollaboration {
                dev1_id,
                dev1_name,
                dev2_id,
                dev2_name,
                shared_games_count,
                first_year,
                latest_year,
                shared_games: Vec::new(),
            })
        })?;

        let mut results = Vec::new();
        for res in rows.flatten() {
            results.push(res);
        }

        // Hydrate top pairs with supporting shared titles
        for item in results.iter_mut().take(20) {
            let mut g_stmt = conn.prepare(
                "SELECT g.id, g.app_id, g.name, g.release_year
                 FROM games g
                 JOIN game_developers gd1 ON g.id = gd1.game_id AND gd1.developer_id = ?1
                 JOIN game_developers gd2 ON g.id = gd2.game_id AND gd2.developer_id = ?2
                 ORDER BY g.release_year DESC LIMIT 10",
            )?;

            let games = g_stmt.query_map(params![item.dev1_id, item.dev2_id], |r| {
                Ok(CollaborationGameItem {
                    game_id: r.get(0)?,
                    app_id: r.get(1)?,
                    name: r.get(2)?,
                    release_year: r.get(3)?,
                })
            })?;
            item.shared_games = games.filter_map(|g| g.ok()).collect();
        }

        Ok(results)
    }

    /// Retrieve full details and games for a developer pair
    pub fn get_developer_pair_details(
        conn: &Connection,
        dev1_id: i64,
        dev2_id: i64,
    ) -> Result<Option<DeveloperCollaboration>> {
        let (min_id, max_id) = if dev1_id < dev2_id {
            (dev1_id, dev2_id)
        } else {
            (dev2_id, dev1_id)
        };

        let row_opt = conn.query_row(
            "SELECT d1.canonical_name, d2.canonical_name,
                    COUNT(DISTINCT gd1.game_id) as shared_cnt,
                    MIN(g.release_year) as first_yr,
                    MAX(g.release_year) as latest_yr
             FROM game_developers gd1
             JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id = ?1 AND gd2.developer_id = ?2
             JOIN developers d1 ON d1.id = ?1
             JOIN developers d2 ON d2.id = ?2
             JOIN games g ON gd1.game_id = g.id
             GROUP BY d1.id, d2.id",
            params![min_id, max_id],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                    r.get::<_, Option<i32>>(3)?,
                    r.get::<_, Option<i32>>(4)?,
                ))
            },
        );

        match row_opt {
            Ok((d1_name, d2_name, count, first_y, latest_y)) => {
                let mut g_stmt = conn.prepare(
                    "SELECT g.id, g.app_id, g.name, g.release_year
                     FROM games g
                     JOIN game_developers gd1 ON g.id = gd1.game_id AND gd1.developer_id = ?1
                     JOIN game_developers gd2 ON g.id = gd2.game_id AND gd2.developer_id = ?2
                     ORDER BY g.release_year DESC",
                )?;

                let shared_games: Vec<CollaborationGameItem> = g_stmt
                    .query_map(params![min_id, max_id], |r| {
                        Ok(CollaborationGameItem {
                            game_id: r.get(0)?,
                            app_id: r.get(1)?,
                            name: r.get(2)?,
                            release_year: r.get(3)?,
                        })
                    })?
                    .filter_map(|g| g.ok())
                    .collect();

                Ok(Some(DeveloperCollaboration {
                    dev1_id: min_id,
                    dev1_name: d1_name,
                    dev2_id: max_id,
                    dev2_name: d2_name,
                    shared_games_count: count,
                    first_year: first_y,
                    latest_year: latest_y,
                    shared_games,
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Retrieve publisher-developer relationships
    pub fn get_top_publisher_developer_relations(
        conn: &Connection,
        min_shared: i64,
        limit: usize,
    ) -> Result<Vec<PublisherDeveloperRelation>> {
        let mut stmt = conn.prepare(
            "SELECT gp.publisher_id, p.canonical_name,
                    gd.developer_id, d.canonical_name,
                    COUNT(DISTINCT gp.game_id) as shared_cnt,
                    MIN(g.release_year) as first_yr,
                    MAX(g.release_year) as latest_yr
             FROM game_publishers gp
             JOIN game_developers gd ON gp.game_id = gd.game_id
             JOIN publishers p ON gp.publisher_id = p.id
             JOIN developers d ON gd.developer_id = d.id
             JOIN games g ON gp.game_id = g.id
             WHERE p.is_merged = 0 AND d.is_merged = 0
             GROUP BY gp.publisher_id, gd.developer_id
             HAVING shared_cnt >= ?1
             ORDER BY shared_cnt DESC LIMIT ?2",
        )?;

        let rows = stmt.query_map(params![min_shared, limit as i64], |r| {
            Ok(PublisherDeveloperRelation {
                publisher_id: r.get(0)?,
                publisher_name: r.get(1)?,
                developer_id: r.get(2)?,
                developer_name: r.get(3)?,
                shared_games_count: r.get(4)?,
                first_year: r.get(5)?,
                latest_year: r.get(6)?,
                shared_games: Vec::new(),
                genres: Vec::new(),
            })
        })?;

        let mut results = Vec::new();
        for r in rows.flatten() {
            results.push(r);
        }

        for item in results.iter_mut().take(15) {
            let mut g_stmt = conn.prepare(
                "SELECT g.id, g.app_id, g.name, g.release_year
                 FROM games g
                 JOIN game_publishers gp ON g.id = gp.game_id AND gp.publisher_id = ?1
                 JOIN game_developers gd ON g.id = gd.game_id AND gd.developer_id = ?2
                 ORDER BY g.release_year DESC LIMIT 10",
            )?;
            let games = g_stmt.query_map(params![item.publisher_id, item.developer_id], |r| {
                Ok(CollaborationGameItem {
                    game_id: r.get(0)?,
                    app_id: r.get(1)?,
                    name: r.get(2)?,
                    release_year: r.get(3)?,
                })
            })?;
            item.shared_games = games.filter_map(|g| g.ok()).collect();
        }

        Ok(results)
    }

    /// Historical timeline activity: releases, active devs, active pubs per year
    pub fn get_historical_timeline(conn: &Connection) -> Result<Vec<HistoricalYearActivity>> {
        let mut stmt = conn.prepare(
            "SELECT g.release_year,
                    COUNT(DISTINCT g.id) as game_cnt,
                    COUNT(DISTINCT gd.developer_id) as dev_cnt,
                    COUNT(DISTINCT gp.publisher_id) as pub_cnt
             FROM games g
             LEFT JOIN game_developers gd ON g.id = gd.game_id
             LEFT JOIN game_publishers gp ON g.id = gp.game_id
             WHERE g.release_year IS NOT NULL AND g.release_year >= 1990 AND g.release_year <= 2026
             GROUP BY g.release_year
             ORDER BY g.release_year ASC",
        )?;

        let rows = stmt.query_map([], |r| {
            Ok(HistoricalYearActivity {
                year: r.get(0)?,
                games_released: r.get(1)?,
                active_developers: r.get(2)?,
                active_publishers: r.get(3)?,
                new_developer_collaborations: 0,
                new_publisher_relationships: 0,
            })
        })?;

        let mut timeline: Vec<HistoricalYearActivity> = rows.filter_map(|r| r.ok()).collect();

        // Calculate new collaborations per year (first shared release year)
        let mut collab_stmt = conn.prepare(
            "SELECT first_yr, COUNT(*) FROM (
                SELECT MIN(g.release_year) as first_yr
                FROM game_developers gd1
                JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id < gd2.developer_id
                JOIN games g ON gd1.game_id = g.id
                WHERE g.release_year IS NOT NULL
                GROUP BY gd1.developer_id, gd2.developer_id
             ) WHERE first_yr >= 1990 AND first_yr <= 2026
             GROUP BY first_yr",
        )?;
        let collab_map: HashMap<i32, i64> = collab_stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();

        // Calculate new pub relationships per year
        let mut pub_rel_stmt = conn.prepare(
            "SELECT first_yr, COUNT(*) FROM (
                SELECT MIN(g.release_year) as first_yr
                FROM game_publishers gp
                JOIN game_developers gd ON gp.game_id = gd.game_id
                JOIN games g ON gp.game_id = g.id
                WHERE g.release_year IS NOT NULL
                GROUP BY gp.publisher_id, gd.developer_id
             ) WHERE first_yr >= 1990 AND first_yr <= 2026
             GROUP BY first_yr",
        )?;
        let pub_rel_map: HashMap<i32, i64> = pub_rel_stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();

        for item in &mut timeline {
            if let Some(&cnt) = collab_map.get(&item.year) {
                item.new_developer_collaborations = cnt;
            }
            if let Some(&cnt) = pub_rel_map.get(&item.year) {
                item.new_publisher_relationships = cnt;
            }
        }

        Ok(timeline)
    }

    /// Genre trends over time
    pub fn get_genre_growth(conn: &Connection, top_n_genres: usize) -> Result<Vec<GenreYearTrend>> {
        let top_genres: Vec<String> = {
            let mut stmt =
                conn.prepare("SELECT name FROM genres ORDER BY game_count DESC LIMIT ?1")?;
            stmt.query_map(params![top_n_genres as i64], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect()
        };

        if top_genres.is_empty() {
            return Ok(Vec::new());
        }

        let quoted_genres: Vec<String> = top_genres
            .iter()
            .map(|g| format!("'{}'", g.replace('\'', "''")))
            .collect();

        let sql = format!(
            "SELECT g.release_year, gen.name, COUNT(DISTINCT g.id)
             FROM games g
             JOIN game_genres gg ON g.id = gg.game_id
             JOIN genres gen ON gg.genre_id = gen.id
             WHERE gen.name IN ({}) AND g.release_year >= 2005 AND g.release_year <= 2026
             GROUP BY g.release_year, gen.name
             ORDER BY g.release_year ASC, gen.name ASC",
            quoted_genres.join(",")
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |r| {
            Ok(GenreYearTrend {
                year: r.get(0)?,
                genre: r.get(1)?,
                game_count: r.get(2)?,
            })
        })?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }
}
