# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-19

### Added

- **Backend Architecture**:
  - High-performance Rust HTTP service powered by Axum, Tokio, and SQLite (WAL mode, r2d2 connection pooling).
  - Database schema and migrations modeling games, developers, publishers, genres, explicit credit junctions, aliases, and merge audit events.
  - Streaming ingestion pipeline for Parquet (Apache Arrow) and JSON datasets with automatic data quality tracking (sparse records and playtest detection).
  - Native dataset acquisition downloader from Hugging Face with SHA-256 verification.
- **Entity Resolution Subsystem**:
  - Conservative Unicode NFKC normalization and corporate stem reduction (`Ltd`, `LLC`, `Inc`, `GmbH`, `Studios`).
  - Multi-signal fuzzy matching engine combining Levenshtein edit distance, Jaro-Winkler similarity, token set overlap, and stem clustering.
  - Human-in-the-loop candidate management with reversible merge history and audit logging.
- **Graph Analytics**:
  - Bounded neighborhood traversals (1–2 hops) with degree limits and attribute filtering.
  - Derived developer-developer collaboration metrics and publisher-developer relationship tracking.
  - Publisher genre portfolio diversity scoring using Shannon entropy ($H = -\sum p_i \ln p_i$).
  - Historical annual activity timelines and genre growth trends.
- **Frontend Experience**:
  - React 19, TypeScript, and Vite single-page application with responsive dark-slate design.
  - Interactive 2D force-directed canvas graph explorer (`react-force-graph-2d`) with node inspection and neighborhood navigation.
  - Directories and dedicated profiles for Developers, Publishers, and Games.
  - Collaboration Lab workspace and Historical Analysis dashboard.
  - Entity Resolution Center for reviewing duplicate candidate pairs and managing aliases.
  - Global autocomplete search across developers, publishers, and games.
- **Data Export & Tooling**:
  - CSV export for developers, publishers, and collaborations.
  - JSON export for queried network subgraphs.
  - Developer CLI binary supporting `status`, `download`, `ingest`, `candidates`, and `merge` commands.
- **Automated Verification**:
  - Rust unit and integration test suites.
  - Frontend Vitest component and utility test suites.
