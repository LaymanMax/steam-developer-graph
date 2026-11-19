export type NodeType = "developer" | "publisher" | "game";

export interface GraphNode {
  id: string; // e.g. "dev_123", "pub_456", "game_789"
  entity_id: number;
  node_type: NodeType;
  label: string;
  sub_label?: string;
  size: number;
  metric: number;
  year?: number;
  metadata?: Record<string, unknown>;
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
}

export interface GraphLink {
  source: string | GraphNode;
  target: string | GraphNode;
  relationship: string;
  weight: number;
  shared_game_count: number;
  shared_games: string[];
}

export interface Subgraph {
  nodes: GraphNode[];
  links: GraphLink[];
  total_nodes: number;
  total_links: number;
  is_truncated: boolean;
  center_node_id?: string;
}

export interface OverviewSummary {
  total_games: number;
  total_developers: number;
  total_publishers: number;
  total_genres: number;
  dev_pub_relationships: number;
  dev_collab_relationships: number;
  complete_credit_games: number;
  sparse_records: number;
  likely_playtests: number;
  total_aliases: number;
  outstanding_candidates: number;
  top_publishers_by_devs: RankedEntity[];
  top_developers_by_pubs: RankedEntity[];
  top_developers_by_games: RankedEntity[];
  top_publishers_by_entropy: RankedEntity[];
  top_collaborations: RankedCollab[];
}

export interface RankedEntity {
  id: number;
  name: string;
  metric_value: number;
  metric_label: string;
  secondary_label?: string;
}

export interface RankedCollab {
  dev1_id: number;
  dev1_name: string;
  dev2_id: number;
  dev2_name: string;
  shared_games: number;
}

export interface SearchResultItem {
  entity_type: "developer" | "publisher" | "game";
  id: number;
  label: string;
  sub_label?: string;
  matched_alias?: string;
}

export interface DeveloperListItem {
  id: number;
  canonical_name: string;
  game_count: number;
  publisher_count: number;
  genre_count: number;
  collaborator_count: number;
  earliest_release_year?: number;
  latest_release_year?: number;
  total_owner_midpoint: number;
}

export interface DeveloperProfile {
  id: number;
  canonical_name: string;
  normalized_name: string;
  aliases: string[];
  game_count: number;
  publisher_count: number;
  genre_count: number;
  collaborator_count: number;
  earliest_release_year?: number;
  latest_release_year?: number;
  total_owner_lower: number;
  total_owner_upper: number;
  total_owner_midpoint: number;
  catalog: GameSummaryItem[];
  publishers: ConnectedPublisherItem[];
  collaborators: ConnectedDeveloperItem[];
  timeline: YearGameCount[];
  genres: GenreCountItem[];
}

export interface PublisherListItem {
  id: number;
  canonical_name: string;
  game_count: number;
  developer_count: number;
  genre_count: number;
  earliest_release_year?: number;
  latest_release_year?: number;
  total_owner_midpoint: number;
  shannon_entropy: number;
}

export interface PublisherProfile {
  id: number;
  canonical_name: string;
  normalized_name: string;
  aliases: string[];
  game_count: number;
  developer_count: number;
  genre_count: number;
  earliest_release_year?: number;
  latest_release_year?: number;
  total_owner_lower: number;
  total_owner_upper: number;
  total_owner_midpoint: number;
  shannon_entropy: number;
  developer_relationships: ConnectedDeveloperItem[];
  portfolio: GameSummaryItem[];
  genre_distribution: GenreCountItem[];
  timeline: YearGameCount[];
}

export interface GameSummaryItem {
  id: number;
  app_id: number;
  name: string;
  release_date?: string;
  release_year?: number;
  estimated_owners_raw?: string;
  owner_lower: number;
  owner_upper: number;
  owner_midpoint: number;
  price: number;
  header_image?: string;
  is_sparse: boolean;
  is_likely_playtest: boolean;
  developers: EntityRef[];
  publishers: EntityRef[];
  genres: string[];
}

export interface GameDetail {
  id: number;
  app_id: number;
  name: string;
  release_date?: string;
  release_year?: number;
  estimated_owners_raw?: string;
  owner_lower: number;
  owner_upper: number;
  owner_midpoint: number;
  price: number;
  peak_ccu: number;
  positive: number;
  negative: number;
  recommendations: number;
  average_playtime_forever: number;
  median_playtime_forever: number;
  header_image?: string;
  metacritic_score: number;
  is_sparse: boolean;
  is_likely_playtest: boolean;
  steam_store_url: string;
  developers: EntityRef[];
  publishers: EntityRef[];
  genres: string[];
  related_games: GameSummaryItem[];
}

export interface EntityRef {
  id: number;
  name: string;
}

export interface ConnectedPublisherItem {
  publisher_id: number;
  publisher_name: string;
  shared_games_count: number;
  first_year?: number;
  latest_year?: number;
}

export interface ConnectedDeveloperItem {
  developer_id: number;
  developer_name: string;
  shared_games_count: number;
  first_year?: number;
  latest_year?: number;
}

export interface GenreCountItem {
  genre_name: string;
  game_count: number;
  percentage: number;
}

export interface YearGameCount {
  year: number;
  game_count: number;
}

export interface DeveloperCollaboration {
  dev1_id: number;
  dev1_name: string;
  dev2_id: number;
  dev2_name: string;
  shared_games_count: number;
  first_year?: number;
  latest_year?: number;
  shared_games: CollaborationGameItem[];
}

export interface CollaborationGameItem {
  game_id: number;
  app_id: number;
  name: string;
  release_year?: number;
}

export interface PublisherDeveloperRelation {
  publisher_id: number;
  publisher_name: string;
  developer_id: number;
  developer_name: string;
  shared_games_count: number;
  first_year?: number;
  latest_year?: number;
  shared_games: CollaborationGameItem[];
  genres: string[];
}

export interface HistoricalYearActivity {
  year: number;
  games_released: number;
  active_developers: number;
  active_publishers: number;
  new_developer_collaborations: number;
  new_publisher_relationships: number;
}

export interface GenreYearTrend {
  year: number;
  genre: string;
  game_count: number;
}

export interface ResolutionCandidateItem {
  id: number;
  entity_type: string;
  entity_a_id: number;
  entity_a_name: string;
  entity_a_games: number;
  entity_b_id: number;
  entity_b_name: string;
  entity_b_games: number;
  similarity_score: number;
  matching_explanation: string;
  status: string;
  example_games_a: string[];
  example_games_b: string[];
}

export interface MergeHistoryItem {
  id: number;
  entity_type: string;
  source_entity_id: number;
  target_entity_id: number;
  source_canonical_name: string;
  target_canonical_name: string;
  reason?: string;
  created_at: string;
  is_undone: boolean;
  undone_at?: string;
}

export interface ResolutionReport {
  total_observed_names: number;
  total_canonical_developers: number;
  total_canonical_publishers: number;
  auto_resolved_aliases: number;
  manual_confirmed_aliases: number;
  outstanding_candidates: number;
  rejected_candidates: number;
  merged_entities_count: number;
}

export interface DatasetStatus {
  has_data: boolean;
  total_games: number;
  total_developers: number;
  total_publishers: number;
  total_genres: number;
  local_parquet_exists: boolean;
  local_parquet_path: string;
  local_parquet_size_bytes?: number;
  local_parquet_checksum?: string;
  default_hf_url: string;
  latest_ingestion_run?: IngestionRunSummary;
}

export interface IngestionRunSummary {
  id: number;
  run_type: string;
  status: string;
  started_at: string;
  completed_at?: string;
  total_rows_read: number;
  rows_imported: number;
  sparse_rows_count: number;
  playtest_rows_count: number;
}

export interface QualityReport {
  total_games: number;
  sparse_records: number;
  likely_playtests: number;
  games_with_complete_credits: number;
  missing_developers: number;
  missing_publishers: number;
  missing_genres: number;
  missing_release_dates: number;
  issue_breakdown: IssueCount[];
}

export interface IssueCount {
  issue_type: string;
  severity: string;
  count: number;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}
