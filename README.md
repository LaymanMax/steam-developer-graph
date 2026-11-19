# Steam Developer Network Graph

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Rust: 2021](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![React: 19](https://img.shields.io/badge/React-19-61dafb.svg)](https://react.dev/)
[![TypeScript: 5](https://img.shields.io/badge/TypeScript-5-3178c6.svg)](https://www.typescriptlang.org/)

**Steam Developer Network Graph** is a high-performance network analysis and business intelligence application that explores the relationships connecting game developers, publishers, individual titles, genres, release eras, and ownership tiers across the Steam gaming ecosystem.

Rather than presenting isolated leaderboards or static charts, this platform models the games industry as an observational bipartite and multigraph network. It uncovers collaborative development partnerships, multi-publisher strategies, portfolio diversification patterns, and historical business trajectories with interactive graph exploration, entity resolution, and data quality tracking.

---

## Key Features

1. **Interactive Network Explorer**
   - Canvas-based 2D force-directed visualization (`react-force-graph-2d`).
   - Bounded neighborhood exploration with configurable hops (depth 1–3), node limits, year ranges, and genre filters.
   - Distinct node types (Developers, Publishers, Games) with color coding, dynamic radius proportional to degree/catalog size, and interactive inspector pane.
   - Inspection of underlying game credits directly from graph edges.

2. **Entity Resolution System**
   - Built-in multi-signal matching engine resolving noisy raw metadata (e.g., `BioWare` vs `BioWare™`, `Klei Entertainment` vs `Klei Entertainment Inc.`).
   - Conservative normalizer using Unicode NFKC decomposition, whitespace cleanup, and legal suffix reduction (`Ltd`, `LLC`, `GmbH`, `Studios`, `Games`, `Entertainment`).
   - Multi-signal scoring: exact key matching, Levenshtein edit distance, Jaro-Winkler string similarity, and token set overlap.
   - Human-in-the-loop candidate review: approve merges, reject incorrect matches, mark entities as distinctly separate, and assign explicit aliases.
   - Reversible merge operations with complete audit history (`entity_merge_events`).

3. **Developer & Publisher Intelligence**
   - **Developer Directory & Profiles**: Canonical name, alias collection, catalog count, associated publishers, collaborator network, release timeline, and ownership range.
   - **Publisher Directory & Profiles**: Portfolio distribution, distinct developer count, genre diversity measured via Shannon entropy, and multi-year relationships.
   - **Collaboration Lab**: Deep-dive into co-development credit overlap and multi-developer credits.
   - **Historical Analysis**: Chronological release volume, active entity counts, and genre dominance trends over time.

4. **Transparent Data Quality & Ingestion**
   - Automatic classification of sparse metadata records and suspected playtest entries.
   - User-controllable toggles to include or exclude sparse rows and playtests.
   - Ingestion tracker recording source URLs, timestamps, row counts, and checksums.
   - Re-run idempotency: imports can be safely executed repeatedly without creating duplicate entities or corrupting merge histories.

5. **Data Export**
   - One-click CSV export for filtered developer directories, publisher directories, and collaboration pairs.
   - JSON export for queried network subgraphs.

---

## Architecture

```
steam-developer-graph/
├── backend/                  # Rust HTTP backend & analytical engine
│   ├── src/
│   │   ├── api/              # Axum HTTP routes, handlers & JSON response types
│   │   ├── db/               # SQLite connection pool (r2d2), migrations & queries
│   │   ├── entity_resolution/# Normalization, multi-signal matcher & review manager
│   │   ├── export/           # CSV and Subgraph JSON serialization
│   │   ├── graph/            # Graph models, neighborhood traversals & metrics
│   │   ├── ingestion/        # Parquet/JSON streaming reader, quality classification & downloader
│   │   ├── bin/
│   │   │   ├── server.rs     # Axum web server entrypoint (:3001)
│   │   │   └── cli.rs        # Developer CLI for ingestion, status & entity resolution
│   │   └── lib.rs            # Core shared library
│   └── tests/                # Unit tests and end-to-end integration tests
│
├── frontend/                 # React 19 + TypeScript + Vite web application
│   ├── src/
│   │   ├── api/              # Strongly-typed fetch API client
│   │   ├── components/       # UI building blocks, sidebar, global search, force graph
│   │   ├── types/            # TypeScript interfaces matching backend models
│   │   ├── utils/            # Precision formatters for ranges, currency, entropy & numbers
│   │   └── views/            # Screen views (Overview, Network, Profiles, ER Center, Dataset)
│   └── tests/                # Vitest & React Testing Library test suites
│
└── data/                     # Local SQLite database & Parquet dataset storage (git-ignored)
```

### Technology Stack

- **Backend**: Rust 2021, Axum, Tokio, SQLite 3 (`rusqlite` + `r2d2`), Arrow & Parquet, Reqwest, Chrono, Serde.
- **Frontend**: React 19, TypeScript 5, Vite 6, Tailwind CSS tokens, `react-force-graph-2d`, Recharts, Lucide Icons.
- **Testing**: Rust built-in test framework, Vitest, `@testing-library/react`.

---

## Dataset Attribution & Acquisition

The application utilizes the **Steam Games Dataset** compiled by Javier F. (`FronkonGames`):

- **Source**: [Hugging Face: FronkonGames/steam-games-dataset](https://huggingface.co/datasets/FronkonGames/steam-games-dataset)
- **License**: MIT
- **Format**: Parquet (streamed via Apache Arrow) or JSON / JSONL
- **Data Origin**: Steam Web API and SteamSpy

### Data Caveats & Interpretation

1. **Estimated Owners**: Ownership figures originate from SteamSpy sampling brackets (e.g. `100,000 - 200,000`). The application preserves both lower and upper bounds, calculating an approximate midpoint (`~150,000`) for comparative sorting. These values are observational estimates and should not be treated as audited financial sales.
2. **Missing Metadata**: Many independent and legacy releases omit publishers, genre tags, or precise release dates. These fields are preserved as nulls rather than guessed.
3. **Sparse & Playtest Records**: Some records have virtually no metadata or reflect temporary playtests (e.g. `AppID: 2539430 - Black Dragon Mage Playtest`). The ingestion pipeline detects these through heuristics (e.g. missing credits, name patterns, 0-0 owner ranges) and labels them with data quality flags (`is_sparse`, `is_likely_playtest`), giving users full visibility and filtering controls.

---

## Getting Started

### Prerequisites

- **Rust**: Version 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Node.js**: Version 18+ and `pnpm` (or `npm`)

### Installation

```bash
# Clone the repository
git clone https://github.com/LaymanMax/steam-developer-graph.git
cd steam-developer-graph

# Install frontend dependencies
cd frontend && pnpm install && cd ..
```

### Dataset Ingestion

You can ingest data using the built-in CLI:

```bash
# Ingest the included sample fixture (10 representative games with collaborations and playtests)
cargo run --manifest-path backend/Cargo.toml --bin cli -- ingest --file backend/tests/fixtures/sample_games.json

# Check current database status
cargo run --manifest-path backend/Cargo.toml --bin cli -- status
```

To download and ingest the full Hugging Face dataset (124,000+ games):

```bash
# Download the dataset parquet file directly from Hugging Face
cargo run --manifest-path backend/Cargo.toml --bin cli -- download

# Ingest the downloaded parquet file
cargo run --manifest-path backend/Cargo.toml --bin cli -- ingest
```

---

## Running the Application

### 1. Start the Backend API Server

```bash
cargo run --manifest-path backend/Cargo.toml --bin server
# Server listens on http://127.0.0.1:3001
```

### 2. Start the Frontend Development Server

In a new terminal window:

```bash
npm run dev:frontend
# Vite dev server runs on http://localhost:5173
```

Navigate to `http://localhost:5173` in your browser.

---

## CLI Reference

The backend CLI provides data engineering and administrative commands:

```bash
# Print database statistics and data quality metrics
cargo run --manifest-path backend/Cargo.toml --bin cli -- status

# Ingest a specific Parquet or JSON file
cargo run --manifest-path backend/Cargo.toml --bin cli -- ingest --file /path/to/dataset.parquet

# Generate entity resolution candidate pairs
cargo run --manifest-path backend/Cargo.toml --bin cli -- candidates

# Merge an entity into a canonical target
cargo run --manifest-path backend/Cargo.toml --bin cli -- merge --entity-type developer --source-id 12 --target-id 3 --reason "Corporate legal name variant"
```

---

## Running Automated Tests

Run the full end-to-end test suite across backend and frontend:

```bash
npm test
```

Or run individual suites:

```bash
# Run Rust unit tests and integration tests
npm run test:backend

# Run Frontend Vitest tests
npm run test:frontend
```

---

## REST API Reference

| Method | Endpoint                                 | Description                                                               |
| ------ | ---------------------------------------- | ------------------------------------------------------------------------- |
| `GET`  | `/api/overview/summary`                  | Global network summary, top rankings, and health metrics                  |
| `GET`  | `/api/search?q={term}`                   | Multi-entity autocomplete search across developers, publishers, and games |
| `GET`  | `/api/graph/overview`                    | Macro-level bipartite graph of leading developers and publishers          |
| `GET`  | `/api/graph/neighborhood`                | Subgraph neighborhood centered on a node (`node_type`, `id`, `depth`)     |
| `GET`  | `/api/developers`                        | Paginated directory of developers with filtering and sorting              |
| `GET`  | `/api/developers/{id}`                   | Complete developer profile, catalog, collaborators, and timeline          |
| `GET`  | `/api/developers/{id}/network`           | Immediate 1-to-2-hop network subgraph centered on developer               |
| `GET`  | `/api/publishers`                        | Paginated directory of publishers with Shannon entropy metrics            |
| `GET`  | `/api/publishers/{id}`                   | Complete publisher profile, developer partners, and genre portfolio       |
| `GET`  | `/api/games`                             | Paginated games catalog with playtest/sparse toggles                      |
| `GET`  | `/api/games/{id}`                        | Detailed game metadata, developer/publisher credits, and related titles   |
| `GET`  | `/api/collaborations`                    | Top co-development pairs sharing games                                    |
| `GET`  | `/api/collaborations/{dev1}/{dev2}`      | Shared game credits connecting two developers                             |
| `GET`  | `/api/relationships/publisher-developer` | Publisher-developer relationship catalog                                  |
| `GET`  | `/api/historical/timeline`               | Annual game release volume and entity activity                            |
| `GET`  | `/api/historical/genres`                 | Longitudinal genre distribution trends                                    |
| `GET`  | `/api/entity-resolution/candidates`      | Suggested merge candidate pairs for human review                          |
| `POST` | `/api/entity-resolution/merge`           | Execute a canonical entity merge with audit log                           |
| `POST` | `/api/entity-resolution/reject`          | Dismiss a candidate pair                                                  |
| `POST` | `/api/entity-resolution/mark-distinct`   | Permanently mark two entities as separate                                 |
| `POST` | `/api/entity-resolution/undo`            | Reverse a previous merge operation                                        |
| `GET`  | `/api/dataset/status`                    | Ingestion status, local parquet file state, and row counts                |
| `GET`  | `/api/dataset/quality`                   | Data quality report (missing fields, playtests, sparse records)           |
| `GET`  | `/api/export/developers`                 | CSV export of developers                                                  |
| `GET`  | `/api/export/publishers`                 | CSV export of publishers                                                  |
| `GET`  | `/api/export/collaborations`             | CSV export of collaboration pairs                                         |
| `GET`  | `/api/export/graph`                      | JSON export of queried subgraph                                           |

---

## Analytical Methodology

### Shannon Entropy Portfolio Diversity

To measure whether a publisher focuses on a single niche or maintains a diversified catalog across multiple genres, we compute the Shannon entropy ($H$) of their genre distribution:

$$H = -\sum_{i=1}^{k} p_i \ln(p_i)$$

Where $p_i$ is the proportion of the publisher's games belonging to genre $i$.

- **Low entropy ($\sim 0$)**: Highly specialized (e.g. only Strategy or only Simulation).
- **High entropy ($> 2.0$)**: Broadly diversified across Action, Adventure, RPG, Strategy, and Indie genres.

### Derived Collaboration Edges

Steam does not publish explicit contract records between business entities. Therefore, relationships are strictly observational:

- An edge exists between Developer $A$ and Publisher $B$ if and only if there exists a Game $G$ where $A \in \text{Developers}(G)$ and $B \in \text{Publishers}(G)$.
- A collaboration edge exists between Developer $A$ and Developer $B$ ($A \neq B$) if and only if there exists a Game $G$ where $\{A, B\} \subseteq \text{Developers}(G)$.
  Every edge in the application retains references to the underlying titles acting as evidential provenance.

---

## License

This project is licensed under the [MIT License](./LICENSE).

Dataset distributed under the [MIT License](https://huggingface.co/datasets/FronkonGames/steam-games-dataset) courtesy of Javier F. (`FronkonGames`).
