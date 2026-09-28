
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

pub const DATABASE_URL_ENV: &str = "FIFA_WORLD_CUP_2026_SERVICE_CORE_DATABASE_URL";
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
            "MatchStage" => Some(std::sync::Arc::new(crate::MatchStage::entity_descriptor())),
            "MatchStatus" => Some(std::sync::Arc::new(crate::MatchStatus::entity_descriptor())),
            "GoalCategory" => Some(std::sync::Arc::new(crate::GoalCategory::entity_descriptor())),
            "CardCategory" => Some(std::sync::Arc::new(crate::CardCategory::entity_descriptor())),
            "Confederation" => Some(std::sync::Arc::new(crate::Confederation::entity_descriptor())),
            "Tournament" => Some(std::sync::Arc::new(crate::Tournament::entity_descriptor())),
            "TournamentTeam" => Some(std::sync::Arc::new(crate::TournamentTeam::entity_descriptor())),
            "MatchGroup" => Some(std::sync::Arc::new(crate::MatchGroup::entity_descriptor())),
            "TournamentMatch" => Some(std::sync::Arc::new(crate::TournamentMatch::entity_descriptor())),
            "MatchGoal" => Some(std::sync::Arc::new(crate::MatchGoal::entity_descriptor())),
            "MatchCard" => Some(std::sync::Arc::new(crate::MatchCard::entity_descriptor())),
            "GroupStanding" => Some(std::sync::Arc::new(crate::GroupStanding::entity_descriptor())),
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
        .with_entity("MatchStage")
        .with_entity("MatchStatus")
        .with_entity("GoalCategory")
        .with_entity("CardCategory")
        .with_entity("Confederation")
        .with_entity("Tournament")
        .with_entity("TournamentTeam")
        .with_entity("MatchGroup")
        .with_entity("TournamentMatch")
        .with_entity("MatchGoal")
        .with_entity("MatchCard")
        .with_entity("GroupStanding")
}

pub fn behavior_registry() -> teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry {
    teaql_runtime::InMemoryEntityDataServiceBehaviorRegistry::new()
        .with_behavior("MatchStage", MatchStageBehavior::default())
        .with_behavior("MatchStatus", MatchStatusBehavior::default())
        .with_behavior("GoalCategory", GoalCategoryBehavior::default())
        .with_behavior("CardCategory", CardCategoryBehavior::default())
        .with_behavior("Confederation", ConfederationBehavior::default())
        .with_behavior("Tournament", TournamentBehavior::default())
        .with_behavior("TournamentTeam", TournamentTeamBehavior::default())
        .with_behavior("MatchGroup", MatchGroupBehavior::default())
        .with_behavior("TournamentMatch", TournamentMatchBehavior::default())
        .with_behavior("MatchGoal", MatchGoalBehavior::default())
        .with_behavior("MatchCard", MatchCardBehavior::default())
        .with_behavior("GroupStanding", GroupStandingBehavior::default())
}

pub fn checker_registry() -> teaql_runtime::InMemoryCheckerRegistry {
    teaql_runtime::InMemoryCheckerRegistry::new()
        .with_checker(teaql_runtime::TypedEntityChecker::<MatchStage, _>::new(MatchStageChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MatchStatus, _>::new(MatchStatusChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<GoalCategory, _>::new(GoalCategoryChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<CardCategory, _>::new(CardCategoryChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Confederation, _>::new(ConfederationChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<Tournament, _>::new(TournamentChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TournamentTeam, _>::new(TournamentTeamChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MatchGroup, _>::new(MatchGroupChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<TournamentMatch, _>::new(TournamentMatchChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MatchGoal, _>::new(MatchGoalChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<MatchCard, _>::new(MatchCardChecker::default()))
        .with_checker(teaql_runtime::TypedEntityChecker::<GroupStanding, _>::new(GroupStandingChecker::default()))
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
        let root_rows = crate::Q::tournaments().select_self_fields().with_id_is(1_u64).comment("what: locate generated Domain Root").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        let domain_root = if let Some(entity) = root_rows.data.into_iter().next() { entity } else {
            let mut entity = Tournament::runtime_new(context.entity_runtime_state());
            entity.update_id(1_u64);
            context.initialize_generated_bootstrap_entity(&mut entity, Tournament::ENTITY_NAME, 1_u64)?;
            entity.update_tournament_name("FIFA World Cup 2026");
            entity.update_host_countries("United States");
            entity.update_start_date(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            entity.update_end_date(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            entity.update_total_teams(48_i64);
            teaql_runtime::AuditedSaveExt::save(entity.audit_as("create generated Domain Root Tournament"), context).await?
        };
        context.set_generated_bootstrap_active_root(Tournament::ENTITY_NAME, domain_root.id())?;
        let rows_constant_match_stage_1001 = crate::Q::match_stages().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1001) = rows_constant_match_stage_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1001.name() != "Group" { constant_match_stage_1001.update_name("Group"); changed = true; }
            if constant_match_stage_1001.code() != "GROUP" { constant_match_stage_1001.update_code("GROUP"); changed = true; }
            if constant_match_stage_1001.tournament_id() != 1_u64 { constant_match_stage_1001.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1001.audit_as("reconcile model constant MatchStage(1001)"), context).await?; }
        } else {
            let mut constant_match_stage_1001 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1001, MatchStage::ENTITY_NAME, 1001_u64)?;
            constant_match_stage_1001.update_name("Group");
            constant_match_stage_1001.update_code("GROUP");
            constant_match_stage_1001.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1001.audit_as("create model constant MatchStage(1001)"), context).await?;
        }
        let rows_constant_match_stage_1002 = crate::Q::match_stages().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1002) = rows_constant_match_stage_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1002.name() != "Round of 32" { constant_match_stage_1002.update_name("Round of 32"); changed = true; }
            if constant_match_stage_1002.code() != "ROUND_OF_32" { constant_match_stage_1002.update_code("ROUND_OF_32"); changed = true; }
            if constant_match_stage_1002.tournament_id() != 1_u64 { constant_match_stage_1002.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1002.audit_as("reconcile model constant MatchStage(1002)"), context).await?; }
        } else {
            let mut constant_match_stage_1002 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1002, MatchStage::ENTITY_NAME, 1002_u64)?;
            constant_match_stage_1002.update_name("Round of 32");
            constant_match_stage_1002.update_code("ROUND_OF_32");
            constant_match_stage_1002.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1002.audit_as("create model constant MatchStage(1002)"), context).await?;
        }
        let rows_constant_match_stage_1003 = crate::Q::match_stages().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1003) = rows_constant_match_stage_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1003.name() != "Round of 16" { constant_match_stage_1003.update_name("Round of 16"); changed = true; }
            if constant_match_stage_1003.code() != "ROUND_OF_16" { constant_match_stage_1003.update_code("ROUND_OF_16"); changed = true; }
            if constant_match_stage_1003.tournament_id() != 1_u64 { constant_match_stage_1003.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1003.audit_as("reconcile model constant MatchStage(1003)"), context).await?; }
        } else {
            let mut constant_match_stage_1003 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1003, MatchStage::ENTITY_NAME, 1003_u64)?;
            constant_match_stage_1003.update_name("Round of 16");
            constant_match_stage_1003.update_code("ROUND_OF_16");
            constant_match_stage_1003.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1003.audit_as("create model constant MatchStage(1003)"), context).await?;
        }
        let rows_constant_match_stage_1004 = crate::Q::match_stages().select_self_fields().with_id_is(1004_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1004) = rows_constant_match_stage_1004.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1004.name() != "Quarter Final" { constant_match_stage_1004.update_name("Quarter Final"); changed = true; }
            if constant_match_stage_1004.code() != "QUARTER_FINAL" { constant_match_stage_1004.update_code("QUARTER_FINAL"); changed = true; }
            if constant_match_stage_1004.tournament_id() != 1_u64 { constant_match_stage_1004.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1004.audit_as("reconcile model constant MatchStage(1004)"), context).await?; }
        } else {
            let mut constant_match_stage_1004 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1004.update_id(1004_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1004, MatchStage::ENTITY_NAME, 1004_u64)?;
            constant_match_stage_1004.update_name("Quarter Final");
            constant_match_stage_1004.update_code("QUARTER_FINAL");
            constant_match_stage_1004.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1004.audit_as("create model constant MatchStage(1004)"), context).await?;
        }
        let rows_constant_match_stage_1005 = crate::Q::match_stages().select_self_fields().with_id_is(1005_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1005) = rows_constant_match_stage_1005.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1005.name() != "Semi Final" { constant_match_stage_1005.update_name("Semi Final"); changed = true; }
            if constant_match_stage_1005.code() != "SEMI_FINAL" { constant_match_stage_1005.update_code("SEMI_FINAL"); changed = true; }
            if constant_match_stage_1005.tournament_id() != 1_u64 { constant_match_stage_1005.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1005.audit_as("reconcile model constant MatchStage(1005)"), context).await?; }
        } else {
            let mut constant_match_stage_1005 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1005.update_id(1005_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1005, MatchStage::ENTITY_NAME, 1005_u64)?;
            constant_match_stage_1005.update_name("Semi Final");
            constant_match_stage_1005.update_code("SEMI_FINAL");
            constant_match_stage_1005.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1005.audit_as("create model constant MatchStage(1005)"), context).await?;
        }
        let rows_constant_match_stage_1006 = crate::Q::match_stages().select_self_fields().with_id_is(1006_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1006) = rows_constant_match_stage_1006.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1006.name() != "Third Place" { constant_match_stage_1006.update_name("Third Place"); changed = true; }
            if constant_match_stage_1006.code() != "THIRD_PLACE" { constant_match_stage_1006.update_code("THIRD_PLACE"); changed = true; }
            if constant_match_stage_1006.tournament_id() != 1_u64 { constant_match_stage_1006.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1006.audit_as("reconcile model constant MatchStage(1006)"), context).await?; }
        } else {
            let mut constant_match_stage_1006 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1006.update_id(1006_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1006, MatchStage::ENTITY_NAME, 1006_u64)?;
            constant_match_stage_1006.update_name("Third Place");
            constant_match_stage_1006.update_code("THIRD_PLACE");
            constant_match_stage_1006.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1006.audit_as("create model constant MatchStage(1006)"), context).await?;
        }
        let rows_constant_match_stage_1007 = crate::Q::match_stages().select_self_fields().with_id_is(1007_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_stage_1007) = rows_constant_match_stage_1007.data.into_iter().next() {
            let mut changed = false;
            if constant_match_stage_1007.name() != "Final" { constant_match_stage_1007.update_name("Final"); changed = true; }
            if constant_match_stage_1007.code() != "FINAL" { constant_match_stage_1007.update_code("FINAL"); changed = true; }
            if constant_match_stage_1007.tournament_id() != 1_u64 { constant_match_stage_1007.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1007.audit_as("reconcile model constant MatchStage(1007)"), context).await?; }
        } else {
            let mut constant_match_stage_1007 = MatchStage::runtime_new(context.entity_runtime_state());
            constant_match_stage_1007.update_id(1007_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_stage_1007, MatchStage::ENTITY_NAME, 1007_u64)?;
            constant_match_stage_1007.update_name("Final");
            constant_match_stage_1007.update_code("FINAL");
            constant_match_stage_1007.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_stage_1007.audit_as("create model constant MatchStage(1007)"), context).await?;
        }
        let rows_constant_match_status_1001 = crate::Q::match_statuses().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_status_1001) = rows_constant_match_status_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_match_status_1001.name() != "Scheduled" { constant_match_status_1001.update_name("Scheduled"); changed = true; }
            if constant_match_status_1001.code() != "SCHEDULED" { constant_match_status_1001.update_code("SCHEDULED"); changed = true; }
            if constant_match_status_1001.tournament_id() != 1_u64 { constant_match_status_1001.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1001.audit_as("reconcile model constant MatchStatus(1001)"), context).await?; }
        } else {
            let mut constant_match_status_1001 = MatchStatus::runtime_new(context.entity_runtime_state());
            constant_match_status_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_status_1001, MatchStatus::ENTITY_NAME, 1001_u64)?;
            constant_match_status_1001.update_name("Scheduled");
            constant_match_status_1001.update_code("SCHEDULED");
            constant_match_status_1001.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1001.audit_as("create model constant MatchStatus(1001)"), context).await?;
        }
        let rows_constant_match_status_1002 = crate::Q::match_statuses().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_status_1002) = rows_constant_match_status_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_match_status_1002.name() != "Live" { constant_match_status_1002.update_name("Live"); changed = true; }
            if constant_match_status_1002.code() != "LIVE" { constant_match_status_1002.update_code("LIVE"); changed = true; }
            if constant_match_status_1002.tournament_id() != 1_u64 { constant_match_status_1002.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1002.audit_as("reconcile model constant MatchStatus(1002)"), context).await?; }
        } else {
            let mut constant_match_status_1002 = MatchStatus::runtime_new(context.entity_runtime_state());
            constant_match_status_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_status_1002, MatchStatus::ENTITY_NAME, 1002_u64)?;
            constant_match_status_1002.update_name("Live");
            constant_match_status_1002.update_code("LIVE");
            constant_match_status_1002.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1002.audit_as("create model constant MatchStatus(1002)"), context).await?;
        }
        let rows_constant_match_status_1003 = crate::Q::match_statuses().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_status_1003) = rows_constant_match_status_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_match_status_1003.name() != "Finished" { constant_match_status_1003.update_name("Finished"); changed = true; }
            if constant_match_status_1003.code() != "FINISHED" { constant_match_status_1003.update_code("FINISHED"); changed = true; }
            if constant_match_status_1003.tournament_id() != 1_u64 { constant_match_status_1003.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1003.audit_as("reconcile model constant MatchStatus(1003)"), context).await?; }
        } else {
            let mut constant_match_status_1003 = MatchStatus::runtime_new(context.entity_runtime_state());
            constant_match_status_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_status_1003, MatchStatus::ENTITY_NAME, 1003_u64)?;
            constant_match_status_1003.update_name("Finished");
            constant_match_status_1003.update_code("FINISHED");
            constant_match_status_1003.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1003.audit_as("create model constant MatchStatus(1003)"), context).await?;
        }
        let rows_constant_match_status_1004 = crate::Q::match_statuses().select_self_fields().with_id_is(1004_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_match_status_1004) = rows_constant_match_status_1004.data.into_iter().next() {
            let mut changed = false;
            if constant_match_status_1004.name() != "Postponed" { constant_match_status_1004.update_name("Postponed"); changed = true; }
            if constant_match_status_1004.code() != "POSTPONED" { constant_match_status_1004.update_code("POSTPONED"); changed = true; }
            if constant_match_status_1004.tournament_id() != 1_u64 { constant_match_status_1004.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1004.audit_as("reconcile model constant MatchStatus(1004)"), context).await?; }
        } else {
            let mut constant_match_status_1004 = MatchStatus::runtime_new(context.entity_runtime_state());
            constant_match_status_1004.update_id(1004_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_match_status_1004, MatchStatus::ENTITY_NAME, 1004_u64)?;
            constant_match_status_1004.update_name("Postponed");
            constant_match_status_1004.update_code("POSTPONED");
            constant_match_status_1004.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_match_status_1004.audit_as("create model constant MatchStatus(1004)"), context).await?;
        }
        let rows_constant_goal_category_1001 = crate::Q::goal_categories().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_goal_category_1001) = rows_constant_goal_category_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_goal_category_1001.name() != "Normal" { constant_goal_category_1001.update_name("Normal"); changed = true; }
            if constant_goal_category_1001.code() != "NORMAL" { constant_goal_category_1001.update_code("NORMAL"); changed = true; }
            if constant_goal_category_1001.tournament_id() != 1_u64 { constant_goal_category_1001.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1001.audit_as("reconcile model constant GoalCategory(1001)"), context).await?; }
        } else {
            let mut constant_goal_category_1001 = GoalCategory::runtime_new(context.entity_runtime_state());
            constant_goal_category_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_goal_category_1001, GoalCategory::ENTITY_NAME, 1001_u64)?;
            constant_goal_category_1001.update_name("Normal");
            constant_goal_category_1001.update_code("NORMAL");
            constant_goal_category_1001.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1001.audit_as("create model constant GoalCategory(1001)"), context).await?;
        }
        let rows_constant_goal_category_1002 = crate::Q::goal_categories().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_goal_category_1002) = rows_constant_goal_category_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_goal_category_1002.name() != "Penalty" { constant_goal_category_1002.update_name("Penalty"); changed = true; }
            if constant_goal_category_1002.code() != "PENALTY" { constant_goal_category_1002.update_code("PENALTY"); changed = true; }
            if constant_goal_category_1002.tournament_id() != 1_u64 { constant_goal_category_1002.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1002.audit_as("reconcile model constant GoalCategory(1002)"), context).await?; }
        } else {
            let mut constant_goal_category_1002 = GoalCategory::runtime_new(context.entity_runtime_state());
            constant_goal_category_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_goal_category_1002, GoalCategory::ENTITY_NAME, 1002_u64)?;
            constant_goal_category_1002.update_name("Penalty");
            constant_goal_category_1002.update_code("PENALTY");
            constant_goal_category_1002.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1002.audit_as("create model constant GoalCategory(1002)"), context).await?;
        }
        let rows_constant_goal_category_1003 = crate::Q::goal_categories().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_goal_category_1003) = rows_constant_goal_category_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_goal_category_1003.name() != "Own Goal" { constant_goal_category_1003.update_name("Own Goal"); changed = true; }
            if constant_goal_category_1003.code() != "OWN_GOAL" { constant_goal_category_1003.update_code("OWN_GOAL"); changed = true; }
            if constant_goal_category_1003.tournament_id() != 1_u64 { constant_goal_category_1003.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1003.audit_as("reconcile model constant GoalCategory(1003)"), context).await?; }
        } else {
            let mut constant_goal_category_1003 = GoalCategory::runtime_new(context.entity_runtime_state());
            constant_goal_category_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_goal_category_1003, GoalCategory::ENTITY_NAME, 1003_u64)?;
            constant_goal_category_1003.update_name("Own Goal");
            constant_goal_category_1003.update_code("OWN_GOAL");
            constant_goal_category_1003.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1003.audit_as("create model constant GoalCategory(1003)"), context).await?;
        }
        let rows_constant_goal_category_1004 = crate::Q::goal_categories().select_self_fields().with_id_is(1004_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_goal_category_1004) = rows_constant_goal_category_1004.data.into_iter().next() {
            let mut changed = false;
            if constant_goal_category_1004.name() != "Free Kick" { constant_goal_category_1004.update_name("Free Kick"); changed = true; }
            if constant_goal_category_1004.code() != "FREE_KICK" { constant_goal_category_1004.update_code("FREE_KICK"); changed = true; }
            if constant_goal_category_1004.tournament_id() != 1_u64 { constant_goal_category_1004.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1004.audit_as("reconcile model constant GoalCategory(1004)"), context).await?; }
        } else {
            let mut constant_goal_category_1004 = GoalCategory::runtime_new(context.entity_runtime_state());
            constant_goal_category_1004.update_id(1004_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_goal_category_1004, GoalCategory::ENTITY_NAME, 1004_u64)?;
            constant_goal_category_1004.update_name("Free Kick");
            constant_goal_category_1004.update_code("FREE_KICK");
            constant_goal_category_1004.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_goal_category_1004.audit_as("create model constant GoalCategory(1004)"), context).await?;
        }
        let rows_constant_card_category_1001 = crate::Q::card_categories().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_card_category_1001) = rows_constant_card_category_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_card_category_1001.name() != "Yellow" { constant_card_category_1001.update_name("Yellow"); changed = true; }
            if constant_card_category_1001.code() != "YELLOW" { constant_card_category_1001.update_code("YELLOW"); changed = true; }
            if constant_card_category_1001.tournament_id() != 1_u64 { constant_card_category_1001.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_card_category_1001.audit_as("reconcile model constant CardCategory(1001)"), context).await?; }
        } else {
            let mut constant_card_category_1001 = CardCategory::runtime_new(context.entity_runtime_state());
            constant_card_category_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_card_category_1001, CardCategory::ENTITY_NAME, 1001_u64)?;
            constant_card_category_1001.update_name("Yellow");
            constant_card_category_1001.update_code("YELLOW");
            constant_card_category_1001.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_card_category_1001.audit_as("create model constant CardCategory(1001)"), context).await?;
        }
        let rows_constant_card_category_1002 = crate::Q::card_categories().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_card_category_1002) = rows_constant_card_category_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_card_category_1002.name() != "Red" { constant_card_category_1002.update_name("Red"); changed = true; }
            if constant_card_category_1002.code() != "RED" { constant_card_category_1002.update_code("RED"); changed = true; }
            if constant_card_category_1002.tournament_id() != 1_u64 { constant_card_category_1002.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_card_category_1002.audit_as("reconcile model constant CardCategory(1002)"), context).await?; }
        } else {
            let mut constant_card_category_1002 = CardCategory::runtime_new(context.entity_runtime_state());
            constant_card_category_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_card_category_1002, CardCategory::ENTITY_NAME, 1002_u64)?;
            constant_card_category_1002.update_name("Red");
            constant_card_category_1002.update_code("RED");
            constant_card_category_1002.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_card_category_1002.audit_as("create model constant CardCategory(1002)"), context).await?;
        }
        let rows_constant_card_category_1003 = crate::Q::card_categories().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_card_category_1003) = rows_constant_card_category_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_card_category_1003.name() != "Second Yellow" { constant_card_category_1003.update_name("Second Yellow"); changed = true; }
            if constant_card_category_1003.code() != "SECOND_YELLOW" { constant_card_category_1003.update_code("SECOND_YELLOW"); changed = true; }
            if constant_card_category_1003.tournament_id() != 1_u64 { constant_card_category_1003.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_card_category_1003.audit_as("reconcile model constant CardCategory(1003)"), context).await?; }
        } else {
            let mut constant_card_category_1003 = CardCategory::runtime_new(context.entity_runtime_state());
            constant_card_category_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_card_category_1003, CardCategory::ENTITY_NAME, 1003_u64)?;
            constant_card_category_1003.update_name("Second Yellow");
            constant_card_category_1003.update_code("SECOND_YELLOW");
            constant_card_category_1003.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_card_category_1003.audit_as("create model constant CardCategory(1003)"), context).await?;
        }
        let rows_constant_confederation_1001 = crate::Q::confederations().select_self_fields().with_id_is(1001_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_confederation_1001) = rows_constant_confederation_1001.data.into_iter().next() {
            let mut changed = false;
            if constant_confederation_1001.name() != "AFC" { constant_confederation_1001.update_name("AFC"); changed = true; }
            if constant_confederation_1001.code() != "AFC" { constant_confederation_1001.update_code("AFC"); changed = true; }
            if constant_confederation_1001.tournament_id() != 1_u64 { constant_confederation_1001.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1001.audit_as("reconcile model constant Confederation(1001)"), context).await?; }
        } else {
            let mut constant_confederation_1001 = Confederation::runtime_new(context.entity_runtime_state());
            constant_confederation_1001.update_id(1001_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_confederation_1001, Confederation::ENTITY_NAME, 1001_u64)?;
            constant_confederation_1001.update_name("AFC");
            constant_confederation_1001.update_code("AFC");
            constant_confederation_1001.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1001.audit_as("create model constant Confederation(1001)"), context).await?;
        }
        let rows_constant_confederation_1002 = crate::Q::confederations().select_self_fields().with_id_is(1002_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_confederation_1002) = rows_constant_confederation_1002.data.into_iter().next() {
            let mut changed = false;
            if constant_confederation_1002.name() != "CAF" { constant_confederation_1002.update_name("CAF"); changed = true; }
            if constant_confederation_1002.code() != "CAF" { constant_confederation_1002.update_code("CAF"); changed = true; }
            if constant_confederation_1002.tournament_id() != 1_u64 { constant_confederation_1002.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1002.audit_as("reconcile model constant Confederation(1002)"), context).await?; }
        } else {
            let mut constant_confederation_1002 = Confederation::runtime_new(context.entity_runtime_state());
            constant_confederation_1002.update_id(1002_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_confederation_1002, Confederation::ENTITY_NAME, 1002_u64)?;
            constant_confederation_1002.update_name("CAF");
            constant_confederation_1002.update_code("CAF");
            constant_confederation_1002.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1002.audit_as("create model constant Confederation(1002)"), context).await?;
        }
        let rows_constant_confederation_1003 = crate::Q::confederations().select_self_fields().with_id_is(1003_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_confederation_1003) = rows_constant_confederation_1003.data.into_iter().next() {
            let mut changed = false;
            if constant_confederation_1003.name() != "CONCACAF" { constant_confederation_1003.update_name("CONCACAF"); changed = true; }
            if constant_confederation_1003.code() != "CONCACAF" { constant_confederation_1003.update_code("CONCACAF"); changed = true; }
            if constant_confederation_1003.tournament_id() != 1_u64 { constant_confederation_1003.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1003.audit_as("reconcile model constant Confederation(1003)"), context).await?; }
        } else {
            let mut constant_confederation_1003 = Confederation::runtime_new(context.entity_runtime_state());
            constant_confederation_1003.update_id(1003_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_confederation_1003, Confederation::ENTITY_NAME, 1003_u64)?;
            constant_confederation_1003.update_name("CONCACAF");
            constant_confederation_1003.update_code("CONCACAF");
            constant_confederation_1003.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1003.audit_as("create model constant Confederation(1003)"), context).await?;
        }
        let rows_constant_confederation_1004 = crate::Q::confederations().select_self_fields().with_id_is(1004_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_confederation_1004) = rows_constant_confederation_1004.data.into_iter().next() {
            let mut changed = false;
            if constant_confederation_1004.name() != "CONMEBOL" { constant_confederation_1004.update_name("CONMEBOL"); changed = true; }
            if constant_confederation_1004.code() != "CONMEBOL" { constant_confederation_1004.update_code("CONMEBOL"); changed = true; }
            if constant_confederation_1004.tournament_id() != 1_u64 { constant_confederation_1004.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1004.audit_as("reconcile model constant Confederation(1004)"), context).await?; }
        } else {
            let mut constant_confederation_1004 = Confederation::runtime_new(context.entity_runtime_state());
            constant_confederation_1004.update_id(1004_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_confederation_1004, Confederation::ENTITY_NAME, 1004_u64)?;
            constant_confederation_1004.update_name("CONMEBOL");
            constant_confederation_1004.update_code("CONMEBOL");
            constant_confederation_1004.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1004.audit_as("create model constant Confederation(1004)"), context).await?;
        }
        let rows_constant_confederation_1005 = crate::Q::confederations().select_self_fields().with_id_is(1005_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_confederation_1005) = rows_constant_confederation_1005.data.into_iter().next() {
            let mut changed = false;
            if constant_confederation_1005.name() != "OFC" { constant_confederation_1005.update_name("OFC"); changed = true; }
            if constant_confederation_1005.code() != "OFC" { constant_confederation_1005.update_code("OFC"); changed = true; }
            if constant_confederation_1005.tournament_id() != 1_u64 { constant_confederation_1005.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1005.audit_as("reconcile model constant Confederation(1005)"), context).await?; }
        } else {
            let mut constant_confederation_1005 = Confederation::runtime_new(context.entity_runtime_state());
            constant_confederation_1005.update_id(1005_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_confederation_1005, Confederation::ENTITY_NAME, 1005_u64)?;
            constant_confederation_1005.update_name("OFC");
            constant_confederation_1005.update_code("OFC");
            constant_confederation_1005.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1005.audit_as("create model constant Confederation(1005)"), context).await?;
        }
        let rows_constant_confederation_1006 = crate::Q::confederations().select_self_fields().with_id_is(1006_u64).comment("what: locate generated constant").purpose("why: idempotent runtime bootstrap").execute_for_list(context).await.map_err(|e| teaql_runtime::RuntimeError::Graph(e.to_string()))?;
        if let Some(mut constant_confederation_1006) = rows_constant_confederation_1006.data.into_iter().next() {
            let mut changed = false;
            if constant_confederation_1006.name() != "UEFA" { constant_confederation_1006.update_name("UEFA"); changed = true; }
            if constant_confederation_1006.code() != "UEFA" { constant_confederation_1006.update_code("UEFA"); changed = true; }
            if constant_confederation_1006.tournament_id() != 1_u64 { constant_confederation_1006.update_tournament_id(1_u64); changed = true; }
            if changed { let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1006.audit_as("reconcile model constant Confederation(1006)"), context).await?; }
        } else {
            let mut constant_confederation_1006 = Confederation::runtime_new(context.entity_runtime_state());
            constant_confederation_1006.update_id(1006_u64);
            context.initialize_generated_bootstrap_entity(&mut constant_confederation_1006, Confederation::ENTITY_NAME, 1006_u64)?;
            constant_confederation_1006.update_name("UEFA");
            constant_confederation_1006.update_code("UEFA");
            constant_confederation_1006.update_tournament_id(1_u64);
            let _ = teaql_runtime::AuditedSaveExt::save(constant_confederation_1006.audit_as("create model constant Confederation(1006)"), context).await?;
        }
        Ok(())
}


/// Canonical KSML field to selected JSON wire name, consumed by HTTP/TFP adapters.
pub fn generated_wire_field_mappings() -> std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>> {
    std::collections::BTreeMap::from([
        ("MatchStage".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("MatchStatus".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("GoalCategory".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("CardCategory".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("Confederation".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("name".to_owned(), "name".to_owned()),
            ("code".to_owned(), "code".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("Tournament".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("tournament_name".to_owned(), "tournamentName".to_owned()),
            ("host_countries".to_owned(), "hostCountries".to_owned()),
            ("start_date".to_owned(), "startDate".to_owned()),
            ("end_date".to_owned(), "endDate".to_owned()),
            ("total_teams".to_owned(), "totalTeams".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("TournamentTeam".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("team_name".to_owned(), "teamName".to_owned()),
            ("team_code".to_owned(), "teamCode".to_owned()),
            ("emoji_flag".to_owned(), "emojiFlag".to_owned()),
            ("fifa_ranking".to_owned(), "fifaRanking".to_owned()),
            ("manager_name".to_owned(), "managerName".to_owned()),
            ("confederation".to_owned(), "confederation".to_owned()),
            ("group_letter".to_owned(), "groupLetter".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("MatchGroup".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("group_letter".to_owned(), "groupLetter".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("TournamentMatch".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("match_number".to_owned(), "matchNumber".to_owned()),
            ("match_date".to_owned(), "matchDate".to_owned()),
            ("venue_name".to_owned(), "venueName".to_owned()),
            ("venue_city".to_owned(), "venueCity".to_owned()),
            ("venue_country".to_owned(), "venueCountry".to_owned()),
            ("home_score".to_owned(), "homeScore".to_owned()),
            ("away_score".to_owned(), "awayScore".to_owned()),
            ("extra_time_home".to_owned(), "extraTimeHome".to_owned()),
            ("extra_time_away".to_owned(), "extraTimeAway".to_owned()),
            ("penalty_home".to_owned(), "penaltyHome".to_owned()),
            ("penalty_away".to_owned(), "penaltyAway".to_owned()),
            ("home_team".to_owned(), "homeTeam".to_owned()),
            ("away_team".to_owned(), "awayTeam".to_owned()),
            ("match_stage".to_owned(), "matchStage".to_owned()),
            ("match_group".to_owned(), "matchGroup".to_owned()),
            ("match_status".to_owned(), "matchStatus".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("MatchGoal".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("player_name".to_owned(), "playerName".to_owned()),
            ("minute_scored".to_owned(), "minuteScored".to_owned()),
            ("tournament_match".to_owned(), "tournamentMatch".to_owned()),
            ("tournament_team".to_owned(), "tournamentTeam".to_owned()),
            ("goal_category".to_owned(), "goalCategory".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("MatchCard".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("player_name".to_owned(), "playerName".to_owned()),
            ("minute_issued".to_owned(), "minuteIssued".to_owned()),
            ("tournament_match".to_owned(), "tournamentMatch".to_owned()),
            ("tournament_team".to_owned(), "tournamentTeam".to_owned()),
            ("card_category".to_owned(), "cardCategory".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
            ("version".to_owned(), "version".to_owned())
        ])),
        ("GroupStanding".to_owned(), std::collections::BTreeMap::from([
            ("id".to_owned(), "id".to_owned()),
            ("played".to_owned(), "played".to_owned()),
            ("won".to_owned(), "won".to_owned()),
            ("drawn".to_owned(), "drawn".to_owned()),
            ("lost".to_owned(), "lost".to_owned()),
            ("goals_for".to_owned(), "goalsFor".to_owned()),
            ("goals_against".to_owned(), "goalsAgainst".to_owned()),
            ("goal_difference".to_owned(), "goalDifference".to_owned()),
            ("points".to_owned(), "points".to_owned()),
            ("standing_rank".to_owned(), "standingRank".to_owned()),
            ("tournament_team".to_owned(), "tournamentTeam".to_owned()),
            ("match_group".to_owned(), "matchGroup".to_owned()),
            ("tournament".to_owned(), "tournament".to_owned()),
            ("create_time".to_owned(), "createTime".to_owned()),
            ("update_time".to_owned(), "updateTime".to_owned()),
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
        .entity::<MatchStage>()
        .entity::<MatchStatus>()
        .entity::<GoalCategory>()
        .entity::<CardCategory>()
        .entity::<Confederation>()
        .entity::<Tournament>()
        .entity::<TournamentTeam>()
        .entity::<MatchGroup>()
        .entity::<TournamentMatch>()
        .entity::<MatchGoal>()
        .entity::<MatchCard>()
        .entity::<GroupStanding>()
        .generated_schema_bootstrap(ensure_generated_bootstrap)
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity::<MatchStage>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchStage, _>::new(MatchStageChecker::default()));
    module = module.entity::<MatchStatus>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchStatus, _>::new(MatchStatusChecker::default()));
    module = module.entity::<GoalCategory>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<GoalCategory, _>::new(GoalCategoryChecker::default()));
    module = module.entity::<CardCategory>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<CardCategory, _>::new(CardCategoryChecker::default()));
    module = module.entity::<Confederation>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<Confederation, _>::new(ConfederationChecker::default()));
    module = module.entity::<Tournament>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<Tournament, _>::new(TournamentChecker::default()));
    module = module.entity::<TournamentTeam>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<TournamentTeam, _>::new(TournamentTeamChecker::default()));
    module = module.entity::<MatchGroup>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchGroup, _>::new(MatchGroupChecker::default()));
    module = module.entity::<TournamentMatch>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<TournamentMatch, _>::new(TournamentMatchChecker::default()));
    module = module.entity::<MatchGoal>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchGoal, _>::new(MatchGoalChecker::default()));
    module = module.entity::<MatchCard>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchCard, _>::new(MatchCardChecker::default()));
    module = module.entity::<GroupStanding>();
    module = module.checker(teaql_runtime::TypedEntityChecker::<GroupStanding, _>::new(GroupStandingChecker::default()));
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity_with_behavior::<MatchStage, _>(MatchStageBehavior::default());
    module = module.entity_with_behavior::<MatchStatus, _>(MatchStatusBehavior::default());
    module = module.entity_with_behavior::<GoalCategory, _>(GoalCategoryBehavior::default());
    module = module.entity_with_behavior::<CardCategory, _>(CardCategoryBehavior::default());
    module = module.entity_with_behavior::<Confederation, _>(ConfederationBehavior::default());
    module = module.entity_with_behavior::<Tournament, _>(TournamentBehavior::default());
    module = module.entity_with_behavior::<TournamentTeam, _>(TournamentTeamBehavior::default());
    module = module.entity_with_behavior::<MatchGroup, _>(MatchGroupBehavior::default());
    module = module.entity_with_behavior::<TournamentMatch, _>(TournamentMatchBehavior::default());
    module = module.entity_with_behavior::<MatchGoal, _>(MatchGoalBehavior::default());
    module = module.entity_with_behavior::<MatchCard, _>(MatchCardBehavior::default());
    module = module.entity_with_behavior::<GroupStanding, _>(GroupStandingBehavior::default());
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    let mut module = teaql_runtime::RuntimeModule::new();
    module = module.entity_with_behavior::<MatchStage, _>(MatchStageBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchStage, _>::new(MatchStageChecker::default()));
    module = module.entity_with_behavior::<MatchStatus, _>(MatchStatusBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchStatus, _>::new(MatchStatusChecker::default()));
    module = module.entity_with_behavior::<GoalCategory, _>(GoalCategoryBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<GoalCategory, _>::new(GoalCategoryChecker::default()));
    module = module.entity_with_behavior::<CardCategory, _>(CardCategoryBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<CardCategory, _>::new(CardCategoryChecker::default()));
    module = module.entity_with_behavior::<Confederation, _>(ConfederationBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<Confederation, _>::new(ConfederationChecker::default()));
    module = module.entity_with_behavior::<Tournament, _>(TournamentBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<Tournament, _>::new(TournamentChecker::default()));
    module = module.entity_with_behavior::<TournamentTeam, _>(TournamentTeamBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<TournamentTeam, _>::new(TournamentTeamChecker::default()));
    module = module.entity_with_behavior::<MatchGroup, _>(MatchGroupBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchGroup, _>::new(MatchGroupChecker::default()));
    module = module.entity_with_behavior::<TournamentMatch, _>(TournamentMatchBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<TournamentMatch, _>::new(TournamentMatchChecker::default()));
    module = module.entity_with_behavior::<MatchGoal, _>(MatchGoalBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchGoal, _>::new(MatchGoalChecker::default()));
    module = module.entity_with_behavior::<MatchCard, _>(MatchCardBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<MatchCard, _>::new(MatchCardChecker::default()));
    module = module.entity_with_behavior::<GroupStanding, _>(GroupStandingBehavior::default());
    module = module.checker(teaql_runtime::TypedEntityChecker::<GroupStanding, _>::new(GroupStandingChecker::default()));
    module = module.generated_schema_bootstrap(ensure_generated_bootstrap);
    module
}