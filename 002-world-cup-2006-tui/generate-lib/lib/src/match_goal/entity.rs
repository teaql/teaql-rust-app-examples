use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MatchGoal", table = "match_goal_data", data_service = "sqlite")]
pub struct MatchGoal {
#[teaql(id)]
    id: u64,

// @source model.xml:205
    player_name: String,

// @source model.xml:205
    minute_scored: i32,

// @source model.xml:205
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:205
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:205
#[teaql(column = "tournament_match")]
    tournament_match_id: u64,

// @source model.xml:205
#[teaql(column = "tournament_team")]
    tournament_team_id: u64,

// @source model.xml:205
#[teaql(column = "goal_category")]
    goal_category_id: u64,

// @source model.xml:205
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:205
#[teaql(relation(target = "TournamentMatch", local_key = "tournament_match_id", foreign_key = "id"))]
    tournament_match: Option<crate::TournamentMatch>,

// @source model.xml:205
#[teaql(relation(target = "TournamentTeam", local_key = "tournament_team_id", foreign_key = "id"))]
    tournament_team: Option<crate::TournamentTeam>,

// @source model.xml:205
#[teaql(relation(target = "GoalCategory", local_key = "goal_category_id", foreign_key = "id"))]
    goal_category: Option<crate::GoalCategory>,

// @source model.xml:205
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<crate::Tournament>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MatchGoal {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            player_name: String::new(),
            minute_scored: 0_i32,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            tournament_match_id: 0_u64,
            tournament_team_id: 0_u64,
            goal_category_id: 0_u64,
            tournament_id: 0_u64,
            tournament_match: None,
            tournament_team: None,
            goal_category: None,
            tournament: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MatchGoal", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.tournament_match {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.tournament_team {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.goal_category {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.tournament {
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

    pub fn player_name(&self) -> String {
        self.changed_player_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.player_name.clone())
    }

    pub fn update_player_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.player_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.player_name.clone());
        self.root.set(self.entity_key(), "player_name", value);
        self
    }

    pub fn changed_player_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "player_name")
    }

    pub fn eval_player_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("player_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "player_name".to_string(), attempted_path: "player_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.player_name())
                }}

    pub fn minute_scored(&self) -> i32 {
        self.changed_minute_scored().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.minute_scored)
    }

    pub fn update_minute_scored(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.minute_scored = value.try_i64().map(|value| value as i32).unwrap_or(self.minute_scored.clone());
        self.root.set(self.entity_key(), "minute_scored", value);
        self
    }

    pub fn changed_minute_scored(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "minute_scored")
    }

    pub fn eval_minute_scored(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("minute_scored") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "minute_scored".to_string(), attempted_path: "minute_scored".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.minute_scored())
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
    pub fn tournament_match_id(&self) -> u64 {
        self.changed_tournament_match_id().and_then(|value| value.try_u64()).unwrap_or(self.tournament_match_id)
    }

    pub fn update_tournament_match_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_match_id = value.try_u64().unwrap_or(self.tournament_match_id.clone());
        self.root.set(self.entity_key(), "tournament_match_id", value);
        self
    }

    pub fn changed_tournament_match_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tournament_match_id")
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
        self.root.set(self.entity_key(), "tournament_team_id", value);
        self
    }

    pub fn changed_tournament_team_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tournament_team_id")
    }

    pub fn eval_tournament_team_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("tournament_team_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_team_id".to_string(), attempted_path: "tournament_team_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tournament_team_id())
                }}

    pub fn goal_category_id(&self) -> u64 {
        self.changed_goal_category_id().and_then(|value| value.try_u64()).unwrap_or(self.goal_category_id)
    }

    pub(crate) fn update_goal_category_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.goal_category_id = value.try_u64().unwrap_or(self.goal_category_id.clone());
        self.root.set(self.entity_key(), "goal_category_id", value);
        self
    }

    pub fn changed_goal_category_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "goal_category_id")
    }

    pub fn eval_goal_category_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("goal_category_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_id".to_string(), attempted_path: "goal_category_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.goal_category_id())
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
    pub fn update_goal_category_to_normal(&mut self) -> &mut Self {
        self.update_goal_category_id(1001_u64)
    }

    pub fn goal_category_is_normal(&self) -> bool {
        self.goal_category_id() == 1001_u64
    }
    pub fn update_goal_category_to_penalty(&mut self) -> &mut Self {
        self.update_goal_category_id(1002_u64)
    }

    pub fn goal_category_is_penalty(&self) -> bool {
        self.goal_category_id() == 1002_u64
    }
    pub fn update_goal_category_to_own_goal(&mut self) -> &mut Self {
        self.update_goal_category_id(1003_u64)
    }

    pub fn goal_category_is_own_goal(&self) -> bool {
        self.goal_category_id() == 1003_u64
    }
    pub fn update_goal_category_to_free_kick(&mut self) -> &mut Self {
        self.update_goal_category_id(1004_u64)
    }

    pub fn goal_category_is_free_kick(&self) -> bool {
        self.goal_category_id() == 1004_u64
    }
    pub fn tournament_match(&self) -> Option<&crate::TournamentMatch> {
        self.tournament_match.as_ref()
    }

    pub fn eval_tournament_match(&self) -> teaql_core::eval::EvalResult<&crate::TournamentMatch> {
        if !self.is_loaded("tournament_match") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match".to_string(), attempted_path: "tournament_match".to_string() }
        } else {
            match &self.tournament_match {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn tournament_team(&self) -> Option<&crate::TournamentTeam> {
        self.tournament_team.as_ref()
    }

    pub fn eval_tournament_team(&self) -> teaql_core::eval::EvalResult<&crate::TournamentTeam> {
        if !self.is_loaded("tournament_team") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_team".to_string(), attempted_path: "tournament_team".to_string() }
        } else {
            match &self.tournament_team {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn goal_category(&self) -> Option<&crate::GoalCategory> {
        self.goal_category.as_ref()
    }

    pub fn eval_goal_category(&self) -> teaql_core::eval::EvalResult<&crate::GoalCategory> {
        if !self.is_loaded("goal_category") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category".to_string(), attempted_path: "goal_category".to_string() }
        } else {
            match &self.goal_category {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .match_goal_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

