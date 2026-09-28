
// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/tournament_team
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
#[teaql(reverse_relation(name = "tournament_match_list_as_home_team", target = "TournamentMatch", local_key = "id", foreign_key = "home_team_id", many))]
#[teaql(reverse_relation(name = "tournament_match_list_as_away_team", target = "TournamentMatch", local_key = "id", foreign_key = "away_team_id", many))]
#[teaql(reverse_relation(name = "match_goal_list", target = "MatchGoal", local_key = "id", foreign_key = "tournament_team_id", many))]
#[teaql(reverse_relation(name = "match_card_list", target = "MatchCard", local_key = "id", foreign_key = "tournament_team_id", many))]
#[teaql(reverse_relation(name = "group_standing_list", target = "GroupStanding", local_key = "id", foreign_key = "tournament_team_id", many))]
#[teaql(entity = "TournamentTeam", table = "tournament_team_data", data_service = "sqlite")]
pub struct TournamentTeam {
#[teaql(id)]
    id: u64,

// @source model.xml:144
#[teaql(max_length = 100)]
    team_name: String,

// @source model.xml:144
#[teaql(max_length = 100)]
    team_code: String,

// @source model.xml:144
#[teaql(max_length = 100)]
    emoji_flag: String,

// @source model.xml:144
    fifa_ranking: i64,

// @source model.xml:144
#[teaql(max_length = 100)]
    manager_name: String,

// @source model.xml:144
#[teaql(max_length = 100)]
    group_letter: String,

// @source model.xml:144
    create_time: teaql_core::time::Timestamp,

// @source model.xml:144
    update_time: teaql_core::time::Timestamp,
#[teaql(version)]
    version: i64,
// @source model.xml:144
#[teaql(column = "confederation")]
    confederation_id: u64,

// @source model.xml:144
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:144
#[teaql(relation(target = "Confederation", local_key = "confederation_id", foreign_key = "id"))]
    confederation: Option<Box<crate::Confederation>>,

// @source model.xml:144
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<Box<crate::Tournament>>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TournamentTeam {
    pub const ENTITY_NAME: &'static str = "Tournament Team";

    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRuntimeState) -> Self {
        Self {
            id: 0_u64,
            team_name: String::new(),
            team_code: String::new(),
            emoji_flag: String::new(),
            fifa_ranking: 0_i64,
            manager_name: String::new(),
            group_letter: String::new(),
            create_time: teaql_core::time::Timestamp::now(),
            update_time: teaql_core::time::Timestamp::now(),
            version: 0_i64,
            confederation_id: 0_u64,
            tournament_id: 0_u64,
            confederation: None,
            tournament: None,
            dynamic: BTreeMap::new(),
            __teaql_runtime_state: root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn attach_runtime_state_recursive(&mut self, root: teaql_runtime::EntityRuntimeState) {
        root.adopt_mutations_from(self.__teaql_runtime_state());
        self.__teaql_replace_runtime_state(root.clone());
        if let Some(entity) = &mut self.confederation {
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


    pub fn team_name(&self) -> String {
        self.changed_team_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.team_name.clone())
    }

    pub fn update_team_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.team_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.team_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "team_name", value);
        self
    }

    pub fn changed_team_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "team_name")
    }

    pub fn eval_team_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("team_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "team_name".to_string(), attempted_path: "team_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.team_name())
                }}


    pub fn team_code(&self) -> String {
        self.changed_team_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.team_code.clone())
    }

    pub fn update_team_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.team_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.team_code.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "team_code", value);
        self
    }

    pub fn changed_team_code(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "team_code")
    }

    pub fn eval_team_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("team_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "team_code".to_string(), attempted_path: "team_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.team_code())
                }}


    pub fn emoji_flag(&self) -> String {
        self.changed_emoji_flag().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.emoji_flag.clone())
    }

    pub fn update_emoji_flag(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.emoji_flag = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.emoji_flag.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "emoji_flag", value);
        self
    }

    pub fn changed_emoji_flag(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "emoji_flag")
    }

    pub fn eval_emoji_flag(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("emoji_flag") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "emoji_flag".to_string(), attempted_path: "emoji_flag".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.emoji_flag())
                }}


    pub fn fifa_ranking(&self) -> i64 {
        self.changed_fifa_ranking().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.fifa_ranking)
    }

    pub fn update_fifa_ranking(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.fifa_ranking = value.try_i64().map(|value| value as i64).unwrap_or(self.fifa_ranking.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "fifa_ranking", value);
        self
    }

    pub fn changed_fifa_ranking(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "fifa_ranking")
    }

    pub fn eval_fifa_ranking(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("fifa_ranking") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "fifa_ranking".to_string(), attempted_path: "fifa_ranking".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.fifa_ranking())
                }}


    pub fn manager_name(&self) -> String {
        self.changed_manager_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.manager_name.clone())
    }

    pub fn update_manager_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.manager_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.manager_name.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "manager_name", value);
        self
    }

    pub fn changed_manager_name(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "manager_name")
    }

    pub fn eval_manager_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("manager_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "manager_name".to_string(), attempted_path: "manager_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.manager_name())
                }}


    pub fn group_letter(&self) -> String {
        self.changed_group_letter().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.group_letter.clone())
    }

    pub fn update_group_letter(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.group_letter = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.group_letter.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "group_letter", value);
        self
    }

    pub fn changed_group_letter(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "group_letter")
    }

    pub fn eval_group_letter(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("group_letter") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "group_letter".to_string(), attempted_path: "group_letter".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.group_letter())
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

    pub fn confederation_id(&self) -> u64 {
        self.changed_confederation_id().and_then(|value| value.try_u64()).unwrap_or(self.confederation_id)
    }

    pub(crate) fn update_confederation_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.confederation_id = value.try_u64().unwrap_or(self.confederation_id.clone());
        self.__teaql_runtime_state().set(self.entity_key(), "confederation_id", value);
        self
    }

    pub fn changed_confederation_id(&self) -> Option<teaql_core::Value> {
        self.__teaql_runtime_state().get(&self.entity_key(), "confederation_id")
    }

    pub fn eval_confederation_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("confederation_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_id".to_string(), attempted_path: "confederation_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.confederation_id())
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
    pub fn update_confederation_to_afc(&mut self) -> &mut Self {
        self.update_confederation_id(1001_u64)
    }

    pub fn confederation_is_afc(&self) -> bool {
        self.confederation_id() == 1001_u64
    }
    pub fn update_confederation_to_caf(&mut self) -> &mut Self {
        self.update_confederation_id(1002_u64)
    }

    pub fn confederation_is_caf(&self) -> bool {
        self.confederation_id() == 1002_u64
    }
    pub fn update_confederation_to_concacaf(&mut self) -> &mut Self {
        self.update_confederation_id(1003_u64)
    }

    pub fn confederation_is_concacaf(&self) -> bool {
        self.confederation_id() == 1003_u64
    }
    pub fn update_confederation_to_conmebol(&mut self) -> &mut Self {
        self.update_confederation_id(1004_u64)
    }

    pub fn confederation_is_conmebol(&self) -> bool {
        self.confederation_id() == 1004_u64
    }
    pub fn update_confederation_to_ofc(&mut self) -> &mut Self {
        self.update_confederation_id(1005_u64)
    }

    pub fn confederation_is_ofc(&self) -> bool {
        self.confederation_id() == 1005_u64
    }
    pub fn update_confederation_to_uefa(&mut self) -> &mut Self {
        self.update_confederation_id(1006_u64)
    }

    pub fn confederation_is_uefa(&self) -> bool {
        self.confederation_id() == 1006_u64
    }
    pub fn confederation(&self) -> Option<&crate::Confederation> {
        self.confederation.as_deref().or_else(|| {
            self.__teaql_runtime_state().resolve_entity(self.confederation_id())})
    }

    pub fn eval_confederation(&self) -> teaql_core::eval::EvalResult<&crate::Confederation> {
        match self.confederation() {
            Some(v) => teaql_core::eval::EvalResult::Value(v),
            None if self.is_loaded("confederation") => teaql_core::eval::EvalResult::Null,
            None => teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation".to_string(), attempted_path: "confederation".to_string() },
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
    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn tournament_match_list_as_home_team(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::TournamentMatch>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "tournament_match_list_as_home_team",
        )
    }

    pub fn eval_tournament_match_list_as_home_team(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::TournamentMatch>> {
        let relation = self.tournament_match_list_as_home_team();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list_as_home_team".to_string(), attempted_path: "tournament_match_list_as_home_team".to_string() },
        }
    }

    /// Returns the relation view installed by the query that loaded this entity.
    /// This method never performs an implicit database query.
    pub fn tournament_match_list_as_away_team(&self) -> teaql_runtime::RelationHandle<'_, teaql_core::SmartList<crate::TournamentMatch>> {
        self.__teaql_runtime_state().relation_list(
            <Self as teaql_core::TeaqlEntity>::ENTITY_NAME,
            self.id(),
            "tournament_match_list_as_away_team",
        )
    }

    pub fn eval_tournament_match_list_as_away_team(&self) -> teaql_core::eval::EvalResult<&teaql_core::SmartList<crate::TournamentMatch>> {
        let relation = self.tournament_match_list_as_away_team();
        match relation.state() {
            teaql_runtime::LoadedRelation::Loaded | teaql_runtime::LoadedRelation::Empty => teaql_core::eval::EvalResult::Value(relation.value().expect("loaded list relation must have a value")),
            teaql_runtime::LoadedRelation::NotLoaded => teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list_as_away_team".to_string(), attempted_path: "tournament_match_list_as_away_team".to_string() },
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
