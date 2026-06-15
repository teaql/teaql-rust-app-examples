# World Cup 2026 - Rust Edition 🏆

A high-performance, interactive command-line application built in Rust to display the FIFA World Cup 2026 groups and rankings. 

This project is a native Rust port of the original Java implementation, powered by the **TeaQL** data engine and **SQLite**. It compiles into a standalone, ultra-minimal native executable (less than 7MB), achieving zero cold-start overhead and lightning-fast execution without requiring a JVM or any external runtime dependencies.

## Features ✨
* **Interactive CLI Shell:** Full REPL experience with `wc2026>` prompt.
* **Groups View:** View all groups (A-L) with flawless text alignment, emojis, and dynamic color-coding.
* **Rankings View:** View global standings of all 48 teams.
* **TeaQL Integration:** Leveraging the generated TeaQL Rust macros and ORM to map directly to an embedded SQLite database.
* **Ultra-Minimal Native Build:** Fully static musl-based compilation available, packaged into a `< 7MB` scratch Docker image.

## Commands 🛠️
When running the interactive CLI, you can use:
* `group <A-L>` - View standings for a specific group (e.g., `group A`).
* `groups` - View standings for all groups.
* `rank` - View the overall global ranking.
* `clear` - Clear the terminal screen.
* `exit` or `quit` - Exit the application.

## Getting Started 🚀

### 1. Run Locally
To run the project directly on your machine:
```bash
cd rust-workspace
cargo run
```

### 2. Build Release Version
To compile a highly optimized release binary:
```bash
cd rust-workspace
cargo build --release
```

### 3. Docker (Minimal Scratch Image)
This repository includes a multi-stage Dockerfile that builds a pure, statically-linked native executable on Alpine and packages it into an empty `scratch` image.
```bash
docker build -t worldcup2026-rust .
docker run -it --rm worldcup2026-rust group A
```
*(The pre-built minimal image is also available on Docker Hub at `teaql/worldcup2026:latest`)*

## Project Structure 🏗️
* `models/` - Original domain models used to define the schema.
* `generate-lib/` - The generated TeaQL Rust entity library containing the strongly-typed data structures and SQL mappers.
* `rust-workspace/` - The main Rust application containing the interactive CLI and business logic.

## APIs Used 📚
This project extensively uses the TeaQL framework APIs across various scenarios:

### 1. Q API (Query API)
The `Q` API provides strongly-typed, chainable methods for querying the database.
* **Scenario: Simple Filtering**
  ```rust
  let g_opt = Q::match_groups()
      .with_group_letter_is("A")
      .purpose("cli")
      .execute_for_list(ctx).await?.data.pop();
  ```
* **Scenario: Relational Joins & Complex Sorting**
  Fetch standings alongside nested relational data (like teams) and apply multiple ordering rules.
  ```rust
  let standings = Q::group_standings()
      .select_tournament_team_with(Q::tournament_teams().select_self())
      .with_match_group_matching(Q::match_groups().with_id_is(g.id()))
      .order_by_points_desc()
      .order_by_goal_difference_desc()
      .order_by_goals_for_desc()
      .purpose("cli").execute_for_list(ctx).await?;
  ```


### 2. E API (Expression Facade)
The `E` API provides a safe, strongly-typed way to extract values and navigate relations from loaded entities.
* **Scenario: Extracting Entity Data**
  ```rust
  let name = E::tournament(entity)
      .get_tournament_name()
      .eval();
  ```

### 3. Entity API
The Entity API provides state mutation capabilities (inserting, updating, deleting) paired with robust audit logging.
* **Scenario: Data Seeding & Audited Insertion**
  Create new records, mutate strongly-typed fields, and persist them to the database while recording the exact "purpose" for the audit logs.
  ```rust
  let mut t = Q::tournaments().new_entity(ctx);
  t.update_tournament_name("FIFA World Cup 2026".to_string());
  t.update_total_teams(48);
  
  // Saves the entity while generating a trace log
  t.audit_as("Seed tournament").save(ctx).await?;
  ```
