use crate::db::queries::DbQueries;
use crate::graph::metrics::GraphMetricsService;
use crate::graph::model::Subgraph;
use rusqlite::Connection;

pub struct ExportService;

impl ExportService {
    pub fn export_developers_csv(
        conn: &Connection,
        search: Option<&str>,
        genre: Option<&str>,
        min_games: Option<i64>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (items, _) = DbQueries::list_developers(
            conn,
            search,
            genre,
            min_games,
            Some("game_count"),
            true,
            1,
            5000,
        )?;

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "Developer ID",
            "Canonical Name",
            "Game Count",
            "Publisher Count",
            "Genre Count",
            "Collaborator Count",
            "Earliest Release",
            "Latest Release",
            "Total Estimated Midpoint Owners",
        ])?;

        for d in items {
            wtr.write_record(&[
                d.id.to_string(),
                d.canonical_name,
                d.game_count.to_string(),
                d.publisher_count.to_string(),
                d.genre_count.to_string(),
                d.collaborator_count.to_string(),
                d.earliest_release_year
                    .map_or("".to_string(), |y| y.to_string()),
                d.latest_release_year
                    .map_or("".to_string(), |y| y.to_string()),
                d.total_owner_midpoint.to_string(),
            ])?;
        }

        let data = String::from_utf8(wtr.into_inner()?)?;
        Ok(data)
    }

    pub fn export_publishers_csv(
        conn: &Connection,
        search: Option<&str>,
        genre: Option<&str>,
        min_games: Option<i64>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (items, _) = DbQueries::list_publishers(
            conn,
            search,
            genre,
            min_games,
            Some("game_count"),
            true,
            1,
            5000,
        )?;

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "Publisher ID",
            "Canonical Name",
            "Game Count",
            "Developer Count",
            "Genre Count",
            "Earliest Release",
            "Latest Release",
            "Total Estimated Midpoint Owners",
            "Genre Diversity (Shannon Entropy)",
        ])?;

        for p in items {
            wtr.write_record(&[
                p.id.to_string(),
                p.canonical_name,
                p.game_count.to_string(),
                p.developer_count.to_string(),
                p.genre_count.to_string(),
                p.earliest_release_year
                    .map_or("".to_string(), |y| y.to_string()),
                p.latest_release_year
                    .map_or("".to_string(), |y| y.to_string()),
                p.total_owner_midpoint.to_string(),
                format!("{:.4}", p.shannon_entropy),
            ])?;
        }

        let data = String::from_utf8(wtr.into_inner()?)?;
        Ok(data)
    }

    pub fn export_collaborations_csv(
        conn: &Connection,
        min_shared: i64,
        genre: Option<&str>,
        min_year: Option<i32>,
        max_year: Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let collabs = GraphMetricsService::get_top_collaborations(
            conn, min_shared, genre, min_year, max_year, 5000,
        )?;

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "Developer 1 ID",
            "Developer 1 Name",
            "Developer 2 ID",
            "Developer 2 Name",
            "Shared Games Count",
            "First Shared Year",
            "Latest Shared Year",
        ])?;

        for c in collabs {
            wtr.write_record(&[
                c.dev1_id.to_string(),
                c.dev1_name,
                c.dev2_id.to_string(),
                c.dev2_name,
                c.shared_games_count.to_string(),
                c.first_year.map_or("".to_string(), |y| y.to_string()),
                c.latest_year.map_or("".to_string(), |y| y.to_string()),
            ])?;
        }

        let data = String::from_utf8(wtr.into_inner()?)?;
        Ok(data)
    }

    pub fn export_graph_json(subgraph: &Subgraph) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(subgraph)?;
        Ok(json)
    }
}
