// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/process
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
#[teaql(entity = "Process", table = "process_data", data_service = "sqlite")]
pub struct Process {
#[teaql(id)]
    id: u64,

// @source MODEL.xml:40
    pid: i64,

// @source MODEL.xml:40
    name: String,

// @source MODEL.xml:40
    state: String,

// @source MODEL.xml:40
    ppid: i64,

// @source MODEL.xml:40
    cmdline: String,

// @source MODEL.xml:40
    thread_count: i64,

// @source MODEL.xml:40
    memory_rss_kb: i64,

// @source MODEL.xml:40
    memory_vms_kb: i64,

// @source MODEL.xml:40
    cpu_user_ticks: i64,

// @source MODEL.xml:40
    cpu_system_ticks: i64,

// @source MODEL.xml:40
    create_time: chrono::DateTime<chrono::Utc>,

// @source MODEL.xml:40
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source MODEL.xml:40
#[teaql(column = "system_info")]
    system_info_id: u64,
// @source MODEL.xml:40
#[teaql(relation(target = "SystemInfo", local_key = "system_info_id", foreign_key = "id"))]
    system_info: Option<crate::SystemInfo>,
#[teaql(relation(target = "Thread", local_key = "id", foreign_key = "process_id", many))]
    thread_list: SmartList<crate::Thread>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Process {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            pid: 0_i64,
            name: String::new(),
            state: String::new(),
            ppid: 0_i64,
            cmdline: String::new(),
            thread_count: 0_i64,
            memory_rss_kb: 0_i64,
            memory_vms_kb: 0_i64,
            cpu_user_ticks: 0_i64,
            cpu_system_ticks: 0_i64,
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            system_info_id: 0_u64,
            system_info: None,
            thread_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Process", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.system_info {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.thread_list {
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

    pub fn pid(&self) -> i64 {
        self.changed_pid().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.pid)
    }

    pub fn update_pid(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.pid = value.try_i64().map(|value| value as i64).unwrap_or(self.pid.clone());
        self.root.set(self.entity_key(), "pid", value);
        self
    }

    pub fn changed_pid(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "pid")
    }

    pub fn eval_pid(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("pid") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "pid".to_string(), attempted_path: "pid".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.pid())
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

    pub fn ppid(&self) -> i64 {
        self.changed_ppid().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.ppid)
    }

    pub fn update_ppid(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.ppid = value.try_i64().map(|value| value as i64).unwrap_or(self.ppid.clone());
        self.root.set(self.entity_key(), "ppid", value);
        self
    }

    pub fn changed_ppid(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "ppid")
    }

    pub fn eval_ppid(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("ppid") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "ppid".to_string(), attempted_path: "ppid".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.ppid())
                }}

    pub fn cmdline(&self) -> String {
        self.changed_cmdline().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.cmdline.clone())
    }

    pub fn update_cmdline(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.cmdline = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.cmdline.clone());
        self.root.set(self.entity_key(), "cmdline", value);
        self
    }

    pub fn changed_cmdline(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "cmdline")
    }

    pub fn eval_cmdline(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("cmdline") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "cmdline".to_string(), attempted_path: "cmdline".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.cmdline())
                }}

    pub fn thread_count(&self) -> i64 {
        self.changed_thread_count().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.thread_count)
    }

    pub fn update_thread_count(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.thread_count = value.try_i64().map(|value| value as i64).unwrap_or(self.thread_count.clone());
        self.root.set(self.entity_key(), "thread_count", value);
        self
    }

    pub fn changed_thread_count(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "thread_count")
    }

    pub fn eval_thread_count(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("thread_count") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "thread_count".to_string(), attempted_path: "thread_count".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.thread_count())
                }}

    pub fn memory_rss_kb(&self) -> i64 {
        self.changed_memory_rss_kb().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.memory_rss_kb)
    }

    pub fn update_memory_rss_kb(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.memory_rss_kb = value.try_i64().map(|value| value as i64).unwrap_or(self.memory_rss_kb.clone());
        self.root.set(self.entity_key(), "memory_rss_kb", value);
        self
    }

    pub fn changed_memory_rss_kb(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "memory_rss_kb")
    }

    pub fn eval_memory_rss_kb(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("memory_rss_kb") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "memory_rss_kb".to_string(), attempted_path: "memory_rss_kb".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.memory_rss_kb())
                }}

    pub fn memory_vms_kb(&self) -> i64 {
        self.changed_memory_vms_kb().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.memory_vms_kb)
    }

    pub fn update_memory_vms_kb(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.memory_vms_kb = value.try_i64().map(|value| value as i64).unwrap_or(self.memory_vms_kb.clone());
        self.root.set(self.entity_key(), "memory_vms_kb", value);
        self
    }

    pub fn changed_memory_vms_kb(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "memory_vms_kb")
    }

    pub fn eval_memory_vms_kb(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("memory_vms_kb") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "memory_vms_kb".to_string(), attempted_path: "memory_vms_kb".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.memory_vms_kb())
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
    pub fn system_info_id(&self) -> u64 {
        self.changed_system_info_id().and_then(|value| value.try_u64()).unwrap_or(self.system_info_id)
    }

    pub fn update_system_info_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.system_info_id = value.try_u64().unwrap_or(self.system_info_id.clone());
        self.root.set(self.entity_key(), "system_info_id", value);
        self
    }

    pub fn changed_system_info_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "system_info_id")
    }

    pub fn eval_system_info_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("system_info_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_info_id".to_string(), attempted_path: "system_info_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.system_info_id())
                }}
    pub fn system_info(&self) -> Option<&crate::SystemInfo> {
        self.system_info.as_ref()
    }

    pub fn eval_system_info(&self) -> teaql_core::eval::EvalResult<&crate::SystemInfo> {
        if !self.is_loaded("system_info") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_info".to_string(), attempted_path: "system_info".to_string() }
        } else {
            match &self.system_info {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn thread_list(&self) -> &SmartList<crate::Thread> {
        &self.thread_list
    }

    pub fn thread_list_mut(&mut self) -> &mut SmartList<crate::Thread> {
        &mut self.thread_list
    }

    pub fn eval_thread_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Thread>> {
        if !self.is_loaded("thread_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "thread_list".to_string(), attempted_path: "thread_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.thread_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ProcessRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .process_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

