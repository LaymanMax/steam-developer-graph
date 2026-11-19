use crate::graph::model::{GraphLink, GraphNode, NodeType, Subgraph};
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphFilterParams {
    #[serde(default)]
    pub min_year: Option<i32>,
    #[serde(default)]
    pub max_year: Option<i32>,
    #[serde(default)]
    pub genre: Option<String>,
    #[serde(default)]
    pub min_collaborations: Option<i64>,
    #[serde(default)]
    pub exclude_sparse: bool,
    #[serde(default)]
    pub exclude_playtests: bool,
    #[serde(default)]
    pub limit: Option<usize>,
}

pub struct GraphQueryService;

impl GraphQueryService {
    /// Returns the focused graph neighborhood around a center entity.
    pub fn get_neighborhood(
        conn: &Connection,
        node_type: NodeType,
        entity_id: i64,
        depth: usize,
        filters: &GraphFilterParams,
    ) -> Result<Subgraph> {
        let max_nodes = filters.limit.unwrap_or(80);
        let mut nodes: HashMap<String, GraphNode> = HashMap::new();
        let mut links: Vec<GraphLink> = Vec::new();
        let mut visited_entities: HashSet<(NodeType, i64)> = HashSet::new();

        let center_node_id = match node_type {
            NodeType::Developer => format!("dev_{}", entity_id),
            NodeType::Publisher => format!("pub_{}", entity_id),
            NodeType::Game => format!("game_{}", entity_id),
        };

        // Add center node
        match node_type {
            NodeType::Developer => {
                if let Ok(dev) = Self::fetch_developer_node(conn, entity_id) {
                    nodes.insert(center_node_id.clone(), dev);
                    visited_entities.insert((NodeType::Developer, entity_id));
                }
            }
            NodeType::Publisher => {
                if let Ok(pub_node) = Self::fetch_publisher_node(conn, entity_id) {
                    nodes.insert(center_node_id.clone(), pub_node);
                    visited_entities.insert((NodeType::Publisher, entity_id));
                }
            }
            NodeType::Game => {
                if let Ok(game_node) = Self::fetch_game_node(conn, entity_id) {
                    nodes.insert(center_node_id.clone(), game_node);
                    visited_entities.insert((NodeType::Game, entity_id));
                }
            }
        }

        // Expand 1st degree
        match node_type {
            NodeType::Developer => {
                Self::expand_developer(
                    conn,
                    entity_id,
                    &mut nodes,
                    &mut links,
                    &mut visited_entities,
                    filters,
                    max_nodes,
                )?;
            }
            NodeType::Publisher => {
                Self::expand_publisher(
                    conn,
                    entity_id,
                    &mut nodes,
                    &mut links,
                    &mut visited_entities,
                    filters,
                    max_nodes,
                )?;
            }
            NodeType::Game => {
                Self::expand_game(
                    conn,
                    entity_id,
                    &mut nodes,
                    &mut links,
                    &mut visited_entities,
                    filters,
                    max_nodes,
                )?;
            }
        }

        // If depth == 2 and remaining node capacity
        if depth > 1 && nodes.len() < max_nodes {
            let current_devs: Vec<i64> = nodes
                .values()
                .filter(|n| n.node_type == NodeType::Developer && n.entity_id != entity_id)
                .map(|n| n.entity_id)
                .take(3)
                .collect();

            for dev_id in current_devs {
                if nodes.len() >= max_nodes {
                    break;
                }
                let _ = Self::expand_developer(
                    conn,
                    dev_id,
                    &mut nodes,
                    &mut links,
                    &mut visited_entities,
                    filters,
                    max_nodes,
                );
            }
        }

        let is_truncated = nodes.len() >= max_nodes;
        let node_list: Vec<GraphNode> = nodes.into_values().collect();
        let total_nodes = node_list.len();
        let total_links = links.len();

        Ok(Subgraph {
            nodes: node_list,
            links,
            total_nodes,
            total_links,
            is_truncated,
            center_node_id: Some(center_node_id),
        })
    }

    /// Overview ecosystem subgraph for the Network Explorer landing view
    pub fn get_ecosystem_overview(
        conn: &Connection,
        filters: &GraphFilterParams,
    ) -> Result<Subgraph> {
        let max_nodes = filters.limit.unwrap_or(120);
        let mut nodes: HashMap<String, GraphNode> = HashMap::new();
        let mut links: Vec<GraphLink> = Vec::new();
        let mut visited: HashSet<(NodeType, i64)> = HashSet::new();

        // Top 20 developers by game count
        let mut dev_stmt = conn.prepare(
            "SELECT id, canonical_name, game_count, publisher_count, earliest_release_year, latest_release_year, total_owner_midpoint
             FROM developers 
             WHERE is_merged = 0 AND game_count > 0
             ORDER BY game_count DESC LIMIT 25",
        )?;
        let top_devs = dev_stmt.query_map([], |r| {
            Ok(GraphNode {
                id: format!("dev_{}", r.get::<_, i64>(0)?),
                entity_id: r.get(0)?,
                node_type: NodeType::Developer,
                label: r.get(1)?,
                sub_label: Some(format!("{} games", r.get::<_, i64>(2)?)),
                size: (r.get::<_, i64>(2)? as f64).sqrt().max(8.0).min(32.0),
                metric: r.get(2)?,
                year: r.get(5)?,
                metadata: None,
            })
        })?;

        for d in top_devs.flatten() {
            visited.insert((NodeType::Developer, d.entity_id));
            nodes.insert(d.id.clone(), d);
        }

        // Top 15 publishers
        let mut pub_stmt = conn.prepare(
            "SELECT id, canonical_name, game_count, developer_count, earliest_release_year, latest_release_year, total_owner_midpoint
             FROM publishers 
             WHERE is_merged = 0 AND game_count > 0
             ORDER BY game_count DESC LIMIT 15",
        )?;
        let top_pubs = pub_stmt.query_map([], |r| {
            Ok(GraphNode {
                id: format!("pub_{}", r.get::<_, i64>(0)?),
                entity_id: r.get(0)?,
                node_type: NodeType::Publisher,
                label: r.get(1)?,
                sub_label: Some(format!(
                    "{} devs, {} games",
                    r.get::<_, i64>(3)?,
                    r.get::<_, i64>(2)?
                )),
                size: (r.get::<_, i64>(2)? as f64).sqrt().max(10.0).min(35.0),
                metric: r.get(2)?,
                year: r.get(5)?,
                metadata: None,
            })
        })?;

        for p in top_pubs.flatten() {
            visited.insert((NodeType::Publisher, p.entity_id));
            nodes.insert(p.id.clone(), p);
        }

        // Prominent shared games connecting these devs and pubs
        let dev_ids: Vec<String> = nodes
            .values()
            .filter(|n| n.node_type == NodeType::Developer)
            .map(|n| n.entity_id.to_string())
            .collect();
        let pub_ids: Vec<String> = nodes
            .values()
            .filter(|n| n.node_type == NodeType::Publisher)
            .map(|n| n.entity_id.to_string())
            .collect();

        if !dev_ids.is_empty() && !pub_ids.is_empty() {
            let sql = format!(
                "SELECT g.id, g.app_id, g.name, g.release_year, g.owner_midpoint, gd.developer_id, gp.publisher_id
                 FROM games g
                 JOIN game_developers gd ON g.id = gd.game_id
                 JOIN game_publishers gp ON g.id = gp.game_id
                 WHERE gd.developer_id IN ({}) AND gp.publisher_id IN ({})
                 ORDER BY g.owner_midpoint DESC LIMIT 40",
                dev_ids.join(","),
                pub_ids.join(",")
            );

            let mut game_stmt = conn.prepare(&sql)?;
            let rows = game_stmt.query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, i64>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, Option<i32>>(3)?,
                    r.get::<_, i64>(4)?,
                    r.get::<_, i64>(5)?,
                    r.get::<_, i64>(6)?,
                ))
            })?;

            for r in rows.flatten() {
                let (gid, _app_id, name, year, midpoint, dev_id, pub_id) = r;
                let game_node_id = format!("game_{}", gid);

                if !nodes.contains_key(&game_node_id) && nodes.len() < max_nodes {
                    nodes.insert(
                        game_node_id.clone(),
                        GraphNode {
                            id: game_node_id.clone(),
                            entity_id: gid,
                            node_type: NodeType::Game,
                            label: name.clone(),
                            sub_label: year.map(|y| format!("{}", y)),
                            size: (midpoint as f64).log10().max(5.0).min(18.0),
                            metric: midpoint,
                            year,
                            metadata: None,
                        },
                    );
                }

                // Dev -> Game link
                let dev_node_id = format!("dev_{}", dev_id);
                if nodes.contains_key(&dev_node_id) && nodes.contains_key(&game_node_id) {
                    links.push(GraphLink {
                        source: dev_node_id,
                        target: game_node_id.clone(),
                        relationship: "developed".to_string(),
                        weight: 1.0,
                        shared_game_count: 1,
                        shared_games: vec![name.clone()],
                    });
                }

                // Pub -> Game link
                let pub_node_id = format!("pub_{}", pub_id);
                if nodes.contains_key(&pub_node_id) && nodes.contains_key(&game_node_id) {
                    links.push(GraphLink {
                        source: pub_node_id,
                        target: game_node_id,
                        relationship: "published".to_string(),
                        weight: 1.0,
                        shared_game_count: 1,
                        shared_games: vec![name],
                    });
                }
            }
        }

        let node_list: Vec<GraphNode> = nodes.into_values().collect();
        let total_nodes = node_list.len();
        let total_links = links.len();

        Ok(Subgraph {
            nodes: node_list,
            links,
            total_nodes,
            total_links,
            is_truncated: total_nodes >= max_nodes,
            center_node_id: None,
        })
    }

    fn expand_developer(
        conn: &Connection,
        dev_id: i64,
        nodes: &mut HashMap<String, GraphNode>,
        links: &mut Vec<GraphLink>,
        visited: &mut HashSet<(NodeType, i64)>,
        filters: &GraphFilterParams,
        max_nodes: usize,
    ) -> Result<()> {
        let dev_node_id = format!("dev_{}", dev_id);

        // Fetch games associated with developer
        let mut query = String::from(
            "SELECT g.id, g.app_id, g.name, g.release_year, g.owner_midpoint, g.is_sparse, g.is_likely_playtest
             FROM games g
             JOIN game_developers gd ON g.id = gd.game_id
             WHERE gd.developer_id = ?1",
        );

        if filters.exclude_sparse {
            query.push_str(" AND g.is_sparse = 0");
        }
        if filters.exclude_playtests {
            query.push_str(" AND g.is_likely_playtest = 0");
        }
        if let Some(min_y) = filters.min_year {
            query.push_str(&format!(" AND g.release_year >= {}", min_y));
        }
        if let Some(max_y) = filters.max_year {
            query.push_str(&format!(" AND g.release_year <= {}", max_y));
        }
        query.push_str(" ORDER BY g.owner_midpoint DESC, g.release_year DESC LIMIT 25");

        let mut stmt = conn.prepare(&query)?;
        let games = stmt.query_map(params![dev_id], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<i32>>(3)?,
                r.get::<_, i64>(4)?,
            ))
        })?;

        for g in games.flatten() {
            if nodes.len() >= max_nodes {
                break;
            }
            let (gid, _appid, name, year, midpoint) = g;
            let game_node_id = format!("game_{}", gid);

            if !nodes.contains_key(&game_node_id) {
                nodes.insert(
                    game_node_id.clone(),
                    GraphNode {
                        id: game_node_id.clone(),
                        entity_id: gid,
                        node_type: NodeType::Game,
                        label: name.clone(),
                        sub_label: year.map(|y| format!("{}", y)),
                        size: (midpoint as f64).log10().max(6.0).min(20.0),
                        metric: midpoint,
                        year,
                        metadata: None,
                    },
                );
                visited.insert((NodeType::Game, gid));
            }

            links.push(GraphLink {
                source: dev_node_id.clone(),
                target: game_node_id.clone(),
                relationship: "developed".to_string(),
                weight: 1.0,
                shared_game_count: 1,
                shared_games: vec![name.clone()],
            });

            // Find publishers of this game
            let mut pub_stmt = conn.prepare(
                "SELECT p.id, p.canonical_name, p.game_count 
                 FROM publishers p 
                 JOIN game_publishers gp ON p.id = gp.publisher_id 
                 WHERE gp.game_id = ?1 AND p.is_merged = 0 LIMIT 3",
            )?;
            let pubs = pub_stmt.query_map(params![gid], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                ))
            })?;

            for p in pubs.flatten() {
                let (pub_id, pub_name, pub_game_count) = p;
                let pub_node_id = format!("pub_{}", pub_id);

                if !nodes.contains_key(&pub_node_id) && nodes.len() < max_nodes {
                    nodes.insert(
                        pub_node_id.clone(),
                        GraphNode {
                            id: pub_node_id.clone(),
                            entity_id: pub_id,
                            node_type: NodeType::Publisher,
                            label: pub_name,
                            sub_label: Some(format!("{} games", pub_game_count)),
                            size: (pub_game_count as f64).sqrt().max(8.0).min(28.0),
                            metric: pub_game_count,
                            year: None,
                            metadata: None,
                        },
                    );
                    visited.insert((NodeType::Publisher, pub_id));
                }

                if nodes.contains_key(&pub_node_id) {
                    links.push(GraphLink {
                        source: pub_node_id,
                        target: game_node_id.clone(),
                        relationship: "published".to_string(),
                        weight: 1.0,
                        shared_game_count: 1,
                        shared_games: vec![name.clone()],
                    });
                }
            }
        }

        // Collaborating developers
        let mut collab_stmt = conn.prepare(
            "SELECT d.id, d.canonical_name, d.game_count, COUNT(DISTINCT gd1.game_id) as shared_cnt
             FROM game_developers gd1
             JOIN game_developers gd2 ON gd1.game_id = gd2.game_id AND gd1.developer_id != gd2.developer_id
             JOIN developers d ON gd2.developer_id = d.id
             WHERE gd1.developer_id = ?1 AND d.is_merged = 0
             GROUP BY d.id
             ORDER BY shared_cnt DESC LIMIT 8",
        )?;
        let collabs = collab_stmt.query_map(params![dev_id], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })?;

        for c in collabs.flatten() {
            let (other_dev_id, other_name, other_games, shared_count) = c;
            let other_node_id = format!("dev_{}", other_dev_id);

            if !nodes.contains_key(&other_node_id) && nodes.len() < max_nodes {
                nodes.insert(
                    other_node_id.clone(),
                    GraphNode {
                        id: other_node_id.clone(),
                        entity_id: other_dev_id,
                        node_type: NodeType::Developer,
                        label: other_name,
                        sub_label: Some(format!("{} games", other_games)),
                        size: (other_games as f64).sqrt().max(7.0).min(25.0),
                        metric: other_games,
                        year: None,
                        metadata: None,
                    },
                );
                visited.insert((NodeType::Developer, other_dev_id));
            }

            if nodes.contains_key(&other_node_id) {
                links.push(GraphLink {
                    source: dev_node_id.clone(),
                    target: other_node_id,
                    relationship: "collaborated".to_string(),
                    weight: shared_count as f64,
                    shared_game_count: shared_count,
                    shared_games: Vec::new(),
                });
            }
        }

        Ok(())
    }

    fn expand_publisher(
        conn: &Connection,
        pub_id: i64,
        nodes: &mut HashMap<String, GraphNode>,
        links: &mut Vec<GraphLink>,
        visited: &mut HashSet<(NodeType, i64)>,
        filters: &GraphFilterParams,
        max_nodes: usize,
    ) -> Result<()> {
        let pub_node_id = format!("pub_{}", pub_id);

        let mut query = String::from(
            "SELECT g.id, g.app_id, g.name, g.release_year, g.owner_midpoint
             FROM games g
             JOIN game_publishers gp ON g.id = gp.game_id
             WHERE gp.publisher_id = ?1",
        );
        if filters.exclude_sparse {
            query.push_str(" AND g.is_sparse = 0");
        }
        if filters.exclude_playtests {
            query.push_str(" AND g.is_likely_playtest = 0");
        }
        query.push_str(" ORDER BY g.owner_midpoint DESC, g.release_year DESC LIMIT 25");

        let mut stmt = conn.prepare(&query)?;
        let games = stmt.query_map(params![pub_id], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<i32>>(3)?,
                r.get::<_, i64>(4)?,
            ))
        })?;

        for g in games.flatten() {
            if nodes.len() >= max_nodes {
                break;
            }
            let (gid, _, name, year, midpoint) = g;
            let game_node_id = format!("game_{}", gid);

            if !nodes.contains_key(&game_node_id) {
                nodes.insert(
                    game_node_id.clone(),
                    GraphNode {
                        id: game_node_id.clone(),
                        entity_id: gid,
                        node_type: NodeType::Game,
                        label: name.clone(),
                        sub_label: year.map(|y| format!("{}", y)),
                        size: (midpoint as f64).log10().max(6.0).min(20.0),
                        metric: midpoint,
                        year,
                        metadata: None,
                    },
                );
                visited.insert((NodeType::Game, gid));
            }

            links.push(GraphLink {
                source: pub_node_id.clone(),
                target: game_node_id.clone(),
                relationship: "published".to_string(),
                weight: 1.0,
                shared_game_count: 1,
                shared_games: vec![name.clone()],
            });

            // Associated developers
            let mut dev_stmt = conn.prepare(
                "SELECT d.id, d.canonical_name, d.game_count 
                 FROM developers d 
                 JOIN game_developers gd ON d.id = gd.developer_id 
                 WHERE gd.game_id = ?1 AND d.is_merged = 0 LIMIT 3",
            )?;
            let devs = dev_stmt.query_map(params![gid], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                ))
            })?;

            for d in devs.flatten() {
                let (dev_id, dev_name, dev_games) = d;
                let dev_node_id = format!("dev_{}", dev_id);

                if !nodes.contains_key(&dev_node_id) && nodes.len() < max_nodes {
                    nodes.insert(
                        dev_node_id.clone(),
                        GraphNode {
                            id: dev_node_id.clone(),
                            entity_id: dev_id,
                            node_type: NodeType::Developer,
                            label: dev_name,
                            sub_label: Some(format!("{} games", dev_games)),
                            size: (dev_games as f64).sqrt().max(7.0).min(26.0),
                            metric: dev_games,
                            year: None,
                            metadata: None,
                        },
                    );
                    visited.insert((NodeType::Developer, dev_id));
                }

                if nodes.contains_key(&dev_node_id) {
                    links.push(GraphLink {
                        source: dev_node_id,
                        target: game_node_id.clone(),
                        relationship: "developed".to_string(),
                        weight: 1.0,
                        shared_game_count: 1,
                        shared_games: vec![name.clone()],
                    });
                }
            }
        }

        Ok(())
    }

    fn expand_game(
        conn: &Connection,
        game_id: i64,
        nodes: &mut HashMap<String, GraphNode>,
        links: &mut Vec<GraphLink>,
        visited: &mut HashSet<(NodeType, i64)>,
        _filters: &GraphFilterParams,
        max_nodes: usize,
    ) -> Result<()> {
        let game_node_id = format!("game_{}", game_id);

        // Fetch credited developers
        let mut dev_stmt = conn.prepare(
            "SELECT d.id, d.canonical_name, d.game_count
             FROM developers d
             JOIN game_developers gd ON d.id = gd.developer_id
             WHERE gd.game_id = ?1 AND d.is_merged = 0",
        )?;
        let devs = dev_stmt.query_map(params![game_id], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })?;

        for d in devs.flatten() {
            let (dev_id, dev_name, dev_games) = d;
            let dev_node_id = format!("dev_{}", dev_id);

            if !nodes.contains_key(&dev_node_id) && nodes.len() < max_nodes {
                nodes.insert(
                    dev_node_id.clone(),
                    GraphNode {
                        id: dev_node_id.clone(),
                        entity_id: dev_id,
                        node_type: NodeType::Developer,
                        label: dev_name,
                        sub_label: Some(format!("{} games", dev_games)),
                        size: (dev_games as f64).sqrt().max(8.0).min(28.0),
                        metric: dev_games,
                        year: None,
                        metadata: None,
                    },
                );
                visited.insert((NodeType::Developer, dev_id));
            }

            if nodes.contains_key(&dev_node_id) {
                links.push(GraphLink {
                    source: dev_node_id,
                    target: game_node_id.clone(),
                    relationship: "developed".to_string(),
                    weight: 1.0,
                    shared_game_count: 1,
                    shared_games: Vec::new(),
                });
            }
        }

        // Fetch publishers
        let mut pub_stmt = conn.prepare(
            "SELECT p.id, p.canonical_name, p.game_count
             FROM publishers p
             JOIN game_publishers gp ON p.id = gp.publisher_id
             WHERE gp.game_id = ?1 AND p.is_merged = 0",
        )?;
        let pubs = pub_stmt.query_map(params![game_id], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })?;

        for p in pubs.flatten() {
            let (pub_id, pub_name, pub_games) = p;
            let pub_node_id = format!("pub_{}", pub_id);

            if !nodes.contains_key(&pub_node_id) && nodes.len() < max_nodes {
                nodes.insert(
                    pub_node_id.clone(),
                    GraphNode {
                        id: pub_node_id.clone(),
                        entity_id: pub_id,
                        node_type: NodeType::Publisher,
                        label: pub_name,
                        sub_label: Some(format!("{} games", pub_games)),
                        size: (pub_games as f64).sqrt().max(8.0).min(28.0),
                        metric: pub_games,
                        year: None,
                        metadata: None,
                    },
                );
                visited.insert((NodeType::Publisher, pub_id));
            }

            if nodes.contains_key(&pub_node_id) {
                links.push(GraphLink {
                    source: pub_node_id,
                    target: game_node_id.clone(),
                    relationship: "published".to_string(),
                    weight: 1.0,
                    shared_game_count: 1,
                    shared_games: Vec::new(),
                });
            }
        }

        Ok(())
    }

    fn fetch_developer_node(conn: &Connection, dev_id: i64) -> Result<GraphNode> {
        conn.query_row(
            "SELECT id, canonical_name, game_count, publisher_count, earliest_release_year, latest_release_year, total_owner_midpoint
             FROM developers WHERE id = ?1",
            params![dev_id],
            |r| {
                Ok(GraphNode {
                    id: format!("dev_{}", r.get::<_, i64>(0)?),
                    entity_id: r.get(0)?,
                    node_type: NodeType::Developer,
                    label: r.get(1)?,
                    sub_label: Some(format!("{} games", r.get::<_, i64>(2)?)),
                    size: (r.get::<_, i64>(2)? as f64).sqrt().max(10.0).min(32.0),
                    metric: r.get(2)?,
                    year: r.get(5)?,
                    metadata: None,
                })
            },
        )
    }

    fn fetch_publisher_node(conn: &Connection, pub_id: i64) -> Result<GraphNode> {
        conn.query_row(
            "SELECT id, canonical_name, game_count, developer_count, earliest_release_year, latest_release_year, total_owner_midpoint
             FROM publishers WHERE id = ?1",
            params![pub_id],
            |r| {
                Ok(GraphNode {
                    id: format!("pub_{}", r.get::<_, i64>(0)?),
                    entity_id: r.get(0)?,
                    node_type: NodeType::Publisher,
                    label: r.get(1)?,
                    sub_label: Some(format!("{} games", r.get::<_, i64>(2)?)),
                    size: (r.get::<_, i64>(2)? as f64).sqrt().max(10.0).min(32.0),
                    metric: r.get(2)?,
                    year: r.get(5)?,
                    metadata: None,
                })
            },
        )
    }

    fn fetch_game_node(conn: &Connection, game_id: i64) -> Result<GraphNode> {
        conn.query_row(
            "SELECT id, app_id, name, release_year, owner_midpoint FROM games WHERE id = ?1",
            params![game_id],
            |r| {
                let midpoint: i64 = r.get(4)?;
                Ok(GraphNode {
                    id: format!("game_{}", r.get::<_, i64>(0)?),
                    entity_id: r.get(0)?,
                    node_type: NodeType::Game,
                    label: r.get(2)?,
                    sub_label: r.get::<_, Option<i32>>(3)?.map(|y| format!("{}", y)),
                    size: (midpoint as f64).log10().max(6.0).min(22.0),
                    metric: midpoint,
                    year: r.get(3)?,
                    metadata: None,
                })
            },
        )
    }
}
