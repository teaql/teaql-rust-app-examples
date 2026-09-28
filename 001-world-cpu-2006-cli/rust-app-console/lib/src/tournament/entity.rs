
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/tournament
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
#[teaql(reverse_relation(name = "match_stage_list", target = "MatchStage", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "match_status_list", target = "MatchStatus", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "goal_category_list", target = "GoalCategory", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "card_category_list", target = "CardCategory", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "confederation_list", target = "Confederation", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "tournament_team_list", target = "TournamentTeam", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "match_group_list", target = "MatchGroup", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "tournament_match_list", target = "TournamentMatch", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "match_goal_list", target = "MatchGoal", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "match_card_list", target = "MatchCard", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(reverse_relation(name = "group_standing_list", target = "GroupStanding", local_key = "id", foreign_key = "tournament_id", many))]
#[teaql(entity = "Tournament", table = "tournament_data", data_service = "sqlite")]
pub struct Tournament {
#[teaql(id)]
    id: u64,

// @source model.xml:127
#[teaql(max_length = 100)]
    tournament_name: String,

// @source model.xml:127
#[teaql(max_length = 100)]
    host_countries: String,

// @source model.xml:127
    start_date: chrono::NaiveDate,

// @source model.xml:127
    end_date: chrono::NaiveDate,

// @source model.xml:127
    total_teams: i64,

// @source model.xml:127
    create_time: teaql_core::time::Timestamp,

// @source model.xml:127
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Tournament {
    pub const ENTITY_NAME: &'static str = "Tournament";

    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRuntimeState) -> Self {
        Self {
            id: 0_u64,
            tournament_name: String::new(),
            host_countries: String::new(),
            start_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            end_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            total_teams: 0_i64,
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            __teaql_runtime_state: root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn attach_runtime_state_recursive(&mut self, root: teaql_runtime::EntityRuntimeState) {
        root.adopt_mutations_from(self.__teaql_runtime_state());
        self.__teaql_replace_runtime_state(root.clone());
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


    pub fn tournament_name(&self) -> String {
        self.changed_tournament_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.tournament_name.clone())
    }

    pub fn update_tournament_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.tournament_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "tournament_name", value);
        self
    }

    pub fn changed_tournament_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "tournament_name")
    }

    pub fn eval_tournament_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("tournament_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_name".to_string(), attempted_path: "tournament_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tournament_name())
                }}


    pub fn host_countries(&self) -> String {
        self.changed_host_countries().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.host_countries.clone())
    }

    pub fn update_host_countries(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.host_countries = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.host_countries.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "host_countries", value);
        self
    }

    pub fn changed_host_countries(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "host_countries")
    }

    pub fn eval_host_countries(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("host_countries") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "host_countries".to_string(), attempted_path: "host_countries".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.host_countries())
                }}


    pub fn start_date(&self) -> chrono::NaiveDate {
        self.changed_start_date().and_then(|value| value.try_date()).unwrap_or(self.start_date)
    }

    pub fn update_start_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.start_date = value.try_date().unwrap_or(self.start_date.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "start_date", value);
        self
    }

    pub fn changed_start_date(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "start_date")
    }

    pub fn eval_start_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("start_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "start_date".to_string(), attempted_path: "start_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.start_date())
                }}


    pub fn end_date(&self) -> chrono::NaiveDate {
        self.changed_end_date().and_then(|value| value.try_date()).unwrap_or(self.end_date)
    }

    pub fn update_end_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.end_date = value.try_date().unwrap_or(self.end_date.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "end_date", value);
        self
    }

    pub fn changed_end_date(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "end_date")
    }

    pub fn eval_end_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("end_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "end_date".to_string(), attempted_path: "end_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.end_date())
                }}


    pub fn total_teams(&self) -> i64 {
        self.changed_total_teams().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.total_teams)
    }

    pub fn update_total_teams(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.total_teams = value.try_i64().map(|value| value as i64).unwrap_or(self.total_teams.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "total_teams", value);
        self
    }

    pub fn changed_total_teams(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "total_teams")
    }

    pub fn eval_total_teams(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("total_teams") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "total_teams".to_string(), attempted_path: "total_teams".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.total_teams())
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

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn match_stage_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::MatchStage>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "match_stage_list",
        )
    }

    pub fn eval_match_stage_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::MatchStage>> {
        let relation = self.match_stage_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_list".to_string(), attempted_path: "match_stage_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn match_status_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::MatchStatus>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "match_status_list",
        )
    }

    pub fn eval_match_status_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::MatchStatus>> {
        let relation = self.match_status_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_list".to_string(), attempted_path: "match_status_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn goal_category_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::GoalCategory>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "goal_category_list",
        )
    }

    pub fn eval_goal_category_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::GoalCategory>> {
        let relation = self.goal_category_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_list".to_string(), attempted_path: "goal_category_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn card_category_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::CardCategory>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "card_category_list",
        )
    }

    pub fn eval_card_category_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::CardCategory>> {
        let relation = self.card_category_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "card_category_list".to_string(), attempted_path: "card_category_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn confederation_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::Confederation>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "confederation_list",
        )
    }

    pub fn eval_confederation_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::Confederation>> {
        let relation = self.confederation_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_list".to_string(), attempted_path: "confederation_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn tournament_team_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::TournamentTeam>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "tournament_team_list",
        )
    }

    pub fn eval_tournament_team_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::TournamentTeam>> {
        let relation = self.tournament_team_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_team_list".to_string(), attempted_path: "tournament_team_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn match_group_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::MatchGroup>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "match_group_list",
        )
    }

    pub fn eval_match_group_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::MatchGroup>> {
        let relation = self.match_group_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_group_list".to_string(), attempted_path: "match_group_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn tournament_match_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::TournamentMatch>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "tournament_match_list",
        )
    }

    pub fn eval_tournament_match_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::TournamentMatch>> {
        let relation = self.tournament_match_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list".to_string(), attempted_path: "tournament_match_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn match_goal_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::MatchGoal>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "match_goal_list",
        )
    }

    pub fn eval_match_goal_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::MatchGoal>> {
        let relation = self.match_goal_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_goal_list".to_string(), attempted_path: "match_goal_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn match_card_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::MatchCard>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "match_card_list",
        )
    }

    pub fn eval_match_card_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::MatchCard>> {
        let relation = self.match_card_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_card_list".to_string(), attempted_path: "match_card_list".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn group_standing_list(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::GroupStanding>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "group_standing_list",
        )
    }

    pub fn eval_group_standing_list(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::GroupStanding>> {
        let relation = self.group_standing_list();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "group_standing_list".to_string(), attempted_path: "group_standing_list".to_string() },
        }
    }

}
