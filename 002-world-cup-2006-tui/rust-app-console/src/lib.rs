//! #  Service Workspace
//!
//! **Before writing queries**, read the generated `AGENTS.md` at the workspace root.
//! It contains the entity list and the exact `cargo teaql` commands to fetch API prompts.

pub use fifa_world_cup_2026_service_core::{teaql_core, E, Q};

pub fn generated_domain_crate() -> &'static str {
    "fifa-world-cup-2026-service-core"
}