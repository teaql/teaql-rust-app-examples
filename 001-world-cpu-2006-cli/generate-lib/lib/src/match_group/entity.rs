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
#[teaql(entity = "MatchGroup", table = "match_group_data", data_service = "sqlite")]
pub struct MatchGroup {
#[teaql(id)]
    id: u64,

// @source model.xml:158
    group_letter: String,

// @source model.xml:158
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:158
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:158
#[teaql(column = "tournament")]
    tournament_id: u64,
// @source model.xml:158
#[teaql(relation(target = "Tournament", local_key = "tournament_id", foreign_key = "id"))]
    tournament: Option<crate::Tournament>,
#[teaql(relation(target = "TournamentMatch", local_key = "id", foreign_key = "match_group_id", many))]
    tournament_match_list: SmartList<crate::TournamentMatch>,
#[teaql(relation(target = "GroupStanding", local_key = "id", foreign_key = "match_group_id", many))]
    group_standing_list: SmartList<crate::GroupStanding>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MatchGroup {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            group_letter: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            tournament_id: 0_u64,
            tournament: None,
            tournament_match_list: Default::default(),
            group_standing_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MatchGroup", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.tournament {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tournament_match_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.group_standing_list {
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
    pub fn tournament_match_list(&self) -> &SmartList<crate::TournamentMatch> {
        &self.tournament_match_list
    }

    pub fn tournament_match_list_mut(&mut self) -> &mut SmartList<crate::TournamentMatch> {
        &mut self.tournament_match_list
    }

    pub fn eval_tournament_match_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TournamentMatch>> {
        if !self.is_loaded("tournament_match_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tournament_match_list".to_string(), attempted_path: "tournament_match_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.tournament_match_list)
        }
    }

    pub fn group_standing_list(&self) -> &SmartList<crate::GroupStanding> {
        &self.group_standing_list
    }

    pub fn group_standing_list_mut(&mut self) -> &mut SmartList<crate::GroupStanding> {
        &mut self.group_standing_list
    }

    pub fn eval_group_standing_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::GroupStanding>> {
        if !self.is_loaded("group_standing_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "group_standing_list".to_string(), attempted_path: "group_standing_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.group_standing_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlRepositoryError<C::MatchGroupRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .match_group_repository()
            .map_err(|err| teaql_runtime::RepositoryError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self).await
    }
}

