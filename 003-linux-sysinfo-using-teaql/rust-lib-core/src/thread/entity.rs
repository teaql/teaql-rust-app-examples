// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/thread
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Thread", table = "thread_data", data_service = "sqlite")]
pub struct Thread {
#[teaql(id)]
    id: u64,

// @source MODEL.xml:53
    tid: i64,

// @source MODEL.xml:53
    name: String,

// @source MODEL.xml:53
    state: String,

// @source MODEL.xml:53
    process_pid: i64,

// @source MODEL.xml:53
    cpu_user_ticks: i64,

// @source MODEL.xml:53
    cpu_system_ticks: i64,

// @source MODEL.xml:53
    create_time: chrono::DateTime<chrono::Utc>,

// @source MODEL.xml:53
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source MODEL.xml:53
#[teaql(column = "process")]
    process_id: u64,
// @source MODEL.xml:53
#[teaql(relation(target = "Process", local_key = "process_id", foreign_key = "id"))]
    process: Option<crate::Process>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Thread {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            tid: 0_i64,
            name: String::new(),
            state: String::new(),
            process_pid: 0_i64,
            cpu_user_ticks: 0_i64,
            cpu_system_ticks: 0_i64,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            process_id: 0_u64,
            process: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Thread", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.process {
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

    pub fn tid(&self) -> i64 {
        self.changed_tid().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.tid)
    }

    pub fn update_tid(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tid = value.try_i64().map(|value| value as i64).unwrap_or(self.tid.clone());
        self.root.set(self.entity_key(), "tid", value);
        self
    }

    pub fn changed_tid(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tid")
    }

    pub fn eval_tid(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("tid") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tid".to_string(), attempted_path: "tid".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tid())
                }}

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
                }}

    pub fn state(&self) -> String {
        self.changed_state().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.state.clone())
    }

    pub fn update_state(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.state = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.state.clone());
        self.root.set(self.entity_key(), "state", value);
        self
    }

    pub fn changed_state(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "state")
    }

    pub fn eval_state(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("state") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "state".to_string(), attempted_path: "state".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.state())
                }}

    pub fn process_pid(&self) -> i64 {
        self.changed_process_pid().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.process_pid)
    }

    pub fn update_process_pid(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.process_pid = value.try_i64().map(|value| value as i64).unwrap_or(self.process_pid.clone());
        self.root.set(self.entity_key(), "process_pid", value);
        self
    }

    pub fn changed_process_pid(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "process_pid")
    }

    pub fn eval_process_pid(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("process_pid") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "process_pid".to_string(), attempted_path: "process_pid".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.process_pid())
                }}

    pub fn cpu_user_ticks(&self) -> i64 {
        self.changed_cpu_user_ticks().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.cpu_user_ticks)
    }

    pub fn update_cpu_user_ticks(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.cpu_user_ticks = value.try_i64().map(|value| value as i64).unwrap_or(self.cpu_user_ticks.clone());
        self.root.set(self.entity_key(), "cpu_user_ticks", value);
        self
    }

    pub fn changed_cpu_user_ticks(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "cpu_user_ticks")
    }

    pub fn eval_cpu_user_ticks(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("cpu_user_ticks") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "cpu_user_ticks".to_string(), attempted_path: "cpu_user_ticks".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.cpu_user_ticks())
                }}

    pub fn cpu_system_ticks(&self) -> i64 {
        self.changed_cpu_system_ticks().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.cpu_system_ticks)
    }

    pub fn update_cpu_system_ticks(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.cpu_system_ticks = value.try_i64().map(|value| value as i64).unwrap_or(self.cpu_system_ticks.clone());
        self.root.set(self.entity_key(), "cpu_system_ticks", value);
        self
    }

    pub fn changed_cpu_system_ticks(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "cpu_system_ticks")
    }

    pub fn eval_cpu_system_ticks(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("cpu_system_ticks") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "cpu_system_ticks".to_string(), attempted_path: "cpu_system_ticks".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.cpu_system_ticks())
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
    pub fn process_id(&self) -> u64 {
        self.changed_process_id().and_then(|value| value.try_u64()).unwrap_or(self.process_id)
    }

    pub fn update_process_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.process_id = value.try_u64().unwrap_or(self.process_id.clone());
        self.root.set(self.entity_key(), "process_id", value);
        self
    }

    pub fn changed_process_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "process_id")
    }

    pub fn eval_process_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("process_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "process_id".to_string(), attempted_path: "process_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.process_id())
                }}
    pub fn process(&self) -> Option<&crate::Process> {
        self.process.as_ref()
    }

    pub fn eval_process(&self) -> teaql_core::eval::EvalResult<&crate::Process> {
        if !self.is_loaded("process") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "process".to_string(), attempted_path: "process".to_string() }
        } else {
            match &self.process {
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
        &self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .thread_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

