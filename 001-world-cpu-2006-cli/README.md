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
