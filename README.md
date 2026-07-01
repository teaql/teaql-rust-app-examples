# TeaQL Rust App Examples

This repository contains various example applications built using **TeaQL** in Rust. The examples cover a wide range of use cases from interactive command-line applications, Terminal User Interfaces (TUI), and OS-level integrations.

## Available Examples

### 1. [001-world-cup-2006-cli](./001-world-cpu-2006-cli)
A high-performance, interactive command-line application (REPL) built in Rust to display the FIFA World Cup 2026 groups and rankings. 
- **Features**: Interactive CLI shell, Group/Ranking views.
- **Tech Stack**: Rust, TeaQL Core, SQLite Provider.

### 2. [002-world-cup-2006-tui](./002-world-cup-2006-tui)
A Terminal User Interface (TUI) version of the World Cup application.
- **Features**: Rich terminal rendering (using `ratatui`), keyboard navigation.
- **Tech Stack**: Rust, TeaQL Core, SQLite Provider, `ratatui`, `crossterm`.

### 3. [003-linux-sysinfo-using-teaql](./003-linux-sysinfo-using-teaql)
An example demonstrating how to query the underlying Linux system (`/proc` filesystem) using TeaQL. 
- **Features**: Real-time system monitoring, process and thread inspection with full type-safety.
- **Tech Stack**: Rust, TeaQL Core, `teaql-provider-linux`, `ratatui`.
- **Highlights**: Demonstrates the expressive `Q::` (Query) and `E::` (Expression) APIs to query system processes natively.

---

*Each directory contains its own `README.md` with detailed instructions on how to run and explore the example.*
