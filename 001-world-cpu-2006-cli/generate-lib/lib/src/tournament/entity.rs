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
#[teaql(entity = "Tournament", table = "tournament_data", data_service = "sqlite")]
pub struct Tournament {
#[teaql(id)]
    id: u64,

// @source model.xml:130
    tournament_name: String,

// @source model.xml:130
    host_countries: String,

// @source model.xml:130
    start_date: chrono::NaiveDate,

// @source model.xml:130
    end_date: chrono::NaiveDate,

// @source model.xml:130
    total_teams: i32,

// @source model.xml:130
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:130
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(boxed_relations)]
    pub _relations: Box<TournamentReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Tournament {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            tournament_name: String::new(),
            host_countries: String::new(),
            start_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            end_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            total_teams: 0_i32,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            _relations: Box::new(TournamentReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Tournament", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
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

    pub fn tournament_name(&self) -> String {
        self.changed_tournament_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.tournament_name.clone())
    }

    pub fn update_tournament_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tournament_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.tournament_name.clone());
        self.root.set(self.entity_key(), "tournament_name", value);
        self
    }

    pub fn changed_tournament_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tournament_name")
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
        self.root.set(self.entity_key(), "host_countries", value);
        self
    }

    pub fn changed_host_countries(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "host_countries")
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
        self.root.set(self.entity_key(), "start_date", value);
        self
    }

    pub fn changed_start_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "start_date")
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
        self.root.set(self.entity_key(), "end_date", value);
        self
    }

    pub fn changed_end_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "end_date")
    }

    pub fn eval_end_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("end_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "end_date".to_string(), attempted_path: "end_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.end_date())
                }}

    pub fn total_teams(&self) -> i32 {
        self.changed_total_teams().and_then(|value| value.try_i64()).map(|value| value as i32).unwrap_or(self.total_teams)
    }

    pub fn update_total_teams(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.total_teams = value.try_i64().map(|value| value as i32).unwrap_or(self.total_teams.clone());
        self.root.set(self.entity_key(), "total_teams", value);
        self
    }

    pub fn changed_total_teams(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "total_teams")
    }

    pub fn eval_total_teams(&self) -> teaql_core::eval::EvalResult<i32> {
        if !self.is_loaded("total_teams") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "total_teams".to_string(), attempted_path: "total_teams".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.total_teams())
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
    pub fn match_stage_list(&self) -> &SmartList<crate::MatchStage> {
        &self._relations.match_stage_list
    }

    pub fn match_stage_list_mut(&mut self) -> &mut SmartList<crate::MatchStage> {
        &mut self._relations.match_stage_list
    }

    pub fn eval_match_stage_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchStage>> {
        if !self.is_loaded("match_stage_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_stage_list".to_string(), attempted_path: "match_stage_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.match_stage_list)
        }
    }

    pub fn match_status_list(&self) -> &SmartList<crate::MatchStatus> {
        &self._relations.match_status_list
    }

    pub fn match_status_list_mut(&mut self) -> &mut SmartList<crate::MatchStatus> {
        &mut self._relations.match_status_list
    }

    pub fn eval_match_status_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchStatus>> {
        if !self.is_loaded("match_status_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_status_list".to_string(), attempted_path: "match_status_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.match_status_list)
        }
    }

    pub fn goal_category_list(&self) -> &SmartList<crate::GoalCategory> {
        &self._relations.goal_category_list
    }

    pub fn goal_category_list_mut(&mut self) -> &mut SmartList<crate::GoalCategory> {
        &mut self._relations.goal_category_list
    }

    pub fn eval_goal_category_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::GoalCategory>> {
        if !self.is_loaded("goal_category_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "goal_category_list".to_string(), attempted_path: "goal_category_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.goal_category_list)
        }
    }

    pub fn card_category_list(&self) -> &SmartList<crate::CardCategory> {
        &self._relations.card_category_list
    }

    pub fn card_category_list_mut(&mut self) -> &mut SmartList<crate::CardCategory> {
        &mut self._relations.card_category_list
    }

    pub fn eval_card_category_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CardCategory>> {
        if !self.is_loaded("card_category_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "card_category_list".to_string(), attempted_path: "card_category_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.card_category_list)
        }
    }

    pub fn confederation_list(&self) -> &SmartList<crate::Confederation> {
        &self._relations.confederation_list
    }

    pub fn confederation_list_mut(&mut self) -> &mut SmartList<crate::Confederation> {
        &mut self._relations.confederation_list
    }

    pub fn eval_confederation_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Confederation>> {
        if !self.is_loaded("confederation_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "confederation_list".to_string(), attempted_path: "confederation_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.confederation_list)
        }
    }

    pub fn tournament_team_list(&self) -> &SmartList<crate::TournamentTeam> {
        &self._relations.tournament_team_list
    }

    pub fn tournament_team_list_mut(&mut self) -> &mut SmartList<crate::TournamentTeam> {
        &mut self._relations.tournament_team_list
    }

    pub fn eval_tournament_team_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TournamentTeam>> {
        if !self.is_loaded("tournament_team_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_team_list".to_string(), attempted_path: "tournament_team_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tournament_team_list)
        }
    }

    pub fn match_group_list(&self) -> &SmartList<crate::MatchGroup> {
        &self._relations.match_group_list
    }

    pub fn match_group_list_mut(&mut self) -> &mut SmartList<crate::MatchGroup> {
        &mut self._relations.match_group_list
    }

    pub fn eval_match_group_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MatchGroup>> {
        if !self.is_loaded("match_group_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "match_group_list".to_string(), attempted_path: "match_group_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.match_group_list)
        }
    }

    pub fn tournament_match_list(&self) -> &SmartList<crate::TournamentMatch> {
        &self._relations.tournament_match_list
    }

    pub fn tournament_match_list_mut(&mut self) -> &mut SmartList<crate::TournamentMatch> {
        &mut self._relations.tournament_match_list
    }

    pub fn eval_tournament_match_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TournamentMatch>> {
        if !self.is_loaded("tournament_match_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list".to_string(), attempted_path: "tournament_match_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tournament_match_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::TournamentRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct TournamentReverseRelations {
#[teaql(relation(target = "MatchStage", local_key = "id", foreign_key = "tournament_id", many))]
    match_stage_list: SmartList<crate::MatchStage>,
#[teaql(relation(target = "MatchStatus", local_key = "id", foreign_key = "tournament_id", many))]
    match_status_list: SmartList<crate::MatchStatus>,
#[teaql(relation(target = "GoalCategory", local_key = "id", foreign_key = "tournament_id", many))]
    goal_category_list: SmartList<crate::GoalCategory>,
#[teaql(relation(target = "CardCategory", local_key = "id", foreign_key = "tournament_id", many))]
    card_category_list: SmartList<crate::CardCategory>,
#[teaql(relation(target = "Confederation", local_key = "id", foreign_key = "tournament_id", many))]
    confederation_list: SmartList<crate::Confederation>,
#[teaql(relation(target = "TournamentTeam", local_key = "id", foreign_key = "tournament_id", many))]
    tournament_team_list: SmartList<crate::TournamentTeam>,
#[teaql(relation(target = "MatchGroup", local_key = "id", foreign_key = "tournament_id", many))]
    match_group_list: SmartList<crate::MatchGroup>,
#[teaql(relation(target = "TournamentMatch", local_key = "id", foreign_key = "tournament_id", many))]
    tournament_match_list: SmartList<crate::TournamentMatch>,
#[teaql(relation(target = "MatchGoal", local_key = "id", foreign_key = "tournament_id", many))]
    match_goal_list: SmartList<crate::MatchGoal>,
#[teaql(relation(target = "MatchCard", local_key = "id", foreign_key = "tournament_id", many))]
    match_card_list: SmartList<crate::MatchCard>,
#[teaql(relation(target = "GroupStanding", local_key = "id", foreign_key = "tournament_id", many))]
    group_standing_list: SmartList<crate::GroupStanding>,
}

impl TournamentReverseRelations {
    pub fn new() -> Self {
        Self {
            match_stage_list: Default::default(),
            match_status_list: Default::default(),
            goal_category_list: Default::default(),
            card_category_list: Default::default(),
            confederation_list: Default::default(),
            tournament_team_list: Default::default(),
            match_group_list: Default::default(),
            tournament_match_list: Default::default(),
            match_goal_list: Default::default(),
            match_card_list: Default::default(),
            group_standing_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.match_stage_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.match_status_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.goal_category_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.card_category_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.confederation_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tournament_team_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.match_group_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tournament_match_list {
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
