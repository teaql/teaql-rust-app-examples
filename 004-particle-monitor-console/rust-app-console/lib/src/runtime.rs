
use crate::*;
use teaql_core::TeaqlEntity;

use teaql_provider_sqlite::SqliteProviderExt as _;

pub type DataServiceDialect = teaql_provider_sqlite::SqliteDialect;
pub type DataServiceMutationExecutor = teaql_provider_sqlite::SqliteMutationExecutor;
pub type DataServiceMutationError = teaql_provider_sqlite::MutationExecutorError;
pub type DataServiceIdGenerator = teaql_provider_sqlite::SqliteIdSpaceGenerator;
pub type DataServicePool = std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>;
pub type DataServiceExecutor = ServiceRuntimeExecutor;
pub type ServiceRuntime = teaql_runtime::UserContext;

pub const DATABASE_URL_ENV: &str = "PMS_SERVICE_CORE_DATABASE_URL";
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRuntimeConfig {
    pub database_url: String,
}

impl ServiceRuntimeConfig {
    pub fn from_env() -> Result<Self, ServiceRuntimeError> {
        Ok(Self {
            database_url: env_value(DATABASE_URL_ENV)?,
        })
    }
}

#[derive(Debug)]
pub enum ServiceRuntimeError {
    MissingEnv {
        name: &'static str,
        source: std::env::VarError,
    },
    ConnectionError(String),
    Rusqlite(rusqlite::Error),
    Runtime(teaql_runtime::RuntimeError),
}

impl std::fmt::Display for ServiceRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceRuntimeError::MissingEnv { name, source } => {
                write!(f, "missing environment variable {name}: {source}")
            }
            ServiceRuntimeError::ConnectionError(err) => write!(f, "connection error: {err}"),
            ServiceRuntimeError::Rusqlite(err) => write!(f, "rusqlite error: {err}"),
            ServiceRuntimeError::Runtime(err) => write!(f, "runtime error: {err}"),
        }
    }
}

impl std::error::Error for ServiceRuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceRuntimeError::MissingEnv { source, .. } => Some(source),
            ServiceRuntimeError::ConnectionError(_) => None,
            ServiceRuntimeError::Rusqlite(err) => Some(err),
            ServiceRuntimeError::Runtime(err) => Some(err),
        }
    }
}

impl From<rusqlite::Error> for ServiceRuntimeError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceRuntimeError::Rusqlite(err)
    }
}
impl From<teaql_runtime::RuntimeError> for ServiceRuntimeError {
    fn from(err: teaql_runtime::RuntimeError) -> Self {
        ServiceRuntimeError::Runtime(err)
    }
}

#[derive(Clone)]
pub struct LocalSchemaProvider;

impl teaql_data_service::SchemaProvider for LocalSchemaProvider {
    fn get_entity(&self, name: &str) -> Option<std::sync::Arc<teaql_core::EntityDescriptor>> {
        match name {
            "DeviceSystem" => Some(std::sync::Arc::new(crate::DeviceSystem::entity_descriptor())),
            "SystemStatus" => Some(std::sync::Arc::new(crate::SystemStatus::entity_descriptor())),
            "DeviceSetting" => Some(std::sync::Arc::new(crate::DeviceSetting::entity_descriptor())),
            "SampleRecord" => Some(std::sync::Arc::new(crate::SampleRecord::entity_descriptor())),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ServiceRuntimeExecutor {
    inner: teaql_sql::SqlDataServiceExecutor<
        DataServiceDialect,
        DataServiceMutationExecutor,
        LocalSchemaProvider
    >,
}

impl ServiceRuntimeExecutor {
    pub fn new(inner: DataServiceMutationExecutor) -> Self {
        Self {
            inner: teaql_sql::SqlDataServiceExecutor::new(
                DataServiceDialect::default(),
                inner,
                LocalSchemaProvider
            ),
        }
    }

}

impl teaql_data_service::DataServiceExecutor for ServiceRuntimeExecutor {
    type Error = teaql_sql::SqlExecutorError<DataServiceMutationError>;
    fn capabilities(&self) -> teaql_data_service::DataServiceCapabilities {
        teaql_data_service::DataServiceExecutor::capabilities(&self.inner)
    }
}

impl teaql_data_service::QueryExecutor for ServiceRuntimeExecutor {
    async fn query(&self, request: teaql_data_service::QueryRequest) -> Result<teaql_data_service::QueryResult, Self::Error> {
        teaql_data_service::QueryExecutor::query(&self.inner, request).await
    }
}

impl teaql_data_service::StreamQueryExecutor for ServiceRuntimeExecutor {
    fn query_stream(&self, request: teaql_data_service::QueryRequest, chunk_size: usize) -> teaql_data_service::QueryStream<'_, Self::Error> {
        teaql_data_service::StreamQueryExecutor::query_stream(&self.inner, request, chunk_size)
    }
}

impl teaql_data_service::MutationExecutor for ServiceRuntimeExecutor {
    async fn mutate(&self, request: teaql_data_service::MutationRequest) -> Result<teaql_data_service::MutationResult, Self::Error> {
        teaql_data_service::MutationExecutor::mutate(&self.inner, request).await
    }
}

impl teaql_data_service::TransactionExecutor for ServiceRuntimeExecutor {
    type Tx<'a> = teaql_sql::SqlDataServiceTransaction<'a, DataServiceDialect, <DataServiceMutationExecutor as teaql_sql::SqlTransactionTransport>::Tx<'a>, LocalSchemaProvider> where Self: 'a;

    async fn begin(&self) -> Result<Self::Tx<'_ >, Self::Error> {
        teaql_data_service::TransactionExecutor::begin(&self.inner).await
    }
}

pub async fn service_runtime_from_env() -> Result<ServiceRuntime, ServiceRuntimeError> {
    service_runtime(ServiceRuntimeConfig::from_env()?).await
}

pub async fn service_runtime(config: ServiceRuntimeConfig) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let pool = connect_data_service_pool(&config).await?;
    service_runtime_from_pool(pool).await
}

pub async fn service_runtime_from_pool(pool: DataServicePool) -> Result<ServiceRuntime, ServiceRuntimeError> {
    let mutation_executor = DataServiceMutationExecutor::new(pool);
    let id_generator = DataServiceIdGenerator::from_executor(mutation_executor.clone());let mut context = module_with_behaviors_and_checkers().into_context();
    context.set_internal_id_generator(id_generator);
    context.use_sqlite_provider(mutation_executor.clone());
    let executor = ServiceRuntimeExecutor::new(mutation_executor);
    context.register_executor(executor.clone());
    context.insert_resource(executor);

    // SQL logging is owned by teaql-runtime. The legacy teaql-tool-core
    // environment whitelist rejects runtime logging variables and must not
    // gate generated application startup. Schema remains explicit.

    Ok(context)
}



fn env_value(name: &'static str) -> Result<String, ServiceRuntimeError> {
    std::env::var(name).map_err(|source| ServiceRuntimeError::MissingEnv { name, source })
}

async fn connect_data_service_pool(config: &ServiceRuntimeConfig) -> Result<DataServicePool, ServiceRuntimeError> {
    let url = &config.database_url;
    let sanitized_url = if url.starts_with("sqlite:") { url.strip_prefix("sqlite:").unwrap().trim_start_matches("//") } else { url };
    let pure_file_path = sanitized_url.split('?').next().unwrap_or(sanitized_url);
    let path = std::path::Path::new(pure_file_path);
    if let Some(parent) = path.parent() { if !parent.as_os_str().is_empty() { std::fs::create_dir_all(parent).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?; } }
    Ok(std::sync::Arc::new(std::sync::Mutex::new(rusqlite::Connection::open(pure_file_path).map_err(|e| ServiceRuntimeError::ConnectionError(e.to_string()))?)))
}

pub fn repository_registry() -> teaql_runtime::InMemoryEntityRegistry {
    teaql_runtime::InMemoryEntityRegistry::new()
        .with_entity("DeviceSystem")
        .with_entity("SystemStatus")
        .with_entity("DeviceSetting")
        .with_entity("SampleRecord")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("DeviceSystem", DeviceSystemBehavior::default())
        .with_behavior("SystemStatus", SystemStatusBehavior::default())
        .with_behavior("DeviceSetting", DeviceSettingBehavior::default())
        .with_behavior("SampleRecord", SampleRecordBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<DeviceSystem, _>::new(DeviceSystemChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<SystemStatus, _>::new(SystemStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<DeviceSetting, _>::new(DeviceSettingChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<SampleRecord, _>::new(SampleRecordChecker::default()))
}

fn is_generated_bootstrap_retryable_conflict(error: &teaql_runtime::RuntimeError) -> bool {
    if matches!(error, teaql_runtime::RuntimeError::OptimisticLockConflict { .. }) { return true }
    let teaql_runtime::RuntimeError::Graph(message) = error else { return false };
    (message.contains("UNIQUE constraint failed:") && message.contains("_data.id"))
        || (message.contains("duplicate key value violates unique constraint") && message.contains("_data_pkey"))
        || (message.contains("Duplicate entry") && message.contains("PRIMARY"))
}

fn ensure_generated_bootstrap<'a>(context: &'a teaql_runtime::UserContext) -> teaql_runtime::GeneratedSchemaBootstrapFuture<'a> {
    Box::pin(async move {
        for attempt in 0..5 {
            match ensure_generated_bootstrap_once(context).await {
                Ok(()) => return Ok(()),
                Err(error) if attempt < 4 && is_generated_bootstrap_retryable_conflict(&error) => tokio::task::yield_now().await,
                Err(error) => return Err(error),
            }
        }
        unreachable!("bootstrap attempts always return or fail")
    })
}

async fn ensure_generated_bootstrap_once(context: &teaql_runtime::UserContext) -> Result<(), teaql_runtime::RuntimeError> {
        use teaql_core::Entity as _;
        let root_rows = crate::Q::device_systems().select_self_fields().with_id_is(1_u64).comment("what: locate generated Domain Root").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        let domain_root = if let Some(entity) = root_rows.data.into_iter().next() { entity } else {
            let mut entity = DeviceSystem::runtime_new(context.entity_runtime_state());
            entity.update_id(1_u64);
            context.initialize_generated_bootstrap_entity(&mut entity, DeviceSystem::ENTITY_NAME, 1_u64)?;
            entity.update_name("PMS-GT660X Terminal");
            entity.update_serial_number("PMS-2026-0701");
            teaql_runtime::AuditedSaveExt::save(entity.audit_as("create generated Domain Root DeviceSystem"), context).await?
        };
        context.set_generated_bootstrap_active_root(DeviceSystem::ENTITY_NAME, domain_root.id())?;
        let rows_constant_system_status_1001 = crate::Q::system_statuses().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_system_status_1001) = rows_constant_system_status_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_system_status_1001.device_system_id() != 1_u64 { constant_system_status_1001.update_device_system_id(1_u64); changed = true; }
            if constant_system_status_1001.name() != "Online" { constant_system_status_1001.update_name("Online"); changed = true; }
            if constant_system_status_1001.code() != "ONLINE" { constant_system_status_1001.update_code("ONLINE"); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1001.audit_as("reconcile model constant SystemStatus(1001)"), context).await?; }
        } else {
            let mut constant_system_status_1001 = SystemStatus::runtime_new(context.entity_runtime_state());
            constant_system_status_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_system_status_1001, SystemStatus::ENTITY_NAME, 1001_u64)?;
            constant_system_status_1001.update_device_system_id(1_u64);
            constant_system_status_1001.update_name("Online");
            constant_system_status_1001.update_code("ONLINE");
            let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1001.audit_as("create model constant SystemStatus(1001)"), context).await?;
        }
        let rows_constant_system_status_1002 = crate::Q::system_statuses().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_system_status_1002) = rows_constant_system_status_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_system_status_1002.device_system_id() != 1_u64 { constant_system_status_1002.update_device_system_id(1_u64); changed = true; }
            if constant_system_status_1002.name() != "Offline" { constant_system_status_1002.update_name("Offline"); changed = true; }
            if constant_system_status_1002.code() != "OFFLINE" { constant_system_status_1002.update_code("OFFLINE"); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1002.audit_as("reconcile model constant SystemStatus(1002)"), context).await?; }
        } else {
            let mut constant_system_status_1002 = SystemStatus::runtime_new(context.entity_runtime_state());
            constant_system_status_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_system_status_1002, SystemStatus::ENTITY_NAME, 1002_u64)?;
            constant_system_status_1002.update_device_system_id(1_u64);
            constant_system_status_1002.update_name("Offline");
            constant_system_status_1002.update_code("OFFLINE");
            let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1002.audit_as("create model constant SystemStatus(1002)"), context).await?;
        }
        let rows_constant_system_status_1003 = crate::Q::system_statuses().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_system_status_1003) = rows_constant_system_status_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_system_status_1003.device_system_id() != 1_u64 { constant_system_status_1003.update_device_system_id(1_u64); changed = true; }
            if constant_system_status_1003.name() != "Sampling" { constant_system_status_1003.update_name("Sampling"); changed = true; }
            if constant_system_status_1003.code() != "SAMPLING" { constant_system_status_1003.update_code("SAMPLING"); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1003.audit_as("reconcile model constant SystemStatus(1003)"), context).await?; }
        } else {
            let mut constant_system_status_1003 = SystemStatus::runtime_new(context.entity_runtime_state());
            constant_system_status_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_system_status_1003, SystemStatus::ENTITY_NAME, 1003_u64)?;
            constant_system_status_1003.update_device_system_id(1_u64);
            constant_system_status_1003.update_name("Sampling");
            constant_system_status_1003.update_code("SAMPLING");
            let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1003.audit_as("create model constant SystemStatus(1003)"), context).await?;
        }
        let rows_constant_system_status_1004 = crate::Q::system_statuses().select_self_fields().with_id_is(1004_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_system_status_1004) = rows_constant_system_status_1004.data.into_iter().next() {
            let mut changed = false;
            if constant_system_status_1004.device_system_id() != 1_u64 { constant_system_status_1004.update_device_system_id(1_u64); changed = true; }
            if constant_system_status_1004.name() != "Calibrating" { constant_system_status_1004.update_name("Calibrating"); changed = true; }
            if constant_system_status_1004.code() != "CALIBRATING" { constant_system_status_1004.update_code("CALIBRATING"); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1004.audit_as("reconcile model constant SystemStatus(1004)"), context).await?; }
        } else {
            let mut constant_system_status_1004 = SystemStatus::runtime_new(context.entity_runtime_state());
            constant_system_status_1004.update_id(1004_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_system_status_1004, SystemStatus::ENTITY_NAME, 1004_u64)?;
            constant_system_status_1004.update_device_system_id(1_u64);
            constant_system_status_1004.update_name("Calibrating");
            constant_system_status_1004.update_code("CALIBRATING");
            let _ = teaql_runtime::AuditedSaveExt::save(constant_system_status_1004.audit_as("create model constant SystemStatus(1004)"), context).await?;
        }
        Ok(())
}


/// Canonical KSML field to selected JSON wire name, consumed by HTTP/TFP adapters.
pub fn generated_wire_field_mappings() -> std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>> {
    std::collections::BTreeMap::from([
        ("DeviceSystem".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("serial_number".to_owned(), "serialNumber".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("SystemStatus".to_owned(), std::collections::BTreeMap::from([
            ("device_system".to_owned(), "deviceSystem".to_owned()),
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("DeviceSetting".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("device_system".to_owned(), "deviceSystem".to_owned()),
            ("calibration_point".to_owned(), "calibrationPoint".to_owned()),
            ("data_keep_days".to_owned(), "dataKeepDays".to_owned()),
            ("sampling_frequency".to_owned(), "samplingFrequency".to_owned()),
            ("password_enabled".to_owned(), "passwordEnabled".to_owned()),
            ("password_hash".to_owned(), "passwordHash".to_owned()),
            ("super_password_hash".to_owned(), "superPasswordHash".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("SampleRecord".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("device_system".to_owned(), "deviceSystem".to_owned()),
            ("system_status".to_owned(), "systemStatus".to_owned()),
            ("sample_time".to_owned(), "sampleTime".to_owned()),
            ("gas".to_owned(), "gas".to_owned()),
            ("lref".to_owned(), "lref".to_owned()),
            ("impurity1".to_owned(), "impurity1".to_owned()),
            ("impurity2".to_owned(), "impurity2".to_owned()),
            ("impurity3".to_owned(), "impurity3".to_owned()),
            ("impurity4".to_owned(), "impurity4".to_owned()),
            ("impurity5".to_owned(), "impurity5".to_owned()),
            ("impurity6".to_owned(), "impurity6".to_owned()),
            ("impurity7".to_owned(), "impurity7".to_owned()),
            ("impurity8".to_owned(), "impurity8".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ]))
    ])
}

/// Accepted legacy aliases; empty until explicitly declared by the model.
pub fn generated_wire_field_aliases() -> std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>> {
    std::collections::BTreeMap::new()
}

pub fn module() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<DeviceSystem>()
        .entity::<SystemStatus>()
        .entity::<DeviceSetting>()
        .entity::<SampleRecord>()
        .generated_schema_bootstrap(ensure_generated_bootstrap)
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity::<DeviceSystem>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<DeviceSystem, _>::new(DeviceSystemChecker::default()));
    module = module.entity::<SystemStatus>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<SystemStatus, _>::new(SystemStatusChecker::default()));
    module = module.entity::<DeviceSetting>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<DeviceSetting, _>::new(DeviceSettingChecker::default()));
    module = module.entity::<SampleRecord>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<SampleRecord, _>::new(SampleRecordChecker::default()));
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity_with_behavior::<DeviceSystem, _>(DeviceSystemBehavior::default());
    module = module.entity_with_behavior::<SystemStatus, _>(SystemStatusBehavior::default());
    module = module.entity_with_behavior::<DeviceSetting, _>(DeviceSettingBehavior::default());
    module = module.entity_with_behavior::<SampleRecord, _>(SampleRecordBehavior::default());
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity_with_behavior::<DeviceSystem, _>(DeviceSystemBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<DeviceSystem, _>::new(DeviceSystemChecker::default()));
    module = module.entity_with_behavior::<SystemStatus, _>(SystemStatusBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<SystemStatus, _>::new(SystemStatusChecker::default()));
    module = module.entity_with_behavior::<DeviceSetting, _>(DeviceSettingBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<DeviceSetting, _>::new(DeviceSettingChecker::default()));
    module = module.entity_with_behavior::<SampleRecord, _>(SampleRecordBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<SampleRecord, _>::new(SampleRecordChecker::default()));
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}