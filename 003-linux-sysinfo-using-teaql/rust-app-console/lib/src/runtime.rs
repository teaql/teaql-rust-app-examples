use crate::*;
use teaql_core::TeaqlEntity;

pub type DataServiceExecutor = teaql_provider_linux::LinuxDataServiceExecutor;
pub type ServiceRuntime = teaql_runtime::UserContext;

#[derive(Debug)]
pub enum ServiceRuntimeError {
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
    fn from(err: teaql_runtime::RuntimeError) -> Self {
        ServiceRuntimeError::Runtime(err)
    }
}

pub async fn service_runtime_from_env() -> Result<ServiceRuntime, ServiceRuntimeError> {
    service_runtime().await
}

pub async fn service_runtime() -> Result<ServiceRuntime, ServiceRuntimeError> {
    let mut context = module_with_behaviors_and_checkers().into_context();
    context.register_executor(DataServiceExecutor::new());
    Ok(context)
}

pub fn repository_registry() -> teaql_runtime::InMemoryEntityRegistry {
    teaql_runtime::InMemoryEntityRegistry::new()
        .with_entity("SystemInfo")
        .with_entity("Process")
        .with_entity("Thread")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("SystemInfo", SystemInfoBehavior::default())
        .with_behavior("Process", ProcessBehavior::default())
        .with_behavior("Thread", ThreadBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<SystemInfo, _>::new(SystemInfoChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Process, _>::new(ProcessChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Thread, _>::new(ThreadChecker::default()))
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<SystemInfo>()
        .entity::<Process>()
        .entity::<Thread>()
        .initial_graph(teaql_runtime::GraphNode::new("SystemInfo")
            .value("id", 1_u64)
            .value("hostname", "localhost")
            .value("cpu_count", 16_i64)
            .value("memory_total_bytes", 85899345920_i64)
            .value("memory_available_bytes", 85899345920_i64)
            .value("load_avg_1", "1.5")
            .value("load_avg_5", "1.2")
            .value("load_avg_15", "1.0")
            .value("uptime_seconds", "3600.0")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<SystemInfo>()
        .checker(teaql_runtime::TypedEntityChecker::<SystemInfo, _>::new(SystemInfoChecker::default()))
        .entity::<Process>()
        .checker(teaql_runtime::TypedEntityChecker::<Process, _>::new(ProcessChecker::default()))
        .entity::<Thread>()
        .checker(teaql_runtime::TypedEntityChecker::<Thread, _>::new(ThreadChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("SystemInfo")
            .value("id", 1_u64)
            .value("hostname", "localhost")
            .value("cpu_count", 16_i64)
            .value("memory_total_bytes", 85899345920_i64)
            .value("memory_available_bytes", 85899345920_i64)
            .value("load_avg_1", "1.5")
            .value("load_avg_5", "1.2")
            .value("load_avg_15", "1.0")
            .value("uptime_seconds", "3600.0")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<SystemInfo, _>(SystemInfoBehavior::default())
        .entity_with_behavior::<Process, _>(ProcessBehavior::default())
        .entity_with_behavior::<Thread, _>(ThreadBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("SystemInfo")
            .value("id", 1_u64)
            .value("hostname", "localhost")
            .value("cpu_count", 16_i64)
            .value("memory_total_bytes", 85899345920_i64)
            .value("memory_available_bytes", 85899345920_i64)
            .value("load_avg_1", "1.5")
            .value("load_avg_5", "1.2")
            .value("load_avg_15", "1.0")
            .value("uptime_seconds", "3600.0")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<SystemInfo, _>(SystemInfoBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<SystemInfo, _>::new(SystemInfoChecker::default()))
        .entity_with_behavior::<Process, _>(ProcessBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Process, _>::new(ProcessChecker::default()))
        .entity_with_behavior::<Thread, _>(ThreadBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Thread, _>::new(ThreadChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("SystemInfo")
            .value("id", 1_u64)
            .value("hostname", "localhost")
            .value("cpu_count", 16_i64)
            .value("memory_total_bytes", 85899345920_i64)
            .value("memory_available_bytes", 85899345920_i64)
            .value("load_avg_1", "1.5")
            .value("load_avg_5", "1.2")
            .value("load_avg_15", "1.0")
            .value("uptime_seconds", "3600.0")
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
}