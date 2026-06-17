use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "GroupStanding", table = "group_standing_data", data_service = "sqlite")]
pub struct GroupStanding {
#[teaql(id)]
    id: u64,

// @source model.xml:247
    played: i32,

// @source model.xml:247
    won: i32,

// @source model.xml:247
    drawn: i32,

// @source model.xml:247
    lost: i32,

// @source model.xml:247
    goals_for: i32,

// @source model.xml:247
    goals_against: i32,

// @source model.xml:247
    goal_difference: i32,

// @source model.xml:247
    points: i32,

// @source model.xml:247
    standing_rank: i32,

// @source model.xml:247
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:247
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:247
#[teaql(column = "tournament_team")]
    tournament_team_id: u64,

// @source model.xml:247
#[teaql(column = "match_group")]
    match_group_id: u64,

// @source model.xml:247
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:247
#[teaql(relation(target = "TournamentTeam", local_key = "tournament_team_id", foreign_key = "id"))]
    tournament_team: Option<crate::TournamentTeam>,

// @source model.xml:247
#[teaql(relation(target = "MatchGroup", local_key = "match_group_id", foreign_key = "id"))]
    match_group: Option<crate::MatchGroup>,

// @source model.xml:247
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<crate::Tournament>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl GroupStanding {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            played: 0_i32,
            won: 0_i32,
            drawn: 0_i32,
            lost: 0_i32,
            goals_for: 0_i32,
            goals_against: 0_i32,
            goal_difference: 0_i32,
            points: 0_i32,
            standing_rank: 0_i32,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            tournament_team_id: 0_u64,
            match_group_id: 0_u64,
            tournament_id: 0_u64,
            tournament_team: None,
            match_group: None,
            tournament: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("GroupStanding", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.tournament_team {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.match_group {
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

    pub fn played(&self) -> i32 {
        self.changed_played().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.played)
    }

    pub fn update_played(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.played = value.try_i64().map(|value| value as i32).unwrap_or(self.played.clone());
        self.root.set(self.entity_key(), "played", value);
        self
    }

    pub fn changed_played(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "played")
    }

    pub fn eval_played(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("played") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "played".to_string(), attempted_path: "played".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.played())
                }}

    pub fn won(&self) -> i32 {
        self.changed_won().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.won)
    }

    pub fn update_won(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.won = value.try_i64().map(|value| value as i32).unwrap_or(self.won.clone());
        self.root.set(self.entity_key(), "won", value);
        self
    }

    pub fn changed_won(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "won")
    }

    pub fn eval_won(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("won") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "won".to_string(), attempted_path: "won".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.won())
                }}

    pub fn drawn(&self) -> i32 {
        self.changed_drawn().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.drawn)
    }

    pub fn update_drawn(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.drawn = value.try_i64().map(|value| value as i32).unwrap_or(self.drawn.clone());
        self.root.set(self.entity_key(), "drawn", value);
        self
    }

    pub fn changed_drawn(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "drawn")
    }

    pub fn eval_drawn(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("drawn") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "drawn".to_string(), attempted_path: "drawn".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.drawn())
                }}

    pub fn lost(&self) -> i32 {
        self.changed_lost().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.lost)
    }

    pub fn update_lost(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.lost = value.try_i64().map(|value| value as i32).unwrap_or(self.lost.clone());
        self.root.set(self.entity_key(), "lost", value);
        self
    }

    pub fn changed_lost(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "lost")
    }

    pub fn eval_lost(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("lost") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "lost".to_string(), attempted_path: "lost".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.lost())
                }}

    pub fn goals_for(&self) -> i32 {
        self.changed_goals_for().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.goals_for)
    }

    pub fn update_goals_for(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.goals_for = value.try_i64().map(|value| value as i32).unwrap_or(self.goals_for.clone());
        self.root.set(self.entity_key(), "goals_for", value);
        self
    }

    pub fn changed_goals_for(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "goals_for")
    }

    pub fn eval_goals_for(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("goals_for") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "goals_for".to_string(), attempted_path: "goals_for".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.goals_for())
                }}

    pub fn goals_against(&self) -> i32 {
        self.changed_goals_against().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.goals_against)
    }

    pub fn update_goals_against(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.goals_against = value.try_i64().map(|value| value as i32).unwrap_or(self.goals_against.clone());
        self.root.set(self.entity_key(), "goals_against", value);
        self
    }

    pub fn changed_goals_against(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "goals_against")
    }

    pub fn eval_goals_against(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("goals_against") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "goals_against".to_string(), attempted_path: "goals_against".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.goals_against())
                }}

    pub fn goal_difference(&self) -> i32 {
        self.changed_goal_difference().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.goal_difference)
    }

    pub fn update_goal_difference(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.goal_difference = value.try_i64().map(|value| value as i32).unwrap_or(self.goal_difference.clone());
        self.root.set(self.entity_key(), "goal_difference", value);
        self
    }

    pub fn changed_goal_difference(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "goal_difference")
    }

    pub fn eval_goal_difference(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("goal_difference") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_difference".to_string(), attempted_path: "goal_difference".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.goal_difference())
                }}

    pub fn points(&self) -> i32 {
        self.changed_points().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.points)
    }

    pub fn update_points(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.points = value.try_i64().map(|value| value as i32).unwrap_or(self.points.clone());
        self.root.set(self.entity_key(), "points", value);
        self
    }

    pub fn changed_points(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "points")
    }

    pub fn eval_points(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("points") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "points".to_string(), attempted_path: "points".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.points())
                }}

    pub fn standing_rank(&self) -> i32 {
        self.changed_standing_rank().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.standing_rank)
    }

    pub fn update_standing_rank(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.standing_rank = value.try_i64().map(|value| value as i32).unwrap_or(self.standing_rank.clone());
        self.root.set(self.entity_key(), "standing_rank", value);
        self
    }

    pub fn changed_standing_rank(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "standing_rank")
    }

    pub fn eval_standing_rank(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("standing_rank") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "standing_rank".to_string(), attempted_path: "standing_rank".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.standing_rank())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .group_standing_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

