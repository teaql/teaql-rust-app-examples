use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "TournamentMatch", table = "tournament_match_data", data_service = "sqlite")]
pub struct TournamentMatch {
#[teaql(id)]
    id: u64,

// @source model.xml:190
    match_number: i32,

// @source model.xml:190
    match_date: chrono::NaiveDate,

// @source model.xml:190
    venue_name: String,

// @source model.xml:190
    venue_city: String,

// @source model.xml:190
    venue_country: String,

// @source model.xml:190
    home_score: i32,

// @source model.xml:190
    away_score: i32,

// @source model.xml:190
    extra_time_home: i32,

// @source model.xml:190
    extra_time_away: i32,

// @source model.xml:190
    penalty_home: i32,

// @source model.xml:190
    penalty_away: i32,

// @source model.xml:190
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:190
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:190
#[teaql(column = "home_team")]
    home_team_id: u64,

// @source model.xml:190
#[teaql(column = "away_team")]
    away_team_id: u64,

// @source model.xml:190
#[teaql(column = "match_stage")]
    match_stage_id: u64,

// @source model.xml:190
#[teaql(column = "match_group")]
    match_group_id: u64,

// @source model.xml:190
#[teaql(column = "match_status")]
    match_status_id: u64,

// @source model.xml:190
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:190
#[teaql(relation(target = "TournamentTeam", local_key = "home_team_id", foreign_key = "id"))]
    home_team: Option<crate::TournamentTeam>,

// @source model.xml:190
#[teaql(relation(target = "TournamentTeam", local_key = "away_team_id", foreign_key = "id"))]
    away_team: Option<crate::TournamentTeam>,

// @source model.xml:190
#[teaql(relation(target = "MatchStage", local_key = "match_stage_id", foreign_key = "id"))]
    match_stage: Option<crate::MatchStage>,

// @source model.xml:190
#[teaql(relation(target = "MatchGroup", local_key = "match_group_id", foreign_key = "id"))]
    match_group: Option<crate::MatchGroup>,

// @source model.xml:190
#[teaql(relation(target = "MatchStatus", local_key = "match_status_id", foreign_key = "id"))]
    match_status: Option<crate::MatchStatus>,

// @source model.xml:190
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<crate::Tournament>,
#[teaql(relation(target = "MatchGoal", local_key = "id", foreign_key = "tournament_match_id", many))]
    match_goal_list: SmartList<crate::MatchGoal>,
#[teaql(relation(target = "MatchCard", local_key = "id", foreign_key = "tournament_match_id", many))]
    match_card_list: SmartList<crate::MatchCard>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TournamentMatch {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            match_number: 0_i32,
            match_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            venue_name: String::new(),
            venue_city: String::new(),
            venue_country: String::new(),
            home_score: 0_i32,
            away_score: 0_i32,
            extra_time_home: 0_i32,
            extra_time_away: 0_i32,
            penalty_home: 0_i32,
            penalty_away: 0_i32,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            home_team_id: 0_u64,
            away_team_id: 0_u64,
            match_stage_id: 0_u64,
            match_group_id: 0_u64,
            match_status_id: 0_u64,
            tournament_id: 0_u64,
            home_team: None,
            away_team: None,
            match_stage: None,
            match_group: None,
            match_status: None,
            tournament: None,
            match_goal_list: Default::default(),
            match_card_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TournamentMatch", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.home_team {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.away_team {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.match_stage {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.match_group {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.match_status {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.tournament {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.match_goal_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.match_card_list {
            entity.attach_root_recursive(root.clone());
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
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn match_number(&self) -> i32 {
        self.changed_match_number().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.match_number)
    }

    pub fn update_match_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.match_number = value.try_i64().map(|value| value as i32).unwrap_or(self.match_number.clone());
        self.root.set(self.entity_key(), "match_number", value);
        self
    }

    pub fn changed_match_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "match_number")
    }

    pub fn eval_match_number(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("match_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_number".to_string(), attempted_path: "match_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.match_number())
                }}

    pub fn match_date(&self) -> chrono::NaiveDate {
        self.changed_match_date().and_then(|value| value.try_date()).unwrap_or(self.match_date)
    }

    pub fn update_match_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.match_date = value.try_date().unwrap_or(self.match_date.clone());
        self.root.set(self.entity_key(), "match_date", value);
        self
    }

    pub fn changed_match_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "match_date")
    }

    pub fn eval_match_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("match_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_date".to_string(), attempted_path: "match_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.match_date())
                }}

    pub fn venue_name(&self) -> String {
        self.changed_venue_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.venue_name.clone())
    }

    pub fn update_venue_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.venue_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.venue_name.clone());
        self.root.set(self.entity_key(), "venue_name", value);
        self
    }

    pub fn changed_venue_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "venue_name")
    }

    pub fn eval_venue_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("venue_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "venue_name".to_string(), attempted_path: "venue_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.venue_name())
                }}

    pub fn venue_city(&self) -> String {
        self.changed_venue_city().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.venue_city.clone())
    }

    pub fn update_venue_city(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.venue_city = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.venue_city.clone());
        self.root.set(self.entity_key(), "venue_city", value);
        self
    }

    pub fn changed_venue_city(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "venue_city")
    }

    pub fn eval_venue_city(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("venue_city") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "venue_city".to_string(), attempted_path: "venue_city".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.venue_city())
                }}

    pub fn venue_country(&self) -> String {
        self.changed_venue_country().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.venue_country.clone())
    }

    pub fn update_venue_country(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.venue_country = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.venue_country.clone());
        self.root.set(self.entity_key(), "venue_country", value);
        self
    }

    pub fn changed_venue_country(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "venue_country")
    }

    pub fn eval_venue_country(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("venue_country") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "venue_country".to_string(), attempted_path: "venue_country".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.venue_country())
                }}

    pub fn home_score(&self) -> i32 {
        self.changed_home_score().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.home_score)
    }

    pub fn update_home_score(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.home_score = value.try_i64().map(|value| value as i32).unwrap_or(self.home_score.clone());
        self.root.set(self.entity_key(), "home_score", value);
        self
    }

    pub fn changed_home_score(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "home_score")
    }

    pub fn eval_home_score(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("home_score") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "home_score".to_string(), attempted_path: "home_score".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.home_score())
                }}

    pub fn away_score(&self) -> i32 {
        self.changed_away_score().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.away_score)
    }

    pub fn update_away_score(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.away_score = value.try_i64().map(|value| value as i32).unwrap_or(self.away_score.clone());
        self.root.set(self.entity_key(), "away_score", value);
        self
    }

    pub fn changed_away_score(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "away_score")
    }

    pub fn eval_away_score(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("away_score") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "away_score".to_string(), attempted_path: "away_score".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.away_score())
                }}

    pub fn extra_time_home(&self) -> i32 {
        self.changed_extra_time_home().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.extra_time_home)
    }

    pub fn update_extra_time_home(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.extra_time_home = value.try_i64().map(|value| value as i32).unwrap_or(self.extra_time_home.clone());
        self.root.set(self.entity_key(), "extra_time_home", value);
        self
    }

    pub fn changed_extra_time_home(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "extra_time_home")
    }

    pub fn eval_extra_time_home(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("extra_time_home") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_time_home".to_string(), attempted_path: "extra_time_home".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.extra_time_home())
                }}

    pub fn extra_time_away(&self) -> i32 {
        self.changed_extra_time_away().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.extra_time_away)
    }

    pub fn update_extra_time_away(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.extra_time_away = value.try_i64().map(|value| value as i32).unwrap_or(self.extra_time_away.clone());
        self.root.set(self.entity_key(), "extra_time_away", value);
        self
    }

    pub fn changed_extra_time_away(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "extra_time_away")
    }

    pub fn eval_extra_time_away(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("extra_time_away") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_time_away".to_string(), attempted_path: "extra_time_away".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.extra_time_away())
                }}

    pub fn penalty_home(&self) -> i32 {
        self.changed_penalty_home().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.penalty_home)
    }

    pub fn update_penalty_home(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.penalty_home = value.try_i64().map(|value| value as i32).unwrap_or(self.penalty_home.clone());
        self.root.set(self.entity_key(), "penalty_home", value);
        self
    }

    pub fn changed_penalty_home(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "penalty_home")
    }

    pub fn eval_penalty_home(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("penalty_home") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "penalty_home".to_string(), attempted_path: "penalty_home".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.penalty_home())
                }}

    pub fn penalty_away(&self) -> i32 {
        self.changed_penalty_away().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.penalty_away)
    }

    pub fn update_penalty_away(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.penalty_away = value.try_i64().map(|value| value as i32).unwrap_or(self.penalty_away.clone());
        self.root.set(self.entity_key(), "penalty_away", value);
        self
    }

    pub fn changed_penalty_away(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "penalty_away")
    }

    pub fn eval_penalty_away(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("penalty_away") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "penalty_away".to_string(), attempted_path: "penalty_away".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.penalty_away())
                }}

    pub fn create_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_timestamp().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.update_time = value.try_timestamp().unwrap_or(self.update_time.clone());
        self.root.set(self.entity_key(), "update_time", value);
        self
    }

    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
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
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn home_team_id(&self) -> u64 {
        self.changed_home_team_id().and_then(|value| value.try_u64()).unwrap_or(self.home_team_id)
    }

    pub fn update_home_team_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.home_team_id = value.try_u64().unwrap_or(self.home_team_id.clone());
        self.root.set(self.entity_key(), "home_team_id", value);
        self
    }

    pub fn changed_home_team_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "home_team_id")
    }

    pub fn eval_home_team_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("home_team_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "home_team_id".to_string(), attempted_path: "home_team_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.home_team_id())
                }}

    pub fn away_team_id(&self) -> u64 {
        self.changed_away_team_id().and_then(|value| value.try_u64()).unwrap_or(self.away_team_id)
    }

    pub fn update_away_team_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.away_team_id = value.try_u64().unwrap_or(self.away_team_id.clone());
        self.root.set(self.entity_key(), "away_team_id", value);
        self
    }

    pub fn changed_away_team_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "away_team_id")
    }

    pub fn eval_away_team_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("away_team_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "away_team_id".to_string(), attempted_path: "away_team_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.away_team_id())
                }}

    pub fn match_stage_id(&self) -> u64 {
        self.changed_match_stage_id().and_then(|value| value.try_u64()).unwrap_or(self.match_stage_id)
    }

    pub(crate) fn update_match_stage_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.match_stage_id = value.try_u64().unwrap_or(self.match_stage_id.clone());
        self.root.set(self.entity_key(), "match_stage_id", value);
        self
    }

    pub fn changed_match_stage_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "match_stage_id")
    }

    pub fn eval_match_stage_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("match_stage_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_id".to_string(), attempted_path: "match_stage_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.match_stage_id())
                }}

    pub fn match_group_id(&self) -> u64 {
        self.changed_match_group_id().and_then(|value| value.try_u64()).unwrap_or(self.match_group_id)
    }

    pub fn update_match_group_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.match_group_id = value.try_u64().unwrap_or(self.match_group_id.clone());
        self.root.set(self.entity_key(), "match_group_id", value);
        self
    }

    pub fn changed_match_group_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "match_group_id")
    }

    pub fn eval_match_group_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("match_group_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_group_id".to_string(), attempted_path: "match_group_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.match_group_id())
                }}

    pub fn match_status_id(&self) -> u64 {
        self.changed_match_status_id().and_then(|value| value.try_u64()).unwrap_or(self.match_status_id)
    }

    pub(crate) fn update_match_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.match_status_id = value.try_u64().unwrap_or(self.match_status_id.clone());
        self.root.set(self.entity_key(), "match_status_id", value);
        self
    }

    pub fn changed_match_status_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "match_status_id")
    }

    pub fn eval_match_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("match_status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_id".to_string(), attempted_path: "match_status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.match_status_id())
                }}

    pub fn tournament_id(&self) -> u64 {
        self.changed_tournament_id().and_then(|value| value.try_u64()).unwrap_or(self.tournament_id)
    }

    pub fn update_tournament_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_id = value.try_u64().unwrap_or(self.tournament_id.clone());
        self.root.set(self.entity_key(), "tournament_id", value);
        self
    }

    pub fn changed_tournament_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tournament_id")
    }

    pub fn eval_tournament_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("tournament_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_id".to_string(), attempted_path: "tournament_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tournament_id())
                }}
    pub fn update_match_stage_to_group(&mut self) -> &mut Self {
        self.update_match_stage_id(1001_u64)
    }

    pub fn match_stage_is_group(&self) -> bool {
        self.match_stage_id() == 1001_u64
    }
    pub fn update_match_stage_to_round_of32(&mut self) -> &mut Self {
        self.update_match_stage_id(1002_u64)
    }

    pub fn match_stage_is_round_of32(&self) -> bool {
        self.match_stage_id() == 1002_u64
    }
    pub fn update_match_stage_to_round_of16(&mut self) -> &mut Self {
        self.update_match_stage_id(1003_u64)
    }

    pub fn match_stage_is_round_of16(&self) -> bool {
        self.match_stage_id() == 1003_u64
    }
    pub fn update_match_stage_to_quarter_final(&mut self) -> &mut Self {
        self.update_match_stage_id(1004_u64)
    }

    pub fn match_stage_is_quarter_final(&self) -> bool {
        self.match_stage_id() == 1004_u64
    }
    pub fn update_match_stage_to_semi_final(&mut self) -> &mut Self {
        self.update_match_stage_id(1005_u64)
    }

    pub fn match_stage_is_semi_final(&self) -> bool {
        self.match_stage_id() == 1005_u64
    }
    pub fn update_match_stage_to_third_place(&mut self) -> &mut Self {
        self.update_match_stage_id(1006_u64)
    }

    pub fn match_stage_is_third_place(&self) -> bool {
        self.match_stage_id() == 1006_u64
    }
    pub fn update_match_stage_to_final(&mut self) -> &mut Self {
        self.update_match_stage_id(1007_u64)
    }

    pub fn match_stage_is_final(&self) -> bool {
        self.match_stage_id() == 1007_u64
    }

    pub fn update_match_status_to_scheduled(&mut self) -> &mut Self {
        self.update_match_status_id(1001_u64)
    }

    pub fn match_status_is_scheduled(&self) -> bool {
        self.match_status_id() == 1001_u64
    }
    pub fn update_match_status_to_live(&mut self) -> &mut Self {
        self.update_match_status_id(1002_u64)
    }

    pub fn match_status_is_live(&self) -> bool {
        self.match_status_id() == 1002_u64
    }
    pub fn update_match_status_to_finished(&mut self) -> &mut Self {
        self.update_match_status_id(1003_u64)
    }

    pub fn match_status_is_finished(&self) -> bool {
        self.match_status_id() == 1003_u64
    }
    pub fn update_match_status_to_postponed(&mut self) -> &mut Self {
        self.update_match_status_id(1004_u64)
    }

    pub fn match_status_is_postponed(&self) -> bool {
        self.match_status_id() == 1004_u64
    }
    pub fn home_team(&self) -> Option<&crate::TournamentTeam> {
        self.home_team.as_ref()
    }

    pub fn eval_home_team(&self) -> teaql_core::eval::EvalResult<&crate::TournamentTeam> {
        if !self.is_loaded("home_team") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "home_team".to_string(), attempted_path: "home_team".to_string() }
        } else {
            match &self.home_team {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn away_team(&self) -> Option<&crate::TournamentTeam> {
        self.away_team.as_ref()
    }

    pub fn eval_away_team(&self) -> teaql_core::eval::EvalResult<&crate::TournamentTeam> {
        if !self.is_loaded("away_team") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "away_team".to_string(), attempted_path: "away_team".to_string() }
        } else {
            match &self.away_team {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn match_stage(&self) -> Option<&crate::MatchStage> {
        self.match_stage.as_ref()
    }

    pub fn eval_match_stage(&self) -> teaql_core::eval::EvalResult<&crate::MatchStage> {
        if !self.is_loaded("match_stage") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage".to_string(), attempted_path: "match_stage".to_string() }
        } else {
            match &self.match_stage {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn match_group(&self) -> Option<&crate::MatchGroup> {
        self.match_group.as_ref()
    }

    pub fn eval_match_group(&self) -> teaql_core::eval::EvalResult<&crate::MatchGroup> {
        if !self.is_loaded("match_group") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_group".to_string(), attempted_path: "match_group".to_string() }
        } else {
            match &self.match_group {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn match_status(&self) -> Option<&crate::MatchStatus> {
        self.match_status.as_ref()
    }

    pub fn eval_match_status(&self) -> teaql_core::eval::EvalResult<&crate::MatchStatus> {
        if !self.is_loaded("match_status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status".to_string(), attempted_path: "match_status".to_string() }
        } else {
            match &self.match_status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn tournament(&self) -> Option<&crate::Tournament> {
        self.tournament.as_ref()
    }

    pub fn eval_tournament(&self) -> teaql_core::eval::EvalResult<&crate::Tournament> {
        if !self.is_loaded("tournament") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament".to_string(), attempted_path: "tournament".to_string() }
        } else {
            match &self.tournament {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn match_goal_list(&self) -> &SmartList<crate::MatchGoal> {
        &self.match_goal_list
    }

    pub fn match_goal_list_mut(&mut self) -> &mut SmartList<crate::MatchGoal> {
        &mut self.match_goal_list
    }

    pub fn eval_match_goal_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchGoal>> {
        if !self.is_loaded("match_goal_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_goal_list".to_string(), attempted_path: "match_goal_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.match_goal_list)
        }
    }

    pub fn match_card_list(&self) -> &SmartList<crate::MatchCard> {
        &self.match_card_list
    }

    pub fn match_card_list_mut(&mut self) -> &mut SmartList<crate::MatchCard> {
        &mut self.match_card_list
    }

    pub fn eval_match_card_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchCard>> {
        if !self.is_loaded("match_card_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_card_list".to_string(), attempted_path: "match_card_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.match_card_list)
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub(crate) async fn save<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_match_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

