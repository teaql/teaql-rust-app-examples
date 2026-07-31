# Particle Monitor System (PMS) Console & TUI 🔬

A high-performance, interactive cleanroom particle monitoring system terminal application built in Rust using **Ratatui (TUI)** and **TeaQL (Rust Domain Engine)** with **SQLite**.

---

## Features ✨

- **5 Interactive Views (Tabs)**:
  1. **Home**: Real-time particle spectrum table (0.1 ~ 5.0 μm channels), gas flow rate ($L/min$), LRef voltage ($V$), and live sampling controls (`S` key or touch button).
  2. **Chart**: Time-series particle concentration trend line chart.
  3. **History**: Historical sampling records table with date filter & row selection.
  4. **Setting**: Device configuration (Calibration Point, Data Keep Days, Sampling Frequency) with password security locking.
  5. **Password**: Password credentials management (Set New Password, Disable Protection via Super Password).

- **Touchscreen & Mouse Compatible**:
  - Full Mouse Capture support (`EnableMouseCapture`).
  - Direct touch/click header tab switching.
  - Interactive Date Picker (Calendar) widget with month pagination and day selection.
  - History row click selection & mouse wheel scrolling (`ScrollUp` / `ScrollDown`).
  - Touch-friendly form field focus and action buttons.

- **TeaQL Core Integration**:
  - Strongly-typed KSML domain model (`models/model.xml`).
  - Type-safe chainable query API (`Q::...`) with audit trace enforcement (`.comment()`, `.purpose()`).
  - Audited entity mutations (`.update_*()`, `.audit_as()`).
  - Automatic zero-code privacy field masking (`_audit_mask_fields`).

---

## Getting Started 🚀

### 1. Build and Run

```bash
# Navigate to app console directory
cd rust-app-console

# Run directly
cargo run

# Build optimized release binary
cargo build --release
```

### 2. Run Tests

```bash
cd rust-app-console
cargo test
```

---

## Project Structure 🏗️

- `models/` — KSML domain model definition (`model.xml`).
- `rust-lib-core/` — Read-only generated TeaQL domain library crate (`pms-service-core`).
- `rust-app-console/` — Ratatui TUI application console workspace crate.
