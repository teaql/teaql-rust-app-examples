#![allow(unused_imports)]
#![allow(async_fn_in_trait)]
use std::{collections::BTreeMap, future::Future, marker::PhantomData};

use serde_json::Value as JsonValue;
use teaql_core::{
    BinaryOp, Expr, Record,
    RelationAggregate as RuntimeRelationAggregate, SelectQuery, SmartList,
};
use teaql_runtime::{ContextError, GraphNode, DataServiceError, RuntimeError, UserContext};

pub(crate) const COUNT_ALIAS: &str = "count";
pub(crate) const TYPE_FIELD: &str = "internal_type";
pub(crate) const TYPE_GROUP_FIELD: &str = "type_group";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Between,
    In,
    NotIn,
    Contain,
    NotContain,
    BeginWith,
    NotBeginWith,
    EndWith,
    NotEndWith,
    SoundsLike,
    IsNull,
    IsNotNull,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DateRange<T> {
    pub start: T,
    pub end: T,
}

impl<T> DateRange<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

pub trait EntityReference {
    fn entity_id_value(self) -> teaql_core::Value;
}

pub trait TeaqlRecordDataService {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn fetch_all(&self, query: &SelectQuery) -> Result<Vec<Record>, DataServiceError<Self::Error>>;

    async fn fetch_smart_list(&self, query: &SelectQuery) -> Result<SmartList<Record>, DataServiceError<Self::Error>>;

    async fn fetch_smart_list_with_relation_aggregates(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<Record>, DataServiceError<Self::Error>>;

    async fn fetch_stream(&self, query: &SelectQuery) -> Result<Vec<teaql_data_service::StreamChunk>, DataServiceError<Self::Error>>;
}

pub trait TeaqlEntityDataService: TeaqlRecordDataService {
    async fn fetch_enhanced_entities<T>(&self, query: &SelectQuery) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn fetch_enhanced_entities_with_relation_aggregates<T>(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn save_entity_graph<T>(&self, entity: T) -> Result<GraphNode, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;
}

impl<'a, E> TeaqlRecordDataService for teaql_runtime::EntityDataService<'a, E>
where
    E: teaql_data_service::QueryExecutor + teaql_data_service::MutationExecutor + teaql_data_service::StreamQueryExecutor + Send + Sync + 'static,
{
    type Error = E::Error;

    async fn fetch_all(&self, query: &SelectQuery) -> Result<Vec<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_all(self, query).await
    }

    async fn fetch_smart_list(&self, query: &SelectQuery) -> Result<SmartList<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_smart_list(self, query).await
    }

    async fn fetch_smart_list_with_relation_aggregates(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_smart_list_with_relation_aggregates(
            self,
            query,
            relation_aggregates,
        ).await
    }

    async fn fetch_stream(&self, query: &SelectQuery) -> Result<Vec<teaql_data_service::StreamChunk>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_stream(self, query).await
    }
}

impl<'a, E> TeaqlEntityDataService for teaql_runtime::EntityDataService<'a, E>
where
    E: teaql_data_service::QueryExecutor + teaql_data_service::MutationExecutor + teaql_data_service::StreamQueryExecutor + Send + Sync + 'static,
{
    async fn fetch_enhanced_entities<T>(&self, query: &SelectQuery) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::fetch_enhanced_entities(self, query).await
    }

    async fn fetch_enhanced_entities_with_relation_aggregates<T>(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::fetch_enhanced_entities_with_relation_aggregates(
            self,
            query,
            relation_aggregates,
        ).await
    }

    async fn save_entity_graph<T>(&self, entity: T) -> Result<GraphNode, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::save_entity_graph(self, entity).await
    }
}

pub type TeaqlDataServiceError<R> = DataServiceError<<R as TeaqlRecordDataService>::Error>;

pub trait TeaqlRuntime {
    fn user_context(&self) -> &UserContext;

    fn fetch_facet_smart_list(
        &self,
        entity: &str,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
        trace_context: Vec<teaql_core::TraceNode>,
    ) -> impl std::future::Future<Output = Result<SmartList<Record>, RuntimeError>> + Send;
}

/// Internal trait for data_service access. Application code should not use this trait directly.
#[doc(hidden)]
pub trait AuditedSave<'a, C>
where
    C: TeaqlDataServiceProvider + ?Sized + 'a,
{
    type Error;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>>;
}



pub trait TeaqlDataServiceProvider: TeaqlRuntime {
    type MatchStageDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn match_stage_data_service(&self) -> Result<Self::MatchStageDataService<'_>, ContextError>;
    type MatchStatusDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn match_status_data_service(&self) -> Result<Self::MatchStatusDataService<'_>, ContextError>;
    type GoalCategoryDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn goal_category_data_service(&self) -> Result<Self::GoalCategoryDataService<'_>, ContextError>;
    type CardCategoryDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn card_category_data_service(&self) -> Result<Self::CardCategoryDataService<'_>, ContextError>;
    type ConfederationDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn confederation_data_service(&self) -> Result<Self::ConfederationDataService<'_>, ContextError>;
    type TournamentDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn tournament_data_service(&self) -> Result<Self::TournamentDataService<'_>, ContextError>;
    type TournamentTeamDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn tournament_team_data_service(&self) -> Result<Self::TournamentTeamDataService<'_>, ContextError>;
    type MatchGroupDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn match_group_data_service(&self) -> Result<Self::MatchGroupDataService<'_>, ContextError>;
    type TournamentMatchDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn tournament_match_data_service(&self) -> Result<Self::TournamentMatchDataService<'_>, ContextError>;
    type MatchGoalDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn match_goal_data_service(&self) -> Result<Self::MatchGoalDataService<'_>, ContextError>;
    type MatchCardDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn match_card_data_service(&self) -> Result<Self::MatchCardDataService<'_>, ContextError>;
    type GroupStandingDataService<'a>: TeaqlEntityDataService + 'a
    where
        Self: 'a;

    fn group_standing_data_service(&self) -> Result<Self::GroupStandingDataService<'_>, ContextError>;
}

#[allow(async_fn_in_trait)]
pub trait TeaqlUserContextExt {
    async fn commit_data(&self) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>;

    async fn transaction_data<F, Fut>(&self, f: F) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>>;
}

impl TeaqlUserContextExt for teaql_runtime::UserContext {
    async fn commit_data(&self) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>> {
        self.commit_changes::<crate::runtime::DataServiceExecutor>().await
    }

    async fn transaction_data<F, Fut>(&self, f: F) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>>,
    {
        let executor = self.require_resource::<crate::runtime::DataServiceExecutor>().map_err(|err| {
            DataServiceError::Runtime(RuntimeError::Graph(format!(
                "cannot start transaction without executor: {err}"
            )))
        })?;
        let root = self.entity_root();

        let tx = teaql_data_service::TransactionExecutor::begin(&*executor).await.map_err(DataServiceError::Executor)?;
        root.push_change_set();

        let result = f().await;
        match result {
            Ok(()) => {
                root.pop_change_set();
                teaql_data_service::Transaction::commit(tx).await.map_err(DataServiceError::Executor)?;
                Ok(())
            }
            Err(err) => {
                root.pop_change_set();
                teaql_data_service::Transaction::rollback(tx).await.map_err(DataServiceError::Executor)?;
                Err(err)
            }
        }
    }
}

impl TeaqlRuntime for teaql_runtime::UserContext {
    fn user_context(&self) -> &UserContext {
        self
    }

    async fn fetch_facet_smart_list(
        &self,
        entity: &str,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
        trace_context: Vec<teaql_core::TraceNode>,
    ) -> Result<SmartList<Record>, RuntimeError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>(entity)
            .map_err(|err| RuntimeError::Graph(err.to_string()))?
            .with_trace_context(trace_context)
            .fetch_smart_list_with_relation_aggregates(query, relation_aggregates)
            .await
            .map_err(|err| RuntimeError::Graph(err.to_string()))
    }
}

impl TeaqlDataServiceProvider for teaql_runtime::UserContext {
    type MatchStageDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn match_stage_data_service(&self) -> Result<Self::MatchStageDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MatchStage")
    }

    type MatchStatusDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn match_status_data_service(&self) -> Result<Self::MatchStatusDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MatchStatus")
    }

    type GoalCategoryDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn goal_category_data_service(&self) -> Result<Self::GoalCategoryDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GoalCategory")
    }

    type CardCategoryDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn card_category_data_service(&self) -> Result<Self::CardCategoryDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CardCategory")
    }

    type ConfederationDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn confederation_data_service(&self) -> Result<Self::ConfederationDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Confederation")
    }

    type TournamentDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tournament_data_service(&self) -> Result<Self::TournamentDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Tournament")
    }

    type TournamentTeamDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tournament_team_data_service(&self) -> Result<Self::TournamentTeamDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TournamentTeam")
    }

    type MatchGroupDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn match_group_data_service(&self) -> Result<Self::MatchGroupDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MatchGroup")
    }

    type TournamentMatchDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tournament_match_data_service(&self) -> Result<Self::TournamentMatchDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TournamentMatch")
    }

    type MatchGoalDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn match_goal_data_service(&self) -> Result<Self::MatchGoalDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MatchGoal")
    }

    type MatchCardDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn match_card_data_service(&self) -> Result<Self::MatchCardDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MatchCard")
    }

    type GroupStandingDataService<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn group_standing_data_service(&self) -> Result<Self::GroupStandingDataService<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("GroupStanding")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuerySelection {
    pub query: SelectQuery,
    pub relation_selections: Vec<RelationSelection>,
    pub relation_filters: Vec<RelationFilter>,
    pub child_enhancements: Vec<QuerySelection>,
    pub query_options: QueryOptions,
}

impl QuerySelection {
    pub fn new(query: impl Into<SelectQuery>) -> Self {
        Self {
            query: query.into(),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
        }
    }

    pub fn into_query(self) -> SelectQuery {
        let query = apply_relation_selections(self.query, self.relation_selections);
        apply_runtime_metadata(query, &self.query_options, &self.child_enhancements)
    }
}

impl From<SelectQuery> for QuerySelection {
    fn from(query: SelectQuery) -> Self {
        QuerySelection::new(query)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationSelection {
    pub name: String,
    pub query: SelectQuery,
    pub relation_selections: Vec<RelationSelection>,
    pub relation_filters: Vec<RelationFilter>,
    pub child_enhancements: Vec<QuerySelection>,
    pub query_options: QueryOptions,
}

impl RelationSelection {
    pub fn new(name: impl Into<String>, selection: impl Into<QuerySelection>) -> Self {
        let selection = selection.into();
        Self {
            name: name.into(),
            query: selection.query,
            relation_selections: selection.relation_selections,
            relation_filters: selection.relation_filters,
            child_enhancements: selection.child_enhancements,
            query_options: selection.query_options,
        }
    }

    pub fn into_query(self) -> SelectQuery {
        let query = apply_relation_selections(self.query, self.relation_selections);
        apply_runtime_metadata(query, &self.query_options, &self.child_enhancements)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationFilter {
    pub name: String,
    pub query: SelectQuery,
    pub relation_selections: Vec<RelationSelection>,
    pub relation_filters: Vec<RelationFilter>,
    pub child_enhancements: Vec<QuerySelection>,
    pub query_options: QueryOptions,
}

impl RelationFilter {
    pub fn new(name: impl Into<String>, selection: impl Into<QuerySelection>) -> Self {
        let selection = selection.into();
        Self {
            name: name.into(),
            query: selection.query,
            relation_selections: selection.relation_selections,
            relation_filters: selection.relation_filters,
            child_enhancements: selection.child_enhancements,
            query_options: selection.query_options,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct QueryOptions {
    pub comment: Option<String>,
    pub raw_sql: Option<String>,
    pub raw_sql_search_criteria: Vec<String>,
    pub dynamic_properties: Vec<RawDynamicProperty>,
    pub raw_projections: Vec<RawProjection>,
    pub relation_aggregates: Vec<RelationAggregate>,
    pub object_group_bys: Vec<ObjectGroupBy>,
    pub facets: Vec<FacetRequest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsafeRawSqlSegment {
    sql: String,
}

impl UnsafeRawSqlSegment {
    pub fn trusted(sql: impl Into<String>) -> Self {
        Self { sql: sql.into() }
    }

    pub fn into_sql(self) -> String {
        self.sql
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawDynamicProperty {
    pub property_name: String,
    pub raw_sql_segment: String,
}

impl RawDynamicProperty {
    pub fn new(property_name: impl Into<String>, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        Self {
            property_name: property_name.into(),
            raw_sql_segment: raw_sql_segment.into_sql(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawProjection {
    pub property_name: String,
    pub raw_sql_segment: String,
}

impl RawProjection {
    pub fn new(property_name: impl Into<String>, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        Self {
            property_name: property_name.into(),
            raw_sql_segment: raw_sql_segment.into_sql(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationAggregate {
    pub relation_name: String,
    pub alias: String,
    pub query: QuerySelection,
    pub single_result: bool,
}

impl RelationAggregate {
    pub fn new(
        relation_name: impl Into<String>,
        alias: impl Into<String>,
        query: impl Into<QuerySelection>,
        single_result: bool,
    ) -> Self {
        Self {
            relation_name: relation_name.into(),
            alias: alias.into(),
            query: query.into(),
            single_result,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FacetRequest {
    pub facet_name: String,
    pub relation_name: String,
    pub query: QuerySelection,
    pub include_all_facets: bool,
}

impl FacetRequest {
    pub fn new(
        facet_name: impl Into<String>,
        relation_name: impl Into<String>,
        query: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        Self {
            facet_name: facet_name.into(),
            relation_name: relation_name.into(),
            query: query.into(),
            include_all_facets,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectGroupBy {
    pub property_name: String,
    pub storage_field: String,
    pub query: QuerySelection,
}

impl ObjectGroupBy {
    pub fn new(
        property_name: impl Into<String>,
        storage_field: impl Into<String>,
        query: impl Into<QuerySelection>,
    ) -> Self {
        Self {
            property_name: property_name.into(),
            storage_field: storage_field.into(),
            query: query.into(),
        }
    }
}

pub(crate) fn apply_relation_selections(
    mut query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
) -> SelectQuery {
    for selection in relation_selections {
        query = query.relation_query(selection.name.clone(), selection.into_query());
    }
    query
}

pub(crate) fn runtime_relation_aggregates(options: &QueryOptions) -> Vec<RuntimeRelationAggregate> {
    options
        .relation_aggregates
        .iter()
        .map(|aggregate| {
            RuntimeRelationAggregate::new(
                aggregate.relation_name.clone(),
                aggregate.alias.clone(),
                aggregate.query.clone().into_query(),
                aggregate.single_result,
            )
        })
        .collect()
}

pub(crate) async fn execute_facets<C>(
    ctx: &C,
    outer_query: &SelectQuery,
    options: &QueryOptions,
) -> Result<BTreeMap<String, SmartList<Record>>, RuntimeError>
where
    C: TeaqlRuntime + ?Sized,
{
    let mut facets = BTreeMap::new();
    for facet in &options.facets {
        let mut selection = facet.query.clone();
        merge_outer_filter_into_facet_aggregates(&mut selection, outer_query);
        if !facet.include_all_facets {
            selection = restrict_facet_to_outer_query(ctx, selection, outer_query, &facet.relation_name)?;
        }
        let relation_aggregates = runtime_relation_aggregates(&selection.query_options);
        let query = apply_runtime_metadata(
            selection.query,
            &selection.query_options,
            &selection.child_enhancements,
        );
        let mut chain = outer_query.trace_chain.clone();
        chain.push(teaql_core::TraceNode { 
            entity_type: query.entity.clone(),
            entity_id: None,
            comment: facet.facet_name.clone(),
        });

        let facet_rows = ctx.fetch_facet_smart_list(&query.entity, &query, &relation_aggregates, chain).await?;
        facets.insert(facet.facet_name.clone(), facet_rows);
    }
    Ok(facets)
}

pub(crate) fn merge_outer_filter_into_facet_aggregates(selection: &mut QuerySelection, outer_query: &SelectQuery) {
    let Some(filter) = outer_query.filter.clone() else {
        return;
    };
    for aggregate in &mut selection.query_options.relation_aggregates {
        if aggregate.query.query.entity == outer_query.entity {
            aggregate.query.query = aggregate.query.query.clone().and_filter(filter.clone());
        }
    }
}

pub(crate) fn restrict_facet_to_outer_query<C>(
    ctx: &C,
    mut selection: QuerySelection,
    outer_query: &SelectQuery,
    relation_name: &str,
) -> Result<QuerySelection, RuntimeError>
where
    C: TeaqlRuntime + ?Sized,
{
    let descriptor = ctx
        .user_context()
        .entity(&outer_query.entity)
        .cloned()
        .ok_or_else(|| RuntimeError::Graph(format!("missing entity: {}", outer_query.entity)))?;
    let relation = descriptor
        .relation_by_name(relation_name)
        .cloned()
        .ok_or_else(|| RuntimeError::MissingRelation {
            entity: outer_query.entity.clone(),
            relation: relation_name.to_owned(),
        })?;
    let mut subquery = outer_query.clone();
    subquery.projection.clear();
    subquery.expr_projection.clear();
    subquery.order_by.clear();
    subquery.slice = None;
    subquery.aggregates.clear();
    subquery.group_by.clear();
    subquery.relations.clear();
    selection.query = selection.query.and_filter(Expr::in_subquery(
        relation.foreign_key,
        descriptor,
        subquery,
        relation.local_key,
    ));
    Ok(selection)
}

pub(crate) fn attach_facets<T>(rows: &mut SmartList<T>, facets: BTreeMap<String, SmartList<Record>>) {
    for (name, facet) in facets {
        rows.add_facet(name, facet);
    }
}

pub(crate) fn apply_runtime_metadata(
    mut query: SelectQuery,
    options: &QueryOptions,
    child_enhancements: &[QuerySelection],
) -> SelectQuery {
    if let Some(c) = options.comment.clone() {
        query = query.comment(c);
    }
    query.raw_sql = options.raw_sql.clone();
    query.raw_sql_search_criteria = options.raw_sql_search_criteria.clone();
    query.dynamic_properties = options
        .dynamic_properties
        .iter()
        .map(|projection| {
            teaql_core::RawSqlProjection::new(
                projection.property_name.clone(),
                projection.raw_sql_segment.clone(),
            )
        })
        .collect();
    query.raw_projections = options
        .raw_projections
        .iter()
        .map(|projection| {
            teaql_core::RawSqlProjection::new(
                projection.property_name.clone(),
                projection.raw_sql_segment.clone(),
            )
        })
        .collect();
    query.object_group_bys = options
        .object_group_bys
        .iter()
        .map(|group_by| {
            teaql_core::ObjectGroupBy::new(
                group_by.property_name.clone(),
                group_by.storage_field.clone(),
                group_by.query.clone().into_query(),
            )
        })
        .collect();
    query.child_enhancements = child_enhancements
        .iter()
        .cloned()
        .map(QuerySelection::into_query)
        .collect();
    query
}

pub(crate) fn field_operator_expr(
    field: &str,
    operator: FieldOperator,
    values: Vec<teaql_core::Value>,
) -> Expr {
    match operator {
        FieldOperator::Equal => Expr::eq(field, required_value(operator, &values, 0)),
        FieldOperator::NotEqual => Expr::ne(field, required_value(operator, &values, 0)),
        FieldOperator::GreaterThan => Expr::gt(field, required_value(operator, &values, 0)),
        FieldOperator::GreaterThanOrEqual => Expr::gte(field, required_value(operator, &values, 0)),
        FieldOperator::LessThan => Expr::lt(field, required_value(operator, &values, 0)),
        FieldOperator::LessThanOrEqual => Expr::lte(field, required_value(operator, &values, 0)),
        FieldOperator::Between => Expr::between(
            field,
            required_value(operator, &values, 0),
            required_value(operator, &values, 1),
        ),
        FieldOperator::In => Expr::in_list(field, values),
        FieldOperator::NotIn => Expr::not_in_list(field, values),
        FieldOperator::Contain => Expr::contain(field, required_text(operator, &values, 0)),
        FieldOperator::NotContain => Expr::not_contain(field, required_text(operator, &values, 0)),
        FieldOperator::BeginWith => Expr::begin_with(field, required_text(operator, &values, 0)),
        FieldOperator::NotBeginWith => Expr::not_begin_with(field, required_text(operator, &values, 0)),
        FieldOperator::EndWith => Expr::end_with(field, required_text(operator, &values, 0)),
        FieldOperator::NotEndWith => Expr::not_end_with(field, required_text(operator, &values, 0)),
        FieldOperator::SoundsLike => Expr::sound_like(field, required_value(operator, &values, 0)),
        FieldOperator::IsNull => Expr::is_null(field),
        FieldOperator::IsNotNull => Expr::is_not_null(field),
    }
}

pub(crate) fn field_operator_column_expr(field: &str, operator: FieldOperator, other_field: &str) -> Expr {
    let binary_op = match operator {
        FieldOperator::Equal => BinaryOp::Eq,
        FieldOperator::NotEqual => BinaryOp::Ne,
        FieldOperator::GreaterThan => BinaryOp::Gt,
        FieldOperator::GreaterThanOrEqual => BinaryOp::Gte,
        FieldOperator::LessThan => BinaryOp::Lt,
        FieldOperator::LessThanOrEqual => BinaryOp::Lte,
        FieldOperator::Contain => BinaryOp::Like,
        FieldOperator::NotContain => BinaryOp::NotLike,
        FieldOperator::BeginWith => BinaryOp::Like,
        FieldOperator::NotBeginWith => BinaryOp::NotLike,
        FieldOperator::EndWith => BinaryOp::Like,
        FieldOperator::NotEndWith => BinaryOp::NotLike,
        unsupported => panic!("{unsupported:?} is not supported for property-to-property filters"),
    };
    Expr::compare_columns(field, binary_op, other_field)
}

pub(crate) fn dynamic_json_value_to_teaql_value(value: &JsonValue) -> teaql_core::Value {
    match value {
        JsonValue::Null => teaql_core::Value::Null,
        JsonValue::Bool(value) => teaql_core::Value::Bool(*value),
        JsonValue::Number(value) => {
            if let Some(value) = value.as_i64() {
                teaql_core::Value::I64(value)
            } else if let Some(value) = value.as_u64() {
                teaql_core::Value::U64(value)
            } else if let Some(value) = value.as_f64() {
                teaql_core::Value::F64(value)
            } else {
                teaql_core::Value::Null
            }
        }
        JsonValue::String(value) => teaql_core::Value::Text(value.trim().to_owned()),
        JsonValue::Array(values) => teaql_core::Value::List(
            values
                .iter()
                .map(dynamic_json_value_to_teaql_value)
                .collect(),
        ),
        JsonValue::Object(object) => object
            .get("id")
            .map(dynamic_json_value_to_teaql_value)
            .unwrap_or(teaql_core::Value::Null),
    }
}

pub(crate) fn dynamic_json_values(value: &JsonValue) -> Vec<teaql_core::Value> {
    match value {
        JsonValue::Array(values) => values
            .iter()
            .map(dynamic_json_value_to_teaql_value)
            .collect(),
        value => vec![dynamic_json_value_to_teaql_value(value)],
    }
}

pub(crate) fn dynamic_json_operator(value: &JsonValue) -> FieldOperator {
    match value {
        JsonValue::String(value) if value.eq_ignore_ascii_case("__is_null__") => FieldOperator::IsNull,
        JsonValue::String(value) if value.eq_ignore_ascii_case("__is_not_null__") => {
            FieldOperator::IsNotNull
        }
        JsonValue::String(_) => FieldOperator::Contain,
        JsonValue::Number(_) | JsonValue::Bool(_) => FieldOperator::Equal,
        JsonValue::Array(values)
            if values
                .first()
                .map(JsonValue::is_string)
                .unwrap_or(false) =>
        {
            FieldOperator::In
        }
        JsonValue::Array(values)
            if values
                .first()
                .map(JsonValue::is_object)
                .unwrap_or(false) =>
        {
            FieldOperator::In
        }
        JsonValue::Array(values) if values.len() == 2 => FieldOperator::Between,
        _ => FieldOperator::Equal,
    }
}

pub(crate) fn dynamic_json_filter_expr(field: &str, value: &JsonValue) -> Expr {
    let operator = dynamic_json_operator(value);
    field_operator_expr(field, operator, dynamic_json_values(value))
}

pub(crate) fn dynamic_json_u64_field(object: &serde_json::Map<String, JsonValue>, field: &str) -> Option<u64> {
    object.get(field).and_then(|value| {
        value
            .as_u64()
            .or_else(|| value.as_i64().and_then(|value| u64::try_from(value).ok()))
    })
}

pub(crate) fn remove_default_live_filter(filter: Option<Expr>) -> Option<Expr> {
    let default_filter = Expr::gt("version", 0_i64);
    remove_filter_expr(filter?, &default_filter)
}

pub(crate) fn remove_filter_expr(filter: Expr, target: &Expr) -> Option<Expr> {
    if &filter == target {
        return None;
    }
    match filter {
        Expr::And(parts) => {
            let mut retained = parts
                .into_iter()
                .filter_map(|part| remove_filter_expr(part, target))
                .collect::<Vec<_>>();
            match retained.len() {
                0 => None,
                1 => retained.pop(),
                _ => Some(Expr::And(retained)),
            }
        }
        other => Some(other),
    }
}

pub(crate) fn required_value(
    operator: FieldOperator,
    values: &[teaql_core::Value],
    index: usize,
) -> teaql_core::Value {
    values.get(index).cloned().unwrap_or_else(|| {
        panic!("{operator:?} requires value at index {index}")
    })
}

pub(crate) fn required_text(operator: FieldOperator, values: &[teaql_core::Value], index: usize) -> String {
    match required_value(operator, values, index) {
        teaql_core::Value::Text(value) => value,
        value => panic!("{operator:?} requires text value, got {value:?}"),
    }
}

impl EntityReference for teaql_core::Value {
    fn entity_id_value(self) -> teaql_core::Value {
        self
    }
}

impl EntityReference for u64 {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::Value::U64(self)
    }
}
