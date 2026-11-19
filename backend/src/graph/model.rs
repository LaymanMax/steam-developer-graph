use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Developer,
    Publisher,
    Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String, // e.g. "dev_123", "pub_456", "game_789"
    pub entity_id: i64,
    pub node_type: NodeType,
    pub label: String,
    pub sub_label: Option<String>,
    pub size: f64,
    pub metric: i64, // e.g. game_count or degree
    pub year: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub relationship: String, // "developed", "published", "collaborated", "published_by"
    pub weight: f64,
    pub shared_game_count: i64,
    pub shared_games: Vec<String>, // titles or AppIDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subgraph {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
    pub total_nodes: usize,
    pub total_links: usize,
    pub is_truncated: bool,
    pub center_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperCollaboration {
    pub dev1_id: i64,
    pub dev1_name: String,
    pub dev2_id: i64,
    pub dev2_name: String,
    pub shared_games_count: i64,
    pub first_year: Option<i32>,
    pub latest_year: Option<i32>,
    pub shared_games: Vec<CollaborationGameItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationGameItem {
    pub game_id: i64,
    pub app_id: i64,
    pub name: String,
    pub release_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherDeveloperRelation {
    pub publisher_id: i64,
    pub publisher_name: String,
    pub developer_id: i64,
    pub developer_name: String,
    pub shared_games_count: i64,
    pub first_year: Option<i32>,
    pub latest_year: Option<i32>,
    pub shared_games: Vec<CollaborationGameItem>,
    pub genres: Vec<String>,
}
