
//! Generated TeaQL domain crate for `fifa-world-cup-2026-service-core`.
//!
//! **Before writing queries**, read the `AGENTS.md` at the workspace root.
//! It contains the entity list and the exact `cargo teaql` commands to fetch API prompts.
//!
//! The generated library is not the API-discovery surface. Read the generated
//! application's `AGENTS.md`, then request model-aware object/field Assist.
//! A registry dependency does not require vendoring or browsing generated
//! domain-library source to learn method names. If Assist lacks an operation,
//! report `MISSING_ASSIST` for that path.

pub mod e;
pub mod q;
pub mod request_support;
pub mod runtime;
pub mod sample_data;
pub mod match_stage;
pub mod match_status;
pub mod goal_category;
pub mod card_category;
pub mod confederation;
pub mod tournament;
pub mod tournament_team;
pub mod match_group;
pub mod tournament_match;
pub mod match_goal;
pub mod match_card;
pub mod group_standing;

pub use teaql_core;
pub use teaql_runtime::LedgerEntity;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use match_stage::*;
pub use match_status::*;
pub use goal_category::*;
pub use card_category::*;
pub use confederation::*;
pub use tournament::*;
pub use tournament_team::*;
pub use match_group::*;
pub use tournament_match::*;
pub use match_goal::*;
pub use match_card::*;
pub use group_standing::*;