# Contributing to Steam Developer Network Graph

Thank you for your interest in contributing to **Steam Developer Network Graph**! We welcome contributions of all kinds, including feature implementations, graph algorithm improvements, bug fixes, documentation enhancements, and data quality refinements.

---

## Code of Conduct

All contributors and participants are expected to adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md). Please read it to understand our community standards.

---

## Development Setup

### Prerequisites

- **Rust**: 1.75+ with Cargo (`rustup update`)
- **Node.js**: 18+ and `pnpm` (or `npm`)

### Local Setup Steps

1. **Fork and Clone**

   ```bash
   git clone https://github.com/LaymanMax/steam-developer-graph.git
   cd steam-developer-graph
   ```

2. **Install Dependencies**

   ```bash
   # Install frontend dependencies
   cd frontend && pnpm install && cd ..
   ```

3. **Ingest Test Fixtures**

   ```bash
   # Ingest sample fixture for fast local iteration
   cargo run --manifest-path backend/Cargo.toml --bin cli -- ingest --file backend/tests/fixtures/sample_games.json
   ```

4. **Run Development Servers**

   ```bash
   # Terminal 1: Backend API server (runs on :3001)
   npm run dev:backend

   # Terminal 2: Frontend Vite server (runs on :5173)
   npm run dev:frontend
   ```

---

## Development Workflow & Coding Standards

### Project Structure

- `backend/`: Rust Axum application, analytical query engine, entity resolution, and SQLite database migrations.
- `frontend/`: React 19, TypeScript, and Vite single-page application using canvas-based force-directed graph visualization and Recharts.
- `data/`: Local SQLite database and Parquet storage (ignored by git).

### Testing

Always verify that automated tests pass before submitting a Pull Request:

```bash
# Run both backend and frontend tests
npm test

# Or run separately:
npm run test:backend
npm run test:frontend
```

### Type Checking and Building

Ensure that TypeScript compiles cleanly without warnings or errors:

```bash
npm run build:frontend
```

---

## Submitting Pull Requests

1. Create a descriptive feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make targeted, focused commits with clear messages explaining the rationale.
3. Add unit or integration tests for new functionality, bug fixes, or analytical metrics.
4. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
5. Open a Pull Request against the `main` branch. Provide a concise summary of the changes, any relevant screenshots for UI updates, and confirmation that tests pass.

---

## Reporting Issues & Security

- **Bugs & Enhancements**: Please file an issue using GitHub Issues with reproducible steps, expected versus actual behavior, and relevant logs.
- **Security Vulnerabilities**: Please review [.github/SECURITY.md](./.github/SECURITY.md) for instructions on confidential vulnerability disclosures.
