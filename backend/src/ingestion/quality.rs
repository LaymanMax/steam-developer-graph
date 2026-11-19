use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityReport {
    pub total_games: i64,
    pub sparse_records: i64,
    pub likely_playtests: i64,
    pub games_with_complete_credits: i64,
    pub missing_developers: i64,
    pub missing_publishers: i64,
    pub missing_genres: i64,
    pub missing_release_dates: i64,
    pub issue_breakdown: Vec<IssueCount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCount {
    pub issue_type: String,
    pub severity: String,
    pub count: i64,
}

pub struct QualityTracker;

impl QualityTracker {
    pub fn record_issue(
        conn: &Connection,
        run_id: Option<i64>,
        app_id: i64,
        game_name: &str,
        issue_type: &str,
        severity: &str,
        details: &str,
    ) -> Result<()> {
        conn.execute(
            "INSERT INTO data_quality_issues 
             (ingestion_run_id, app_id, game_name, issue_type, severity, details, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))",
            params![run_id, app_id, game_name, issue_type, severity, details],
        )?;
        Ok(())
    }

    pub fn generate_report(conn: &Connection) -> Result<QualityReport> {
        let total_games: i64 = conn.query_row("SELECT COUNT(*) FROM games", [], |r| r.get(0))?;
        let sparse_records: i64 =
            conn.query_row("SELECT COUNT(*) FROM games WHERE is_sparse = 1", [], |r| {
                r.get(0)
            })?;
        let likely_playtests: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games WHERE is_likely_playtest = 1",
            [],
            |r| r.get(0),
        )?;

        let games_with_complete_credits: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT g.id) 
             FROM games g
             WHERE EXISTS (SELECT 1 FROM game_developers gd WHERE gd.game_id = g.id)
               AND EXISTS (SELECT 1 FROM game_publishers gp WHERE gp.game_id = g.id)
               AND EXISTS (SELECT 1 FROM game_genres gg WHERE gg.game_id = g.id)
               AND g.release_year IS NOT NULL",
            [],
            |r| r.get(0),
        )?;

        let missing_developers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games g WHERE NOT EXISTS (SELECT 1 FROM game_developers gd WHERE gd.game_id = g.id)",
            [],
            |r| r.get(0),
        )?;

        let missing_publishers: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games g WHERE NOT EXISTS (SELECT 1 FROM game_publishers gp WHERE gp.game_id = g.id)",
            [],
            |r| r.get(0),
        )?;

        let missing_genres: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games g WHERE NOT EXISTS (SELECT 1 FROM game_genres gg WHERE gg.game_id = g.id)",
            [],
            |r| r.get(0),
        )?;

        let missing_release_dates: i64 = conn.query_row(
            "SELECT COUNT(*) FROM games WHERE release_year IS NULL",
            [],
            |r| r.get(0),
        )?;

        let mut stmt = conn.prepare(
            "SELECT issue_type, severity, COUNT(*) as cnt 
             FROM data_quality_issues 
             GROUP BY issue_type, severity 
             ORDER BY cnt DESC",
        )?;

        let issue_breakdown = stmt
            .query_map([], |r| {
                Ok(IssueCount {
                    issue_type: r.get(0)?,
                    severity: r.get(1)?,
                    count: r.get(2)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(QualityReport {
            total_games,
            sparse_records,
            likely_playtests,
            games_with_complete_credits,
            missing_developers,
            missing_publishers,
            missing_genres,
            missing_release_dates,
            issue_breakdown,
        })
    }
}
