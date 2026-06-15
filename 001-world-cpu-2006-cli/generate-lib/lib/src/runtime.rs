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

pub const DATABASE_URL_ENV: &str = "FIFA_WORLD_CUP_2026_SERVICE_DATABASE_URL";
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
    async fn query_stream(&self, request: teaql_data_service::QueryRequest, chunk_size: usize) -> Result<Vec<teaql_data_service::StreamChunk>, Self::Error> {
        teaql_data_service::StreamQueryExecutor::query_stream(&self.inner, request, chunk_size).await
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
    context.insert_resource(ServiceRuntimeExecutor::new(mutation_executor));

    // 自动加载 Zero-Code 审计配置与 Schema 模式
    let env_config = teaql_tool_core::audit_config_from_env(&[
        "match_stage_data", "match_status_data", "goal_category_data", "card_category_data", "confederation_data", "tournament_data", "tournament_team_data", "match_group_data", "tournament_match_data", "match_goal_data", "match_card_data", "group_standing_data"
    ]);
    let schema_mode = env_config.schema_mode;
    context.insert_resource(env_config.config.clone());
    context.insert_resource(env_config);

    match schema_mode {
        teaql_tool_core::SchemaMode::Execute => {
            context.ensure_schema().await?;
        }
        teaql_tool_core::SchemaMode::DryRun => {
            // DryRun: 目前等效于验证
            context.ensure_schema().await?;
        }
        teaql_tool_core::SchemaMode::Verify => {
            context.ensure_schema().await?;
        }
    }

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

pub fn repository_registry() -> teaql_runtime::InMemoryRepositoryRegistry {
    teaql_runtime::InMemoryRepositoryRegistry::new()
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

pub fn behavior_registry() -> teaql_runtime::InMemoryRepositoryBehaviorRegistry {
    teaql_runtime::InMemoryRepositoryBehaviorRegistry::new()
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
        .initial_graph(teaql_runtime::GraphNode::new("Tournament")
            .value("id", 1_u64)
            .value("tournament_name", "FIFA World Cup 2026")
            .value("host_countries", "United States")
            .value("start_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("end_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("total_teams", 48_i32)
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1001_u64)
            .value("name", "Group")
            .value("code", "GROUP")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1002_u64)
            .value("name", "Round of 32")
            .value("code", "ROUND_OF_32")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1003_u64)
            .value("name", "Round of 16")
            .value("code", "ROUND_OF_16")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1004_u64)
            .value("name", "Quarter Final")
            .value("code", "QUARTER_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1005_u64)
            .value("name", "Semi Final")
            .value("code", "SEMI_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1006_u64)
            .value("name", "Third Place")
            .value("code", "THIRD_PLACE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1007_u64)
            .value("name", "Final")
            .value("code", "FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1001_u64)
            .value("name", "Scheduled")
            .value("code", "SCHEDULED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1002_u64)
            .value("name", "Live")
            .value("code", "LIVE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1003_u64)
            .value("name", "Finished")
            .value("code", "FINISHED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1004_u64)
            .value("name", "Postponed")
            .value("code", "POSTPONED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1001_u64)
            .value("name", "Normal")
            .value("code", "NORMAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1002_u64)
            .value("name", "Penalty")
            .value("code", "PENALTY")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1003_u64)
            .value("name", "Own Goal")
            .value("code", "OWN_GOAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1004_u64)
            .value("name", "Free Kick")
            .value("code", "FREE_KICK")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1001_u64)
            .value("name", "Yellow")
            .value("code", "YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1002_u64)
            .value("name", "Red")
            .value("code", "RED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1003_u64)
            .value("name", "Second Yellow")
            .value("code", "SECOND_YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1001_u64)
            .value("name", "AFC")
            .value("code", "AFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1002_u64)
            .value("name", "CAF")
            .value("code", "CAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1003_u64)
            .value("name", "CONCACAF")
            .value("code", "CONCACAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1004_u64)
            .value("name", "CONMEBOL")
            .value("code", "CONMEBOL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1005_u64)
            .value("name", "OFC")
            .value("code", "OFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1006_u64)
            .value("name", "UEFA")
            .value("code", "UEFA")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
}

pub fn module_with_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity::<MatchStage>()
        .checker(teaql_runtime::TypedEntityChecker::<MatchStage, _>::new(MatchStageChecker::default()))
        .entity::<MatchStatus>()
        .checker(teaql_runtime::TypedEntityChecker::<MatchStatus, _>::new(MatchStatusChecker::default()))
        .entity::<GoalCategory>()
        .checker(teaql_runtime::TypedEntityChecker::<GoalCategory, _>::new(GoalCategoryChecker::default()))
        .entity::<CardCategory>()
        .checker(teaql_runtime::TypedEntityChecker::<CardCategory, _>::new(CardCategoryChecker::default()))
        .entity::<Confederation>()
        .checker(teaql_runtime::TypedEntityChecker::<Confederation, _>::new(ConfederationChecker::default()))
        .entity::<Tournament>()
        .checker(teaql_runtime::TypedEntityChecker::<Tournament, _>::new(TournamentChecker::default()))
        .entity::<TournamentTeam>()
        .checker(teaql_runtime::TypedEntityChecker::<TournamentTeam, _>::new(TournamentTeamChecker::default()))
        .entity::<MatchGroup>()
        .checker(teaql_runtime::TypedEntityChecker::<MatchGroup, _>::new(MatchGroupChecker::default()))
        .entity::<TournamentMatch>()
        .checker(teaql_runtime::TypedEntityChecker::<TournamentMatch, _>::new(TournamentMatchChecker::default()))
        .entity::<MatchGoal>()
        .checker(teaql_runtime::TypedEntityChecker::<MatchGoal, _>::new(MatchGoalChecker::default()))
        .entity::<MatchCard>()
        .checker(teaql_runtime::TypedEntityChecker::<MatchCard, _>::new(MatchCardChecker::default()))
        .entity::<GroupStanding>()
        .checker(teaql_runtime::TypedEntityChecker::<GroupStanding, _>::new(GroupStandingChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Tournament")
            .value("id", 1_u64)
            .value("tournament_name", "FIFA World Cup 2026")
            .value("host_countries", "United States")
            .value("start_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("end_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("total_teams", 48_i32)
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1001_u64)
            .value("name", "Group")
            .value("code", "GROUP")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1002_u64)
            .value("name", "Round of 32")
            .value("code", "ROUND_OF_32")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1003_u64)
            .value("name", "Round of 16")
            .value("code", "ROUND_OF_16")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1004_u64)
            .value("name", "Quarter Final")
            .value("code", "QUARTER_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1005_u64)
            .value("name", "Semi Final")
            .value("code", "SEMI_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1006_u64)
            .value("name", "Third Place")
            .value("code", "THIRD_PLACE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1007_u64)
            .value("name", "Final")
            .value("code", "FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1001_u64)
            .value("name", "Scheduled")
            .value("code", "SCHEDULED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1002_u64)
            .value("name", "Live")
            .value("code", "LIVE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1003_u64)
            .value("name", "Finished")
            .value("code", "FINISHED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1004_u64)
            .value("name", "Postponed")
            .value("code", "POSTPONED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1001_u64)
            .value("name", "Normal")
            .value("code", "NORMAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1002_u64)
            .value("name", "Penalty")
            .value("code", "PENALTY")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1003_u64)
            .value("name", "Own Goal")
            .value("code", "OWN_GOAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1004_u64)
            .value("name", "Free Kick")
            .value("code", "FREE_KICK")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1001_u64)
            .value("name", "Yellow")
            .value("code", "YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1002_u64)
            .value("name", "Red")
            .value("code", "RED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1003_u64)
            .value("name", "Second Yellow")
            .value("code", "SECOND_YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1001_u64)
            .value("name", "AFC")
            .value("code", "AFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1002_u64)
            .value("name", "CAF")
            .value("code", "CAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1003_u64)
            .value("name", "CONCACAF")
            .value("code", "CONCACAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1004_u64)
            .value("name", "CONMEBOL")
            .value("code", "CONMEBOL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1005_u64)
            .value("name", "OFC")
            .value("code", "OFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1006_u64)
            .value("name", "UEFA")
            .value("code", "UEFA")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
}

pub fn module_with_behaviors() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<MatchStage, _>(MatchStageBehavior::default())
        .entity_with_behavior::<MatchStatus, _>(MatchStatusBehavior::default())
        .entity_with_behavior::<GoalCategory, _>(GoalCategoryBehavior::default())
        .entity_with_behavior::<CardCategory, _>(CardCategoryBehavior::default())
        .entity_with_behavior::<Confederation, _>(ConfederationBehavior::default())
        .entity_with_behavior::<Tournament, _>(TournamentBehavior::default())
        .entity_with_behavior::<TournamentTeam, _>(TournamentTeamBehavior::default())
        .entity_with_behavior::<MatchGroup, _>(MatchGroupBehavior::default())
        .entity_with_behavior::<TournamentMatch, _>(TournamentMatchBehavior::default())
        .entity_with_behavior::<MatchGoal, _>(MatchGoalBehavior::default())
        .entity_with_behavior::<MatchCard, _>(MatchCardBehavior::default())
        .entity_with_behavior::<GroupStanding, _>(GroupStandingBehavior::default())
        .initial_graph(teaql_runtime::GraphNode::new("Tournament")
            .value("id", 1_u64)
            .value("tournament_name", "FIFA World Cup 2026")
            .value("host_countries", "United States")
            .value("start_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("end_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("total_teams", 48_i32)
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1001_u64)
            .value("name", "Group")
            .value("code", "GROUP")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1002_u64)
            .value("name", "Round of 32")
            .value("code", "ROUND_OF_32")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1003_u64)
            .value("name", "Round of 16")
            .value("code", "ROUND_OF_16")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1004_u64)
            .value("name", "Quarter Final")
            .value("code", "QUARTER_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1005_u64)
            .value("name", "Semi Final")
            .value("code", "SEMI_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1006_u64)
            .value("name", "Third Place")
            .value("code", "THIRD_PLACE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1007_u64)
            .value("name", "Final")
            .value("code", "FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1001_u64)
            .value("name", "Scheduled")
            .value("code", "SCHEDULED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1002_u64)
            .value("name", "Live")
            .value("code", "LIVE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1003_u64)
            .value("name", "Finished")
            .value("code", "FINISHED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1004_u64)
            .value("name", "Postponed")
            .value("code", "POSTPONED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1001_u64)
            .value("name", "Normal")
            .value("code", "NORMAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1002_u64)
            .value("name", "Penalty")
            .value("code", "PENALTY")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1003_u64)
            .value("name", "Own Goal")
            .value("code", "OWN_GOAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1004_u64)
            .value("name", "Free Kick")
            .value("code", "FREE_KICK")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1001_u64)
            .value("name", "Yellow")
            .value("code", "YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1002_u64)
            .value("name", "Red")
            .value("code", "RED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1003_u64)
            .value("name", "Second Yellow")
            .value("code", "SECOND_YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1001_u64)
            .value("name", "AFC")
            .value("code", "AFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1002_u64)
            .value("name", "CAF")
            .value("code", "CAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1003_u64)
            .value("name", "CONCACAF")
            .value("code", "CONCACAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1004_u64)
            .value("name", "CONMEBOL")
            .value("code", "CONMEBOL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1005_u64)
            .value("name", "OFC")
            .value("code", "OFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1006_u64)
            .value("name", "UEFA")
            .value("code", "UEFA")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
}

pub fn module_with_behaviors_and_checkers() -> teaql_runtime::RuntimeModule {
    teaql_runtime::RuntimeModule::new()
        .entity_with_behavior::<MatchStage, _>(MatchStageBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MatchStage, _>::new(MatchStageChecker::default()))
        .entity_with_behavior::<MatchStatus, _>(MatchStatusBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MatchStatus, _>::new(MatchStatusChecker::default()))
        .entity_with_behavior::<GoalCategory, _>(GoalCategoryBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<GoalCategory, _>::new(GoalCategoryChecker::default()))
        .entity_with_behavior::<CardCategory, _>(CardCategoryBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<CardCategory, _>::new(CardCategoryChecker::default()))
        .entity_with_behavior::<Confederation, _>(ConfederationBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Confederation, _>::new(ConfederationChecker::default()))
        .entity_with_behavior::<Tournament, _>(TournamentBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<Tournament, _>::new(TournamentChecker::default()))
        .entity_with_behavior::<TournamentTeam, _>(TournamentTeamBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TournamentTeam, _>::new(TournamentTeamChecker::default()))
        .entity_with_behavior::<MatchGroup, _>(MatchGroupBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MatchGroup, _>::new(MatchGroupChecker::default()))
        .entity_with_behavior::<TournamentMatch, _>(TournamentMatchBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<TournamentMatch, _>::new(TournamentMatchChecker::default()))
        .entity_with_behavior::<MatchGoal, _>(MatchGoalBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MatchGoal, _>::new(MatchGoalChecker::default()))
        .entity_with_behavior::<MatchCard, _>(MatchCardBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<MatchCard, _>::new(MatchCardChecker::default()))
        .entity_with_behavior::<GroupStanding, _>(GroupStandingBehavior::default())
        .checker(teaql_runtime::TypedEntityChecker::<GroupStanding, _>::new(GroupStandingChecker::default()))
        .initial_graph(teaql_runtime::GraphNode::new("Tournament")
            .value("id", 1_u64)
            .value("tournament_name", "FIFA World Cup 2026")
            .value("host_countries", "United States")
            .value("start_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("end_date", chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .value("total_teams", 48_i32)
            .value("create_time", chrono::Utc::now())
            .value("update_time", chrono::Utc::now())
            .value("version", 1_i64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1001_u64)
            .value("name", "Group")
            .value("code", "GROUP")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1002_u64)
            .value("name", "Round of 32")
            .value("code", "ROUND_OF_32")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1003_u64)
            .value("name", "Round of 16")
            .value("code", "ROUND_OF_16")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1004_u64)
            .value("name", "Quarter Final")
            .value("code", "QUARTER_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1005_u64)
            .value("name", "Semi Final")
            .value("code", "SEMI_FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1006_u64)
            .value("name", "Third Place")
            .value("code", "THIRD_PLACE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStage")
            .value("id", 1007_u64)
            .value("name", "Final")
            .value("code", "FINAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1001_u64)
            .value("name", "Scheduled")
            .value("code", "SCHEDULED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1002_u64)
            .value("name", "Live")
            .value("code", "LIVE")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1003_u64)
            .value("name", "Finished")
            .value("code", "FINISHED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("MatchStatus")
            .value("id", 1004_u64)
            .value("name", "Postponed")
            .value("code", "POSTPONED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1001_u64)
            .value("name", "Normal")
            .value("code", "NORMAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1002_u64)
            .value("name", "Penalty")
            .value("code", "PENALTY")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1003_u64)
            .value("name", "Own Goal")
            .value("code", "OWN_GOAL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("GoalCategory")
            .value("id", 1004_u64)
            .value("name", "Free Kick")
            .value("code", "FREE_KICK")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1001_u64)
            .value("name", "Yellow")
            .value("code", "YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1002_u64)
            .value("name", "Red")
            .value("code", "RED")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("CardCategory")
            .value("id", 1003_u64)
            .value("name", "Second Yellow")
            .value("code", "SECOND_YELLOW")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1001_u64)
            .value("name", "AFC")
            .value("code", "AFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1002_u64)
            .value("name", "CAF")
            .value("code", "CAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1003_u64)
            .value("name", "CONCACAF")
            .value("code", "CONCACAF")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1004_u64)
            .value("name", "CONMEBOL")
            .value("code", "CONMEBOL")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1005_u64)
            .value("name", "OFC")
            .value("code", "OFC")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
        .initial_graph(teaql_runtime::GraphNode::new("Confederation")
            .value("id", 1006_u64)
            .value("name", "UEFA")
            .value("code", "UEFA")
            .value("version", 1_i64)
            .value("tournament_id", 1_u64))
}