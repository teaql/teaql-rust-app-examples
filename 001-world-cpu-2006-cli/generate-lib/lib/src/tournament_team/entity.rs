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
#[teaql(entity = "TournamentTeam", table = "tournament_team_data", data_service = "sqlite")]
pub struct TournamentTeam {
#[teaql(id)]
    id: u64,

// @source model.xml:147
    team_name: String,

// @source model.xml:147
    team_code: String,

// @source model.xml:147
    emoji_flag: String,

// @source model.xml:147
    fifa_ranking: i32,

// @source model.xml:147
    manager_name: String,

// @source model.xml:147
    group_letter: String,

// @source model.xml:147
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:147
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:147
#[teaql(column = "confederation")]
    confederation_id: u64,

// @source model.xml:147
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:147
#[teaql(relation(target = "Confederation", local_key = "confederation_id", foreign_key = "id"))]
    confederation: Option<crate::Confederation>,

// @source model.xml:147
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<crate::Tournament>,
    #[teaql(boxed_relations)]
    pub _relations: Box<TournamentTeamReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TournamentTeam {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            team_name: String::new(),
            team_code: String::new(),
            emoji_flag: String::new(),
            fifa_ranking: 0_i32,
            manager_name: String::new(),
            group_letter: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            confederation_id: 0_u64,
            tournament_id: 0_u64,
            confederation: None,
            tournament: None,
            _relations: Box::new(TournamentTeamReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TournamentTeam", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.confederation {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.tournament {
            entity.attach_root_recursive(root.clone());
        }
        self._relations.attach_root_recursive(root.clone());
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

    pub fn team_name(&self) -> String {
        self.changed_team_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.team_name.clone())
    }

    pub fn update_team_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.team_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.team_name.clone());
        self.root.set(self.entity_key(), "team_name", value);
        self
    }

    pub fn changed_team_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "team_name")
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
        self.root.set(self.entity_key(), "team_code", value);
        self
    }

    pub fn changed_team_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "team_code")
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
        self.root.set(self.entity_key(), "emoji_flag", value);
        self
    }

    pub fn changed_emoji_flag(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "emoji_flag")
    }

    pub fn eval_emoji_flag(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("emoji_flag") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "emoji_flag".to_string(), attempted_path: "emoji_flag".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.emoji_flag())
                }}

    pub fn fifa_ranking(&self) -> i32 {
        self.changed_fifa_ranking().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.fifa_ranking)
    }

    pub fn update_fifa_ranking(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.fifa_ranking = value.try_i64().map(|value| value as i32).unwrap_or(self.fifa_ranking.clone());
        self.root.set(self.entity_key(), "fifa_ranking", value);
        self
    }

    pub fn changed_fifa_ranking(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "fifa_ranking")
    }

    pub fn eval_fifa_ranking(&self) -> teaql_core::eval::EvalResult<i32> {
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
        self.root.set(self.entity_key(), "manager_name", value);
        self
    }

    pub fn changed_manager_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "manager_name")
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
        self.root.set(self.entity_key(), "group_letter", value);
        self
    }

    pub fn changed_group_letter(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "group_letter")
    }

    pub fn eval_group_letter(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("group_letter") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "group_letter".to_string(), attempted_path: "group_letter".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.group_letter())
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
    pub fn confederation_id(&self) -> u64 {
        self.changed_confederation_id().and_then(|value| value.try_u64()).unwrap_or(self.confederation_id)
    }

    pub(crate) fn update_confederation_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.confederation_id = value.try_u64().unwrap_or(self.confederation_id.clone());
        self.root.set(self.entity_key(), "confederation_id", value);
        self
    }

    pub fn changed_confederation_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "confederation_id")
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
        self.confederation.as_ref()
    }

    pub fn eval_confederation(&self) -> teaql_core::eval::EvalResult<&crate::Confederation> {
        if !self.is_loaded("confederation") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation".to_string(), attempted_path: "confederation".to_string() }
        } else {
            match &self.confederation {
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
    pub fn tournament_match_list_as_home_team(&self) -> &SmartList<crate::TournamentMatch> {
        &self._relations.tournament_match_list_as_home_team
    }

    pub fn tournament_match_list_as_home_team_mut(&mut self) -> &mut SmartList<crate::TournamentMatch> {
        &mut self._relations.tournament_match_list_as_home_team
    }

    pub fn eval_tournament_match_list_as_home_team(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TournamentMatch>> {
        if !self.is_loaded("tournament_match_list_as_home_team") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list_as_home_team".to_string(), attempted_path: "tournament_match_list_as_home_team".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tournament_match_list_as_home_team)
        }
    }

    pub fn tournament_match_list_as_away_team(&self) -> &SmartList<crate::TournamentMatch> {
        &self._relations.tournament_match_list_as_away_team
    }

    pub fn tournament_match_list_as_away_team_mut(&mut self) -> &mut SmartList<crate::TournamentMatch> {
        &mut self._relations.tournament_match_list_as_away_team
    }

    pub fn eval_tournament_match_list_as_away_team(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TournamentMatch>> {
        if !self.is_loaded("tournament_match_list_as_away_team") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list_as_away_team".to_string(), attempted_path: "tournament_match_list_as_away_team".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tournament_match_list_as_away_team)
        }
    }

    pub fn match_goal_list(&self) -> &SmartList<crate::MatchGoal> {
        &self._relations.match_goal_list
    }

    pub fn match_goal_list_mut(&mut self) -> &mut SmartList<crate::MatchGoal> {
        &mut self._relations.match_goal_list
    }

    pub fn eval_match_goal_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchGoal>> {
        if !self.is_loaded("match_goal_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_goal_list".to_string(), attempted_path: "match_goal_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.match_goal_list)
        }
    }

    pub fn match_card_list(&self) -> &SmartList<crate::MatchCard> {
        &self._relations.match_card_list
    }

    pub fn match_card_list_mut(&mut self) -> &mut SmartList<crate::MatchCard> {
        &mut self._relations.match_card_list
    }

    pub fn eval_match_card_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchCard>> {
        if !self.is_loaded("match_card_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_card_list".to_string(), attempted_path: "match_card_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.match_card_list)
        }
    }

    pub fn group_standing_list(&self) -> &SmartList<crate::GroupStanding> {
        &self._relations.group_standing_list
    }

    pub fn group_standing_list_mut(&mut self) -> &mut SmartList<crate::GroupStanding> {
        &mut self._relations.group_standing_list
    }

    pub fn eval_group_standing_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::GroupStanding>> {
        if !self.is_loaded("group_standing_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "group_standing_list".to_string(), attempted_path: "group_standing_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.group_standing_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_team_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct TournamentTeamReverseRelations {
#[teaql(relation(target = "TournamentMatch", local_key = "id", foreign_key = "home_team_id", many))]
    tournament_match_list_as_home_team: SmartList<crate::TournamentMatch>,
#[teaql(relation(target = "TournamentMatch", local_key = "id", foreign_key = "away_team_id", many))]
    tournament_match_list_as_away_team: SmartList<crate::TournamentMatch>,
#[teaql(relation(target = "MatchGoal", local_key = "id", foreign_key = "tournament_team_id", many))]
    match_goal_list: SmartList<crate::MatchGoal>,
#[teaql(relation(target = "MatchCard", local_key = "id", foreign_key = "tournament_team_id", many))]
    match_card_list: SmartList<crate::MatchCard>,
#[teaql(relation(target = "GroupStanding", local_key = "id", foreign_key = "tournament_team_id", many))]
    group_standing_list: SmartList<crate::GroupStanding>,
}

impl TournamentTeamReverseRelations {
    pub fn new() -> Self {
        Self {
            tournament_match_list_as_home_team: Default::default(),
            tournament_match_list_as_away_team: Default::default(),
            match_goal_list: Default::default(),
            match_card_list: Default::default(),
            group_standing_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.tournament_match_list_as_home_team {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tournament_match_list_as_away_team {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.match_goal_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.match_card_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.group_standing_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
