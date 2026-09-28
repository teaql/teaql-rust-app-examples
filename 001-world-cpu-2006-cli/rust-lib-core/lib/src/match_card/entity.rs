
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/match_card
use std::collections::BTreeMap;

use teaql_macros::{teaql_entity, TeaqlEntity};


/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[teaql_entity]
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MatchCard", table = "match_card_data", data_service = "sqlite")]
pub struct MatchCard {
#[teaql(id)]
    id: u64,

// @source model.xml:217
#[teaql(max_length = 100)]
    player_name: String,

// @source model.xml:217
    minute_issued: i64,

// @source model.xml:217
    create_time: teaql_core::time::Timestamp,

// @source model.xml:217
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:217
#[teaql(column = "tournament_match")]
    tournament_match_id: u64,

// @source model.xml:217
#[teaql(column = "tournament_team")]
    tournament_team_id: u64,

// @source model.xml:217
#[teaql(column = "card_category")]
    card_category_id: u64,

// @source model.xml:217
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:217
#[teaql(relation(target = "TournamentMatch", local_key = "tournament_match_id", foreign_key = "id"))]
    tournament_match: Option<Box<crate::TournamentMatch>>,

// @source model.xml:217
#[teaql(relation(target = "TournamentTeam", local_key = "tournament_team_id", foreign_key = "id"))]
    tournament_team: Option<Box<crate::TournamentTeam>>,

// @source model.xml:217
#[teaql(relation(target = "CardCategory", local_key = "card_category_id", foreign_key = "id"))]
    card_category: Option<Box<crate::CardCategory>>,

// @source model.xml:217
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<Box<crate::Tournament>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MatchCard {
    pub const ENTITY_NAME: &'static str = "Match Card";

    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRuntimeState) -> Self {
        Self {
            id: 0_u64,
            player_name: String::new(),
            minute_issued: 0_i64,
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            tournament_match_id: 0_u64,
            tournament_team_id: 0_u64,
            card_category_id: 0_u64,
            tournament_id: 0_u64,
            tournament_match: None,
            tournament_team: None,
            card_category: None,
            tournament: None,
            dynamic: BTreeMap::new(),
            __teaql_runtime_state: root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn attach_runtime_state_recursive(&mut self, root: teaql_runtime::EntityRuntimeState) {
        root.adopt_mutations_from(self.__teaql_runtime_state());
        self.__teaql_replace_runtime_state(root.clone());
        if let Some(entity) = &mut self.tournament_match {
            entity.attach_runtime_state_recursive(root.clone());
        }
        if let Some(entity) = &mut self.tournament_team {
            entity.attach_runtime_state_recursive(root.clone());
        }
        if let Some(entity) = &mut self.card_category {
            entity.attach_runtime_state_recursive(root.clone());
        }
        if let Some(entity) = &mut self.tournament {
            entity.attach_runtime_state_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}


    pub fn player_name(&self) -> String {
        self.changed_player_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.player_name.clone())
    }

    pub fn update_player_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.player_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.player_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "player_name", value);
        self
    }

    pub fn changed_player_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "player_name")
    }

    pub fn eval_player_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("player_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "player_name".to_string(), attempted_path: "player_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.player_name())
                }}


    pub fn minute_issued(&self) -> i64 {
        self.changed_minute_issued().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.minute_issued)
    }

    pub fn update_minute_issued(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.minute_issued = value.try_i64().map(|value| value as i64).unwrap_or(self.minute_issued.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "minute_issued", value);
        self
    }

    pub fn changed_minute_issued(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "minute_issued")
    }

    pub fn eval_minute_issued(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("minute_issued") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "minute_issued".to_string(), attempted_path: "minute_issued".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.minute_issued())
                }}


    pub fn create_time(&self) -> teaql_core::time::Timestamp {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: teaql_core::time::Timestamp) -> &mut Self {
        self.create_time = value;
        let value = teaql_core::Value::from(value);
        self.__teaql_runtime_state().set(self.entity_key(), "create_time", value);
        self
    }
    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<teaql_core::time::Timestamp> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}


    pub fn update_time(&self) -> teaql_core::time::Timestamp {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: teaql_core::time::Timestamp) -> &mut Self {
        self.update_time = value;
        let value = teaql_core::Value::from(value);
        self.__teaql_runtime_state().set(self.entity_key(), "update_time", value);
        self
    }
    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<teaql_core::time::Timestamp> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
                }}


    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}

    pub fn tournament_match_id(&self) -> u64 {
        self.changed_tournament_match_id().and_then(|value| value.try_u64()).unwrap_or(self.tournament_match_id)
    }

    pub fn update_tournament_match_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_match_id = value.try_u64().unwrap_or(self.tournament_match_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "tournament_match_id", value);
        self
    }

    pub fn changed_tournament_match_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "tournament_match_id")
    }

    pub fn eval_tournament_match_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("tournament_match_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_id".to_string(), attempted_path: "tournament_match_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tournament_match_id())
                }}

    pub fn tournament_team_id(&self) -> u64 {
        self.changed_tournament_team_id().and_then(|value| value.try_u64()).unwrap_or(self.tournament_team_id)
    }

    pub fn update_tournament_team_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_team_id = value.try_u64().unwrap_or(self.tournament_team_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "tournament_team_id", value);
        self
    }

    pub fn changed_tournament_team_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "tournament_team_id")
    }

    pub fn eval_tournament_team_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("tournament_team_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_team_id".to_string(), attempted_path: "tournament_team_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tournament_team_id())
                }}

    pub fn card_category_id(&self) -> u64 {
        self.changed_card_category_id().and_then(|value| value.try_u64()).unwrap_or(self.card_category_id)
    }

    pub(crate) fn update_card_category_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.card_category_id = value.try_u64().unwrap_or(self.card_category_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "card_category_id", value);
        self
    }

    pub fn changed_card_category_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "card_category_id")
    }

    pub fn eval_card_category_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("card_category_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "card_category_id".to_string(), attempted_path: "card_category_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.card_category_id())
                }}

    pub fn tournament_id(&self) -> u64 {
        self.changed_tournament_id().and_then(|value| value.try_u64()).unwrap_or(self.tournament_id)
    }

    pub fn update_tournament_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_id = value.try_u64().unwrap_or(self.tournament_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "tournament_id", value);
        self
    }

    pub fn changed_tournament_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "tournament_id")
    }

    pub fn eval_tournament_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("tournament_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_id".to_string(), attempted_path: "tournament_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tournament_id())
                }}
    pub fn update_card_category_to_yellow(&mut self) -> &mut Self {
        self.update_card_category_id(1001_u64)
    }

    pub fn card_category_is_yellow(&self) -> bool {
        self.card_category_id() == 1001_u64
    }
    pub fn update_card_category_to_red(&mut self) -> &mut Self {
        self.update_card_category_id(1002_u64)
    }

    pub fn card_category_is_red(&self) -> bool {
        self.card_category_id() == 1002_u64
    }
    pub fn update_card_category_to_second_yellow(&mut self) -> &mut Self {
        self.update_card_category_id(1003_u64)
    }

    pub fn card_category_is_second_yellow(&self) -> bool {
        self.card_category_id() == 1003_u64
    }
    pub fn tournament_match(&self) -> Option<&crate::TournamentMatch> {
        self.tournament_match.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.tournament_match_id())})
    }

    pub fn eval_tournament_match(&self) -> teaql_core::eval::EvalResult<&crate::TournamentMatch> {
        match self.tournament_match() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None if self.is_loaded("tournament_match") => teaql_core::eval::EvalResult::Null,
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match".to_string(), attempted_path: "tournament_match".to_string() },
        }
    }

    pub fn tournament_team(&self) -> Option<&crate::TournamentTeam> {
        self.tournament_team.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.tournament_team_id())})
    }

    pub fn eval_tournament_team(&self) -> teaql_core::eval::EvalResult<&crate::TournamentTeam> {
        match self.tournament_team() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None if self.is_loaded("tournament_team") => teaql_core::eval::EvalResult::Null,
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_team".to_string(), attempted_path: "tournament_team".to_string() },
        }
    }

    pub fn card_category(&self) -> Option<&crate::CardCategory> {
        self.card_category.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.card_category_id())})
    }

    pub fn eval_card_category(&self) -> teaql_core::eval::EvalResult<&crate::CardCategory> {
        match self.card_category() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None if self.is_loaded("card_category") => teaql_core::eval::EvalResult::Null,
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "card_category".to_string(), attempted_path: "card_category".to_string() },
        }
    }

    pub fn tournament(&self) -> Option<&crate::Tournament> {
        self.tournament.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.tournament_id())})
    }

    pub fn eval_tournament(&self) -> teaql_core::eval::EvalResult<&crate::Tournament> {
        match self.tournament() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None if self.is_loaded("tournament") => teaql_core::eval::EvalResult::Null,
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament".to_string(), attempted_path: "tournament".to_string() },
        }
    }

}

