//! Generated TeaQL domain crate for `fifa-world-cup-2026-service-core`.
//!
//! **Before writing queries**, read the `AGENTS.md` at the workspace root.
//! It contains the entity list and the exact `cargo teaql` commands to fetch API prompts.
//!
//! AI coding agents must read this crate's `AGENTS.md` before using generated
//! APIs. If this crate was downloaded from a Cargo registry, locate the
//! unpacked crate source or vendor the dependency, then read `AGENTS.md` from
//! the crate root before writing code against it.

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