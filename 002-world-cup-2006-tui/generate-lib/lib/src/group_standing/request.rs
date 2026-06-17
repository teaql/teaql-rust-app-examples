use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{RepositoryError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::GroupStanding {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::GroupStanding {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

#[derive(Debug)]
pub struct GroupStandingRequest<R = crate::GroupStanding> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for GroupStandingRequest<R> {
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
            relation_selections: self.relation_selections.clone(),
            relation_filters: self.relation_filters.clone(),
            child_enhancements: self.child_enhancements.clone(),
            query_options: self.query_options.clone(),
            marker: PhantomData,
        }
    }
}

impl<R> GroupStandingRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("GroupStanding"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> GroupStandingRequest<T> {
        GroupStandingRequest {
            query: self.query,
            relation_selections: self.relation_selections,
            relation_filters: self.relation_filters,
            child_enhancements: self.child_enhancements,
            query_options: self.query_options,
            marker: PhantomData,
        }
    }

    pub fn query(&self) -> &SelectQuery {
        &self.query
    }

    pub fn relation_selections(&self) -> &[RelationSelection] {
        &self.relation_selections
    }

    pub fn relation_filters(&self) -> &[RelationFilter] {
        &self.relation_filters
    }

    pub fn child_enhancements(&self) -> &[QuerySelection] {
        &self.child_enhancements
    }

    pub fn query_options(&self) -> &QueryOptions {
        &self.query_options
    }

    pub fn into_query(self) -> SelectQuery {
        self.query
    }


    pub fn purpose(self, purpose: impl Into<String>) -> crate::PurposedQuery<Self> {
        crate::PurposedQuery::new(self, purpose)
    }

    pub(crate) async fn _execute_for_list<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .group_standing_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_enhanced_entities_with_relation_aggregates::<R>(
            &query,
            &relation_aggregates,
        ).await?;
        let facets = execute_facets(ctx, &query, &query_options)
            .await
            .map_err(RepositoryError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .group_standing_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let chunks = repository.fetch_stream(&query)
            .await?;
        Ok(chunks)
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let rows = self.limit(1)._execute_for_list(ctx).await?;
        Ok(rows.into_iter().next())
    }

    pub(crate) async fn _execute_for_one<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        self._execute_for_first(ctx).await
    }


    pub async fn execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let total_count = self.clone()._execute_for_count(ctx).await?;
        let mut rows = self.page_offset(offset, limit)._execute_for_list(ctx).await?;
        rows.total_count = Some(total_count);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_count<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<u64, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .group_standing_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query;
        query.projection.clear();
        query.expr_projection.clear();
        query.order_by.clear();
        query.slice = None;
        query.relations.clear();
        query = query.count(COUNT_ALIAS);
        let rows = repository.fetch_all(&query).await?;
        rows.first()
            .and_then(|row| row.get(COUNT_ALIAS))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| RepositoryError::Runtime(RuntimeError::Graph(format!("count result for GroupStanding is missing or not numeric"))))
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .group_standing_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .group_standing_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let outer_query = self.query.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_smart_list_with_relation_aggregates(&query, &relation_aggregates).await?;
        let facets = execute_facets(ctx, &outer_query, &query_options)
            .await
            .map_err(RepositoryError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_record<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<Record>, TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let records = self.limit(1)._execute_for_records(ctx).await?;
        Ok(records.into_iter().next())
    }

    pub fn search_with_text(mut self, text: impl Into<String>) -> Self {
        self.query = self.query.search_with_text(text);
        self
    }

    pub fn filter(mut self, filter: Expr) -> Self {
        self.query = self.query.filter(filter);
        self
    }

    pub fn and_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.and_filter(filter);
        self
    }

    pub fn or_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.or_filter(filter);
        self
    }

    pub fn append_search_criteria(self, criteria: Expr) -> Self {
        self.and_filter(criteria)
    }

    pub fn filter_property(
        mut self,
        property1: impl AsRef<str>,
        operator: FieldOperator,
        property2: impl AsRef<str>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_column_expr(
            property1.as_ref(),
            operator,
            property2.as_ref(),
        ));
        self
    }

    pub fn with_deleted_rows(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self
    }

    pub fn deleted_rows_only(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self.query = self.query.and_filter(Expr::lte("version", 0_i64));
        self
    }

    pub fn match_types(
        mut self,
        types: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(TYPE_FIELD, types.into_iter().map(Into::into)));
        self
    }


    pub fn with_type_group(mut self) -> Self {
        self.query = self.query.project(TYPE_GROUP_FIELD);
        self
    }

    pub fn matching_any_of(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let entity = EntityDescriptor::new(selection.query.entity.clone());
        self.query = self.query.and_filter(Expr::in_subquery("id", entity, selection.query.clone(), "id"));
        self
    }

    pub fn match_any_of(self, request: impl Into<QuerySelection>) -> Self {
        self.matching_any_of(request)
    }

    pub fn enhance_child(mut self, request: impl Into<QuerySelection>) -> Self {
        self.child_enhancements.push(request.into());
        self
    }

    pub fn enhance_children_if_needed(self) -> Self {
        let request = self;
        request
    }


    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.query_options.comment = Some(comment.into());
        self
    }

    pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql = Some(raw_sql.into_sql());
        self
    }

    pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql_filter(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
        self
    }
    pub fn filter_with_json(self, json_expr: impl Into<String>) -> Self {
        self.merge_dynamic_json_expr(json_expr.into())
    }

    fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
        let json = serde_json::from_str::<JsonValue>(&json_expr)
            .unwrap_or_else(|_| panic!("Input JSON format error: {json_expr}"));
        self.merge_dynamic_json(&json)
    }

    fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
        let Some(object) = json.as_object() else {
            return self;
        };

        for (field, value) in object {
            if field.starts_with('_') {
                continue;
            }
            self = self.apply_dynamic_json_filter(field, value);
        }

        self = self.apply_dynamic_json_order_by(object.get("_orderBy"));

        if let Some(offset) = dynamic_json_u64_field(object, "_start") {
            self = self.skip(offset);
        }
        if let Some(size) = dynamic_json_u64_field(object, "_size") {
            self = self.limit(size);
        }

        if let Some(page_size) = dynamic_json_u64_field(object, "_pageSize") {
            self = self.limit(page_size);
        }
        if let Some(page_number) = dynamic_json_u64_field(object, "_page") {
            if page_number > 0 {
                let size = dynamic_json_u64_field(object, "_pageSize")
                    .or_else(|| self.query.slice.as_ref().and_then(|slice| slice.limit))
                    .unwrap_or(10);
                let offset = page_number.saturating_sub(1).saturating_mul(size);
                self = self.page_offset(offset, size);
            }
        }

        self
    }

    pub(crate) fn apply_dynamic_json_filter(self, field: &str, value: &JsonValue) -> Self {
        if let Some((head, tail)) = field.split_once('.') {
            self.apply_dynamic_json_chain_filter(head, tail, value)
        } else if let Some(storage_field) = Self::dynamic_json_self_field(field) {
            self.and_filter(dynamic_json_filter_expr(storage_field, value))
        } else {
            self
        }
    }

    fn apply_dynamic_json_order_by(mut self, order_by: Option<&JsonValue>) -> Self {
        match order_by {
            Some(JsonValue::String(field)) => {
                if let Some(storage_field) = Self::dynamic_json_self_field(field) {
                    self.query = self.query.order_desc(storage_field);
                }
            }
            Some(JsonValue::Object(order_by)) => {
                self = self.apply_dynamic_json_single_order_by(order_by);
            }
            Some(JsonValue::Array(order_bys)) => {
                for order_by in order_bys {
                    if let Some(order_by) = order_by.as_object() {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                }
            }
            _ => {}
        }
        self
    }

    fn apply_dynamic_json_single_order_by(
        mut self,
        order_by: &serde_json::Map<String, JsonValue>,
    ) -> Self {
        let Some(field) = order_by.get("field").and_then(JsonValue::as_str) else {
            return self;
        };
        let Some(storage_field) = Self::dynamic_json_self_field(field) else {
            return self;
        };
        if order_by
            .get("useAsc")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false)
        {
            self.query = self.query.order_asc(storage_field);
        } else {
            self.query = self.query.order_desc(storage_field);
        }
        self
    }

    fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
        match field {
            "id" => Some("id"),
            "played" => Some("played"),
            "won" => Some("won"),
            "drawn" => Some("drawn"),
            "lost" => Some("lost"),
            "goals_for" => Some("goals_for"),
            "goals_against" => Some("goals_against"),
            "goal_difference" => Some("goal_difference"),
            "points" => Some("points"),
            "standing_rank" => Some("standing_rank"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "tournament_team" | "tournament_team_id" => Some("tournament_team_id"),
            "match_group" | "match_group_id" => Some("match_group_id"),
            "tournament" | "tournament_id" => Some("tournament_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "tournament_team" => {
                self.with_tournament_team_matching(
                    crate::Q::tournament_teams_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "match_group" => {
                self.with_match_group_matching(
                    crate::Q::match_groups_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament" => {
                self.with_tournament_matching(
                    crate::Q::tournaments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            _ => self,
        }
    }

    pub fn create_property_as(
        self,
        property_name: impl Into<String>,
        raw_sql_segment: impl Into<String>,
    ) -> Self {
        self.unsafe_create_property_as(property_name, UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn unsafe_create_property_as(
        mut self,
        property_name: impl Into<String>,
        raw_sql_segment: UnsafeRawSqlSegment,
    ) -> Self {
        self.query_options
            .dynamic_properties
            .push(RawDynamicProperty::new(property_name, raw_sql_segment));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.query = self.query.limit(limit);
        self
    }

    pub fn skip(mut self, offset: u64) -> Self {
        self.query = self.query.offset(offset);
        self
    }

    pub fn offset_only(self, offset: u64) -> Self {
        self.skip(offset)
    }

    pub fn offset(self, offset: u64, size: u64) -> Self {
        self.page_offset(offset, size)
    }

    pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
        self.query = self.query.page(offset, limit);
        self
    }

    pub fn top(self, top_n: u64) -> Self {
        self.limit(top_n)
    }

    pub fn offset_size(self, offset: u64, size: u64) -> Self {
        self.offset(offset, size)
    }

    pub fn unlimited(mut self) -> Self {
        self.query.slice = None;
        self
    }

    pub fn page_number(self, page_number: u64, page_size: u64) -> Self {
        let offset = page_number.saturating_sub(1).saturating_mul(page_size);
        self.page_offset(offset, page_size)
    }

    pub fn page_number_default(self, page_number: u64) -> Self {
        self.page_number(page_number, 10)
    }

    pub fn page(self, page_number: u64, page_size: u64) -> Self {
        self.page_number(page_number, page_size)
    }

    pub fn page_default(self, page_number: u64) -> Self {
        self.page_number_default(page_number)
    }

    pub fn select_self(mut self) -> Self {
        self.query = self.query.project("id");
        self.query = self.query.project("played");
        self.query = self.query.project("won");
        self.query = self.query.project("drawn");
        self.query = self.query.project("lost");
        self.query = self.query.project("goals_for");
        self.query = self.query.project("goals_against");
        self.query = self.query.project("goal_difference");
        self.query = self.query.project("points");
        self.query = self.query.project("standing_rank");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("tournament_team_id");
        self.query = self.query.project("match_group_id");
        self.query = self.query.project("tournament_id");
        self
    }

    pub fn select_self_fields(self) -> Self {
        self.select_self()
    }

    pub fn select_self_without_parent(self) -> Self {
        self.select_self_fields()
    }

    pub fn select_all(self) -> Self {
        let mut request = self.select_self();
        request = request.select_tournament_team();
        request = request.select_match_group();
        request = request.select_tournament();
        request
    }

    pub fn select_children(self) -> Self {
        self.select_all()
    }

    pub fn select_any(self) -> Self {
        self.select_children()
    }

    pub fn group_by(mut self, field: impl Into<String>) -> Self {
        self.query = self.query.group_by(field);
        self
    }

    pub fn aggregate_count(mut self, alias: impl Into<String>) -> Self {
        self.query = self.query.count(alias);
        self
    }

    pub fn aggregate_count_field(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.count_field(field, alias);
        self
    }

    pub fn aggregate_with_function(
        mut self,
        field: impl Into<String>,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.query = self.query.aggregate(Aggregate::new(function, field, alias));
        self
    }

    pub fn aggregate_sum(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.sum(field, alias);
        self
    }

    pub fn aggregate_avg(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.avg(field, alias);
        self
    }

    pub fn aggregate_min(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.min(field, alias);
        self
    }

    pub fn aggregate_max(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.max(field, alias);
        self
    }

    pub fn aggregate_stddev(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev(field, alias);
        self
    }

    pub fn aggregate_stddev_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev_pop(field, alias);
        self
    }

    pub fn aggregate_var_samp(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_samp(field, alias);
        self
    }

    pub fn aggregate_var_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_pop(field, alias);
        self
    }

    pub fn aggregate_bit_and(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_and(field, alias);
        self
    }

    pub fn aggregate_bit_or(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_or(field, alias);
        self
    }

    pub fn aggregate_bit_xor(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_xor(field, alias);
        self
    }

    pub fn enable_aggregation_cache(mut self) -> Self {
        self.query = self.query.enable_aggregation_cache();
        self
    }

    pub fn enable_aggregation_cache_for(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.enable_aggregation_cache_for(cache_expired_millis);
        self
    }

    pub fn propagate_aggregation_cache(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.propagate_aggregation_cache(cache_expired_millis);
        self
    }

    pub fn select_id(mut self) -> Self {
        self.query = self.query.project("id");
        self
    }

    pub fn project_id(self) -> Self {
        self.select_id()
    }

    pub fn select_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("id", raw_sql_segment));
        self
    }

    pub fn group_by_id(self) -> Self {
        self.group_by("id")
    }

    pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("id"));
        request
    }

    pub fn group_by_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("id")
            .aggregate_with_function("id", alias, function)
    }

    pub fn count_id(self) -> Self {
        self.count_id_as("id_count")
    }

    pub fn count_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("id", alias)
    }

    pub fn sum_id(self) -> Self {
        self.sum_id_as("sum_id")
    }

    pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("id", alias)
    }

    pub fn avg_id(self) -> Self {
        self.avg_id_as("avg_id")
    }

    pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("id", alias)
    }

    pub fn min_id(self) -> Self {
        self.min_id_as("min_id")
    }

    pub fn min_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("id", alias)
    }

    pub fn max_id(self) -> Self {
        self.max_id_as("max_id")
    }

    pub fn max_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("id", alias)
    }

    pub fn unselect_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "id");
        self
    }


    pub fn with_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("id", value));
        self
    }



    pub fn with_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("id", value));
        self
    }

    pub fn with_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn order_by_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("id");
        self
    }

    pub fn order_by_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("id");
        self
    }

    pub fn order_by_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("id");
        self
    }

    pub fn order_by_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("id");
        self
    }

    pub fn select_played(mut self) -> Self {
        self.query = self.query.project("played");
        self
    }

    pub fn project_played(self) -> Self {
        self.select_played()
    }

    pub fn select_played_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_played_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_played_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("played", raw_sql_segment));
        self
    }

    pub fn select_played_with_function(self, function: AggregateFunction) -> Self {
        self.select_played_as_with_function("played", function)
    }

    pub fn select_played_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("played", alias, function)
    }

    pub fn group_by_played(self) -> Self {
        self.group_by("played")
    }

    pub fn group_by_played_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("played");
        request.query = request
            .query
            .project_expr(alias, Expr::column("played"));
        request
    }

    pub fn group_by_played_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("played")
            .aggregate_with_function("played", alias, function)
    }

    pub fn count_played(self) -> Self {
        self.count_played_as("played_count")
    }

    pub fn count_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("played", alias)
    }

    pub fn sum_played(self) -> Self {
        self.sum_played_as("sum_played")
    }

    pub fn sum_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("played", alias)
    }

    pub fn avg_played(self) -> Self {
        self.avg_played_as("avg_played")
    }

    pub fn avg_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("played", alias)
    }

    pub fn min_played(self) -> Self {
        self.min_played_as("min_played")
    }

    pub fn min_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("played", alias)
    }

    pub fn max_played(self) -> Self {
        self.max_played_as("max_played")
    }

    pub fn max_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("played", alias)
    }

    pub fn standard_deviation_played(self) -> Self {
        self.standard_deviation_played_as("stdDev_played")
    }

    pub fn standard_deviation_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("played", alias)
    }

    pub fn square_root_of_population_standard_deviation_played(self) -> Self {
        self.square_root_of_population_standard_deviation_played_as("stdDevPop_played")
    }

    pub fn square_root_of_population_standard_deviation_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("played", alias)
    }

    pub fn sample_variance_played(self) -> Self {
        self.sample_variance_played_as("varSamp_played")
    }

    pub fn sample_variance_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("played", alias)
    }

    pub fn sample_population_variance_played(self) -> Self {
        self.sample_population_variance_played_as("varPop_played")
    }

    pub fn sample_population_variance_played_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("played", alias)
    }

    pub fn unselect_played(mut self) -> Self {
        self.query.projection.retain(|field| field != "played");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "played");
        self
    }


    pub fn with_played(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "played",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_played_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "played",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_played_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("played", value));
        self
    }



    pub fn with_played_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("played", value));
        self
    }

    pub fn with_played_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("played", value));
        self
    }

    pub fn with_played_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("played", value));
        self
    }

    pub fn with_played_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("played", value));
        self
    }

    pub fn with_played_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("played", value));
        self
    }

    pub fn with_played_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("played", lower, upper));
        self
    }

    pub fn with_played_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "played",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_played_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "played",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_played_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "played",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_played_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("played", value));
        self
    }

    pub fn with_played_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("played", value));
        self
    }

    pub fn with_played_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("played"));
        self
    }



    pub fn with_played_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("played"));
        self
    }


    pub fn order_by_played_asc(mut self) -> Self {
        self.query = self.query.order_asc("played");
        self
    }

    pub fn order_by_played_desc(mut self) -> Self {
        self.query = self.query.order_desc("played");
        self
    }

    pub fn order_by_played_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("played");
        self
    }

    pub fn order_by_played_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("played");
        self
    }

    pub fn select_won(mut self) -> Self {
        self.query = self.query.project("won");
        self
    }

    pub fn project_won(self) -> Self {
        self.select_won()
    }

    pub fn select_won_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_won_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_won_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("won", raw_sql_segment));
        self
    }

    pub fn select_won_with_function(self, function: AggregateFunction) -> Self {
        self.select_won_as_with_function("won", function)
    }

    pub fn select_won_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("won", alias, function)
    }

    pub fn group_by_won(self) -> Self {
        self.group_by("won")
    }

    pub fn group_by_won_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("won");
        request.query = request
            .query
            .project_expr(alias, Expr::column("won"));
        request
    }

    pub fn group_by_won_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("won")
            .aggregate_with_function("won", alias, function)
    }

    pub fn count_won(self) -> Self {
        self.count_won_as("won_count")
    }

    pub fn count_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("won", alias)
    }

    pub fn sum_won(self) -> Self {
        self.sum_won_as("sum_won")
    }

    pub fn sum_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("won", alias)
    }

    pub fn avg_won(self) -> Self {
        self.avg_won_as("avg_won")
    }

    pub fn avg_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("won", alias)
    }

    pub fn min_won(self) -> Self {
        self.min_won_as("min_won")
    }

    pub fn min_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("won", alias)
    }

    pub fn max_won(self) -> Self {
        self.max_won_as("max_won")
    }

    pub fn max_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("won", alias)
    }

    pub fn standard_deviation_won(self) -> Self {
        self.standard_deviation_won_as("stdDev_won")
    }

    pub fn standard_deviation_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("won", alias)
    }

    pub fn square_root_of_population_standard_deviation_won(self) -> Self {
        self.square_root_of_population_standard_deviation_won_as("stdDevPop_won")
    }

    pub fn square_root_of_population_standard_deviation_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("won", alias)
    }

    pub fn sample_variance_won(self) -> Self {
        self.sample_variance_won_as("varSamp_won")
    }

    pub fn sample_variance_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("won", alias)
    }

    pub fn sample_population_variance_won(self) -> Self {
        self.sample_population_variance_won_as("varPop_won")
    }

    pub fn sample_population_variance_won_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("won", alias)
    }

    pub fn unselect_won(mut self) -> Self {
        self.query.projection.retain(|field| field != "won");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "won");
        self
    }


    pub fn with_won(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "won",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_won_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "won",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_won_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("won", value));
        self
    }



    pub fn with_won_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("won", value));
        self
    }

    pub fn with_won_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("won", value));
        self
    }

    pub fn with_won_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("won", value));
        self
    }

    pub fn with_won_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("won", value));
        self
    }

    pub fn with_won_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("won", value));
        self
    }

    pub fn with_won_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("won", lower, upper));
        self
    }

    pub fn with_won_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "won",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_won_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "won",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_won_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "won",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_won_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("won", value));
        self
    }

    pub fn with_won_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("won", value));
        self
    }

    pub fn with_won_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("won"));
        self
    }



    pub fn with_won_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("won"));
        self
    }


    pub fn order_by_won_asc(mut self) -> Self {
        self.query = self.query.order_asc("won");
        self
    }

    pub fn order_by_won_desc(mut self) -> Self {
        self.query = self.query.order_desc("won");
        self
    }

    pub fn order_by_won_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("won");
        self
    }

    pub fn order_by_won_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("won");
        self
    }

    pub fn select_drawn(mut self) -> Self {
        self.query = self.query.project("drawn");
        self
    }

    pub fn project_drawn(self) -> Self {
        self.select_drawn()
    }

    pub fn select_drawn_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_drawn_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_drawn_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("drawn", raw_sql_segment));
        self
    }

    pub fn select_drawn_with_function(self, function: AggregateFunction) -> Self {
        self.select_drawn_as_with_function("drawn", function)
    }

    pub fn select_drawn_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("drawn", alias, function)
    }

    pub fn group_by_drawn(self) -> Self {
        self.group_by("drawn")
    }

    pub fn group_by_drawn_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("drawn");
        request.query = request
            .query
            .project_expr(alias, Expr::column("drawn"));
        request
    }

    pub fn group_by_drawn_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("drawn")
            .aggregate_with_function("drawn", alias, function)
    }

    pub fn count_drawn(self) -> Self {
        self.count_drawn_as("drawn_count")
    }

    pub fn count_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("drawn", alias)
    }

    pub fn sum_drawn(self) -> Self {
        self.sum_drawn_as("sum_drawn")
    }

    pub fn sum_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("drawn", alias)
    }

    pub fn avg_drawn(self) -> Self {
        self.avg_drawn_as("avg_drawn")
    }

    pub fn avg_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("drawn", alias)
    }

    pub fn min_drawn(self) -> Self {
        self.min_drawn_as("min_drawn")
    }

    pub fn min_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("drawn", alias)
    }

    pub fn max_drawn(self) -> Self {
        self.max_drawn_as("max_drawn")
    }

    pub fn max_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("drawn", alias)
    }

    pub fn standard_deviation_drawn(self) -> Self {
        self.standard_deviation_drawn_as("stdDev_drawn")
    }

    pub fn standard_deviation_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("drawn", alias)
    }

    pub fn square_root_of_population_standard_deviation_drawn(self) -> Self {
        self.square_root_of_population_standard_deviation_drawn_as("stdDevPop_drawn")
    }

    pub fn square_root_of_population_standard_deviation_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("drawn", alias)
    }

    pub fn sample_variance_drawn(self) -> Self {
        self.sample_variance_drawn_as("varSamp_drawn")
    }

    pub fn sample_variance_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("drawn", alias)
    }

    pub fn sample_population_variance_drawn(self) -> Self {
        self.sample_population_variance_drawn_as("varPop_drawn")
    }

    pub fn sample_population_variance_drawn_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("drawn", alias)
    }

    pub fn unselect_drawn(mut self) -> Self {
        self.query.projection.retain(|field| field != "drawn");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "drawn");
        self
    }


    pub fn with_drawn(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "drawn",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_drawn_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "drawn",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_drawn_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("drawn", value));
        self
    }



    pub fn with_drawn_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("drawn", value));
        self
    }

    pub fn with_drawn_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("drawn", value));
        self
    }

    pub fn with_drawn_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("drawn", value));
        self
    }

    pub fn with_drawn_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("drawn", value));
        self
    }

    pub fn with_drawn_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("drawn", value));
        self
    }

    pub fn with_drawn_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("drawn", lower, upper));
        self
    }

    pub fn with_drawn_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "drawn",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_drawn_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "drawn",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_drawn_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "drawn",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_drawn_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("drawn", value));
        self
    }

    pub fn with_drawn_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("drawn", value));
        self
    }

    pub fn with_drawn_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("drawn"));
        self
    }



    pub fn with_drawn_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("drawn"));
        self
    }


    pub fn order_by_drawn_asc(mut self) -> Self {
        self.query = self.query.order_asc("drawn");
        self
    }

    pub fn order_by_drawn_desc(mut self) -> Self {
        self.query = self.query.order_desc("drawn");
        self
    }

    pub fn order_by_drawn_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("drawn");
        self
    }

    pub fn order_by_drawn_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("drawn");
        self
    }

    pub fn select_lost(mut self) -> Self {
        self.query = self.query.project("lost");
        self
    }

    pub fn project_lost(self) -> Self {
        self.select_lost()
    }

    pub fn select_lost_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_lost_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_lost_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("lost", raw_sql_segment));
        self
    }

    pub fn select_lost_with_function(self, function: AggregateFunction) -> Self {
        self.select_lost_as_with_function("lost", function)
    }

    pub fn select_lost_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("lost", alias, function)
    }

    pub fn group_by_lost(self) -> Self {
        self.group_by("lost")
    }

    pub fn group_by_lost_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("lost");
        request.query = request
            .query
            .project_expr(alias, Expr::column("lost"));
        request
    }

    pub fn group_by_lost_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("lost")
            .aggregate_with_function("lost", alias, function)
    }

    pub fn count_lost(self) -> Self {
        self.count_lost_as("lost_count")
    }

    pub fn count_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("lost", alias)
    }

    pub fn sum_lost(self) -> Self {
        self.sum_lost_as("sum_lost")
    }

    pub fn sum_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("lost", alias)
    }

    pub fn avg_lost(self) -> Self {
        self.avg_lost_as("avg_lost")
    }

    pub fn avg_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("lost", alias)
    }

    pub fn min_lost(self) -> Self {
        self.min_lost_as("min_lost")
    }

    pub fn min_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("lost", alias)
    }

    pub fn max_lost(self) -> Self {
        self.max_lost_as("max_lost")
    }

    pub fn max_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("lost", alias)
    }

    pub fn standard_deviation_lost(self) -> Self {
        self.standard_deviation_lost_as("stdDev_lost")
    }

    pub fn standard_deviation_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("lost", alias)
    }

    pub fn square_root_of_population_standard_deviation_lost(self) -> Self {
        self.square_root_of_population_standard_deviation_lost_as("stdDevPop_lost")
    }

    pub fn square_root_of_population_standard_deviation_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("lost", alias)
    }

    pub fn sample_variance_lost(self) -> Self {
        self.sample_variance_lost_as("varSamp_lost")
    }

    pub fn sample_variance_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("lost", alias)
    }

    pub fn sample_population_variance_lost(self) -> Self {
        self.sample_population_variance_lost_as("varPop_lost")
    }

    pub fn sample_population_variance_lost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("lost", alias)
    }

    pub fn unselect_lost(mut self) -> Self {
        self.query.projection.retain(|field| field != "lost");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "lost");
        self
    }


    pub fn with_lost(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "lost",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_lost_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "lost",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_lost_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("lost", value));
        self
    }



    pub fn with_lost_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("lost", value));
        self
    }

    pub fn with_lost_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("lost", value));
        self
    }

    pub fn with_lost_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("lost", value));
        self
    }

    pub fn with_lost_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("lost", value));
        self
    }

    pub fn with_lost_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("lost", value));
        self
    }

    pub fn with_lost_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("lost", lower, upper));
        self
    }

    pub fn with_lost_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "lost",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_lost_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "lost",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_lost_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "lost",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_lost_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("lost", value));
        self
    }

    pub fn with_lost_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("lost", value));
        self
    }

    pub fn with_lost_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("lost"));
        self
    }



    pub fn with_lost_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("lost"));
        self
    }


    pub fn order_by_lost_asc(mut self) -> Self {
        self.query = self.query.order_asc("lost");
        self
    }

    pub fn order_by_lost_desc(mut self) -> Self {
        self.query = self.query.order_desc("lost");
        self
    }

    pub fn order_by_lost_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("lost");
        self
    }

    pub fn order_by_lost_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("lost");
        self
    }

    pub fn select_goals_for(mut self) -> Self {
        self.query = self.query.project("goals_for");
        self
    }

    pub fn project_goals_for(self) -> Self {
        self.select_goals_for()
    }

    pub fn select_goals_for_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_goals_for_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_goals_for_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("goals_for", raw_sql_segment));
        self
    }

    pub fn select_goals_for_with_function(self, function: AggregateFunction) -> Self {
        self.select_goals_for_as_with_function("goals_for", function)
    }

    pub fn select_goals_for_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("goals_for", alias, function)
    }

    pub fn group_by_goals_for(self) -> Self {
        self.group_by("goals_for")
    }

    pub fn group_by_goals_for_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("goals_for");
        request.query = request
            .query
            .project_expr(alias, Expr::column("goals_for"));
        request
    }

    pub fn group_by_goals_for_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("goals_for")
            .aggregate_with_function("goals_for", alias, function)
    }

    pub fn count_goals_for(self) -> Self {
        self.count_goals_for_as("goals_for_count")
    }

    pub fn count_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("goals_for", alias)
    }

    pub fn sum_goals_for(self) -> Self {
        self.sum_goals_for_as("sum_goals_for")
    }

    pub fn sum_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("goals_for", alias)
    }

    pub fn avg_goals_for(self) -> Self {
        self.avg_goals_for_as("avg_goals_for")
    }

    pub fn avg_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("goals_for", alias)
    }

    pub fn min_goals_for(self) -> Self {
        self.min_goals_for_as("min_goals_for")
    }

    pub fn min_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("goals_for", alias)
    }

    pub fn max_goals_for(self) -> Self {
        self.max_goals_for_as("max_goals_for")
    }

    pub fn max_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("goals_for", alias)
    }

    pub fn standard_deviation_goals_for(self) -> Self {
        self.standard_deviation_goals_for_as("stdDev_goals_for")
    }

    pub fn standard_deviation_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("goals_for", alias)
    }

    pub fn square_root_of_population_standard_deviation_goals_for(self) -> Self {
        self.square_root_of_population_standard_deviation_goals_for_as("stdDevPop_goals_for")
    }

    pub fn square_root_of_population_standard_deviation_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("goals_for", alias)
    }

    pub fn sample_variance_goals_for(self) -> Self {
        self.sample_variance_goals_for_as("varSamp_goals_for")
    }

    pub fn sample_variance_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("goals_for", alias)
    }

    pub fn sample_population_variance_goals_for(self) -> Self {
        self.sample_population_variance_goals_for_as("varPop_goals_for")
    }

    pub fn sample_population_variance_goals_for_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("goals_for", alias)
    }

    pub fn unselect_goals_for(mut self) -> Self {
        self.query.projection.retain(|field| field != "goals_for");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "goals_for");
        self
    }


    pub fn with_goals_for(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "goals_for",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_goals_for_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "goals_for",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_goals_for_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("goals_for", value));
        self
    }



    pub fn with_goals_for_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("goals_for", value));
        self
    }

    pub fn with_goals_for_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("goals_for", value));
        self
    }

    pub fn with_goals_for_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("goals_for", value));
        self
    }

    pub fn with_goals_for_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("goals_for", value));
        self
    }

    pub fn with_goals_for_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("goals_for", value));
        self
    }

    pub fn with_goals_for_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("goals_for", lower, upper));
        self
    }

    pub fn with_goals_for_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "goals_for",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_goals_for_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "goals_for",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_goals_for_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "goals_for",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_goals_for_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("goals_for", value));
        self
    }

    pub fn with_goals_for_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("goals_for", value));
        self
    }

    pub fn with_goals_for_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("goals_for"));
        self
    }



    pub fn with_goals_for_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("goals_for"));
        self
    }


    pub fn order_by_goals_for_asc(mut self) -> Self {
        self.query = self.query.order_asc("goals_for");
        self
    }

    pub fn order_by_goals_for_desc(mut self) -> Self {
        self.query = self.query.order_desc("goals_for");
        self
    }

    pub fn order_by_goals_for_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("goals_for");
        self
    }

    pub fn order_by_goals_for_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("goals_for");
        self
    }

    pub fn select_goals_against(mut self) -> Self {
        self.query = self.query.project("goals_against");
        self
    }

    pub fn project_goals_against(self) -> Self {
        self.select_goals_against()
    }

    pub fn select_goals_against_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_goals_against_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_goals_against_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("goals_against", raw_sql_segment));
        self
    }

    pub fn select_goals_against_with_function(self, function: AggregateFunction) -> Self {
        self.select_goals_against_as_with_function("goals_against", function)
    }

    pub fn select_goals_against_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("goals_against", alias, function)
    }

    pub fn group_by_goals_against(self) -> Self {
        self.group_by("goals_against")
    }

    pub fn group_by_goals_against_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("goals_against");
        request.query = request
            .query
            .project_expr(alias, Expr::column("goals_against"));
        request
    }

    pub fn group_by_goals_against_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("goals_against")
            .aggregate_with_function("goals_against", alias, function)
    }

    pub fn count_goals_against(self) -> Self {
        self.count_goals_against_as("goals_against_count")
    }

    pub fn count_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("goals_against", alias)
    }

    pub fn sum_goals_against(self) -> Self {
        self.sum_goals_against_as("sum_goals_against")
    }

    pub fn sum_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("goals_against", alias)
    }

    pub fn avg_goals_against(self) -> Self {
        self.avg_goals_against_as("avg_goals_against")
    }

    pub fn avg_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("goals_against", alias)
    }

    pub fn min_goals_against(self) -> Self {
        self.min_goals_against_as("min_goals_against")
    }

    pub fn min_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("goals_against", alias)
    }

    pub fn max_goals_against(self) -> Self {
        self.max_goals_against_as("max_goals_against")
    }

    pub fn max_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("goals_against", alias)
    }

    pub fn standard_deviation_goals_against(self) -> Self {
        self.standard_deviation_goals_against_as("stdDev_goals_against")
    }

    pub fn standard_deviation_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("goals_against", alias)
    }

    pub fn square_root_of_population_standard_deviation_goals_against(self) -> Self {
        self.square_root_of_population_standard_deviation_goals_against_as("stdDevPop_goals_against")
    }

    pub fn square_root_of_population_standard_deviation_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("goals_against", alias)
    }

    pub fn sample_variance_goals_against(self) -> Self {
        self.sample_variance_goals_against_as("varSamp_goals_against")
    }

    pub fn sample_variance_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("goals_against", alias)
    }

    pub fn sample_population_variance_goals_against(self) -> Self {
        self.sample_population_variance_goals_against_as("varPop_goals_against")
    }

    pub fn sample_population_variance_goals_against_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("goals_against", alias)
    }

    pub fn unselect_goals_against(mut self) -> Self {
        self.query.projection.retain(|field| field != "goals_against");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "goals_against");
        self
    }


    pub fn with_goals_against(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "goals_against",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_goals_against_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "goals_against",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_goals_against_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("goals_against", value));
        self
    }



    pub fn with_goals_against_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("goals_against", value));
        self
    }

    pub fn with_goals_against_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("goals_against", value));
        self
    }

    pub fn with_goals_against_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("goals_against", value));
        self
    }

    pub fn with_goals_against_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("goals_against", value));
        self
    }

    pub fn with_goals_against_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("goals_against", value));
        self
    }

    pub fn with_goals_against_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("goals_against", lower, upper));
        self
    }

    pub fn with_goals_against_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "goals_against",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_goals_against_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "goals_against",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_goals_against_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "goals_against",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_goals_against_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("goals_against", value));
        self
    }

    pub fn with_goals_against_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("goals_against", value));
        self
    }

    pub fn with_goals_against_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("goals_against"));
        self
    }



    pub fn with_goals_against_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("goals_against"));
        self
    }


    pub fn order_by_goals_against_asc(mut self) -> Self {
        self.query = self.query.order_asc("goals_against");
        self
    }

    pub fn order_by_goals_against_desc(mut self) -> Self {
        self.query = self.query.order_desc("goals_against");
        self
    }

    pub fn order_by_goals_against_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("goals_against");
        self
    }

    pub fn order_by_goals_against_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("goals_against");
        self
    }

    pub fn select_goal_difference(mut self) -> Self {
        self.query = self.query.project("goal_difference");
        self
    }

    pub fn project_goal_difference(self) -> Self {
        self.select_goal_difference()
    }

    pub fn select_goal_difference_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_goal_difference_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_goal_difference_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("goal_difference", raw_sql_segment));
        self
    }

    pub fn select_goal_difference_with_function(self, function: AggregateFunction) -> Self {
        self.select_goal_difference_as_with_function("goal_difference", function)
    }

    pub fn select_goal_difference_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("goal_difference", alias, function)
    }

    pub fn group_by_goal_difference(self) -> Self {
        self.group_by("goal_difference")
    }

    pub fn group_by_goal_difference_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("goal_difference");
        request.query = request
            .query
            .project_expr(alias, Expr::column("goal_difference"));
        request
    }

    pub fn group_by_goal_difference_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("goal_difference")
            .aggregate_with_function("goal_difference", alias, function)
    }

    pub fn count_goal_difference(self) -> Self {
        self.count_goal_difference_as("goal_difference_count")
    }

    pub fn count_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("goal_difference", alias)
    }

    pub fn sum_goal_difference(self) -> Self {
        self.sum_goal_difference_as("sum_goal_difference")
    }

    pub fn sum_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("goal_difference", alias)
    }

    pub fn avg_goal_difference(self) -> Self {
        self.avg_goal_difference_as("avg_goal_difference")
    }

    pub fn avg_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("goal_difference", alias)
    }

    pub fn min_goal_difference(self) -> Self {
        self.min_goal_difference_as("min_goal_difference")
    }

    pub fn min_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("goal_difference", alias)
    }

    pub fn max_goal_difference(self) -> Self {
        self.max_goal_difference_as("max_goal_difference")
    }

    pub fn max_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("goal_difference", alias)
    }

    pub fn standard_deviation_goal_difference(self) -> Self {
        self.standard_deviation_goal_difference_as("stdDev_goal_difference")
    }

    pub fn standard_deviation_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("goal_difference", alias)
    }

    pub fn square_root_of_population_standard_deviation_goal_difference(self) -> Self {
        self.square_root_of_population_standard_deviation_goal_difference_as("stdDevPop_goal_difference")
    }

    pub fn square_root_of_population_standard_deviation_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("goal_difference", alias)
    }

    pub fn sample_variance_goal_difference(self) -> Self {
        self.sample_variance_goal_difference_as("varSamp_goal_difference")
    }

    pub fn sample_variance_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("goal_difference", alias)
    }

    pub fn sample_population_variance_goal_difference(self) -> Self {
        self.sample_population_variance_goal_difference_as("varPop_goal_difference")
    }

    pub fn sample_population_variance_goal_difference_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("goal_difference", alias)
    }

    pub fn unselect_goal_difference(mut self) -> Self {
        self.query.projection.retain(|field| field != "goal_difference");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "goal_difference");
        self
    }


    pub fn with_goal_difference(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "goal_difference",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_goal_difference_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "goal_difference",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_goal_difference_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("goal_difference", value));
        self
    }



    pub fn with_goal_difference_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("goal_difference", value));
        self
    }

    pub fn with_goal_difference_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("goal_difference", value));
        self
    }

    pub fn with_goal_difference_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("goal_difference", value));
        self
    }

    pub fn with_goal_difference_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("goal_difference", value));
        self
    }

    pub fn with_goal_difference_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("goal_difference", value));
        self
    }

    pub fn with_goal_difference_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("goal_difference", lower, upper));
        self
    }

    pub fn with_goal_difference_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "goal_difference",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_goal_difference_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "goal_difference",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_goal_difference_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "goal_difference",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_goal_difference_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("goal_difference", value));
        self
    }

    pub fn with_goal_difference_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("goal_difference", value));
        self
    }

    pub fn with_goal_difference_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("goal_difference"));
        self
    }



    pub fn with_goal_difference_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("goal_difference"));
        self
    }


    pub fn order_by_goal_difference_asc(mut self) -> Self {
        self.query = self.query.order_asc("goal_difference");
        self
    }

    pub fn order_by_goal_difference_desc(mut self) -> Self {
        self.query = self.query.order_desc("goal_difference");
        self
    }

    pub fn order_by_goal_difference_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("goal_difference");
        self
    }

    pub fn order_by_goal_difference_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("goal_difference");
        self
    }

    pub fn select_points(mut self) -> Self {
        self.query = self.query.project("points");
        self
    }

    pub fn project_points(self) -> Self {
        self.select_points()
    }

    pub fn select_points_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_points_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_points_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("points", raw_sql_segment));
        self
    }

    pub fn select_points_with_function(self, function: AggregateFunction) -> Self {
        self.select_points_as_with_function("points", function)
    }

    pub fn select_points_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("points", alias, function)
    }

    pub fn group_by_points(self) -> Self {
        self.group_by("points")
    }

    pub fn group_by_points_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("points");
        request.query = request
            .query
            .project_expr(alias, Expr::column("points"));
        request
    }

    pub fn group_by_points_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("points")
            .aggregate_with_function("points", alias, function)
    }

    pub fn count_points(self) -> Self {
        self.count_points_as("points_count")
    }

    pub fn count_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("points", alias)
    }

    pub fn sum_points(self) -> Self {
        self.sum_points_as("sum_points")
    }

    pub fn sum_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("points", alias)
    }

    pub fn avg_points(self) -> Self {
        self.avg_points_as("avg_points")
    }

    pub fn avg_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("points", alias)
    }

    pub fn min_points(self) -> Self {
        self.min_points_as("min_points")
    }

    pub fn min_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("points", alias)
    }

    pub fn max_points(self) -> Self {
        self.max_points_as("max_points")
    }

    pub fn max_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("points", alias)
    }

    pub fn standard_deviation_points(self) -> Self {
        self.standard_deviation_points_as("stdDev_points")
    }

    pub fn standard_deviation_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("points", alias)
    }

    pub fn square_root_of_population_standard_deviation_points(self) -> Self {
        self.square_root_of_population_standard_deviation_points_as("stdDevPop_points")
    }

    pub fn square_root_of_population_standard_deviation_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("points", alias)
    }

    pub fn sample_variance_points(self) -> Self {
        self.sample_variance_points_as("varSamp_points")
    }

    pub fn sample_variance_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("points", alias)
    }

    pub fn sample_population_variance_points(self) -> Self {
        self.sample_population_variance_points_as("varPop_points")
    }

    pub fn sample_population_variance_points_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("points", alias)
    }

    pub fn unselect_points(mut self) -> Self {
        self.query.projection.retain(|field| field != "points");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "points");
        self
    }


    pub fn with_points(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "points",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_points_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "points",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_points_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("points", value));
        self
    }



    pub fn with_points_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("points", value));
        self
    }

    pub fn with_points_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("points", value));
        self
    }

    pub fn with_points_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("points", value));
        self
    }

    pub fn with_points_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("points", value));
        self
    }

    pub fn with_points_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("points", value));
        self
    }

    pub fn with_points_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("points", lower, upper));
        self
    }

    pub fn with_points_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "points",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_points_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "points",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_points_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "points",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_points_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("points", value));
        self
    }

    pub fn with_points_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("points", value));
        self
    }

    pub fn with_points_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("points"));
        self
    }



    pub fn with_points_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("points"));
        self
    }


    pub fn order_by_points_asc(mut self) -> Self {
        self.query = self.query.order_asc("points");
        self
    }

    pub fn order_by_points_desc(mut self) -> Self {
        self.query = self.query.order_desc("points");
        self
    }

    pub fn order_by_points_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("points");
        self
    }

    pub fn order_by_points_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("points");
        self
    }

    pub fn select_standing_rank(mut self) -> Self {
        self.query = self.query.project("standing_rank");
        self
    }

    pub fn project_standing_rank(self) -> Self {
        self.select_standing_rank()
    }

    pub fn select_standing_rank_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_standing_rank_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_standing_rank_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("standing_rank", raw_sql_segment));
        self
    }

    pub fn select_standing_rank_with_function(self, function: AggregateFunction) -> Self {
        self.select_standing_rank_as_with_function("standing_rank", function)
    }

    pub fn select_standing_rank_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("standing_rank", alias, function)
    }

    pub fn group_by_standing_rank(self) -> Self {
        self.group_by("standing_rank")
    }

    pub fn group_by_standing_rank_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("standing_rank");
        request.query = request
            .query
            .project_expr(alias, Expr::column("standing_rank"));
        request
    }

    pub fn group_by_standing_rank_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("standing_rank")
            .aggregate_with_function("standing_rank", alias, function)
    }

    pub fn count_standing_rank(self) -> Self {
        self.count_standing_rank_as("standing_rank_count")
    }

    pub fn count_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("standing_rank", alias)
    }

    pub fn sum_standing_rank(self) -> Self {
        self.sum_standing_rank_as("sum_standing_rank")
    }

    pub fn sum_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("standing_rank", alias)
    }

    pub fn avg_standing_rank(self) -> Self {
        self.avg_standing_rank_as("avg_standing_rank")
    }

    pub fn avg_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("standing_rank", alias)
    }

    pub fn min_standing_rank(self) -> Self {
        self.min_standing_rank_as("min_standing_rank")
    }

    pub fn min_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("standing_rank", alias)
    }

    pub fn max_standing_rank(self) -> Self {
        self.max_standing_rank_as("max_standing_rank")
    }

    pub fn max_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("standing_rank", alias)
    }

    pub fn standard_deviation_standing_rank(self) -> Self {
        self.standard_deviation_standing_rank_as("stdDev_standing_rank")
    }

    pub fn standard_deviation_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("standing_rank", alias)
    }

    pub fn square_root_of_population_standard_deviation_standing_rank(self) -> Self {
        self.square_root_of_population_standard_deviation_standing_rank_as("stdDevPop_standing_rank")
    }

    pub fn square_root_of_population_standard_deviation_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("standing_rank", alias)
    }

    pub fn sample_variance_standing_rank(self) -> Self {
        self.sample_variance_standing_rank_as("varSamp_standing_rank")
    }

    pub fn sample_variance_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("standing_rank", alias)
    }

    pub fn sample_population_variance_standing_rank(self) -> Self {
        self.sample_population_variance_standing_rank_as("varPop_standing_rank")
    }

    pub fn sample_population_variance_standing_rank_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("standing_rank", alias)
    }

    pub fn unselect_standing_rank(mut self) -> Self {
        self.query.projection.retain(|field| field != "standing_rank");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "standing_rank");
        self
    }


    pub fn with_standing_rank(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "standing_rank",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_standing_rank_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "standing_rank",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_standing_rank_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("standing_rank", value));
        self
    }



    pub fn with_standing_rank_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("standing_rank", value));
        self
    }

    pub fn with_standing_rank_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("standing_rank", value));
        self
    }

    pub fn with_standing_rank_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("standing_rank", value));
        self
    }

    pub fn with_standing_rank_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("standing_rank", value));
        self
    }

    pub fn with_standing_rank_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("standing_rank", value));
        self
    }

    pub fn with_standing_rank_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("standing_rank", lower, upper));
        self
    }

    pub fn with_standing_rank_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "standing_rank",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_standing_rank_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "standing_rank",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_standing_rank_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "standing_rank",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_standing_rank_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("standing_rank", value));
        self
    }

    pub fn with_standing_rank_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("standing_rank", value));
        self
    }

    pub fn with_standing_rank_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("standing_rank"));
        self
    }



    pub fn with_standing_rank_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("standing_rank"));
        self
    }


    pub fn order_by_standing_rank_asc(mut self) -> Self {
        self.query = self.query.order_asc("standing_rank");
        self
    }

    pub fn order_by_standing_rank_desc(mut self) -> Self {
        self.query = self.query.order_desc("standing_rank");
        self
    }

    pub fn order_by_standing_rank_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("standing_rank");
        self
    }

    pub fn order_by_standing_rank_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("standing_rank");
        self
    }

    pub fn select_create_time(mut self) -> Self {
        self.query = self.query.project("create_time");
        self
    }

    pub fn project_create_time(self) -> Self {
        self.select_create_time()
    }

    pub fn select_create_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_create_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_create_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("create_time", raw_sql_segment));
        self
    }

    pub fn group_by_create_time(self) -> Self {
        self.group_by("create_time")
    }

    pub fn group_by_create_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("create_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("create_time"));
        request
    }

    pub fn group_by_create_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("create_time")
            .aggregate_with_function("create_time", alias, function)
    }

    pub fn count_create_time(self) -> Self {
        self.count_create_time_as("create_time_count")
    }

    pub fn count_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("create_time", alias)
    }

    pub fn sum_create_time(self) -> Self {
        self.sum_create_time_as("sum_create_time")
    }

    pub fn sum_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("create_time", alias)
    }

    pub fn avg_create_time(self) -> Self {
        self.avg_create_time_as("avg_create_time")
    }

    pub fn avg_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("create_time", alias)
    }

    pub fn min_create_time(self) -> Self {
        self.min_create_time_as("min_create_time")
    }

    pub fn min_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("create_time", alias)
    }

    pub fn max_create_time(self) -> Self {
        self.max_create_time_as("max_create_time")
    }

    pub fn max_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("create_time", alias)
    }

    pub fn unselect_create_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "create_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "create_time");
        self
    }


    pub fn with_create_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_create_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_create_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("create_time", value));
        self
    }



    pub fn with_create_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("create_time", value));
        self
    }

    pub fn with_create_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("create_time", value));
        self
    }

    pub fn with_create_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("create_time", value));
        self
    }

    pub fn with_create_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("create_time", lower, upper));
        self
    }

    pub fn with_create_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "create_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_create_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("create_time"));
        self
    }



    pub fn with_create_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("create_time"));
        self
    }


    pub fn order_by_create_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("create_time");
        self
    }

    pub fn order_by_create_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("create_time");
        self
    }

    pub fn select_update_time(mut self) -> Self {
        self.query = self.query.project("update_time");
        self
    }

    pub fn project_update_time(self) -> Self {
        self.select_update_time()
    }

    pub fn select_update_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_update_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_update_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("update_time", raw_sql_segment));
        self
    }

    pub fn group_by_update_time(self) -> Self {
        self.group_by("update_time")
    }

    pub fn group_by_update_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("update_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("update_time"));
        request
    }

    pub fn group_by_update_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("update_time")
            .aggregate_with_function("update_time", alias, function)
    }

    pub fn count_update_time(self) -> Self {
        self.count_update_time_as("update_time_count")
    }

    pub fn count_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("update_time", alias)
    }

    pub fn sum_update_time(self) -> Self {
        self.sum_update_time_as("sum_update_time")
    }

    pub fn sum_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("update_time", alias)
    }

    pub fn avg_update_time(self) -> Self {
        self.avg_update_time_as("avg_update_time")
    }

    pub fn avg_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("update_time", alias)
    }

    pub fn min_update_time(self) -> Self {
        self.min_update_time_as("min_update_time")
    }

    pub fn min_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("update_time", alias)
    }

    pub fn max_update_time(self) -> Self {
        self.max_update_time_as("max_update_time")
    }

    pub fn max_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("update_time", alias)
    }

    pub fn unselect_update_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "update_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "update_time");
        self
    }


    pub fn with_update_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_update_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_update_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("update_time", value));
        self
    }



    pub fn with_update_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("update_time", value));
        self
    }

    pub fn with_update_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("update_time", value));
        self
    }

    pub fn with_update_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("update_time", value));
        self
    }

    pub fn with_update_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("update_time", lower, upper));
        self
    }

    pub fn with_update_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "update_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_update_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("update_time"));
        self
    }



    pub fn with_update_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("update_time"));
        self
    }


    pub fn order_by_update_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("update_time");
        self
    }

    pub fn order_by_update_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("update_time");
        self
    }

    pub fn select_version(mut self) -> Self {
        self.query = self.query.project("version");
        self
    }

    pub fn project_version(self) -> Self {
        self.select_version()
    }

    pub fn select_version_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_version_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_version_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("version", raw_sql_segment));
        self
    }

    pub fn group_by_version(self) -> Self {
        self.group_by("version")
    }

    pub fn group_by_version_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("version");
        request.query = request
            .query
            .project_expr(alias, Expr::column("version"));
        request
    }

    pub fn group_by_version_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("version")
            .aggregate_with_function("version", alias, function)
    }

    pub fn count_version(self) -> Self {
        self.count_version_as("version_count")
    }

    pub fn count_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("version", alias)
    }

    pub fn sum_version(self) -> Self {
        self.sum_version_as("sum_version")
    }

    pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("version", alias)
    }

    pub fn avg_version(self) -> Self {
        self.avg_version_as("avg_version")
    }

    pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("version", alias)
    }

    pub fn min_version(self) -> Self {
        self.min_version_as("min_version")
    }

    pub fn min_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("version", alias)
    }

    pub fn max_version(self) -> Self {
        self.max_version_as("max_version")
    }

    pub fn max_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("version", alias)
    }

    pub fn unselect_version(mut self) -> Self {
        self.query.projection.retain(|field| field != "version");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "version");
        self
    }

    pub fn order_by_version_asc(mut self) -> Self {
        self.query = self.query.order_asc("version");
        self
    }

    pub fn order_by_version_desc(mut self) -> Self {
        self.query = self.query.order_desc("version");
        self
    }

    pub fn order_by_version_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("version");
        self
    }

    pub fn order_by_version_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("version");
        self
    }
    pub fn filter_by_tournament_team(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("tournament_team_id", value.entity_id_value()));
        self
    }

    pub fn with_tournament_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "tournament_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_team", selection));
        self
    }


    pub fn without_tournament_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "tournament_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_team", selection));
        self
    }


    pub fn have_tournament_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tournament_team_id"));
        self
    }

    pub fn have_no_tournament_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tournament_team_id"));
        self
    }


    pub fn group_by_tournament_team(self) -> Self {
        self.group_by("tournament_team_id")
    }

    pub fn group_by_tournament_team_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tournament_team_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tournament_team_id"));
        request
    }

    pub fn group_by_tournament_team_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tournament_team_id")
            .aggregate_with_function("tournament_team_id", alias, function)
    }

    pub fn group_by_tournament_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("tournament_team_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "tournament_team",
            "tournament_team_id",
            request,
        ));
        self
    }

    pub fn group_by_tournament_team_with_details(self) -> Self {
        self.group_by_tournament_team_with_details_from(crate::Q::tournament_teams().unlimited())
    }

    pub fn group_by_tournament_team_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_tournament_team_with(request)
    }


    pub fn roll_up_to_tournament_team(self) -> Self {
        self.roll_up_to_tournament_team_with(crate::Q::tournament_teams().unlimited())
    }

    pub fn roll_up_to_tournament_team_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_tournament_team_matching(selection.clone())
            .group_by_tournament_team_with(selection)
    }

    pub fn count_tournament_team(self) -> Self {
        self.count_tournament_team_as("tournament_team_count")
    }

    pub fn count_tournament_team_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tournament_team_id", alias)
    }

    pub fn unselect_tournament_team(mut self) -> Self {
        self.query.projection.retain(|field| field != "tournament_team_id");
        self.query.relations.retain(|relation| relation.name != "tournament_team");
        self
    }


    pub fn filter_by_match_group(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("match_group_id", value.entity_id_value()));
        self
    }

    pub fn with_match_group_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "match_group_id",
            <crate::MatchGroup as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_group", selection));
        self
    }


    pub fn without_match_group_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "match_group_id",
            <crate::MatchGroup as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_group", selection));
        self
    }


    pub fn have_match_group(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("match_group_id"));
        self
    }

    pub fn have_no_match_group(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("match_group_id"));
        self
    }


    pub fn group_by_match_group(self) -> Self {
        self.group_by("match_group_id")
    }

    pub fn group_by_match_group_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("match_group_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("match_group_id"));
        request
    }

    pub fn group_by_match_group_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("match_group_id")
            .aggregate_with_function("match_group_id", alias, function)
    }

    pub fn group_by_match_group_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("match_group_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "match_group",
            "match_group_id",
            request,
        ));
        self
    }

    pub fn group_by_match_group_with_details(self) -> Self {
        self.group_by_match_group_with_details_from(crate::Q::match_groups().unlimited())
    }

    pub fn group_by_match_group_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_match_group_with(request)
    }


    pub fn roll_up_to_match_group(self) -> Self {
        self.roll_up_to_match_group_with(crate::Q::match_groups().unlimited())
    }

    pub fn roll_up_to_match_group_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_match_group_matching(selection.clone())
            .group_by_match_group_with(selection)
    }

    pub fn count_match_group(self) -> Self {
        self.count_match_group_as("match_group_count")
    }

    pub fn count_match_group_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("match_group_id", alias)
    }

    pub fn unselect_match_group(mut self) -> Self {
        self.query.projection.retain(|field| field != "match_group_id");
        self.query.relations.retain(|relation| relation.name != "match_group");
        self
    }


    pub fn filter_by_tournament(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("tournament_id", value.entity_id_value()));
        self
    }

    pub fn with_tournament_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "tournament_id",
            <crate::Tournament as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament", selection));
        self
    }


    pub fn without_tournament_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "tournament_id",
            <crate::Tournament as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament", selection));
        self
    }


    pub fn have_tournament(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tournament_id"));
        self
    }

    pub fn have_no_tournament(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tournament_id"));
        self
    }


    pub fn group_by_tournament(self) -> Self {
        self.group_by("tournament_id")
    }

    pub fn group_by_tournament_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tournament_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tournament_id"));
        request
    }

    pub fn group_by_tournament_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tournament_id")
            .aggregate_with_function("tournament_id", alias, function)
    }

    pub fn group_by_tournament_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("tournament_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "tournament",
            "tournament_id",
            request,
        ));
        self
    }

    pub fn group_by_tournament_with_details(self) -> Self {
        self.group_by_tournament_with_details_from(crate::Q::tournaments().unlimited())
    }

    pub fn group_by_tournament_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_tournament_with(request)
    }


    pub fn roll_up_to_tournament(self) -> Self {
        self.roll_up_to_tournament_with(crate::Q::tournaments().unlimited())
    }

    pub fn roll_up_to_tournament_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_tournament_matching(selection.clone())
            .group_by_tournament_with(selection)
    }

    pub fn count_tournament(self) -> Self {
        self.count_tournament_as("tournament_count")
    }

    pub fn count_tournament_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tournament_id", alias)
    }

    pub fn unselect_tournament(mut self) -> Self {
        self.query.projection.retain(|field| field != "tournament_id");
        self.query.relations.retain(|relation| relation.name != "tournament");
        self
    }
    pub fn select_tournament_team(mut self) -> Self {
        self.query = self.query.relation("tournament_team");
        self
    }

    pub fn select_tournament_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament_team", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament_team", selection));
        self
}

    pub fn facet_by_tournament_team_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_tournament_team_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_tournament_team_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "tournament_team",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_match_group(mut self) -> Self {
        self.query = self.query.relation("match_group");
        self
    }

    pub fn select_match_group_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("match_group", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("match_group", selection));
        self
}

    pub fn facet_by_match_group_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_match_group_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_match_group_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "match_group",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_tournament(mut self) -> Self {
        self.query = self.query.relation("tournament");
        self
    }

    pub fn select_tournament_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament", selection));
        self
}

    pub fn facet_by_tournament_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_tournament_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_tournament_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "tournament",
            request,
            include_all_facets,
        ));
        self
    }
}

impl<R> Default for GroupStandingRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< GroupStandingRequest<R> > for SelectQuery {
    fn from(request: GroupStandingRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< GroupStandingRequest<R> > for QuerySelection {
    fn from(request: GroupStandingRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::GroupStanding> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlRepositoryError<C::GroupStandingRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<GroupStandingRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::GroupStanding
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::GroupStanding::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> GroupStandingRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode {
            entity_type: self.inner.query.entity.clone(),
            entity_id: None,
            comment: self.purpose,
        });
        self.inner
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlRepositoryError<C::GroupStandingRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
